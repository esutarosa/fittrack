use crate::app::auth::{AuthClient, AuthClientError};
use crate::shared::contracts::{CreateWorkoutSetRequest, UpdateWorkoutSetRequest, WorkoutSetDto};

impl AuthClient {
    pub fn list_workout_sets(
        &self,
        token: &str,
        workout_id: i64,
    ) -> Result<Vec<WorkoutSetDto>, AuthClientError> {
        self.http.get(&format!("/workouts/{workout_id}/sets"), Some(token))
    }

    pub fn create_workout_set(
        &self,
        token: &str,
        workout_id: i64,
        request: &CreateWorkoutSetRequest,
    ) -> Result<WorkoutSetDto, AuthClientError> {
        self.http.post(&format!("/workouts/{workout_id}/sets"), request, Some(token))
    }

    pub fn update_workout_set(
        &self,
        token: &str,
        workout_id: i64,
        workout_set_id: i64,
        request: &UpdateWorkoutSetRequest,
    ) -> Result<WorkoutSetDto, AuthClientError> {
        self.http.patch(
            &format!("/workouts/{workout_id}/sets/{workout_set_id}"),
            request,
            Some(token),
        )
    }

    pub fn delete_workout_set(
        &self,
        token: &str,
        workout_id: i64,
        workout_set_id: i64,
    ) -> Result<WorkoutSetDto, AuthClientError> {
        self.http.delete(&format!("/workouts/{workout_id}/sets/{workout_set_id}"), Some(token))
    }
}
