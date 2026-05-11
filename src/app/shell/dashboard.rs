use eframe::egui::{RichText, Ui};

use super::{Screen, strings};
use crate::shared::i18n::AppLanguage;
use crate::shared::ui as theme;

pub(super) fn render_header(
    ui: &mut Ui,
    screen: Screen,
    language: AppLanguage,
    colors: theme::Colors,
) {
    ui.label(RichText::new(strings::screen_title(screen, language)).size(30.0).strong());
    ui.label(
        RichText::new(strings::screen_description(screen, language))
            .size(13.0)
            .color(colors.text_muted),
    );
}

pub(super) fn render_dashboard(
    ui: &mut Ui,
    language: AppLanguage,
    exercise_count: usize,
    workout_count: usize,
    progress_count: usize,
    colors: theme::Colors,
    layout: theme::Layout,
) {
    let metrics = strings::metric_copy(language);
    ui.columns(3, |columns| {
        metric_card(
            &mut columns[0],
            metrics[0].0,
            &workout_count.to_string(),
            metrics[0].1,
            colors,
            layout,
        );
        metric_card(
            &mut columns[1],
            metrics[1].0,
            &exercise_count.to_string(),
            metrics[1].1,
            colors,
            layout,
        );
        metric_card(
            &mut columns[2],
            metrics[2].0,
            &progress_count.to_string(),
            metrics[2].1,
            colors,
            layout,
        );
    });
}

fn metric_card(
    ui: &mut Ui,
    title: &str,
    value: &str,
    detail: &str,
    colors: theme::Colors,
    layout: theme::Layout,
) {
    theme::card_frame().show(ui, |ui| {
        ui.label(RichText::new(title).size(13.0).color(colors.text_muted));
        ui.add_space(8.0);
        ui.label(RichText::new(value).size(28.0).strong());
        ui.add_space(4.0);
        ui.label(RichText::new(detail).size(12.0).color(colors.text_muted));
        ui.add_space(layout.section_gap);
    });
}
