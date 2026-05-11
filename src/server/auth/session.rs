use axum::http::{HeaderMap, header};

use crate::server::error::ApiError;
use crate::server::storage::AppState;

pub fn require_user_id(headers: &HeaderMap, state: &AppState) -> Result<i64, ApiError> {
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or_else(|| ApiError::unauthorized("Missing bearer token"))?;

    state.jwt.user_id(token)
}
