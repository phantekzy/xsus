use xsus::Xsus;

fn main() {
    let mut client = Xsus::new("http://example.com");
    client.interceptors.request.push(Box::new(|mut req| {
        req.headers.insert(
            "X-Custom-Auth".to_string(),
            "Xsus-Security-Token".to_string(),
        );
        req
    }));
}
