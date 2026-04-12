use crate::{error::XsusError, request::Request, utils::parse_url};
use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

pub fn execute_network_call(req: &Request, timeout: Duration) -> Result<String, XsusError> {
    let url_info = parse_url(&req.url).map_err(XsusError::InvalidUrl)?;
    let addr = format!("{}:{}", url_info.host, url_info.port);

    let socket_addrs = addr
        .to_socket_addrs()
        .map_err(|_| XsusError::Network(format!("DNS failure: {}", url_info.host)))?;

    let mut stream = None;
    for sa in socket_addrs {
        if let Ok(s) = TcpStream::connect_timeout(&sa, timeout) {
            stream = Some(s);
            break;
        }
    }

    let mut stream = stream.ok_or(XsusError::Network("Connection failed".into()))?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;

    let raw_http = req.to_http_string(&url_info.path, &url_info.host);
    stream.write_all(raw_http.as_bytes())?;
    stream.flush()?;

    let mut response_text = String::new();
    stream.read_to_string(&mut response_text)?;
    Ok(response_text)
}
