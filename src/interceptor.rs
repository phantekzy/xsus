use crate::{request::Request, response::Response};

pub type RequestInterceptor = Box<dyn Fn(Request) -> Request>;
pub type ResponseInterceptor = Box<dyn Fn(Response) -> Response>;

pub struct Interceptors {
    pub request: Vec<RequestInterceptor>,
    pub response: Vec<ResponseInterceptor>,
}

impl Interceptors {
    pub fn new() -> Self {
        Self {
            request: Vec::new(),
            response: Vec::new(),
        }
    }
}
