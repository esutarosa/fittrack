use eframe::egui::{CornerRadius, Frame, Margin, Stroke};

use super::{semantic_colors, tokens};

pub fn card_frame() -> Frame {
    let colors = semantic_colors();
    let tokens = tokens();

    Frame::new()
        .fill(colors.surface_elevated)
        .stroke(Stroke::new(1.0, colors.border))
        .corner_radius(CornerRadius::same(tokens.card_radius))
        .inner_margin(Margin::same(tokens.card_padding))
}

pub fn sidebar_frame() -> Frame {
    let colors = semantic_colors();
    let tokens = tokens();

    Frame::new()
        .fill(colors.surface)
        .stroke(Stroke::new(1.0, colors.border))
        .corner_radius(CornerRadius::same(tokens.window_radius))
        .inner_margin(Margin::same(tokens.card_padding))
}

pub fn page_frame() -> Frame {
    let colors = semantic_colors();

    Frame::new().fill(colors.background).stroke(Stroke::NONE)
}
