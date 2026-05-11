use eframe::egui::{CornerRadius, Frame, Margin, Stroke};

use super::{colors, layout};

pub fn card_frame() -> Frame {
    let colors = colors();
    let layout = layout();

    Frame::new()
        .fill(colors.surface_elevated)
        .stroke(Stroke::new(1.0, colors.border))
        .corner_radius(CornerRadius::same(layout.card_radius))
        .inner_margin(Margin::same(layout.card_padding))
}

pub fn compact_card_frame() -> Frame {
    let colors = colors();
    let layout = layout();

    Frame::new()
        .fill(colors.surface_elevated)
        .stroke(Stroke::new(1.0, colors.border))
        .corner_radius(CornerRadius::same(layout.card_radius))
        .inner_margin(Margin::symmetric(8, 8))
}

pub fn sidebar_frame() -> Frame {
    let colors = colors();
    let layout = layout();

    Frame::new()
        .fill(colors.surface)
        .stroke(Stroke::new(1.0, colors.border))
        .corner_radius(CornerRadius::same(layout.window_radius))
        .inner_margin(Margin::same(layout.card_padding))
}

pub fn page_frame() -> Frame {
    let colors = colors();

    Frame::new().fill(colors.background).stroke(Stroke::NONE)
}
