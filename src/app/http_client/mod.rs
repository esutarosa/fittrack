mod error;

use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;

use anyhow::{Context, Result};
pub use error::HttpClientError;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::shared::contracts::ApiErrorResponse;
use crate::shared::utils::{map_display_error, parse_http_response};

#[derive(Clone)]
pub struct HttpClient {
    host: String,
    port: u16,
    base_path: String,
}

impl HttpClient {
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
            _ => (authority.to_owned(), 80),
        };

        Ok(Self {
            host,
            port,
            base_path: format!("/{}", base_path.trim_matches('/')).trim_end_matches('/').to_owned(),
        })
    }

    pub fn get<Res>(&self, path: &str, token: Option<&str>) -> Result<Res, HttpClientError>
    where
        Res: DeserializeOwned,
    {
        self.send_without_body("GET", path, token)
    }

    pub fn post_without_auth<Req, Res>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<Res, HttpClientError>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        self.send("POST", path, Some(body), None)
    }

    pub fn post<Req, Res>(
        &self,
        path: &str,
        body: &Req,
        token: Option<&str>,
    ) -> Result<Res, HttpClientError>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        self.send("POST", path, Some(body), token)
    }

    pub fn patch<Req, Res>(
        &self,
        path: &str,
        body: &Req,
        token: Option<&str>,
    ) -> Result<Res, HttpClientError>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        self.send("PATCH", path, Some(body), token)
    }

    pub fn delete<Res>(&self, path: &str, token: Option<&str>) -> Result<Res, HttpClientError>
    where
        Res: DeserializeOwned,
    {
        self.send_without_body("DELETE", path, token)
    }

    fn send<Req, Res>(
        &self,
        method: &str,
        path: &str,
        body: Option<&Req>,
        token: Option<&str>,
    ) -> Result<Res, HttpClientError>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        let request_body = body
            .map(|body| {
                map_display_error(serde_json::to_vec(body), HttpClientError::InvalidResponse)
            })
            .transpose()?
            .unwrap_or_default();
        let mut stream = open_stream(&self.host, self.port)?;
        let auth_header = auth_header(token);
        let content_type =
            if request_body.is_empty() { "" } else { "Content-Type: application/json\r\n" };
        let request = format!(
            "{method} {} HTTP/1.1\r\nHost: {}\r\nAccept: application/json\r\n{}{}Connection: close\r\nContent-Length: {}\r\n\r\n",
            self.request_path(path),
            self.host,
            auth_header,
            content_type,
            request_body.len()
        );

        write_request(&mut stream, request.as_bytes())?;
        if !request_body.is_empty() {
            write_request(&mut stream, &request_body)?;
        }
        read_response(stream, self)
    }

    fn send_without_body<Res>(
        &self,
        method: &str,
        path: &str,
        token: Option<&str>,
    ) -> Result<Res, HttpClientError>
    where
        Res: DeserializeOwned,
    {
        let mut stream = open_stream(&self.host, self.port)?;
        let request = format!(
            "{method} {} HTTP/1.1\r\nHost: {}\r\nAccept: application/json\r\n{}Connection: close\r\nContent-Length: 0\r\n\r\n",
            self.request_path(path),
            self.host,
            auth_header(token),
        );
        write_request(&mut stream, request.as_bytes())?;
        read_response(stream, self)
    }

    fn request_path(&self, path: &str) -> String {
        if self.base_path.is_empty() {
            path.to_owned()
        } else {
            format!("{}{}", self.base_path, path)
        }
    }

    fn parse_response<Res>(&self, response: &[u8]) -> Result<Res, HttpClientError>
    where
        Res: DeserializeOwned,
    {
        let parsed =
            map_display_error(parse_http_response(response), HttpClientError::InvalidResponse)?;
        if (200..300).contains(&parsed.status) {
            map_display_error(
                serde_json::from_slice::<Res>(&parsed.body),
                HttpClientError::InvalidResponse,
            )
        } else {
            let message = serde_json::from_slice::<ApiErrorResponse>(&parsed.body)
                .map(|error| error.error)
                .unwrap_or_else(|_| format!("HTTP {}", parsed.status));
            Err(HttpClientError::Server(message))
        }
    }
}

fn auth_header(token: Option<&str>) -> String {
    token.map(|token| format!("Authorization: Bearer {token}\r\n")).unwrap_or_default()
}

fn open_stream(host: &str, port: u16) -> Result<TcpStream, HttpClientError> {
    map_display_error(TcpStream::connect((host, port)), HttpClientError::Network)
}

fn write_request(stream: &mut TcpStream, payload: &[u8]) -> Result<(), HttpClientError> {
    map_display_error(stream.write_all(payload), HttpClientError::Network)
}

fn read_response<Res>(mut stream: TcpStream, client: &HttpClient) -> Result<Res, HttpClientError>
where
    Res: DeserializeOwned,
{
    let mut response = Vec::new();
    map_display_error(stream.read_to_end(&mut response), HttpClientError::Network)?;
    client.parse_response(&response)
}
