mod session;
mod sets;

use anyhow::Result;
pub use session::Session;

use crate::app::http_client::{HttpClient, HttpClientError};
use crate::shared::contracts::{
    AuthResponse, CreateExerciseRequest, CreateWorkoutRequest, ExerciseDto, LoginRequest,
    ProgressRecordDto, RegisterRequest, UpdateExerciseRequest, UpdateWorkoutRequest, WorkoutDto,
};

pub type AuthClientError = HttpClientError;

#[derive(Clone)]
pub struct AuthClient {
    pub(crate) http: HttpClient,
}

impl AuthClient {
    pub fn from_env() -> Result<Self> {
        Ok(Self { http: HttpClient::from_env()? })
    }

    pub fn register(
        &self,
        username: &str,
        password: &str,
        confirm_password: &str,
    ) -> Result<AuthResponse, AuthClientError> {
        self.http.post_without_auth(
            "/auth/register",
            &RegisterRequest {
                username: username.to_owned(),
                password: password.to_owned(),
                confirm_password: confirm_password.to_owned(),
            },
        )
    }

    pub fn login(&self, username: &str, password: &str) -> Result<AuthResponse, AuthClientError> {
        self.http.post_without_auth(
            "/auth/login",
            &LoginRequest { username: username.to_owned(), password: password.to_owned() },
        )
    }

    pub fn list_exercises(&self, token: &str) -> Result<Vec<ExerciseDto>, AuthClientError> {
        self.http.get("/exercises", Some(token))
    }

    pub fn create_exercise(
        &self,
        token: &str,
        name: &str,
        muscle_group: &str,
    ) -> Result<ExerciseDto, AuthClientError> {
        self.http.post(
            "/exercises",
            &CreateExerciseRequest { name: name.to_owned(), muscle_group: muscle_group.to_owned() },
            Some(token),
        )
    }

    pub fn update_exercise(
        &self,
        token: &str,
        exercise_id: i64,
        name: &str,
        muscle_group: &str,
    ) -> Result<ExerciseDto, AuthClientError> {
        self.http.patch(
            &format!("/exercises/{exercise_id}"),
            &UpdateExerciseRequest { name: name.to_owned(), muscle_group: muscle_group.to_owned() },
            Some(token),
        )
    }

    pub fn delete_exercise(
        &self,
        token: &str,
        exercise_id: i64,
    ) -> Result<ExerciseDto, AuthClientError> {
        self.http.delete(&format!("/exercises/{exercise_id}"), Some(token))
    }

    pub fn list_workouts(&self, token: &str) -> Result<Vec<WorkoutDto>, AuthClientError> {
        self.http.get("/workouts", Some(token))
    }

    pub fn create_workout(
        &self,
        token: &str,
        title: &str,
        workout_date: &str,
        notes: Option<&str>,
    ) -> Result<WorkoutDto, AuthClientError> {
        self.http.post(
            "/workouts",
            &CreateWorkoutRequest {
                title: title.to_owned(),
                workout_date: workout_date.to_owned(),
                notes: notes.map(str::to_owned),
            },
            Some(token),
        )
    }

    pub fn update_workout(
        &self,
        token: &str,
        workout_id: i64,
        title: &str,
        workout_date: &str,
        notes: Option<&str>,
    ) -> Result<WorkoutDto, AuthClientError> {
        self.http.patch(
            &format!("/workouts/{workout_id}"),
            &UpdateWorkoutRequest {
                title: title.to_owned(),
                workout_date: workout_date.to_owned(),
                notes: notes.map(str::to_owned),
            },
            Some(token),
        )
    }

    pub fn delete_workout(
        &self,
        token: &str,
        workout_id: i64,
    ) -> Result<WorkoutDto, AuthClientError> {
        self.http.delete(&format!("/workouts/{workout_id}"), Some(token))
    }

    pub fn list_progress(
        &self,
        token: &str,
        exercise_id: i64,
    ) -> Result<Vec<ProgressRecordDto>, AuthClientError> {
        self.http.get(&format!("/progress/{exercise_id}"), Some(token))
    }
}
