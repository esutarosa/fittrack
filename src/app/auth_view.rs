use eframe::egui::{Align2, Area, Id, RichText, ScrollArea, Ui, vec2};

use super::auth::{AuthClient, AuthClientError, Session};
use super::auth_form::{AuthMode, auth_copy, field, mode_switcher, password_input, text_input};
use crate::shared::i18n::AppLanguage;
use crate::shared::ui as theme;

pub struct AuthView {
    mode: AuthMode,
    username: String,
    password: String,
    confirm_password: String,
    reveal_passwords: bool,
    message: Option<String>,
}

impl Default for AuthView {
    fn default() -> Self {
        Self {
            mode: AuthMode::Login,
            username: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            reveal_passwords: false,
            message: None,
        }
    }
}

impl AuthView {
    pub fn reset(&mut self) {
        self.mode = AuthMode::Login;
        self.password.clear();
        self.confirm_password.clear();
        self.reveal_passwords = false;
        self.message = None;
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        client: &AuthClient,
        language: &mut AppLanguage,
    ) -> Option<Session> {
        let layout = theme::layout();
        let mut signed_in = None;
        let viewport_height = ui.available_height();
        let content_height = auth_content_height(self.mode, layout);
        let card_width =
            layout.auth_card_width.min((ui.available_width() - layout.page_padding * 2.0).max(0.0));

        if viewport_height > content_height + layout.page_padding * 2.0 {
            Area::new(Id::new("auth_card"))
                .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
                .default_width(card_width)
                .movable(false)
                .interactable(true)
                .show(ui.ctx(), |ui| {
                    ui.set_width(card_width);
                    ui.set_max_width(card_width);
                    theme::card_frame().show(ui, |ui| {
                        show_card(
                            ui,
                            self,
                            client,
                            layout,
                            language,
                            &mut signed_in,
                            card_width - f32::from(layout.card_padding) * 2.0,
                        );
                    });
                });
        } else {
            ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.add_space(layout.page_padding);
                ui.vertical_centered(|ui| {
                    ui.set_width(card_width);
                    ui.set_max_width(card_width);
                    theme::card_frame().show(ui, |ui| {
                        show_card(
                            ui,
                            self,
                            client,
                            layout,
                            language,
                            &mut signed_in,
                            card_width - f32::from(layout.card_padding) * 2.0,
                        );
                    });
                });
                ui.add_space(layout.page_padding);
            });
        }

        signed_in
    }
}

fn show_card(
    ui: &mut Ui,
    view: &mut AuthView,
    client: &AuthClient,
    layout: theme::Layout,
    language: &mut AppLanguage,
    signed_in: &mut Option<Session>,
    content_width: f32,
) {
    let colors = theme::colors();
    let strings = auth_copy(view.mode, *language);

    ui.vertical(|ui| {
        ui.set_min_width(content_width);
        ui.set_max_width(content_width);
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.with_layout(
                eframe::egui::Layout::right_to_left(eframe::egui::Align::Center),
                |ui| {
                    theme::language_toggle(ui, language);
                },
            );
        });
        ui.add_space(20.0);
        ui.vertical_centered(|ui| {
            ui.label(RichText::new("FitTrack").size(30.0).strong());
            ui.add_space(8.0);
            ui.label(RichText::new(strings.title).size(18.0).strong());
            ui.label(RichText::new(strings.description).size(13.0).color(colors.text_muted));
        });

        ui.add_space(layout.section_gap + 4.0);
        mode_switcher(ui, &mut view.mode, layout, strings);
        ui.add_space(layout.section_gap);

        field(ui, strings.username_label, |ui| {
            text_input(ui, &mut view.username, layout);
        });
        ui.add_space(8.0);

        field(ui, strings.password_label, |ui| {
            password_input(
                ui,
                &mut view.password,
                &mut view.reveal_passwords,
                layout,
                colors.text_muted,
            );
        });

        if view.mode == AuthMode::Register {
            field(ui, strings.confirm_password_label, |ui| {
                password_input(
                    ui,
                    &mut view.confirm_password,
                    &mut view.reveal_passwords,
                    layout,
                    colors.text_muted,
                );
            });
        }

        ui.add_space(layout.section_gap);

        let submit = theme::centered_button(
            ui,
            strings.submit_label,
            vec2(ui.available_width(), layout.nav_item_height),
            false,
            layout.button_radius,
        );

        if submit.clicked() {
            let result = match view.mode {
                AuthMode::Login => client.login(&view.username, &view.password),
                AuthMode::Register => {
                    client.register(&view.username, &view.password, &view.confirm_password)
                }
            };

            match result {
                Ok(auth) => {
                    view.message =
                        Some(format!("{} {}", strings.signed_in_prefix, auth.user.username));
                    view.password.clear();
                    view.confirm_password.clear();
                    view.reveal_passwords = false;
                    *signed_in = Some(Session::from_auth(auth));
                }
                Err(error) => {
                    view.message = Some(display_error(error));
                }
            }
        }

        if let Some(message) = &view.message {
            ui.add_space(8.0);
            ui.vertical_centered(|ui| {
                ui.label(RichText::new(message).size(12.0).color(colors.text_muted));
            });
        }
    });
}

fn display_error(error: AuthClientError) -> String {
    error.to_string()
}

fn auth_content_height(mode: AuthMode, layout: theme::Layout) -> f32 {
    let confirm_block = if mode == AuthMode::Register { layout.input_height + 32.0 } else { 0.0 };

    292.0 + layout.nav_item_height * 2.0 + layout.section_gap * 3.0 + confirm_block
}
