use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use super::config::ServerConfig;
use crate::server::auth::JwtService;
use crate::server::error::ApiError;
use crate::shared::utils::map_display_error;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt: JwtService,
}

impl AppState {
    pub async fn new(config: &ServerConfig) -> Result<Self, ApiError> {
        let pool = map_display_error(
            PgPoolOptions::new().max_connections(5).connect(&config.database_url).await,
            ApiError::internal,
        )?;

        Ok(Self { pool, jwt: JwtService::new(&config.jwt_secret, config.jwt_ttl_seconds) })
    }
}
