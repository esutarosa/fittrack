use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub error: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub id: i64,
    pub username: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserDto,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExerciseDto {
    pub id: i64,
    pub name: String,
    pub muscle_group: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateExerciseRequest {
    pub name: String,
    pub muscle_group: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateExerciseRequest {
    pub name: String,
    pub muscle_group: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkoutDto {
    pub id: i64,
    pub title: String,
    pub workout_date: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateWorkoutRequest {
    pub title: String,
    pub workout_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateWorkoutRequest {
    pub title: String,
    pub workout_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkoutSetDto {
    pub id: i64,
    pub workout_id: i64,
    pub exercise_id: i64,
    pub exercise_name: String,
    pub muscle_group: String,
    pub set_order: i32,
    pub weight: f64,
    pub repetitions: i32,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateWorkoutSetRequest {
    pub exercise_id: i64,
    pub set_order: i32,
    pub weight: f64,
    pub repetitions: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateWorkoutSetRequest {
    pub exercise_id: i64,
    pub set_order: i32,
    pub weight: f64,
    pub repetitions: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgressRecordDto {
    pub workout_date: String,
    pub workout_title: String,
    pub max_weight: f64,
    pub max_repetitions: i32,
    pub total_volume: f64,
}
