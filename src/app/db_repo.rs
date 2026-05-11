use anyhow::Result;
use chrono::{NaiveDate, Utc};
use rusqlite::{OptionalExtension, params};

use super::{
    db::AppDatabase,
    models::{Exercise, StoredUser, Workout, WorkoutSet},
};

impl AppDatabase {
    pub fn find_user_by_username(&self, username: &str) -> Result<Option<StoredUser>> {
        self.conn
            .query_row(
                r#"
                SELECT id, username, password_hash, created_at
                FROM users
                WHERE username = ?1
                "#,
                params![username],
                |row| {
                    Ok(StoredUser {
                        id: row.get(0)?,
                        username: row.get(1)?,
                        password_hash: row.get(2)?,
                        created_at: row.get(3)?,
                    })
                },
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn find_user_by_id(&self, id: i64) -> Result<Option<StoredUser>> {
        self.conn
            .query_row(
                r#"
                SELECT id, username, password_hash, created_at
                FROM users
                WHERE id = ?1
                "#,
                params![id],
                |row| {
                    Ok(StoredUser {
                        id: row.get(0)?,
                        username: row.get(1)?,
                        password_hash: row.get(2)?,
                        created_at: row.get(3)?,
                    })
                },
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn create_user(&self, username: &str, password_hash: &str) -> Result<StoredUser> {
        let created_at = Utc::now().naive_utc();
        self.conn.execute(
            r#"
            INSERT INTO users (username, password_hash, created_at)
            VALUES (?1, ?2, ?3)
            "#,
            params![username, password_hash, created_at],
        )?;

        let id = self.conn.last_insert_rowid();
        Ok(StoredUser {
            id,
            username: username.to_owned(),
            password_hash: password_hash.to_owned(),
            created_at,
        })
    }

    pub fn create_exercise(&self, user_id: i64, name: &str, muscle_group: &str) -> Result<Exercise> {
        let created_at = Utc::now().naive_utc();
        self.conn.execute(
            r#"
            INSERT INTO exercises (user_id, name, muscle_group, created_at)
            VALUES (?1, ?2, ?3, ?4)
            "#,
            params![user_id, name, muscle_group, created_at],
        )?;

        Ok(Exercise {
            id: self.conn.last_insert_rowid(),
            user_id,
            name: name.to_owned(),
            muscle_group: muscle_group.to_owned(),
            created_at,
        })
    }

    pub fn list_exercises(&self, user_id: i64) -> Result<Vec<Exercise>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT id, user_id, name, muscle_group, created_at
            FROM exercises
            WHERE user_id = ?1
            ORDER BY name COLLATE NOCASE
            "#,
        )?;

        stmt.query_map(params![user_id], |row| {
            Ok(Exercise {
                id: row.get(0)?,
                user_id: row.get(1)?,
                name: row.get(2)?,
                muscle_group: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
    }

    pub fn create_workout(
        &self,
        user_id: i64,
        title: &str,
        workout_date: NaiveDate,
        notes: Option<&str>,
    ) -> Result<Workout> {
        let created_at = Utc::now().naive_utc();
        self.conn.execute(
            r#"
            INSERT INTO workouts (user_id, title, workout_date, notes, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            params![user_id, title, workout_date, notes, created_at],
        )?;

        Ok(Workout {
            id: self.conn.last_insert_rowid(),
            user_id,
            title: title.to_owned(),
            workout_date,
            notes: notes.map(ToOwned::to_owned),
            created_at,
        })
    }

    pub fn list_workouts(&self, user_id: i64) -> Result<Vec<Workout>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT id, user_id, title, workout_date, notes, created_at
            FROM workouts
            WHERE user_id = ?1
            ORDER BY workout_date DESC, id DESC
            "#,
        )?;

        stmt.query_map(params![user_id], |row| {
            Ok(Workout {
                id: row.get(0)?,
                user_id: row.get(1)?,
                title: row.get(2)?,
                workout_date: row.get(3)?,
                notes: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
    }

    pub fn create_workout_set(
        &self,
        workout_id: i64,
        exercise_id: i64,
        set_order: i32,
        weight: f64,
        repetitions: i32,
    ) -> Result<WorkoutSet> {
        let created_at = Utc::now().naive_utc();
        self.conn.execute(
            r#"
            INSERT INTO workout_sets (workout_id, exercise_id, set_order, weight, repetitions, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![workout_id, exercise_id, set_order, weight, repetitions, created_at],
        )?;

        Ok(WorkoutSet {
            id: self.conn.last_insert_rowid(),
            workout_id,
            exercise_id,
            set_order,
            weight,
            repetitions,
            created_at,
        })
    }

    pub fn list_workout_sets(&self, workout_id: i64) -> Result<Vec<WorkoutSet>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT id, workout_id, exercise_id, set_order, weight, repetitions, created_at
            FROM workout_sets
            WHERE workout_id = ?1
            ORDER BY set_order ASC, id ASC
            "#,
        )?;

        stmt.query_map(params![workout_id], |row| {
            Ok(WorkoutSet {
                id: row.get(0)?,
                workout_id: row.get(1)?,
                exercise_id: row.get(2)?,
                set_order: row.get(3)?,
                weight: row.get(4)?,
                repetitions: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
    }
}
