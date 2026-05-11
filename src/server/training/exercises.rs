use axum::Json;
use axum::extract::{Path, State};
use axum::http::HeaderMap;

use crate::server::auth::require_user_id;
use crate::server::error::ApiError;
use crate::server::storage::{AppState, repo};
use crate::shared::contracts::{CreateExerciseRequest, ExerciseDto, UpdateExerciseRequest};

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

pub async fn update_exercise(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(exercise_id): Path<i64>,
    Json(request): Json<UpdateExerciseRequest>,
) -> Result<Json<ExerciseDto>, ApiError> {
    let user_id = require_user_id(&headers, &state)?;
    let row = repo::update_exercise(
        &state.pool,
        user_id,
        exercise_id,
        &request.name,
        &request.muscle_group,
    )
    .await?;
    Ok(Json(row.ok_or_else(|| ApiError::not_found("Exercise not found"))?.to_dto()))
}

pub async fn delete_exercise(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(exercise_id): Path<i64>,
) -> Result<Json<ExerciseDto>, ApiError> {
    let user_id = require_user_id(&headers, &state)?;
    let row = repo::delete_exercise(&state.pool, user_id, exercise_id).await?;
    Ok(Json(row.ok_or_else(|| ApiError::not_found("Exercise not found"))?.to_dto()))
}
