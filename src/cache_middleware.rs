#![cfg_attr(docsrs, doc(cfg(feature = "sync")))]
#![warn(missing_docs)]

use ureq::{Error, Middleware, MiddlewareNext, Request, Response};

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
    fn handle(&self, request: Request, next: MiddlewareNext) -> Result<Response, Error> {
        // Images have to bypass this middleware entirely because you cannot
        // properly encode/decode as UTF-8. Caching is handled in the client's
        // get_image() function instead.
        //
        // Also, we don't want to cache the version list.
        if request.url().ends_with(".png") || request.url().ends_with("/api/versions.json") {
            return next.handle(request);
        }

        let cache_key = request.url().to_owned();

        if let Ok(data) = cacache_sync::read(&self.directory, &cache_key) {
            return Response::new(200, "OK", &String::from_utf8_lossy(&data));
        }

        let response = next.handle(request)?;
        if response.status() != 200 {
            return Ok(response);
        }

        let response_str = response.into_string()?;
        let _ = cacache_sync::write(&self.directory, &cache_key, response_str.clone());

        Response::new(200, "OK", &response_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{self, mock};
    use std::{env::temp_dir, fs::remove_dir_all};
    use ureq::AgentBuilder;

    #[test]
    fn first_request_creates_cache() {
        let _m = mock("GET", "/file.txt").with_status(200).with_body("some example text").create();

        let full_url = format!("{}/file.txt", mockito::server_url());

        let cache_dir = temp_dir().join("test01");
        let _ = remove_dir_all(&cache_dir);

        let agent = AgentBuilder::new()
            .middleware(CacheMiddleware::new(&cache_dir.to_string_lossy()))
            .build();

        let response = agent.get(&full_url).call().unwrap();
        assert_eq!(response.status(), 200);
        assert_eq!(response.into_string().unwrap(), "some example text");
        assert!(cache_dir.read_dir().unwrap().next().is_some());
    }

    #[test]
    fn second_request_reads_cache() {
        let full_url = format!("{}/file.txt", mockito::server_url());

        let cache_dir = temp_dir().join("test02");
        let _ = remove_dir_all(&cache_dir);

        let agent = AgentBuilder::new()
            .middleware(CacheMiddleware::new(&cache_dir.to_string_lossy()))
            .build();

        let _m = mock("GET", "/file.txt").with_status(200).with_body("some example text").create();

        let _ = agent.get(&full_url).call().unwrap();

        // Removes the previous mock so this URL shouldn't work.
        mockito::reset();
        assert!(agent.get(&format!("{}/other-file.txt", mockito::server_url())).call().is_err());

        let response = agent.get(&full_url).call().unwrap();
        assert_eq!(response.status(), 200);
        assert_eq!(response.into_string().unwrap(), "some example text");
    }
}
