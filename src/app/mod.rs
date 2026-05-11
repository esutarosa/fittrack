mod screen;

use eframe::egui::{self, RichText};

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
        let colors = theme::semantic_colors();

        egui::CentralPanel::default()
            .frame(theme::page_frame())
            .show_inside(ui, |ui| {
                ui.add_space(24.0);
                ui.label(RichText::new(self.active_screen.title()).size(30.0).strong());
                ui.label(
                    RichText::new(self.active_screen.description())
                        .size(13.0)
                        .color(colors.text_muted),
                );
            });
    }
}
