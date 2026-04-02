pub struct ParsedUrl {
    pub host: String,
    pub port: String,
    pub path: String,
}

pub fn parse_url(url: &str) -> Result<ParsedUrl, String> {
    let reminder = url
        .strip_prefix("http://")
        .ok_or("Only http is supported")?;
}
