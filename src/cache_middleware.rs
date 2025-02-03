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
        // We always want an up-to-date version list.
        if request.uri().path().ends_with("/api/versions.json") {
            return next.handle(request);
        }

        let is_image = request.uri().path().ends_with(".png");
        let cache_key = request.uri().to_string();
        if let Ok(data) = cacache::read_sync(&self.directory, &cache_key) {
            let data_type = if is_image { "image/png" } else { "application/json" };
            return Ok(Response::builder()
                .header("Content-Type", data_type)
                .header("Content-Length", data.len())
                .status(200)
                .body(Body::builder().mime_type(data_type).data(data))?);
        }

        let mut response = next.handle(request)?;
        if response.status() != 200 {
            return Ok(response);
        }

        let body_mut = response.body_mut();
        if let Ok(body) = body_mut.read_to_vec() {
            let _ = cacache::write_sync(&self.directory, cache_key, body.clone());
            let mut body_builder = Body::builder();
            let mut reponse_builder = Response::builder();
            if let Some(mime_type) = body_mut.mime_type() {
                body_builder = body_builder.mime_type(mime_type);
                reponse_builder = reponse_builder.header("Content-Type", mime_type);
            }
            return Ok(reponse_builder
                .header("Content-Length", body.len())
                .status(200)
                .body(body_builder.data(body))?);
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use std::{env::temp_dir, fs::remove_dir_all, path::Path};
    use ureq::Agent;

    fn build_agent(cache_dir: &Path) -> Agent {
        Agent::config_builder()
            .middleware(CacheMiddleware::new(&cache_dir.to_string_lossy()))
            .build()
            .into()
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
