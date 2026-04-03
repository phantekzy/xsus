use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Clone, Debug)]
pub struct Request {
    pub method: Method,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn new(method: Method, url: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), "Vivec".to_string());
        headers.insert("Connection".to_string(), "close".to_string());
        headers.insert("Accept".to_string(), "*/*".to_string());

        Self {
            method,
            url: url.to_string(),
            headers,
            body: None,
        }
    }
}
