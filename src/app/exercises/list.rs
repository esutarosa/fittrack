use eframe::egui::{Align, Layout, RichText, Ui, vec2};

use crate::app::auth::{AuthClient, Session};
use crate::app::exercises::{ExercisesCopy, ExercisesState};
use crate::shared::i18n::AppLanguage;
use crate::shared::ui as theme;

pub(super) fn render_list(
    ui: &mut Ui,
    state: &mut ExercisesState,
    client: &AuthClient,
    session: &Session,
    copy: ExercisesCopy,
    language: AppLanguage,
) {
    theme::compact_card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new(copy.list_title).size(18.0).strong());
            if ui.button(copy.refresh_button).clicked() {
                state.reload(client, session);
            }
        });
        ui.add_space(8.0);
        if state.items.is_empty() {
            ui.label(copy.empty_state);
            return;
        }

        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), 24.0),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.label(RichText::new(copy.exercise_column).strong());
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new(copy.actions_column).strong());
                });
            },
        );
        ui.add_space(8.0);

        for item in state.items.clone() {
            ui.push_id(item.id, |ui| {
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), 32.0),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.label(&item.name);
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 24.0;
                                if ui.small_button(copy.delete_button).clicked() {
                                    state.delete(client, session, item.id, language);
                                }
                                if ui.small_button(copy.edit_button).clicked() {
                                    state.start_edit(&item);
                                }
                            });
                        });
                    },
                );
            });
            ui.add_space(8.0);
        }
    });
}
