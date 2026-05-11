use eframe::egui::{Align2, FontId, Response, Sense, StrokeKind, Ui, Vec2};

pub fn centered_button(
    ui: &mut Ui,
    label: &str,
    size: Vec2,
    selected: bool,
    corner_radius: u8,
) -> Response {
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact_selectable(&response, selected);
        ui.painter().rect(
            rect,
            corner_radius,
            visuals.weak_bg_fill,
            visuals.bg_stroke,
            StrokeKind::Middle,
        );
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            label,
            FontId::proportional(16.0),
            visuals.text_color(),
        );
    }

    response
}

pub fn full_width_button(
    ui: &mut Ui,
    label: &str,
    height: f32,
    selected: bool,
    corner_radius: u8,
) -> Response {
    centered_button(ui, label, Vec2::new(ui.available_width(), height), selected, corner_radius)
}
