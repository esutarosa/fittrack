use chrono::{NaiveDate, NaiveDateTime};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredUser {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

impl StoredUser {
    pub fn into_user(self) -> User {
        User {
            id: self.id,
            username: self.username,
            created_at: self.created_at,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exercise {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub muscle_group: String,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Workout {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub workout_date: NaiveDate,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorkoutSet {
    pub id: i64,
    pub workout_id: i64,
    pub exercise_id: i64,
    pub set_order: i32,
    pub weight: f64,
    pub repetitions: i32,
    pub created_at: NaiveDateTime,
}
