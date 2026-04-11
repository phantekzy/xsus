use std::time::Duration;

use crate::{
    error::XsusError,
    interceptor::Interceptors,
    request::{Method, Request},
    response::Response,
};

pub struct Xsus {
    pub base_url: String,
    pub timeout: Duration,
    pub interceptors: Interceptors,
}

impl Xsus {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            timeout: Duration::from_secs(10),
            interceptors: Interceptors::new(),
        }
    }
    pub fn get(&self, path: &str) -> Result<Response, XsusError> {
        let full_url = format!("{}{}", self.base_url, path);
        let mut req = Request::new(Method::GET, &full_url);

        for interceptor in self.interceptors.request {
            req = interceptor(req)
        }
    }
}
