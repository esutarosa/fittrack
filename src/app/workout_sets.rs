mod actions;

use eframe::egui::{ComboBox, Grid, RichText, Ui};

use self::actions::{delete_set, save_set};
use crate::app::auth::{AuthClient, Session};
use crate::app::workouts::{WorkoutsCopy, WorkoutsState};
use crate::shared::contracts::ExerciseDto;
use crate::shared::i18n::AppLanguage;
use crate::shared::ui as theme;

pub struct WorkoutDetailsProps<'a> {
    pub client: &'a AuthClient,
    pub session: &'a Session,
    pub exercises: &'a [ExerciseDto],
    pub layout: theme::Layout,
    pub copy: WorkoutsCopy,
    pub language: AppLanguage,
}

pub fn render_workout_details(
    ui: &mut Ui,
    state: &mut WorkoutsState,
    props: WorkoutDetailsProps<'_>,
) {
    let WorkoutDetailsProps { client, session, exercises, layout, copy, language } = props;
    if state.sets_loaded_for != state.selected_workout_id {
        state.reload_sets(client, session);
    }

    theme::card_frame().show(ui, |ui| {
        ui.label(RichText::new(copy.list_title).size(18.0).strong());
        ui.add_space(8.0);

        for workout in state.workouts.clone() {
            let selected = state.selected_workout_id == Some(workout.id);
            ui.push_id(workout.id, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(
                            selected,
                            format!("{} · {}", workout.workout_date, workout.title),
                        )
                        .clicked()
                    {
                        state.selected_workout_id = Some(workout.id);
                        state.sets_loaded_for = None;
                        state.editing_set_id = None;
                        state.weight.clear();
                        state.repetitions.clear();
                    }
                    if ui.small_button(copy.edit_button).clicked() {
                        state.start_workout_edit(&workout);
                    }
                    if ui.small_button(copy.delete_button).clicked() {
                        state.delete_workout(client, session, workout.id, language);
                    }
                });
            });
        }

        if state.workouts.is_empty() {
            ui.add_space(8.0);
            ui.label(copy.empty_workouts);
            return;
        }

        ui.add_space(layout.section_gap);
        ui.separator();
        ui.add_space(layout.section_gap);
        ui.label(RichText::new(copy.sets_title).size(18.0).strong());

        if exercises.is_empty() {
            ui.add_space(8.0);
            ui.label(copy.no_exercises);
            return;
        }

        ComboBox::from_id_salt("set_exercise_id")
            .selected_text(selected_exercise_name(exercises, state.selected_exercise_id, language))
            .show_ui(ui, |ui| {
                for exercise in exercises {
                    ui.selectable_value(
                        &mut state.selected_exercise_id,
                        Some(exercise.id),
                        &exercise.name,
                    );
                }
            });
        ui.add_space(8.0);
        theme::singleline_hint(ui, &mut state.weight, copy.weight_hint);
        ui.add_space(8.0);
        theme::singleline_hint(ui, &mut state.repetitions, copy.reps_hint);
        ui.add_space(8.0);

        let primary = if state.editing_set_id.is_some() {
            copy.update_set_button
        } else {
            copy.add_set_button
        };
        if theme::full_width_button(
            ui,
            primary,
            layout.nav_item_height,
            false,
            layout.button_radius,
        )
        .clicked()
        {
            save_set(state, client, session, copy);
        }
        if state.editing_set_id.is_some()
            && theme::full_width_button(
                ui,
                copy.cancel_set_button,
                layout.nav_item_height,
                false,
                layout.button_radius,
            )
            .clicked()
        {
            state.editing_set_id = None;
            state.weight.clear();
            state.repetitions.clear();
        }

        ui.add_space(layout.section_gap);
        if state.sets.is_empty() {
            ui.label(copy.empty_sets);
            return;
        }

        Grid::new("workout_set_grid").striped(true).show(ui, |ui| {
            ui.label("#");
            ui.label(copy.exercise_column);
            ui.label(copy.weight_column);
            ui.label(copy.reps_column);
            ui.label(copy.actions_column);
            ui.end_row();

            for set in state.sets.clone() {
                ui.push_id(set.id, |ui| {
                    ui.label(set.set_order.to_string());
                    ui.label(&set.exercise_name);
                    ui.label(format!("{:.1}", set.weight));
                    ui.label(set.repetitions.to_string());
                    ui.horizontal(|ui| {
                        if ui.small_button(copy.edit_button).clicked() {
                            state.editing_set_id = Some(set.id);
                            state.selected_exercise_id = Some(set.exercise_id);
                            state.weight = format!("{:.1}", set.weight);
                            state.repetitions = set.repetitions.to_string();
                        }
                        if ui.small_button(copy.delete_button).clicked() {
                            delete_set(state, client, session, set.id, copy.set_deleted);
                        }
                    });
                    ui.end_row();
                });
            }
        });
    });
}

fn selected_exercise_name(
    exercises: &[ExerciseDto],
    selected_id: Option<i64>,
    language: AppLanguage,
) -> String {
    exercises
        .iter()
        .find(|item| Some(item.id) == selected_id)
        .map(|item| item.name.clone())
        .unwrap_or_else(|| match language {
            AppLanguage::English => "Select exercise".to_owned(),
            AppLanguage::Ukrainian => "Оберіть вправу".to_owned(),
        })
}
