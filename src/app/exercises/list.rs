use eframe::egui::{Grid, RichText, Ui};

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
    theme::card_frame().show(ui, |ui| {
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

        Grid::new("exercise_grid").striped(true).show(ui, |ui| {
            ui.label(RichText::new(copy.name_column).strong());
            ui.label(RichText::new(copy.group_column).strong());
            ui.label(RichText::new(copy.actions_column).strong());
            ui.end_row();

            for item in state.items.clone() {
                ui.push_id(item.id, |ui| {
                    ui.label(&item.name);
                    ui.label(&item.muscle_group);
                    ui.horizontal(|ui| {
                        if ui.small_button(copy.edit_button).clicked() {
                            state.start_edit(&item);
                        }
                        if ui.small_button(copy.delete_button).clicked() {
                            state.delete(client, session, item.id, language);
                        }
                    });
                    ui.end_row();
                });
            }
        });
    });
}
