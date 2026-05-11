use crate::server::error::ApiError;
use crate::server::storage::{AppState, repo};

pub(super) async fn assert_workout_owner(
    state: &AppState,
    workout_id: i64,
    user_id: i64,
) -> Result<(), ApiError> {
    let workout = repo::find_workout(&state.pool, user_id, workout_id).await?;
    let workout = workout.ok_or_else(|| ApiError::not_found("Workout not found"))?;
    if workout.user_id != user_id {
        return Err(ApiError::unauthorized("Workout does not belong to the current user"));
    }
    Ok(())
}

pub(super) async fn assert_exercise_owner(
    state: &AppState,
    exercise_id: i64,
    user_id: i64,
) -> Result<(), ApiError> {
    let exercise = repo::find_exercise(&state.pool, user_id, exercise_id).await?;
    let exercise = exercise.ok_or_else(|| ApiError::not_found("Exercise not found"))?;
    if exercise.user_id != user_id {
        return Err(ApiError::unauthorized("Exercise does not belong to the current user"));
    }
    Ok(())
}

pub(super) async fn assert_workout_set(
    workout_id: i64,
    workout_set_id: i64,
    state: &AppState,
) -> Result<(), ApiError> {
    let workout_set = repo::find_workout_set(&state.pool, workout_id, workout_set_id).await?;
    if workout_set.is_none() {
        return Err(ApiError::not_found("Workout set not found"));
    }
    Ok(())
}

pub(super) fn parse_workout_date(value: &str) -> Result<chrono::NaiveDate, ApiError> {
    chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|_| ApiError::bad_request("workout_date must use YYYY-MM-DD"))
}
