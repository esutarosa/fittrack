use axum::Json;
use axum::extract::{Path, State};
use axum::http::HeaderMap;

use super::access::{
    assert_exercise_owner, assert_workout_owner, assert_workout_set, parse_workout_date,
};
use crate::server::auth::require_user_id;
use crate::server::error::ApiError;
use crate::server::storage::{AppState, repo};
use crate::shared::contracts::{
    CreateWorkoutRequest, CreateWorkoutSetRequest, UpdateWorkoutRequest, UpdateWorkoutSetRequest,
    WorkoutDto, WorkoutSetDto,
};

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
    let row = repo::create_workout(
        &state.pool,
        user_id,
        &request.title,
        parse_workout_date(&request.workout_date)?,
    )
    .await?;
    Ok(Json(row.to_dto()))
}

pub async fn update_workout(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(workout_id): Path<i64>,
    Json(request): Json<UpdateWorkoutRequest>,
) -> Result<Json<WorkoutDto>, ApiError> {
    let user_id = require_user_id(&headers, &state)?;
    let row = repo::update_workout(
        &state.pool,
        user_id,
        workout_id,
        &request.title,
        parse_workout_date(&request.workout_date)?,
    )
    .await?;
    Ok(Json(row.ok_or_else(|| ApiError::not_found("Workout not found"))?.to_dto()))
}

pub async fn delete_workout(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(workout_id): Path<i64>,
) -> Result<Json<WorkoutDto>, ApiError> {
    let user_id = require_user_id(&headers, &state)?;
    let row = repo::delete_workout(&state.pool, user_id, workout_id).await?;
    Ok(Json(row.ok_or_else(|| ApiError::not_found("Workout not found"))?.to_dto()))
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
    assert_exercise_owner(&state, request.exercise_id, user_id).await?;
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

pub async fn update_workout_set(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((workout_id, workout_set_id)): Path<(i64, i64)>,
    Json(request): Json<UpdateWorkoutSetRequest>,
) -> Result<Json<WorkoutSetDto>, ApiError> {
    let user_id = require_user_id(&headers, &state)?;
    assert_workout_owner(&state, workout_id, user_id).await?;
    assert_exercise_owner(&state, request.exercise_id, user_id).await?;
    assert_workout_set(workout_id, workout_set_id, &state).await?;
    let row = repo::update_workout_set(
        &state.pool,
        workout_id,
        workout_set_id,
        request.exercise_id,
        request.set_order,
        request.weight,
        request.repetitions,
    )
    .await?;
    Ok(Json(row.ok_or_else(|| ApiError::not_found("Workout set not found"))?.to_dto()))
}

pub async fn delete_workout_set(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((workout_id, workout_set_id)): Path<(i64, i64)>,
) -> Result<Json<WorkoutSetDto>, ApiError> {
    let user_id = require_user_id(&headers, &state)?;
    assert_workout_owner(&state, workout_id, user_id).await?;
    assert_workout_set(workout_id, workout_set_id, &state).await?;
    let row = repo::delete_workout_set(&state.pool, workout_id, workout_set_id).await?;
    Ok(Json(row.ok_or_else(|| ApiError::not_found("Workout set not found"))?.to_dto()))
}
