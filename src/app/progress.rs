use eframe::egui::{Button, ComboBox, Grid, RichText, Ui, vec2};

use crate::app::auth::{AuthClient, Session};
use crate::shared::contracts::{ExerciseDto, ProgressRecordDto};
use crate::shared::i18n::AppLanguage;
use crate::shared::ui as theme;

#[derive(Default)]
pub struct ProgressState {
    selected_exercise_id: Option<i64>,
    loaded_for: Option<i64>,
    items: Vec<ProgressRecordDto>,
    message: Option<String>,
}

impl ProgressState {
    pub fn render(
        &mut self,
        ui: &mut Ui,
        client: &AuthClient,
        session: &Session,
        exercises: &[ExerciseDto],
        language: AppLanguage,
    ) {
        let layout = theme::layout();
        let copy = copy(language);
        if self.selected_exercise_id.is_none() {
            self.selected_exercise_id = exercises.first().map(|item| item.id);
        }
        if self.loaded_for != self.selected_exercise_id {
            self.reload(client, session);
        }

        theme::card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(copy.title).size(18.0).strong());
                if ui
                    .add(Button::new(copy.refresh).min_size(vec2(96.0, layout.nav_item_height)))
                    .clicked()
                {
                    self.reload(client, session);
                }
            });
            ui.add_space(8.0);

            if exercises.is_empty() {
                ui.label(copy.no_exercises);
                return;
            }

            ComboBox::from_id_salt("progress_exercise_id")
                .selected_text(selected_name(exercises, self.selected_exercise_id))
                .show_ui(ui, |ui| {
                    for exercise in exercises {
                        ui.selectable_value(
                            &mut self.selected_exercise_id,
                            Some(exercise.id),
                            &exercise.name,
                        );
                    }
                });
            ui.add_space(layout.section_gap);

            if let Some(message) = &self.message {
                ui.label(message);
                ui.add_space(8.0);
            }

            if self.items.is_empty() {
                ui.label(copy.empty_state);
                return;
            }

            Grid::new("progress_grid")
                .striped(true)
                .spacing(vec2(24.0, ui.spacing().item_spacing.y))
                .show(ui, |ui| {
                    ui.label(RichText::new(copy.date_column).strong());
                    ui.label(RichText::new(copy.workout_column).strong());
                    ui.label(RichText::new(copy.weight_column).strong());
                    ui.label(RichText::new(copy.reps_column).strong());
                    ui.label(RichText::new(copy.volume_column).strong());
                    ui.end_row();

                    for item in &self.items {
                        ui.label(&item.workout_date);
                        ui.label(&item.workout_title);
                        ui.label(format!("{:.1}", item.max_weight));
                        ui.label(item.max_repetitions.to_string());
                        ui.label(format!("{:.1}", item.total_volume));
                        ui.end_row();
                    }
                });
        });
    }

    fn reload(&mut self, client: &AuthClient, session: &Session) {
        if let Some(exercise_id) = self.selected_exercise_id {
            match client.list_progress(&session.token, exercise_id) {
                Ok(items) => {
                    self.items = items;
                    self.loaded_for = Some(exercise_id);
                    self.message = None;
                }
                Err(error) => self.message = Some(error.to_string()),
            }
        }
    }

    pub fn record_count(&self) -> usize {
        self.items.len()
    }
}

fn selected_name(exercises: &[ExerciseDto], selected_id: Option<i64>) -> String {
    exercises
        .iter()
        .find(|item| Some(item.id) == selected_id)
        .map(|item| item.name.clone())
        .unwrap_or_else(|| "Select exercise".to_owned())
}

#[derive(Clone, Copy)]
struct ProgressCopy {
    title: &'static str,
    refresh: &'static str,
    no_exercises: &'static str,
    empty_state: &'static str,
    date_column: &'static str,
    workout_column: &'static str,
    weight_column: &'static str,
    reps_column: &'static str,
    volume_column: &'static str,
}

fn copy(language: AppLanguage) -> ProgressCopy {
    match language {
        AppLanguage::English => ProgressCopy {
            title: "Progress",
            refresh: "Refresh",
            no_exercises: "Create an exercise to view progress.",
            empty_state: "No progress records yet.",
            date_column: "Date",
            workout_column: "Workout",
            weight_column: "Max weight",
            reps_column: "Max reps",
            volume_column: "Volume",
        },
        AppLanguage::Ukrainian => ProgressCopy {
            title: "Прогрес",
            refresh: "Оновити",
            no_exercises: "Створіть вправу, щоб побачити прогрес.",
            empty_state: "Записів прогресу поки немає.",
            date_column: "Дата",
            workout_column: "Тренування",
            weight_column: "Макс. вага",
            reps_column: "Макс. повт.",
            volume_column: "Об'єм",
        },
    }
}
