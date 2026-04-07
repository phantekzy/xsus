use std::collections::HashMap;

use crate::error::XsusError;

pub fn parse_response(raw: &str) -> Result<Result, XsusError> {
    let mut lines = raw.lines();

    let status_line = lines
        .next()
        .ok_or(XsusError::Parse("Empty response".into()))?;
    let status_code = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .ok_or(XsusError::Parse("Invalid status code".into()))?;

    let mut headers = HashMap::new();
    let mut body = String::new();
    let mut is_body = false;

    for line in lines {
        if line.is_empty() {
            is_body = true;
            continue;
        }
        if !is_body {
            if let Some((k, v)) = line.split_once(": ") {
                headers.insert(k.to_lowercase(), v.to_string());
            }
        } else {
            body.push_str(line);
            body.push('\n');
        }
    }
}
