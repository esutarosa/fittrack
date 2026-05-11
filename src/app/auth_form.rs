use eframe::egui::{Align, Color32, FontId, Margin, RichText, Sense, TextEdit, Ui, vec2};

use crate::shared::i18n::{AppLanguage, AuthStrings, auth_strings};
use crate::shared::icons::{paint_eye, paint_eye_off};
use crate::shared::ui as theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum AuthMode {
    Login,
    Register,
}

pub(super) fn auth_copy(mode: AuthMode, language: AppLanguage) -> AuthStrings {
    auth_strings(language, mode == AuthMode::Register)
}

pub(super) fn mode_switcher(
    ui: &mut Ui,
    mode: &mut AuthMode,
    layout: theme::Layout,
    strings: AuthStrings,
) {
    ui.horizontal(|ui| {
        let button_width = (ui.available_width() - ui.spacing().item_spacing.x) * 0.5;
        let login = theme::centered_button(
            ui,
            strings.switch_login,
            vec2(button_width, layout.nav_item_height),
            *mode == AuthMode::Login,
            layout.button_radius,
        );
        let register = theme::centered_button(
            ui,
            strings.switch_register,
            vec2(button_width, layout.nav_item_height),
            *mode == AuthMode::Register,
            layout.button_radius,
        );

        if login.clicked() {
            *mode = AuthMode::Login;
        }

        if register.clicked() {
            *mode = AuthMode::Register;
        }
    });
}

pub(super) fn field(ui: &mut Ui, label: &str, add_input: impl FnOnce(&mut Ui)) {
    ui.label(RichText::new(label).size(12.0).strong());
    add_input(ui);
}

pub(super) fn text_input(ui: &mut Ui, value: &mut String, layout: theme::Layout) {
    ui.add_sized(
        vec2(ui.available_width(), layout.input_height),
        TextEdit::singleline(value)
            .desired_width(f32::INFINITY)
            .font(FontId::proportional(layout.input_font_size))
            .horizontal_align(Align::LEFT)
            .vertical_align(Align::Center)
            .margin(Margin::symmetric(layout.input_padding_x, 0)),
    );
}

pub(super) fn password_input(
    ui: &mut Ui,
    value: &mut String,
    reveal: &mut bool,
    layout: theme::Layout,
    icon_color: Color32,
) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        let field_width =
            (ui.available_width() - layout.icon_button_size - ui.spacing().item_spacing.x).max(0.0);

        ui.add_sized(
            vec2(field_width, layout.input_height),
            TextEdit::singleline(value)
                .font(FontId::proportional(layout.input_font_size))
                .horizontal_align(Align::LEFT)
                .vertical_align(Align::Center)
                .margin(Margin::symmetric(layout.input_padding_x, 0))
                .password(!*reveal),
        );

        let response = ui.allocate_response(
            vec2(layout.icon_button_size, layout.icon_button_size),
            Sense::click(),
        );
        if response.clicked() {
            *reveal = !*reveal;
        }

        if *reveal {
            paint_eye(ui, &response, icon_color);
        } else {
            paint_eye_off(ui, &response, icon_color);
        }
    });
}
