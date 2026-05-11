pub mod auth;
pub mod auth_view;
pub mod db;
pub mod models;
pub mod screen;
pub mod shell;

mod db_repo;

use eframe::egui;

use crate::app::models::User;
use crate::shared::ui as theme;

pub fn run() -> anyhow::Result<()> {
    let db = db::AppDatabase::open_default()?;
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "FitTrack",
        native_options,
        Box::new(move |_cc| Ok(Box::new(FitTrackApp::new(db)))),
    )
    .map_err(Into::into)
}

struct FitTrackApp {
    db: db::AppDatabase,
    active_screen: screen::Screen,
    auth_view: auth_view::AuthView,
    current_user: Option<User>,
}

impl FitTrackApp {
    fn new(db: db::AppDatabase) -> Self {
        Self {
            db,
            active_screen: screen::Screen::Dashboard,
            auth_view: auth_view::AuthView::default(),
            current_user: None,
        }
    }
}

impl eframe::App for FitTrackApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        theme::apply(ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Some(user) = self.current_user.as_ref() {
            if shell::render_shell(ui, &mut self.active_screen, user) {
                self.current_user = None;
                self.active_screen = screen::Screen::Dashboard;
                self.auth_view.reset();
            }
            return;
        }

        if let Some(user) = self.auth_view.show(ui, &self.db) {
            self.active_screen = screen::Screen::Dashboard;
            self.current_user = Some(user);
        }
    }
}
