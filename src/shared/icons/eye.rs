use eframe::egui::{Color32, Response, Shape, Stroke, Ui, vec2};

pub fn paint_eye(ui: &Ui, response: &Response, color: Color32) {
    let rect = response.rect.shrink2(vec2(2.0, 4.0));
    let center = rect.center();
    let half_width = rect.width() * 0.5;
    let half_height = rect.height() * 0.32;
    let stroke = Stroke::new(1.5, color);
    let points = vec![
        center + vec2(-half_width, 0.0),
        center + vec2(-half_width * 0.45, -half_height),
        center + vec2(half_width * 0.45, -half_height),
        center + vec2(half_width, 0.0),
        center + vec2(half_width * 0.45, half_height),
        center + vec2(-half_width * 0.45, half_height),
        center + vec2(-half_width, 0.0),
    ];

    ui.painter().add(Shape::line(points, stroke));
    ui.painter().circle_stroke(center, rect.height() * 0.18, stroke);
}
