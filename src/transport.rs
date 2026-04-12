use std::{net::TcpStream, time::Duration};

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
}
