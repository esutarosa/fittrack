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
pub struct WorkoutDto {
    pub id: i64,
    pub title: String,
    pub workout_date: String,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateWorkoutRequest {
    pub title: String,
    pub workout_date: String,
    pub notes: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkoutSetDto {
    pub id: i64,
    pub workout_id: i64,
    pub exercise_id: i64,
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
