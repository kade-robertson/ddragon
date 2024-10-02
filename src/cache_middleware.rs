#![cfg_attr(docsrs, doc(cfg(feature = "sync")))]
#![warn(missing_docs)]

use ureq::http::{Request, Response};
use ureq::middleware::{Middleware, MiddlewareNext};
use ureq::{Body, Error, SendBody};

/// Handles caching responses locally.
pub struct CacheMiddleware {
    directory: String,
}

impl CacheMiddleware {
    /// Creates a new middleware, with the directory you would like cached
    /// files to go in specified. Cache file structure beyond that is dictated
    /// by `cacache`.
    pub fn new(directory: &str) -> Self {
        Self { directory: directory.to_owned() }
    }
}

impl Middleware for CacheMiddleware {
    fn handle(
        &self,
        request: Request<SendBody>,
        next: MiddlewareNext,
    ) -> Result<Response<Body>, Error> {
        // Images have to bypass this middleware entirely because you cannot
        // properly encode/decode as UTF-8. Caching is handled in the client's
        // get_image() function instead.
        //
        // Also, we don't want to cache the version list.
        if request.uri().path().ends_with(".png")
            || request.uri().path().ends_with("/api/versions.json")
        {
            return next.handle(request);
        }

        let cache_key = request.uri().to_string();
        if let Ok(data) = cacache::read_sync(&self.directory, &cache_key) {
            // error here since I can't convert http::Body to ureq::Body.
            return Response::builder().status(200).body(data);
        }

        let response = next.handle(request)?;
        if response.status() != 200 {
            return Ok(response);
        }

        // This also fails because I consume the response, so I need to make a new
        // one.
        let response_str = response.into_body().read_to_string()?;
        let _ = cacache::write_sync(&self.directory, &cache_key, response_str.clone());
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use std::{env::temp_dir, fs::remove_dir_all, path::PathBuf};
    use ureq::{middleware::MiddlewareChain, Agent, Config};

    fn build_agent(cache_dir: &PathBuf) -> Agent {
        let mut middleware = MiddlewareChain::default();
        middleware.add(CacheMiddleware::new(&cache_dir.to_string_lossy()));

        Agent::new_with_config(Config { middleware, ..Config::default() })
    }

    #[test]
    fn first_request_creates_cache() {
        let mut server = Server::new();
        let _m = server
            .mock("GET", "/file.txt")
            .with_status(200)
            .with_body("some example text")
            .create();

        let full_url = format!("{}/file.txt", server.url());

        let cache_dir = temp_dir().join("test01");
        let _ = remove_dir_all(&cache_dir);

        let agent = build_agent(&cache_dir);

        let response = agent.get(&full_url).call().unwrap();
        assert_eq!(response.status(), 200);
        assert_eq!(response.into_body().read_to_string().unwrap(), "some example text");
        assert!(cache_dir.read_dir().unwrap().next().is_some());
    }

    #[test]
    fn second_request_reads_cache() {
        let mut server = Server::new();
        let full_url = format!("{}/file.txt", server.url());

        let cache_dir = temp_dir().join("test02");
        let _ = remove_dir_all(&cache_dir);

        let agent = build_agent(&cache_dir);

        {
            let _m = server
                .mock("GET", "/file.txt")
                .with_status(200)
                .with_body("some example text")
                .create();

            let _ = agent.get(&full_url).call().unwrap();
        }

        assert!(agent.get(&format!("{}/other-file.txt", server.url())).call().is_err());

        let response = agent.get(&full_url).call().unwrap();
        assert_eq!(response.status(), 200);
        assert_eq!(response.into_body().read_to_string().unwrap(), "some example text");
    }
}
