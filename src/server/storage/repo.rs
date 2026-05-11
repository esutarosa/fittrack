use chrono::NaiveDate;
use sqlx::PgPool;

use crate::server::error::ApiError;
use crate::server::storage::rows::{ExerciseRow, UserRow, WorkoutRow, WorkoutSetRow};
use crate::shared::utils::map_display_error;

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
            ORDER BY name ASC
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await,
    )
}

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

pub async fn create_workout_set(
    pool: &PgPool,
    workout_id: i64,
    exercise_id: i64,
    set_order: i32,
    weight: f64,
    repetitions: i32,
) -> Result<WorkoutSetRow, ApiError> {
    internal_conflict(
        sqlx::query_as::<_, WorkoutSetRow>(
            r#"
        INSERT INTO workout_sets (workout_id, exercise_id, set_order, weight, repetitions)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, workout_id, exercise_id, set_order, weight, repetitions, created_at
        "#,
        )
        .bind(workout_id)
        .bind(exercise_id)
        .bind(set_order)
        .bind(weight)
        .bind(repetitions)
        .fetch_one(pool)
        .await,
    )
}

pub async fn list_workout_sets(
    pool: &PgPool,
    workout_id: i64,
) -> Result<Vec<WorkoutSetRow>, ApiError> {
    internal(
        sqlx::query_as::<_, WorkoutSetRow>(
            r#"
            SELECT id, workout_id, exercise_id, set_order, weight, repetitions, created_at
            FROM workout_sets
            WHERE workout_id = $1
            ORDER BY set_order ASC, id ASC
            "#,
        )
        .bind(workout_id)
        .fetch_all(pool)
        .await,
    )
}

fn internal<T, E>(result: Result<T, E>) -> Result<T, ApiError>
where
    E: std::fmt::Display,
{
    map_display_error(result, ApiError::internal)
}

fn internal_conflict<T>(result: Result<T, sqlx::Error>) -> Result<T, ApiError> {
    result.map_err(|error| {
        if let Some(message) = database_conflict(&error) {
            return ApiError::conflict(message);
        }

        ApiError::internal(error.to_string())
    })
}

fn database_conflict(error: &sqlx::Error) -> Option<&'static str> {
    match error {
        sqlx::Error::Database(database_error) if database_error.constraint().is_some() => {
            Some("A record with the same key already exists")
        }
        _ => None,
    }
}
