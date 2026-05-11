use sqlx::PgPool;

use super::shared::{internal, internal_conflict};
use crate::server::error::ApiError;
use crate::server::storage::rows::ExerciseRow;

pub async fn create_exercise(
    pool: &PgPool,
    user_id: i64,
    name: &str,
    muscle_group: &str,
) -> Result<ExerciseRow, ApiError> {
    internal(
        sqlx::query_as::<_, ExerciseRow>(
            r#"
            INSERT INTO exercises (user_id, name, muscle_group)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, name, muscle_group, created_at
            "#,
        )
        .bind(user_id)
        .bind(name)
        .bind(muscle_group)
        .fetch_one(pool)
        .await,
    )
}

pub async fn list_exercises(pool: &PgPool, user_id: i64) -> Result<Vec<ExerciseRow>, ApiError> {
    internal(
        sqlx::query_as::<_, ExerciseRow>(
            r#"
            SELECT id, user_id, name, muscle_group, created_at
            FROM exercises
            WHERE user_id = $1
            ORDER BY created_at DESC, id DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await,
    )
}

pub async fn find_exercise(
    pool: &PgPool,
    user_id: i64,
    exercise_id: i64,
) -> Result<Option<ExerciseRow>, ApiError> {
    internal(
        sqlx::query_as::<_, ExerciseRow>(
            r#"
            SELECT id, user_id, name, muscle_group, created_at
            FROM exercises
            WHERE user_id = $1 AND id = $2
            "#,
        )
        .bind(user_id)
        .bind(exercise_id)
        .fetch_optional(pool)
        .await,
    )
}

pub async fn update_exercise(
    pool: &PgPool,
    user_id: i64,
    exercise_id: i64,
    name: &str,
    muscle_group: &str,
) -> Result<Option<ExerciseRow>, ApiError> {
    internal(
        sqlx::query_as::<_, ExerciseRow>(
            r#"
            UPDATE exercises
            SET name = $3, muscle_group = $4
            WHERE user_id = $1 AND id = $2
            RETURNING id, user_id, name, muscle_group, created_at
            "#,
        )
        .bind(user_id)
        .bind(exercise_id)
        .bind(name)
        .bind(muscle_group)
        .fetch_optional(pool)
        .await,
    )
}

pub async fn delete_exercise(
    pool: &PgPool,
    user_id: i64,
    exercise_id: i64,
) -> Result<Option<ExerciseRow>, ApiError> {
    internal_conflict(
        sqlx::query_as::<_, ExerciseRow>(
            r#"
            DELETE FROM exercises
            WHERE user_id = $1 AND id = $2
            RETURNING id, user_id, name, muscle_group, created_at
            "#,
        )
        .bind(user_id)
        .bind(exercise_id)
        .fetch_optional(pool)
        .await,
    )
}
