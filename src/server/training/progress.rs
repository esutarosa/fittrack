use axum::Json;
use axum::extract::{Path, State};
use axum::http::HeaderMap;

use super::access::assert_exercise_owner;
use crate::server::auth::require_user_id;
use crate::server::error::ApiError;
use crate::server::storage::{AppState, repo};
use crate::shared::contracts::ProgressRecordDto;

pub async fn list_progress(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(exercise_id): Path<i64>,
) -> Result<Json<Vec<ProgressRecordDto>>, ApiError> {
    let user_id = require_user_id(&headers, &state)?;
    assert_exercise_owner(&state, exercise_id, user_id).await?;
    let rows = repo::list_progress(&state.pool, user_id, exercise_id).await?;
    Ok(Json(rows.iter().map(|row| row.to_dto()).collect()))
}
