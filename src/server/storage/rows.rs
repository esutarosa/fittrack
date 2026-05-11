use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;

use crate::shared::contracts::{
    ExerciseDto, ProgressRecordDto, UserDto, WorkoutDto, WorkoutSetDto,
};
use crate::shared::utils::{format_date, format_datetime};

#[derive(Clone, Debug, FromRow)]
pub struct UserRow {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

impl UserRow {
    pub fn to_dto(&self) -> UserDto {
        UserDto {
            id: self.id,
            username: self.username.clone(),
            created_at: format_datetime(self.created_at),
        }
    }
}

#[derive(Clone, Debug, FromRow)]
pub struct ExerciseRow {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub muscle_group: String,
    pub created_at: NaiveDateTime,
}

impl ExerciseRow {
    pub fn to_dto(&self) -> ExerciseDto {
        ExerciseDto {
            id: self.id,
            name: self.name.clone(),
            muscle_group: self.muscle_group.clone(),
            created_at: format_datetime(self.created_at),
        }
    }
}

#[derive(Clone, Debug, FromRow)]
pub struct WorkoutRow {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub workout_date: NaiveDate,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
}

impl WorkoutRow {
    pub fn to_dto(&self) -> WorkoutDto {
        WorkoutDto {
            id: self.id,
            title: self.title.clone(),
            workout_date: format_date(self.workout_date),
            notes: self.notes.clone(),
            created_at: format_datetime(self.created_at),
        }
    }
}

#[derive(Clone, Debug, FromRow)]
pub struct WorkoutSetRow {
    pub id: i64,
    pub workout_id: i64,
    pub exercise_id: i64,
    pub exercise_name: String,
    pub muscle_group: String,
    pub set_order: i32,
    pub weight: f64,
    pub repetitions: i32,
    pub created_at: NaiveDateTime,
}

impl WorkoutSetRow {
    pub fn to_dto(&self) -> WorkoutSetDto {
        WorkoutSetDto {
            id: self.id,
            workout_id: self.workout_id,
            exercise_id: self.exercise_id,
            exercise_name: self.exercise_name.clone(),
            muscle_group: self.muscle_group.clone(),
            set_order: self.set_order,
            weight: self.weight,
            repetitions: self.repetitions,
            created_at: format_datetime(self.created_at),
        }
    }
}

#[derive(Clone, Debug, FromRow)]
pub struct ProgressRecordRow {
    pub workout_date: NaiveDate,
    pub workout_title: String,
    pub max_weight: f64,
    pub max_repetitions: i32,
    pub total_volume: f64,
}

impl ProgressRecordRow {
    pub fn to_dto(&self) -> ProgressRecordDto {
        ProgressRecordDto {
            workout_date: format_date(self.workout_date),
            workout_title: self.workout_title.clone(),
            max_weight: self.max_weight,
            max_repetitions: self.max_repetitions,
            total_volume: self.total_volume,
        }
    }
}
