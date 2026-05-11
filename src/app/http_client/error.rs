use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum HttpClientError {
    Network(String),
    Server(String),
    InvalidResponse(String),
}

impl Display for HttpClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network(message) => f.write_str(message),
            Self::Server(message) => f.write_str(message),
            Self::InvalidResponse(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for HttpClientError {}
