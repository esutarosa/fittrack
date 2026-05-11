use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct ParsedHttpResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum HttpParseError {
    MissingHeaders,
    MissingStatusLine,
    MissingHttpVersion,
    MissingStatus,
    InvalidUtf8(String),
    InvalidChunkedBody,
    TruncatedChunkedBody,
}

impl Display for HttpParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingHeaders => f.write_str("missing HTTP headers"),
            Self::MissingStatusLine => f.write_str("missing status line"),
            Self::MissingHttpVersion => f.write_str("missing HTTP version"),
            Self::MissingStatus => f.write_str("missing HTTP status"),
            Self::InvalidUtf8(message) => f.write_str(message),
            Self::InvalidChunkedBody => f.write_str("invalid chunked body"),
            Self::TruncatedChunkedBody => f.write_str("truncated chunked body"),
        }
    }
}

impl std::error::Error for HttpParseError {}

pub fn parse_http_response(response: &[u8]) -> Result<ParsedHttpResponse, HttpParseError> {
    let separator = b"\r\n\r\n";
    let headers_end = response
        .windows(separator.len())
        .position(|window| window == separator)
        .ok_or(HttpParseError::MissingHeaders)?;

    let headers = std::str::from_utf8(&response[..headers_end])
        .map_err(|error| HttpParseError::InvalidUtf8(error.to_string()))?;
    let status = parse_status_code(headers)?;
    let body = decode_body(headers, &response[headers_end + separator.len()..])?;

    Ok(ParsedHttpResponse { status, body })
}

fn parse_status_code(headers: &str) -> Result<u16, HttpParseError> {
    let status_line = headers.lines().next().ok_or(HttpParseError::MissingStatusLine)?;
    let mut parts = status_line.split_whitespace();
    let _http_version = parts.next().ok_or(HttpParseError::MissingHttpVersion)?;
    let status = parts
        .next()
        .ok_or(HttpParseError::MissingStatus)?
        .parse::<u16>()
        .map_err(|error| HttpParseError::InvalidUtf8(error.to_string()))?;
    Ok(status)
}

fn decode_body(headers: &str, body: &[u8]) -> Result<Vec<u8>, HttpParseError> {
    if headers
        .lines()
        .any(|line| line.to_ascii_lowercase().starts_with("transfer-encoding: chunked"))
    {
        return decode_chunked_body(body);
    }

    Ok(body.to_vec())
}

fn decode_chunked_body(mut body: &[u8]) -> Result<Vec<u8>, HttpParseError> {
    let mut output = Vec::new();

    loop {
        let size_end = body
            .windows(2)
            .position(|window| window == b"\r\n")
            .ok_or(HttpParseError::InvalidChunkedBody)?;
        let size_text = std::str::from_utf8(&body[..size_end])
            .map_err(|error| HttpParseError::InvalidUtf8(error.to_string()))?;
        let chunk_size = usize::from_str_radix(size_text.trim(), 16)
            .map_err(|error| HttpParseError::InvalidUtf8(error.to_string()))?;
        body = &body[size_end + 2..];

        if chunk_size == 0 {
            break;
        }

        if body.len() < chunk_size + 2 {
            return Err(HttpParseError::TruncatedChunkedBody);
        }

        output.extend_from_slice(&body[..chunk_size]);
        body = &body[chunk_size + 2..];
    }

    Ok(output)
}
