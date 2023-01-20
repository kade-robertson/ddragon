use ureq::{Error, Middleware, MiddlewareNext, Request, Response};

pub struct CacheMiddleware {
    directory: String,
}

impl CacheMiddleware {
    pub fn new(directory: &str) -> Self {
        Self {
            directory: directory.to_owned(),
        }
    }
}

impl Middleware for CacheMiddleware {
    fn handle(&self, request: Request, next: MiddlewareNext) -> Result<Response, Error> {
        let cache_key = request.url().to_owned();

        if let Ok(data) = cacache::read_sync(&self.directory, &cache_key) {
            return Response::new(200, "OK", &String::from_utf8_lossy(&data));
        }

        let response = next.handle(request)?;
        if response.status() != 200 {
            return Ok(response);
        }

        let response_str = response.into_string()?;
        let _ = cacache::write_sync(&self.directory, &cache_key, response_str.clone());

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
        let _m = mock("GET", "/file.txt")
            .with_status(200)
            .with_body("some example text")
            .create();

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

        let _m = mock("GET", "/file.txt")
            .with_status(200)
            .with_body("some example text")
            .create();

        let _ = agent.get(&full_url).call().unwrap();

        // Removes the previous mock so this URL shouldn't work.
        mockito::reset();
        assert!(agent
            .get(&format!("{}/other-file.txt", mockito::server_url()))
            .call()
            .is_err());

        let response = agent.get(&full_url).call().unwrap();
        assert_eq!(response.status(), 200);
        assert_eq!(response.into_string().unwrap(), "some example text");
    }
}