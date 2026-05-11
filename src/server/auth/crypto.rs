use argon2::Argon2;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use rand::rngs::OsRng;

use crate::server::error::ApiError;
use crate::shared::utils::map_display_error;

pub fn normalize_username(username: &str) -> Result<&str, ApiError> {
    let username = username.trim();

    if username.is_empty() {
        return Err(ApiError::bad_request("Username is required"));
    }

    Ok(username)
}

pub fn validate_password(password: &str, confirm_password: Option<&str>) -> Result<(), ApiError> {
    if password.trim().is_empty() {
        return Err(ApiError::bad_request("Password is required"));
    }

    if password.len() < 8 {
        return Err(ApiError::bad_request("Password must be at least 8 characters"));
    }

    if let Some(confirm_password) = confirm_password
        && password != confirm_password
    {
        return Err(ApiError::bad_request("Passwords do not match"));
    }

    Ok(())
}

pub fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);

    map_display_error(
        Argon2::default().hash_password(password.as_bytes(), &salt),
        ApiError::internal,
    )
    .map(|hash| hash.to_string())
}

pub fn verify_password(password_hash: &str, password: &str) -> Result<(), ApiError> {
    let parsed_hash = map_display_error(PasswordHash::new(password_hash), ApiError::internal)?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError::unauthorized("Invalid username or password"))
}
