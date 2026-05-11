mod copy;
mod form;
mod list;

use eframe::egui::Ui;

use self::copy::{ExercisesCopy, copy};
use self::form::render_form;
use self::list::render_list;
use crate::app::auth::{AuthClient, Session};
use crate::shared::contracts::ExerciseDto;
use crate::shared::i18n::AppLanguage;
use crate::shared::ui as theme;

const MUSCLE_GROUPS: [&str; 8] =
    ["Chest", "Back", "Legs", "Shoulders", "Arms", "Core", "Glutes", "Full Body"];

#[derive(Default)]
pub struct ExercisesState {
    loaded: bool,
    editing_id: Option<i64>,
    name: String,
    selected_group: usize,
    items: Vec<ExerciseDto>,
    message: Option<String>,
}

impl ExercisesState {
    pub fn render(
        &mut self,
        ui: &mut Ui,
        client: &AuthClient,
        session: &Session,
        language: AppLanguage,
    ) {
        if !self.loaded {
            self.reload(client, session);
        }

        let layout = theme::layout();
        let copy = copy(language);
        ui.columns(2, |columns| {
            render_form(&mut columns[0], self, client, session, copy, layout, language);
            render_list(&mut columns[1], self, client, session, copy, language);
        });
    }

    pub fn items(&self) -> &[ExerciseDto] {
        &self.items
    }

    fn form_title(&self, copy: ExercisesCopy) -> &'static str {
        if self.editing_id.is_some() { copy.edit_title } else { copy.form_title }
    }

    fn save(&mut self, client: &AuthClient, session: &Session, language: AppLanguage) {
        let name = self.name.trim();
        if name.is_empty() {
            self.message = Some(copy(language).name_required.to_owned());
            return;
        }

        let result = if let Some(exercise_id) = self.editing_id {
            client.update_exercise(
                &session.token,
                exercise_id,
                name,
                MUSCLE_GROUPS[self.selected_group],
            )
        } else {
            client.create_exercise(&session.token, name, MUSCLE_GROUPS[self.selected_group])
        };

        match result {
            Ok(_) => {
                let message = if self.editing_id.is_some() {
                    copy(language).updated_message
                } else {
                    copy(language).created_message
                };
                self.reset_form();
                self.reload(client, session);
                self.message = Some(message.to_owned());
            }
            Err(error) => self.message = Some(error.to_string()),
        }
    }

    fn delete(
        &mut self,
        client: &AuthClient,
        session: &Session,
        exercise_id: i64,
        language: AppLanguage,
    ) {
        match client.delete_exercise(&session.token, exercise_id) {
            Ok(_) => {
                if self.editing_id == Some(exercise_id) {
                    self.reset_form();
                }
                self.reload(client, session);
                self.message = Some(copy(language).deleted_message.to_owned());
            }
            Err(error) => self.message = Some(error.to_string()),
        }
    }

    fn reload(&mut self, client: &AuthClient, session: &Session) {
        match client.list_exercises(&session.token) {
            Ok(items) => {
                self.items = items;
                self.loaded = true;
                self.message = None;
            }
            Err(error) => self.message = Some(error.to_string()),
        }
    }

    fn reset_form(&mut self) {
        self.editing_id = None;
        self.name.clear();
        self.selected_group = 0;
    }

    fn start_edit(&mut self, item: &ExerciseDto) {
        self.editing_id = Some(item.id);
        self.name = item.name.clone();
        self.selected_group = find_group_index(&item.muscle_group);
    }
}

fn find_group_index(muscle_group: &str) -> usize {
    MUSCLE_GROUPS.iter().position(|group| *group == muscle_group).unwrap_or(0)
}
