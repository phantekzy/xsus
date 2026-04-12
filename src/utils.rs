pub struct ParsedUrl {
    pub scheme: String,
    pub host: String,
    pub port: String,
    pub path: String,
}

pub fn parse_url(url: &str) -> Result<ParsedUrl, String> {
    let (scheme, remainder) = if let Some(r) = url.strip_prefix("https://") {
        ("https", r)
    } else if let Some(r) = url.strip_prefix("http://") {
        ("http", r)
    } else {
        return Err("Unsupported protocol. Use http:// or https://".into());
    };

    let (host_port, path_query) = match remainder.find('/') {
        Some(pos) => (&remainder[..pos], &remainder[pos..]),
        None => (remainder, "/"),
    };

    let (host, port) = if let Some((h, p)) = host_port.split_once(':') {
        (h.to_string(), p.to_string())
    } else {
        let default_port = match scheme {
            "https" => "443",
            "http" => "80",
            _ => "80",
        };
        (host_port.to_string(), default_port.to_string())
    };

    Ok(ParsedUrl {
        scheme: scheme.to_string(),
        host,
        port,
        path: path_query.to_string(),
    })
}
