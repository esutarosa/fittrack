mod screen;
mod shell;

use eframe::egui;

use crate::shared::ui as theme;

pub fn run() -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "FitTrack",
        native_options,
        Box::new(|_cc| Ok(Box::new(FitTrackApp::default()))),
    )
    .map_err(Into::into)
}

struct FitTrackApp {
    active_screen: screen::Screen,
}

impl Default for FitTrackApp {
    fn default() -> Self {
        Self { active_screen: screen::Screen::Dashboard }
    }
}

impl eframe::App for FitTrackApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        theme::apply(ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        shell::render_shell(ui, &mut self.active_screen);
    }
}
