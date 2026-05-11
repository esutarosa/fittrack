use std::path::Path;

use anyhow::{Context, Result};
use rusqlite::Connection;

const DEFAULT_DB_FILE: &str = "fittrack.sqlite";

pub struct AppDatabase {
    pub(crate) conn: Connection,
}

impl AppDatabase {
    pub fn open_default() -> Result<Self> {
        Self::open(DEFAULT_DB_FILE)
    }

    pub fn open_in_memory() -> Result<Self> {
        Self::with_connection(Connection::open_in_memory()?)
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        if let Some(parent) = path
            .as_ref()
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            std::fs::create_dir_all(parent).context("failed to create database directory")?;
        }

        Self::with_connection(Connection::open(path).context("failed to open database")?)
    }

    fn with_connection(conn: Connection) -> Result<Self> {
        let database = Self { conn };
        database.initialize()?;
        Ok(database)
    }

    fn initialize(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS exercises (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                name TEXT NOT NULL,
                muscle_group TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS workouts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                title TEXT NOT NULL,
                workout_date TEXT NOT NULL,
                notes TEXT,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS workout_sets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                workout_id INTEGER NOT NULL REFERENCES workouts(id) ON DELETE CASCADE,
                exercise_id INTEGER NOT NULL REFERENCES exercises(id) ON DELETE RESTRICT,
                set_order INTEGER NOT NULL,
                weight REAL NOT NULL,
                repetitions INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                UNIQUE(workout_id, set_order)
            );

            CREATE INDEX IF NOT EXISTS idx_exercises_user_id ON exercises(user_id);
            CREATE INDEX IF NOT EXISTS idx_workouts_user_id ON workouts(user_id);
            CREATE INDEX IF NOT EXISTS idx_workout_sets_workout_id ON workout_sets(workout_id);
            CREATE INDEX IF NOT EXISTS idx_workout_sets_exercise_id ON workout_sets(exercise_id);
            "#,
        )?;

        Ok(())
    }
}
