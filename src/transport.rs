use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

use crate::{error::XsusError, request::Request, utils::parse_url};

pub fn execute_network_call(req: &Request, timeout: Duration) -> Result<String, XsusError> {
    let url_info = parse_url(&req.url).map_err(XsusError::InvalidUrl)?;
    let addr = format!("{}:{}", url_info.host, url_info.port);
    let mut stream = TcpStream::connect_timeout(
        &addr
            .parse()
            .map_err(|_| XsusError::Network("Could not resolve host".into()))?,
        timeout,
    )?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;

    let raw_http = req.to_http_string(&url_info.path, &url_info.host);
    stream.write_all(raw_http.as_bytes())?;

    let mut response_text = String::new();
    stream.read_to_string(&mut response_text)?;

    Ok(response_text)
}
