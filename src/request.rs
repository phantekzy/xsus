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
        headers.insert("User-Agent".to_string(), "xsus".to_string());
        headers.insert("Connection".to_string(), "close".to_string());
        headers.insert("Accept".to_string(), "*/*".to_string());

        Self {
            method,
            url: url.to_string(),
            headers,
            body: None,
        }
    }

    pub fn to_http_string(&self, path: &str, host: &str) -> String {
        let method_str = match self.method {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
        };
        let mut raw = format!("{} {} HTTP/1.1\r\n", method_str, path);
        raw.push_str(&format!("Host: {}\r\n", host));

        for (k, v) in &self.headers {
            let key_low = k.to_lowercase();
        }
    }
}
