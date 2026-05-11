use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;

use crate::server::auth::{
    hash_password, normalize_username, require_user_id, validate_password, verify_password,
};
use crate::server::error::ApiError;
use crate::server::storage::{AppState, repo};
use crate::shared::contracts::{AuthResponse, LoginRequest, RegisterRequest};

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let username = normalize_username(&request.username)?;
    validate_password(&request.password, Some(request.confirm_password.as_str()))?;

    if repo::find_user_by_username(&state.pool, username).await?.is_some() {
        return Err(ApiError::conflict("Username is already taken"));
    }

    let password_hash = hash_password(&request.password)?;
    let user = repo::create_user(&state.pool, username, &password_hash).await?;
    let token = state.jwt.issue(user.id)?;

    Ok(Json(AuthResponse { token, user: user.to_dto() }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let username = normalize_username(&request.username)?;
    let user = repo::find_user_by_username(&state.pool, username)
        .await?
        .ok_or_else(|| ApiError::unauthorized("Invalid username or password"))?;

    verify_password(&user.password_hash, &request.password)?;

    let token = state.jwt.issue(user.id)?;
    Ok(Json(AuthResponse { token, user: user.to_dto() }))
}

pub async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<crate::shared::contracts::UserDto>, ApiError> {
    let user_id = require_user_id(&headers, &state)?;
    let user = repo::find_user_by_id(&state.pool, user_id)
        .await?
        .ok_or_else(|| ApiError::unauthorized("Invalid session"))?;

    Ok(Json(user.to_dto()))
}
