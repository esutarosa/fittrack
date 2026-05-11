use eframe::egui::{ComboBox, RichText, Ui};

use crate::app::auth::{AuthClient, Session};
use crate::app::exercises::{ExercisesCopy, ExercisesState, MUSCLE_GROUPS};
use crate::shared::i18n::AppLanguage;
use crate::shared::ui as theme;

pub(super) fn render_form(
    ui: &mut Ui,
    state: &mut ExercisesState,
    client: &AuthClient,
    session: &Session,
    copy: ExercisesCopy,
    layout: theme::Layout,
    language: AppLanguage,
) {
    theme::card_frame().show(ui, |ui| {
        ui.label(RichText::new(state.form_title(copy)).size(18.0).strong());
        ui.add_space(8.0);
        theme::field_label(ui, copy.name_label);
        theme::singleline(ui, &mut state.name);
        ui.add_space(8.0);
        theme::field_label(ui, copy.group_label);
        ComboBox::from_id_salt("exercise_muscle_group")
            .selected_text(MUSCLE_GROUPS[state.selected_group])
            .show_ui(ui, |ui| {
                for (index, group) in MUSCLE_GROUPS.iter().enumerate() {
                    ui.selectable_value(&mut state.selected_group, index, *group);
                }
            });
        ui.add_space(layout.section_gap);

        let primary =
            if state.editing_id.is_some() { copy.update_button } else { copy.create_button };
        if theme::full_width_button(
            ui,
            primary,
            layout.nav_item_height,
            false,
            layout.button_radius,
        )
        .clicked()
        {
            state.save(client, session, language);
        }
        if state.editing_id.is_some()
            && theme::full_width_button(
                ui,
                copy.cancel_button,
                layout.nav_item_height,
                false,
                layout.button_radius,
            )
            .clicked()
        {
            state.reset_form();
        }
        if let Some(message) = &state.message {
            ui.add_space(8.0);
            ui.label(RichText::new(message).size(12.0));
        }
    });
}
