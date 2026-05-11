use eframe::egui::{Button, RichText, TextEdit, Ui, vec2};

use super::{auth::AuthService, db::AppDatabase, models::User};
use crate::shared::ui as theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AuthMode {
    Login,
    Register,
}

pub struct AuthView {
    mode: AuthMode,
    username: String,
    password: String,
    confirm_password: String,
    message: Option<String>,
}

impl Default for AuthView {
    fn default() -> Self {
        Self {
            mode: AuthMode::Login,
            username: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            message: None,
        }
    }
}

impl AuthView {
    pub fn reset(&mut self) {
        self.mode = AuthMode::Login;
        self.password.clear();
        self.confirm_password.clear();
        self.message = None;
    }

    pub fn show(&mut self, ui: &mut Ui, db: &AppDatabase) -> Option<User> {
        let colors = theme::semantic_colors();
        let tokens = theme::tokens();
        let mut signed_in = None;

        ui.vertical_centered(|ui| {
            ui.add_space(48.0);
            ui.set_max_width(420.0);

            theme::card_frame().show(ui, |ui| {
                ui.label(RichText::new("FitTrack").size(28.0).strong());
                ui.add_space(4.0);
                ui.label(
                    RichText::new("Local authentication for the desktop MVP")
                        .size(13.0)
                        .color(colors.text_muted),
                );

                ui.add_space(tokens.section_gap);
                mode_switcher(ui, &mut self.mode, colors, tokens);
                ui.add_space(tokens.section_gap);

                field(ui, "Username", |ui| {
                    ui.add(TextEdit::singleline(&mut self.username).desired_width(f32::INFINITY));
                });
                ui.add_space(8.0);

                field(ui, "Password", |ui| {
                    ui.add(
                        TextEdit::singleline(&mut self.password)
                            .password(true)
                            .desired_width(f32::INFINITY),
                    );
                });

                if self.mode == AuthMode::Register {
                    ui.add_space(8.0);
                    field(ui, "Confirm password", |ui| {
                        ui.add(
                            TextEdit::singleline(&mut self.confirm_password)
                                .password(true)
                                .desired_width(f32::INFINITY),
                        );
                    });
                }

                ui.add_space(tokens.section_gap);

                let button_label = if self.mode == AuthMode::Login {
                    "Log in"
                } else {
                    "Create account"
                };

                let submit = ui.add(
                    Button::new(button_label)
                        .min_size(vec2(ui.available_width(), tokens.nav_item_height))
                        .corner_radius(tokens.button_radius),
                );

                if submit.clicked() {
                    let result = match self.mode {
                        AuthMode::Login => {
                            AuthService::login(db, &self.username, &self.password)
                        }
                        AuthMode::Register => AuthService::register(
                            db,
                            &self.username,
                            &self.password,
                            &self.confirm_password,
                        ),
                    };

                    match result {
                        Ok(user) => {
                            self.message = Some(format!("Signed in as {}", user.username));
                            self.password.clear();
                            self.confirm_password.clear();
                            signed_in = Some(user);
                        }
                        Err(error) => {
                            self.message = Some(error.to_string());
                        }
                    }
                }

                if let Some(message) = &self.message {
                    ui.add_space(8.0);
                    ui.label(RichText::new(message).size(12.0).color(colors.text_muted));
                }
            });
        });

        signed_in
    }
}

fn mode_switcher(ui: &mut Ui, mode: &mut AuthMode, colors: theme::SemanticColors, tokens: theme::UiTokens) {
    ui.horizontal(|ui| {
        let login = ui.add(
            Button::selectable(*mode == AuthMode::Login, "Log in")
                .frame_when_inactive(true)
                .corner_radius(tokens.button_radius)
                .min_size(vec2(120.0, tokens.nav_item_height)),
        );

        let register = ui.add(
            Button::selectable(*mode == AuthMode::Register, "Register")
                .frame_when_inactive(true)
                .corner_radius(tokens.button_radius)
                .min_size(vec2(120.0, tokens.nav_item_height)),
        );

        if login.clicked() {
            *mode = AuthMode::Login;
        }

        if register.clicked() {
            *mode = AuthMode::Register;
        }
    });

    ui.add_space(4.0);
    ui.label(
        RichText::new("Use a local username and password. Data stays in SQLite.")
            .size(12.0)
            .color(colors.text_muted),
    );
}

fn field(ui: &mut Ui, label: &str, add_input: impl FnOnce(&mut Ui)) {
    ui.label(RichText::new(label).size(12.0).strong());
    add_input(ui);
}
