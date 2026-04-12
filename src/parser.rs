use crate::{error::XsusError, response::Response};
use std::collections::HashMap;

pub fn parse_response(raw: &str) -> Result<Response, XsusError> {
    let (headers_part, body_part) = if let Some(pos) = raw.find("\r\n\r\n") {
        (&raw[..pos], &raw[pos + 4..])
    } else if let Some(pos) = raw.find("\n\n") {
        (&raw[..pos], &raw[pos + 2..])
    } else {
        return Err(XsusError::Parse("Header/Body boundary missing".into()));
    };

    let mut header_lines = headers_part.lines();
    let status_line = header_lines
        .next()
        .ok_or(XsusError::Parse("Empty response".into()))?;
    let status_code = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .ok_or(XsusError::Parse("Invalid status code".into()))?;

    let mut headers = HashMap::new();
    for line in header_lines {
        if let Some((k, v)) = line.split_once(": ") {
            headers.insert(k.to_lowercase(), v.to_string());
        }
    }

    let is_chunked = headers
        .get("transfer-encoding")
        .map(|v| v.contains("chunked"))
        .unwrap_or(false);
    let final_body = if is_chunked {
        decode_chunks(body_part)?
    } else {
        body_part.to_string()
    };

    Ok(Response {
        status: status_code,
        headers,
        data: final_body.trim().to_string(),
    })
}

fn decode_chunks(body: &str) -> Result<String, XsusError> {
    let mut decoded = String::new();
    let mut remaining = body;

    while !remaining.is_empty() {
        let line_end = remaining
            .find("\r\n")
            .or_else(|| remaining.find('\n'))
            .ok_or(XsusError::Parse("Chunk format error".into()))?;

        let size_str = remaining[..line_end].trim();
        if size_str.is_empty() {
            remaining = &remaining[line_end + 1..];
            continue;
        }

        let size = u32::from_str_radix(size_str, 16)
            .map_err(|_| XsusError::Parse(format!("Hex fail: {}", size_str)))?;

        if size == 0 {
            break;
        }
        remaining = &remaining[line_end..].trim_start();

        if remaining.len() >= size as usize {
            let (chunk, rest) = remaining.split_at(size as usize);
            decoded.push_str(chunk);
            remaining = rest.trim_start();
        } else {
            decoded.push_str(remaining);
            break;
        }
    }
    Ok(decoded)
}
