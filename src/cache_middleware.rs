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
