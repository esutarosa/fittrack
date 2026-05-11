use chrono::NaiveDate;
use sqlx::PgPool;

use super::shared::internal;
use crate::server::error::ApiError;
use crate::server::storage::rows::WorkoutRow;

pub async fn create_workout(
    pool: &PgPool,
    user_id: i64,
    title: &str,
    workout_date: NaiveDate,
    notes: Option<&str>,
) -> Result<WorkoutRow, ApiError> {
    internal(
        sqlx::query_as::<_, WorkoutRow>(
            r#"
            INSERT INTO workouts (user_id, title, workout_date, notes)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, title, workout_date, notes, created_at
            "#,
        )
        .bind(user_id)
        .bind(title)
        .bind(workout_date)
        .bind(notes)
        .fetch_one(pool)
        .await,
    )
}

pub async fn list_workouts(pool: &PgPool, user_id: i64) -> Result<Vec<WorkoutRow>, ApiError> {
    internal(
        sqlx::query_as::<_, WorkoutRow>(
            r#"
            SELECT id, user_id, title, workout_date, notes, created_at
            FROM workouts
            WHERE user_id = $1
            ORDER BY workout_date DESC, id DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await,
    )
}

pub async fn find_workout(
    pool: &PgPool,
    user_id: i64,
    workout_id: i64,
) -> Result<Option<WorkoutRow>, ApiError> {
    internal(
        sqlx::query_as::<_, WorkoutRow>(
            r#"
            SELECT id, user_id, title, workout_date, notes, created_at
            FROM workouts
            WHERE user_id = $1 AND id = $2
            "#,
        )
        .bind(user_id)
        .bind(workout_id)
        .fetch_optional(pool)
        .await,
    )
}

pub async fn update_workout(
    pool: &PgPool,
    user_id: i64,
    workout_id: i64,
    title: &str,
    workout_date: NaiveDate,
    notes: Option<&str>,
) -> Result<Option<WorkoutRow>, ApiError> {
    internal(
        sqlx::query_as::<_, WorkoutRow>(
            r#"
            UPDATE workouts
            SET title = $3, workout_date = $4, notes = $5
            WHERE user_id = $1 AND id = $2
            RETURNING id, user_id, title, workout_date, notes, created_at
            "#,
        )
        .bind(user_id)
        .bind(workout_id)
        .bind(title)
        .bind(workout_date)
        .bind(notes)
        .fetch_optional(pool)
        .await,
    )
}

pub async fn delete_workout(
    pool: &PgPool,
    user_id: i64,
    workout_id: i64,
) -> Result<Option<WorkoutRow>, ApiError> {
    internal(
        sqlx::query_as::<_, WorkoutRow>(
            r#"
            DELETE FROM workouts
            WHERE user_id = $1 AND id = $2
            RETURNING id, user_id, title, workout_date, notes, created_at
            "#,
        )
        .bind(user_id)
        .bind(workout_id)
        .fetch_optional(pool)
        .await,
    )
}
