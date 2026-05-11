mod copy;
mod form;

use chrono::Local;
pub(crate) use copy::{WorkoutsCopy, copy};
use eframe::egui::Ui;
use form::render_workout_form;

use crate::app::auth::{AuthClient, Session};
use crate::app::workout_sets::{WorkoutDetailsProps, render_workout_details};
use crate::shared::contracts::{ExerciseDto, WorkoutDto, WorkoutSetDto};
use crate::shared::i18n::AppLanguage;
use crate::shared::ui as theme;

pub struct WorkoutsState {
    pub(crate) loaded: bool,
    pub(crate) workouts: Vec<WorkoutDto>,
    pub(crate) selected_workout_id: Option<i64>,
    pub(crate) sets_loaded_for: Option<i64>,
    pub(crate) sets: Vec<WorkoutSetDto>,
    pub(crate) editing_workout_id: Option<i64>,
    pub(crate) editing_set_id: Option<i64>,
    pub(crate) title: String,
    pub(crate) workout_date: String,
    pub(crate) notes: String,
    pub(crate) selected_exercise_id: Option<i64>,
    pub(crate) weight: String,
    pub(crate) repetitions: String,
    pub(crate) message: Option<String>,
}

impl Default for WorkoutsState {
    fn default() -> Self {
        Self {
            loaded: false,
            workouts: Vec::new(),
            selected_workout_id: None,
            sets_loaded_for: None,
            sets: Vec::new(),
            editing_workout_id: None,
            editing_set_id: None,
            title: String::new(),
            workout_date: Local::now().date_naive().format("%Y-%m-%d").to_string(),
            notes: String::new(),
            selected_exercise_id: None,
            weight: String::new(),
            repetitions: String::new(),
            message: None,
        }
    }
}

impl WorkoutsState {
    pub fn render(
        &mut self,
        ui: &mut Ui,
        client: &AuthClient,
        session: &Session,
        exercises: &[ExerciseDto],
        language: AppLanguage,
    ) {
        if !self.loaded {
            self.reload_workouts(client, session);
        }
        if self.selected_exercise_id.is_none() {
            self.selected_exercise_id = exercises.first().map(|item| item.id);
        }

        let layout = theme::layout();
        let copy = copy(language);
        ui.columns(2, |columns| {
            render_workout_form(&mut columns[0], self, client, session, layout, copy, language);
            render_workout_details(
                &mut columns[1],
                self,
                WorkoutDetailsProps { client, session, exercises, layout, copy, language },
            );
        });
    }

    pub fn workout_count(&self) -> usize {
        self.workouts.len()
    }

    pub(crate) fn start_workout_edit(&mut self, workout: &WorkoutDto) {
        self.editing_workout_id = Some(workout.id);
        self.title = workout.title.clone();
        self.workout_date = workout.workout_date.clone();
        self.notes = workout.notes.clone().unwrap_or_default();
    }

    pub(crate) fn delete_workout(
        &mut self,
        client: &AuthClient,
        session: &Session,
        workout_id: i64,
        language: AppLanguage,
    ) {
        match client.delete_workout(&session.token, workout_id) {
            Ok(_) => {
                if self.editing_workout_id == Some(workout_id) {
                    self.reset_workout_form();
                }
                self.reload_workouts(client, session);
                self.message = Some(copy(language).workout_deleted.to_owned());
            }
            Err(error) => self.message = Some(error.to_string()),
        }
    }

    pub(crate) fn reload_sets(&mut self, client: &AuthClient, session: &Session) {
        if let Some(workout_id) = self.selected_workout_id {
            match client.list_workout_sets(&session.token, workout_id) {
                Ok(items) => {
                    self.sets = items;
                    self.sets_loaded_for = Some(workout_id);
                    self.message = None;
                }
                Err(error) => self.message = Some(error.to_string()),
            }
        }
    }

    pub(crate) fn reload_workouts(&mut self, client: &AuthClient, session: &Session) {
        match client.list_workouts(&session.token) {
            Ok(items) => {
                self.workouts = items;
                self.loaded = true;
                if !self.workouts.iter().any(|item| Some(item.id) == self.selected_workout_id) {
                    self.selected_workout_id = self.workouts.first().map(|item| item.id);
                    self.sets_loaded_for = None;
                    self.sets.clear();
                }
                self.message = None;
            }
            Err(error) => self.message = Some(error.to_string()),
        }
    }

    pub(crate) fn save_workout(
        &mut self,
        client: &AuthClient,
        session: &Session,
        language: AppLanguage,
    ) {
        let title = self.title.trim();
        if title.is_empty() {
            self.message = Some(copy(language).title_required.to_owned());
            return;
        }

        let result = if let Some(workout_id) = self.editing_workout_id {
            client.update_workout(
                &session.token,
                workout_id,
                title,
                self.workout_date.trim(),
                trimmed_option(&self.notes),
            )
        } else {
            client.create_workout(
                &session.token,
                title,
                self.workout_date.trim(),
                trimmed_option(&self.notes),
            )
        };

        match result {
            Ok(workout) => {
                let message = if self.editing_workout_id.is_some() {
                    copy(language).workout_updated
                } else {
                    copy(language).workout_created
                };
                self.reset_workout_form();
                self.reload_workouts(client, session);
                self.selected_workout_id = Some(workout.id);
                self.message = Some(message.to_owned());
            }
            Err(error) => self.message = Some(error.to_string()),
        }
    }

    pub(crate) fn reset_workout_form(&mut self) {
        self.editing_workout_id = None;
        self.title.clear();
        self.workout_date = Local::now().date_naive().format("%Y-%m-%d").to_string();
        self.notes.clear();
    }
}

fn trimmed_option(value: &str) -> Option<&str> {
    let value = value.trim();
    if value.is_empty() { None } else { Some(value) }
}
