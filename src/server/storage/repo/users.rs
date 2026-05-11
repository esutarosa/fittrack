use sqlx::PgPool;

use super::shared::{internal, internal_conflict};
use crate::server::error::ApiError;
use crate::server::storage::rows::UserRow;

pub async fn find_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<UserRow>, ApiError> {
    internal(
        sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, username, password_hash, created_at
            FROM users
            WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_optional(pool)
        .await,
    )
}

pub async fn find_user_by_id(pool: &PgPool, id: i64) -> Result<Option<UserRow>, ApiError> {
    internal(
        sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, username, password_hash, created_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await,
    )
}

pub async fn create_user(
    pool: &PgPool,
    username: &str,
    password_hash: &str,
) -> Result<UserRow, ApiError> {
    internal_conflict(
        sqlx::query_as::<_, UserRow>(
            r#"
            INSERT INTO users (username, password_hash)
            VALUES ($1, $2)
            RETURNING id, username, password_hash, created_at
            "#,
        )
        .bind(username)
        .bind(password_hash)
        .fetch_one(pool)
        .await,
    )
}
