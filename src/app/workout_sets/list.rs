use eframe::egui::{Align, Label, Layout, RichText, Ui, vec2};

use super::actions::delete_set;
use crate::app::auth::{AuthClient, Session};
use crate::app::workouts::{WorkoutsCopy, WorkoutsState};

pub(super) fn render_sets_list(
    ui: &mut Ui,
    state: &mut WorkoutsState,
    client: &AuthClient,
    session: &Session,
    copy: WorkoutsCopy,
) {
    let order_width = 28.0;
    let weight_width = 72.0;
    let reps_width = 96.0;
    let actions_width = 152.0;
    let gap = 24.0;
    let right_width = weight_width + gap + reps_width + gap + actions_width;

    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), 24.0),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.add_sized(vec2(order_width, 24.0), Label::new(RichText::new("#").strong()));
            ui.add_space(12.0);
            ui.label(RichText::new(copy.exercise_column).strong());
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.allocate_ui_with_layout(
                    vec2(right_width, 24.0),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.add_sized(
                            vec2(actions_width, 24.0),
                            Label::new(RichText::new(copy.actions_column).strong()),
                        );
                        ui.add_space(gap);
                        ui.add_sized(
                            vec2(reps_width, 24.0),
                            Label::new(RichText::new(copy.reps_column).strong()),
                        );
                        ui.add_space(gap);
                        ui.add_sized(
                            vec2(weight_width, 24.0),
                            Label::new(RichText::new(copy.weight_column).strong()),
                        );
                    },
                );
            });
        },
    );
    ui.add_space(8.0);

    for set in state.sets.clone() {
        ui.push_id(set.id, |ui| {
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), 32.0),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.add_sized(vec2(order_width, 32.0), Label::new(set.set_order.to_string()));
                    ui.add_space(12.0);
                    ui.label(&set.exercise_name);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.allocate_ui_with_layout(
                            vec2(right_width, 32.0),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                ui.allocate_ui_with_layout(
                                    vec2(actions_width, 32.0),
                                    Layout::left_to_right(Align::Center),
                                    |ui| {
                                        ui.horizontal(|ui| {
                                            ui.spacing_mut().item_spacing.x = 24.0;
                                            if ui.small_button(copy.delete_button).clicked() {
                                                delete_set(
                                                    state,
                                                    client,
                                                    session,
                                                    set.id,
                                                    copy.set_deleted,
                                                );
                                            }
                                            if ui.small_button(copy.edit_button).clicked() {
                                                state.editing_set_id = Some(set.id);
                                                state.selected_exercise_id = Some(set.exercise_id);
                                                state.weight = format!("{:.1}", set.weight);
                                                state.repetitions = set.repetitions.to_string();
                                            }
                                        });
                                    },
                                );
                                ui.add_space(gap);
                                ui.add_sized(
                                    vec2(reps_width, 32.0),
                                    Label::new(set.repetitions.to_string()),
                                );
                                ui.add_space(gap);
                                ui.add_sized(
                                    vec2(weight_width, 32.0),
                                    Label::new(format!("{:.1}", set.weight)),
                                );
                            },
                        );
                    });
                },
            );
        });
        ui.add_space(8.0);
    }
}
