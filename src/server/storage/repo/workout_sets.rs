use sqlx::PgPool;

use super::shared::{fetch_workout_set, internal, internal_conflict, normalize_workout_set_order};
use crate::server::error::ApiError;
use crate::server::storage::rows::WorkoutSetRow;

pub async fn create_workout_set(
    pool: &PgPool,
    workout_id: i64,
    exercise_id: i64,
    set_order: i32,
    weight: f64,
    repetitions: i32,
) -> Result<WorkoutSetRow, ApiError> {
    let workout_set_id: i64 = internal_conflict(
        sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO workout_sets (workout_id, exercise_id, set_order, weight, repetitions)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
        )
        .bind(workout_id)
        .bind(exercise_id)
        .bind(set_order)
        .bind(weight)
        .bind(repetitions)
        .fetch_one(pool)
        .await,
    )?;

    fetch_workout_set(pool, workout_set_id).await
}

pub async fn find_workout_set(
    pool: &PgPool,
    workout_id: i64,
    workout_set_id: i64,
) -> Result<Option<WorkoutSetRow>, ApiError> {
    internal(
        sqlx::query_as::<_, WorkoutSetRow>(
            r#"
            SELECT
                ws.id,
                ws.workout_id,
                ws.exercise_id,
                e.name AS exercise_name,
                e.muscle_group,
                ws.set_order,
                ws.weight,
                ws.repetitions,
                ws.created_at
            FROM workout_sets ws
            JOIN exercises e ON e.id = ws.exercise_id
            WHERE ws.workout_id = $1 AND ws.id = $2
            "#,
        )
        .bind(workout_id)
        .bind(workout_set_id)
        .fetch_optional(pool)
        .await,
    )
}

pub async fn update_workout_set(
    pool: &PgPool,
    workout_id: i64,
    workout_set_id: i64,
    exercise_id: i64,
    set_order: i32,
    weight: f64,
    repetitions: i32,
) -> Result<Option<WorkoutSetRow>, ApiError> {
    let updated = internal_conflict(
        sqlx::query_scalar::<_, i64>(
            r#"
            UPDATE workout_sets
            SET exercise_id = $3, set_order = $4, weight = $5, repetitions = $6
            WHERE workout_id = $1 AND id = $2
            RETURNING id
            "#,
        )
        .bind(workout_id)
        .bind(workout_set_id)
        .bind(exercise_id)
        .bind(set_order)
        .bind(weight)
        .bind(repetitions)
        .fetch_optional(pool)
        .await,
    )?;

    if let Some(workout_set_id) = updated {
        fetch_workout_set(pool, workout_set_id).await.map(Some)
    } else {
        Ok(None)
    }
}

pub async fn delete_workout_set(
    pool: &PgPool,
    workout_id: i64,
    workout_set_id: i64,
) -> Result<Option<WorkoutSetRow>, ApiError> {
    let deleted = internal(
        sqlx::query_as::<_, WorkoutSetRow>(
            r#"
            DELETE FROM workout_sets ws
            USING exercises e
            WHERE ws.exercise_id = e.id
              AND ws.workout_id = $1
              AND ws.id = $2
            RETURNING
                ws.id,
                ws.workout_id,
                ws.exercise_id,
                e.name AS exercise_name,
                e.muscle_group,
                ws.set_order,
                ws.weight,
                ws.repetitions,
                ws.created_at
            "#,
        )
        .bind(workout_id)
        .bind(workout_set_id)
        .fetch_optional(pool)
        .await,
    )?;

    if deleted.is_some() {
        normalize_workout_set_order(pool, workout_id).await?;
    }

    Ok(deleted)
}

pub async fn list_workout_sets(
    pool: &PgPool,
    workout_id: i64,
) -> Result<Vec<WorkoutSetRow>, ApiError> {
    internal(
        sqlx::query_as::<_, WorkoutSetRow>(
            r#"
            SELECT
                ws.id,
                ws.workout_id,
                ws.exercise_id,
                e.name AS exercise_name,
                e.muscle_group,
                ws.set_order,
                ws.weight,
                ws.repetitions,
                ws.created_at
            FROM workout_sets ws
            JOIN exercises e ON e.id = ws.exercise_id
            WHERE ws.workout_id = $1
            ORDER BY ws.set_order ASC, ws.id ASC
            "#,
        )
        .bind(workout_id)
        .fetch_all(pool)
        .await,
    )
}
