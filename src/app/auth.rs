use std::fmt::{Display, Formatter};

use argon2::{
    Argon2,
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
};
use rand_core::OsRng;

use super::{db::AppDatabase, models::User};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthError {
    EmptyUsername,
    EmptyPassword,
    PasswordTooShort,
    PasswordMismatch,
    UsernameTaken,
    InvalidCredentials,
    Database(String),
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyUsername => f.write_str("Username is required"),
            Self::EmptyPassword => f.write_str("Password is required"),
            Self::PasswordTooShort => f.write_str("Password must be at least 8 characters"),
            Self::PasswordMismatch => f.write_str("Passwords do not match"),
            Self::UsernameTaken => f.write_str("Username is already taken"),
            Self::InvalidCredentials => f.write_str("Invalid username or password"),
            Self::Database(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for AuthError {}

pub struct AuthService;

impl AuthService {
    pub fn register(
        db: &AppDatabase,
        username: &str,
        password: &str,
        confirm_password: &str,
    ) -> Result<User, AuthError> {
        let username = normalize_username(username)?;
        validate_password(password, confirm_password)?;

        if db
            .find_user_by_username(username)
            .map_err(database_error)?
            .is_some()
        {
            return Err(AuthError::UsernameTaken);
        }

        let password_hash = hash_password(password)?;
        let stored_user = db
            .create_user(username, &password_hash)
            .map_err(database_error)?;

        Ok(stored_user.into_user())
    }

    pub fn login(db: &AppDatabase, username: &str, password: &str) -> Result<User, AuthError> {
        let username = normalize_username(username)?;
        let stored_user = db
            .find_user_by_username(username)
            .map_err(database_error)?
            .ok_or(AuthError::InvalidCredentials)?;

        verify_password(&stored_user.password_hash, password)?;
        Ok(stored_user.into_user())
    }
}

fn normalize_username(username: &str) -> Result<&str, AuthError> {
    let username = username.trim();

    if username.is_empty() {
        return Err(AuthError::EmptyUsername);
    }

    Ok(username)
}

fn validate_password(password: &str, confirm_password: &str) -> Result<(), AuthError> {
    if password.is_empty() {
        return Err(AuthError::EmptyPassword);
    }

    if password.len() < 8 {
        return Err(AuthError::PasswordTooShort);
    }

    if password != confirm_password {
        return Err(AuthError::PasswordMismatch);
    }

    Ok(())
}

fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|err| AuthError::Database(err.to_string()))
}

fn verify_password(password_hash: &str, password: &str) -> Result<(), AuthError> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|err| AuthError::Database(err.to_string()))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AuthError::InvalidCredentials)
}

fn database_error(error: anyhow::Error) -> AuthError {
    AuthError::Database(error.to_string())
}
