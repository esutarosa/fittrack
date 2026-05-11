use axum::routing::{get, post};
use axum::{Json, Router};
use tower_http::trace::TraceLayer;

use crate::server::storage::AppState;
use crate::server::{auth, training};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/me", get(auth::me))
        .route("/api/exercises", get(training::list_exercises).post(training::create_exercise))
        .route("/api/workouts", get(training::list_workouts).post(training::create_workout))
        .route(
            "/api/workouts/{workout_id}/sets",
            get(training::list_workout_sets).post(training::create_workout_set),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok".to_owned() })
}

#[derive(serde::Serialize)]
struct HealthResponse {
    status: String,
}
