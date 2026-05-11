use eframe::egui::{RichText, Ui};

use crate::app::auth::{AuthClient, Session};
use crate::app::workouts::{WorkoutsCopy, WorkoutsState};
use crate::shared::i18n::AppLanguage;
use crate::shared::ui as theme;

pub(super) fn render_workout_form(
    ui: &mut Ui,
    state: &mut WorkoutsState,
    client: &AuthClient,
    session: &Session,
    layout: theme::Layout,
    copy: WorkoutsCopy,
    language: AppLanguage,
) {
    theme::card_frame().show(ui, |ui| {
        let title =
            if state.editing_workout_id.is_some() { copy.edit_form_title } else { copy.form_title };
        ui.label(RichText::new(title).size(18.0).strong());
        ui.add_space(8.0);
        theme::field_label(ui, copy.title_label);
        theme::singleline(ui, &mut state.title);
        ui.add_space(8.0);
        theme::field_label(ui, copy.date_label);
        theme::singleline(ui, &mut state.workout_date);
        ui.add_space(8.0);
        theme::field_label(ui, copy.notes_label);
        theme::multiline(ui, &mut state.notes, 3);
        ui.add_space(layout.section_gap);

        let primary = if state.editing_workout_id.is_some() {
            copy.update_button
        } else {
            copy.create_button
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
            state.save_workout(client, session, language);
        }
        if state.editing_workout_id.is_some()
            && theme::full_width_button(
                ui,
                copy.cancel_button,
                layout.nav_item_height,
                false,
                layout.button_radius,
            )
            .clicked()
        {
            state.reset_workout_form();
        }
        if let Some(message) = &state.message {
            ui.add_space(8.0);
            ui.label(message);
        }
    });
}
