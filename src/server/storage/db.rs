use sqlx::PgPool;

use crate::server::error::ApiError;
use crate::shared::utils::map_display_error;

pub async fn init_schema(pool: &PgPool) -> Result<(), ApiError> {
    execute(
        pool,
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id BIGSERIAL PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .await?;
    execute(
        pool,
        r#"
        CREATE TABLE IF NOT EXISTS exercises (
            id BIGSERIAL PRIMARY KEY,
            user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            muscle_group TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .await?;
    execute(
        pool,
        r#"
        CREATE TABLE IF NOT EXISTS workouts (
            id BIGSERIAL PRIMARY KEY,
            user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            title TEXT NOT NULL,
            workout_date DATE NOT NULL,
            notes TEXT,
            created_at TIMESTAMP NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .await?;
    execute(
        pool,
        r#"
        CREATE TABLE IF NOT EXISTS workout_sets (
            id BIGSERIAL PRIMARY KEY,
            workout_id BIGINT NOT NULL REFERENCES workouts(id) ON DELETE CASCADE,
            exercise_id BIGINT NOT NULL REFERENCES exercises(id) ON DELETE RESTRICT,
            set_order INTEGER NOT NULL,
            weight DOUBLE PRECISION NOT NULL,
            repetitions INTEGER NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            UNIQUE(workout_id, set_order)
        )
        "#,
    )
    .await?;

    execute(pool, "CREATE INDEX IF NOT EXISTS idx_exercises_user_id ON exercises(user_id)").await?;
    execute(pool, "CREATE INDEX IF NOT EXISTS idx_workouts_user_id ON workouts(user_id)").await?;
    execute(
        pool,
        "CREATE INDEX IF NOT EXISTS idx_workout_sets_workout_id ON workout_sets(workout_id)",
    )
    .await?;
    execute(
        pool,
        "CREATE INDEX IF NOT EXISTS idx_workout_sets_exercise_id ON workout_sets(exercise_id)",
    )
    .await?;

    Ok(())
}

async fn execute(pool: &PgPool, query: &str) -> Result<(), ApiError> {
    map_display_error(sqlx::query(query).execute(pool).await, ApiError::internal)?;
    Ok(())
}
