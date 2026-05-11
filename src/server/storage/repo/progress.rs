use sqlx::PgPool;

use super::shared::internal;
use crate::server::error::ApiError;
use crate::server::storage::rows::ProgressRecordRow;

pub async fn list_progress(
    pool: &PgPool,
    user_id: i64,
    exercise_id: i64,
) -> Result<Vec<ProgressRecordRow>, ApiError> {
    internal(
        sqlx::query_as::<_, ProgressRecordRow>(
            r#"
            SELECT
                w.workout_date,
                w.title AS workout_title,
                MAX(ws.weight) AS max_weight,
                MAX(ws.repetitions) AS max_repetitions,
                SUM(ws.weight * ws.repetitions) AS total_volume
            FROM workout_sets ws
            JOIN workouts w ON w.id = ws.workout_id
            WHERE w.user_id = $1 AND ws.exercise_id = $2
            GROUP BY w.id, w.workout_date, w.title
            ORDER BY w.workout_date ASC, w.id ASC
            "#,
        )
        .bind(user_id)
        .bind(exercise_id)
        .fetch_all(pool)
        .await,
    )
}
