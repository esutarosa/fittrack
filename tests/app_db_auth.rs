use chrono::NaiveDate;

use fittrack::app::{auth::AuthError, auth::AuthService, db::AppDatabase};

#[test]
fn register_and_login_round_trip() {
    let db = AppDatabase::open_in_memory().expect("database");

    let created = AuthService::register(&db, "alice", "password123", "password123")
        .expect("register");
    assert_eq!(created.username, "alice");

    let signed_in = AuthService::login(&db, "alice", "password123").expect("login");
    assert_eq!(signed_in.id, created.id);
    assert_eq!(signed_in.username, created.username);
}

#[test]
fn rejects_duplicate_username() {
    let db = AppDatabase::open_in_memory().expect("database");

    AuthService::register(&db, "alice", "password123", "password123").expect("register");

    let error = AuthService::register(&db, "alice", "password123", "password123")
        .expect_err("duplicate username");
    assert_eq!(error, AuthError::UsernameTaken);
}

#[test]
fn stores_exercises_workouts_and_sets() {
    let db = AppDatabase::open_in_memory().expect("database");
    let user = AuthService::register(&db, "alice", "password123", "password123")
        .expect("register");

    let exercise = db
        .create_exercise(user.id, "Bench Press", "Chest")
        .expect("exercise");
    let workout = db
        .create_workout(
            user.id,
            "Push Day",
            NaiveDate::from_ymd_opt(2026, 5, 11).expect("date"),
            Some("Focus on controlled tempo"),
        )
        .expect("workout");
    let workout_set = db
        .create_workout_set(workout.id, exercise.id, 1, 100.0, 5)
        .expect("workout set");

    let exercises = db.list_exercises(user.id).expect("list exercises");
    let workouts = db.list_workouts(user.id).expect("list workouts");
    let sets = db.list_workout_sets(workout.id).expect("list sets");

    assert_eq!(exercises.len(), 1);
    assert_eq!(workouts.len(), 1);
    assert_eq!(sets.len(), 1);
    assert_eq!(workout_set.set_order, 1);
    assert_eq!(sets[0].weight, 100.0);
}
