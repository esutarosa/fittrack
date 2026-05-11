use sqlx::PgPool;

use crate::server::error::ApiError;
use crate::server::storage::rows::WorkoutSetRow;
use crate::shared::utils::map_display_error;

pub(super) async fn fetch_workout_set(
    pool: &PgPool,
    workout_set_id: i64,
) -> Result<WorkoutSetRow, ApiError> {
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
            WHERE ws.id = $1
            "#,
        )
        .bind(workout_set_id)
        .fetch_one(pool)
        .await,
    )
}

pub(super) async fn normalize_workout_set_order(
    pool: &PgPool,
    workout_id: i64,
) -> Result<(), ApiError> {
    internal(
        sqlx::query(
            r#"
            WITH ranked AS (
                SELECT id, ROW_NUMBER() OVER (ORDER BY set_order ASC, id ASC) AS rank
                FROM workout_sets
                WHERE workout_id = $1
            )
            UPDATE workout_sets ws
            SET set_order = -ranked.rank
            FROM ranked
            WHERE ws.id = ranked.id
            "#,
        )
        .bind(workout_id)
        .execute(pool)
        .await,
    )?;

    internal(
        sqlx::query(
            r#"
            WITH ranked AS (
                SELECT id, ROW_NUMBER() OVER (ORDER BY set_order DESC, id ASC) AS new_order
                FROM workout_sets
                WHERE workout_id = $1
            )
            UPDATE workout_sets ws
            SET set_order = ranked.new_order
            FROM ranked
            WHERE ws.id = ranked.id
            "#,
        )
        .bind(workout_id)
        .execute(pool)
        .await,
    )?;

    Ok(())
}

pub(super) fn internal<T, E>(result: Result<T, E>) -> Result<T, ApiError>
where
    E: std::fmt::Display,
{
    map_display_error(result, ApiError::internal)
}

pub(super) fn internal_conflict<T>(result: Result<T, sqlx::Error>) -> Result<T, ApiError> {
    result.map_err(|error| {
        if let Some(message) = database_conflict(&error) {
            return ApiError::conflict(message);
        }

        ApiError::internal(error.to_string())
    })
}

fn database_conflict(error: &sqlx::Error) -> Option<&'static str> {
    match error {
        sqlx::Error::Database(database_error) => match database_error.code().as_deref() {
            Some("23505") => Some("A record with the same key already exists"),
            Some("23503") => Some("This record is still used by related data"),
            _ if database_error.constraint().is_some() => {
                Some("A record with the same key already exists")
            }
            _ => None,
        },
        _ => None,
    }
}
