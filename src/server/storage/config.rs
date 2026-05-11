use std::env;
use std::net::SocketAddr;

use anyhow::{Context, Result};

#[derive(Clone)]
pub struct ServerConfig {
    pub bind_addr: SocketAddr,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_ttl_seconds: i64,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self> {
        let bind_addr = env::var("FITTRACK_SERVER_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:7272".to_owned())
            .parse()
            .context("invalid FITTRACK_SERVER_ADDR")?;

        let database_url =
            env::var("DATABASE_URL").context("DATABASE_URL must be set for the server")?;
        let jwt_secret = env::var("JWT_SECRET").context("JWT_SECRET must be set for the server")?;
        let jwt_ttl_seconds = env::var("JWT_TTL_SECONDS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(7 * 24 * 60 * 60);

        Ok(Self { bind_addr, database_url, jwt_secret, jwt_ttl_seconds })
    }
}
