use eframe::egui::{Color32, Response, Stroke, Ui, vec2};

use super::paint_eye;

pub fn paint_eye_off(ui: &Ui, response: &Response, color: Color32) {
    let rect = response.rect.shrink2(vec2(2.0, 4.0));
    let stroke = Stroke::new(1.5, color);
    let slash_pad = 1.0;

    paint_eye(ui, response, color);
    ui.painter().line_segment(
        [
            rect.left_top() + vec2(slash_pad, slash_pad),
            rect.right_bottom() - vec2(slash_pad, slash_pad),
        ],
        stroke,
    );
}
