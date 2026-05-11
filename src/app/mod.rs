pub mod auth;
pub mod auth_view;
pub mod shell;

use eframe::egui;

use crate::shared::ui as theme;

pub fn run() -> anyhow::Result<()> {
    let client = auth::AuthClient::from_env()?;
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "FitTrack",
        native_options,
        Box::new(move |_cc| Ok(Box::new(FitTrackApp::new(client)))),
    )
    .map_err(|error| anyhow::anyhow!(error.to_string()))
}

struct FitTrackApp {
    client: auth::AuthClient,
    active_screen: shell::Screen,
    auth_view: auth_view::AuthView,
    session: Option<auth::Session>,
}

impl FitTrackApp {
    fn new(client: auth::AuthClient) -> Self {
        Self {
            client,
            active_screen: shell::Screen::Dashboard,
            auth_view: auth_view::AuthView::default(),
            session: None,
        }
    }
}

impl eframe::App for FitTrackApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        theme::apply(ctx);

        if let Some(session) = self.session.as_ref() {
            if shell::render_shell(ctx, &mut self.active_screen, session) {
                self.session = None;
                self.active_screen = shell::Screen::Dashboard;
                self.auth_view.reset();
            }
            return;
        }

        egui::CentralPanel::default().frame(theme::page_frame()).show(ctx, |ui| {
            if let Some(session) = self.auth_view.show(ui, &self.client) {
                self.active_screen = shell::Screen::Dashboard;
                self.session = Some(session);
            }
        });
    }
}
