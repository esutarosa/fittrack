use axum::Json;
use axum::extract::{Path, State};
use axum::http::HeaderMap;

use crate::server::auth::require_user_id;
use crate::server::error::ApiError;
use crate::server::storage::{AppState, repo, rows};
use crate::shared::contracts::{
    CreateExerciseRequest, CreateWorkoutRequest, CreateWorkoutSetRequest, ExerciseDto, WorkoutDto,
    WorkoutSetDto,
};
use crate::shared::utils::map_display_error;

pub async fn list_exercises(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<ExerciseDto>>, ApiError> {
    let user_id = require_user_id(&headers, &state)?;
    let rows = repo::list_exercises(&state.pool, user_id).await?;
    Ok(Json(rows.iter().map(|row| row.to_dto()).collect()))
}

pub async fn create_exercise(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<CreateExerciseRequest>,
) -> Result<Json<ExerciseDto>, ApiError> {
    let user_id = require_user_id(&headers, &state)?;
    let row =
        repo::create_exercise(&state.pool, user_id, &request.name, &request.muscle_group).await?;
    Ok(Json(row.to_dto()))
}

pub async fn list_workouts(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<WorkoutDto>>, ApiError> {
    let user_id = require_user_id(&headers, &state)?;
    let rows = repo::list_workouts(&state.pool, user_id).await?;
    Ok(Json(rows.iter().map(|row| row.to_dto()).collect()))
}

pub async fn create_workout(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<CreateWorkoutRequest>,
) -> Result<Json<WorkoutDto>, ApiError> {
    let user_id = require_user_id(&headers, &state)?;
    let workout_date = chrono::NaiveDate::parse_from_str(&request.workout_date, "%Y-%m-%d")
        .map_err(|_| ApiError::bad_request("workout_date must use YYYY-MM-DD"))?;
    let row = repo::create_workout(
        &state.pool,
        user_id,
        &request.title,
        workout_date,
        request.notes.as_deref(),
    )
    .await?;
    Ok(Json(row.to_dto()))
}

pub async fn list_workout_sets(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(workout_id): Path<i64>,
) -> Result<Json<Vec<WorkoutSetDto>>, ApiError> {
    let user_id = require_user_id(&headers, &state)?;
    assert_workout_owner(&state, workout_id, user_id).await?;
    let rows = repo::list_workout_sets(&state.pool, workout_id).await?;
    Ok(Json(rows.iter().map(|row| row.to_dto()).collect()))
}

pub async fn create_workout_set(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(workout_id): Path<i64>,
    Json(request): Json<CreateWorkoutSetRequest>,
) -> Result<Json<WorkoutSetDto>, ApiError> {
    let user_id = require_user_id(&headers, &state)?;
    assert_workout_owner(&state, workout_id, user_id).await?;
    let row = repo::create_workout_set(
        &state.pool,
        workout_id,
        request.exercise_id,
        request.set_order,
        request.weight,
        request.repetitions,
    )
    .await?;
    Ok(Json(row.to_dto()))
}

async fn assert_workout_owner(
    state: &AppState,
    workout_id: i64,
    user_id: i64,
) -> Result<(), ApiError> {
    let workout = sqlx::query_as::<_, rows::WorkoutRow>(
        r#"
        SELECT id, user_id, title, workout_date, notes, created_at
        FROM workouts
        WHERE id = $1
        "#,
    )
    .bind(workout_id)
    .fetch_optional(&state.pool)
    .await;
    let workout = internal(workout)?;

    let workout = workout.ok_or_else(|| ApiError::not_found("Workout not found"))?;
    if workout.user_id != user_id {
        return Err(ApiError::unauthorized("Workout does not belong to the current user"));
    }

    Ok(())
}

fn internal<T, E>(result: Result<T, E>) -> Result<T, ApiError>
where
    E: std::fmt::Display,
{
    map_display_error(result, ApiError::internal)
}
