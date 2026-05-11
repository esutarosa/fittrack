use eframe::egui::{self, CornerRadius, Margin, Stroke, Vec2, Visuals};

use super::{colors, layout};

pub fn apply(ctx: &egui::Context) {
    let colors = colors();
    let layout = layout();
    let mut visuals = Visuals::dark();

    visuals.window_fill = colors.background;
    visuals.panel_fill = colors.surface;
    visuals.window_stroke = Stroke::new(1.0, colors.border);
    visuals.window_corner_radius = CornerRadius::same(layout.window_radius);
    visuals.menu_corner_radius = CornerRadius::same(layout.card_radius);
    visuals.override_text_color = Some(colors.text_primary);
    visuals.hyperlink_color = colors.accent;
    visuals.selection.bg_fill = colors.accent_active;

    visuals.widgets.noninteractive.bg_fill = colors.background;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, colors.border);
    visuals.widgets.noninteractive.fg_stroke.color = colors.text_muted;
    visuals.widgets.inactive.bg_fill = colors.surface_elevated;
    visuals.widgets.inactive.fg_stroke.color = colors.text_primary;
    visuals.widgets.hovered.bg_fill = colors.accent_hover;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, colors.border);
    visuals.widgets.hovered.fg_stroke.color = colors.text_primary;
    visuals.widgets.active.bg_fill = colors.accent;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, colors.border);
    visuals.widgets.active.fg_stroke.color = colors.text_primary;
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(layout.card_radius);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(layout.button_radius);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(layout.button_radius);
    visuals.widgets.active.corner_radius = CornerRadius::same(layout.button_radius);
    visuals.widgets.open.corner_radius = CornerRadius::same(layout.button_radius);

    ctx.style_mut(|style| {
        style.spacing.item_spacing = Vec2::splat(12.0);
        style.spacing.button_padding = Vec2::new(12.0, 8.0);
        style.spacing.window_margin = Margin::same(24);
        style.spacing.menu_margin = Margin::same(12);
        style.spacing.interact_size = Vec2::new(0.0, 40.0);
        style.visuals = visuals;
    });
}
