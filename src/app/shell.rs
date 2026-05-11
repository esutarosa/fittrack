use eframe::egui::{self, Button, Panel, RichText, Ui, vec2};

use crate::app::screen::Screen;
use crate::shared::ui as theme;

pub fn render_shell(ui: &mut Ui, active_screen: &mut Screen) {
    let colors = theme::semantic_colors();
    let tokens = theme::tokens();

    Panel::left("fittrack_sidebar")
        .exact_size(tokens.sidebar_width)
        .resizable(false)
        .frame(theme::sidebar_frame())
        .show_inside(ui, |ui| render_sidebar(ui, active_screen, colors, tokens));

    egui::CentralPanel::default()
        .frame(theme::page_frame())
        .show_inside(ui, |ui| render_main(ui, *active_screen, colors, tokens));
}

fn render_sidebar(
    ui: &mut Ui,
    active_screen: &mut Screen,
    colors: theme::SemanticColors,
    tokens: theme::UiTokens,
) {
    ui.vertical(|ui| {
        ui.add_space(4.0);
        ui.label(RichText::new("FitTrack").size(24.0).strong());
        ui.label(RichText::new("Local workout tracker").size(13.0).color(colors.text_muted));
        ui.add_space(tokens.section_gap);
        ui.separator();
        ui.add_space(tokens.section_gap);

        for screen in Screen::NAV_ITEMS {
            let selected = *active_screen == screen;
            let button = Button::selectable(selected, screen.nav_label())
                .frame_when_inactive(true)
                .corner_radius(tokens.button_radius)
                .min_size(vec2(ui.available_width(), tokens.nav_item_height));

            if ui.add(button).clicked() {
                *active_screen = screen;
            }
        }
    });
}

fn render_main(
    ui: &mut Ui,
    screen: Screen,
    colors: theme::SemanticColors,
    tokens: theme::UiTokens,
) {
    ui.vertical(|ui| {
        ui.add_space(tokens.page_padding);
        render_header(ui, screen, colors);
        ui.add_space(tokens.section_gap);

        match screen {
            Screen::Dashboard => render_dashboard(ui, colors, tokens),
            Screen::Exercises => render_placeholder(ui, screen, colors, tokens),
            Screen::Workouts => render_placeholder(ui, screen, colors, tokens),
            Screen::Progress => render_placeholder(ui, screen, colors, tokens),
        }
    });
}

fn render_header(ui: &mut Ui, screen: Screen, colors: theme::SemanticColors) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(RichText::new(screen.title()).size(30.0).strong());
            ui.label(RichText::new(screen.description()).size(13.0).color(colors.text_muted));
        });
    });
}

fn render_dashboard(ui: &mut Ui, colors: theme::SemanticColors, tokens: theme::UiTokens) {
    ui.columns(3, |columns| {
        metric_card(&mut columns[0], "Workouts", "0", "Current training sessions", colors, tokens);
        metric_card(&mut columns[1], "Exercises", "0", "Exercise library entries", colors, tokens);
        metric_card(&mut columns[2], "Progress", "0", "Tracked exercise records", colors, tokens);
    });

    ui.add_space(tokens.section_gap);

    theme::card_frame().show(ui, |ui| {
        ui.label(RichText::new("Shell ready").size(18.0).strong());
        ui.add_space(8.0);
        for tip in Screen::Dashboard.tips() {
            bullet_line(ui, tip, colors);
        }
    });
}

fn render_placeholder(
    ui: &mut Ui,
    screen: Screen,
    colors: theme::SemanticColors,
    tokens: theme::UiTokens,
) {
    theme::card_frame().show(ui, |ui| {
        ui.label(RichText::new(screen.title()).size(18.0).strong());
        ui.add_space(8.0);
        ui.label(
            RichText::new("This screen is reserved for the next implementation step.")
                .size(13.0)
                .color(colors.text_muted),
        );
        ui.add_space(tokens.section_gap);

        for tip in screen.tips() {
            bullet_line(ui, tip, colors);
        }
    });
}

fn metric_card(
    ui: &mut Ui,
    title: &str,
    value: &str,
    detail: &str,
    colors: theme::SemanticColors,
    tokens: theme::UiTokens,
) {
    theme::card_frame().show(ui, |ui| {
        ui.label(RichText::new(title).size(13.0).color(colors.text_muted));
        ui.add_space(8.0);
        ui.label(RichText::new(value).size(28.0).strong());
        ui.add_space(4.0);
        ui.label(RichText::new(detail).size(12.0).color(colors.text_muted));
        ui.add_space(tokens.section_gap);
    });
}

fn bullet_line(ui: &mut Ui, text: &str, colors: theme::SemanticColors) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("•").size(14.0).color(colors.accent));
        ui.label(RichText::new(text).size(13.0).color(colors.text_primary));
    });
}
