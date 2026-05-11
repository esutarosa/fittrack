use std::env;
use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::net::TcpStream;

use anyhow::{Context, Result};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::shared::contracts::{
    ApiErrorResponse, AuthResponse, LoginRequest, RegisterRequest, UserDto,
};
use crate::shared::utils::{map_display_error, parse_http_response};

#[derive(Clone)]
pub struct AuthClient {
    host: String,
    port: u16,
    base_path: String,
}

#[derive(Clone, Debug)]
pub struct Session {
    pub token: String,
    pub user: UserDto,
}

#[derive(Debug)]
pub enum AuthClientError {
    Network(String),
    Server(String),
    InvalidResponse(String),
}

impl Display for AuthClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network(message) => f.write_str(message),
            Self::Server(message) => f.write_str(message),
            Self::InvalidResponse(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for AuthClientError {}

impl Session {
    pub fn from_auth(auth: AuthResponse) -> Self {
        Self { token: auth.token, user: auth.user }
    }
}

impl AuthClient {
    pub fn from_env() -> Result<Self> {
        let base_url =
            env::var("FITTRACK_API_URL").unwrap_or_else(|_| "http://127.0.0.1:7272/api".to_owned());
        Self::from_base_url(&base_url)
    }

    fn from_base_url(base_url: &str) -> Result<Self> {
        let base_url = base_url.trim_end_matches('/');
        let scheme = base_url
            .strip_prefix("http://")
            .ok_or_else(|| anyhow::anyhow!("FITTRACK_API_URL must use http://"))?;
        let (authority, base_path) = scheme.split_once('/').unwrap_or((scheme, ""));
        let (host, port) = match authority.split_once(':') {
            Some((host, port)) => {
                (host.to_owned(), port.parse().context("invalid port in FITTRACK_API_URL")?)
            }
            None => (authority.to_owned(), 80),
        };

        Ok(Self {
            host,
            port,
            base_path: format!("/{}", base_path.trim_matches('/')).trim_end_matches('/').to_owned(),
        })
    }

    pub fn register(
        &self,
        username: &str,
        password: &str,
        confirm_password: &str,
    ) -> Result<AuthResponse, AuthClientError> {
        self.post(
            "/auth/register",
            &RegisterRequest {
                username: username.to_owned(),
                password: password.to_owned(),
                confirm_password: confirm_password.to_owned(),
            },
        )
    }

    pub fn login(&self, username: &str, password: &str) -> Result<AuthResponse, AuthClientError> {
        self.post(
            "/auth/login",
            &LoginRequest { username: username.to_owned(), password: password.to_owned() },
        )
    }

    fn post<Req, Res>(&self, path: &str, body: &Req) -> Result<Res, AuthClientError>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        let request_body =
            map_display_error(serde_json::to_vec(body), AuthClientError::InvalidResponse)?;
        let request_path = if self.base_path.is_empty() {
            path.to_owned()
        } else {
            format!("{}{}", self.base_path, path)
        };

        let mut stream = map_display_error(
            TcpStream::connect((self.host.as_str(), self.port)),
            AuthClientError::Network,
        )?;

        let request = format!(
            "POST {request_path} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nAccept: application/json\r\nConnection: close\r\nContent-Length: {}\r\n\r\n",
            self.host,
            request_body.len()
        );

        map_display_error(stream.write_all(request.as_bytes()), AuthClientError::Network)?;
        map_display_error(stream.write_all(&request_body), AuthClientError::Network)?;

        let mut response = Vec::new();
        map_display_error(stream.read_to_end(&mut response), AuthClientError::Network)?;

        let parsed =
            map_display_error(parse_http_response(&response), AuthClientError::InvalidResponse)?;

        if (200..300).contains(&parsed.status) {
            map_display_error(
                serde_json::from_slice::<Res>(&parsed.body),
                AuthClientError::InvalidResponse,
            )
        } else {
            let message = serde_json::from_slice::<ApiErrorResponse>(&parsed.body)
                .map(|error| error.error)
                .unwrap_or_else(|_| format!("HTTP {}", parsed.status));
            Err(AuthClientError::Server(message))
        }
    }
}
