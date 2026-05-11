use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::server::error::ApiError;
use crate::shared::utils::map_display_error;

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    ttl_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

impl JwtService {
    pub fn new(secret: &str, ttl_seconds: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            ttl_seconds,
        }
    }

    pub fn issue(&self, user_id: i64) -> Result<String, ApiError> {
        let issued_at =
            map_display_error(SystemTime::now().duration_since(UNIX_EPOCH), ApiError::internal)?;
        let expiry = issued_at + Duration::from_secs(self.ttl_seconds as u64);
        let claims = Claims {
            sub: user_id.to_string(),
            iat: issued_at.as_secs() as usize,
            exp: expiry.as_secs() as usize,
        };

        map_display_error(
            encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key),
            ApiError::internal,
        )
    }

    pub fn verify(&self, token: &str) -> Result<Claims, ApiError> {
        decode::<Claims>(token, &self.decoding_key, &Validation::new(Algorithm::HS256))
            .map(|data| data.claims)
            .map_err(|_| ApiError::unauthorized("Invalid or expired token"))
    }

    pub fn user_id(&self, token: &str) -> Result<i64, ApiError> {
        let claims = self.verify(token)?;
        claims.sub.parse::<i64>().map_err(|_| ApiError::unauthorized("Invalid token subject"))
    }
}
