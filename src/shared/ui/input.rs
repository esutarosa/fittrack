use eframe::egui::{
    Align, CornerRadius, FontId, Frame, Margin, RichText, TextEdit, Ui, Vec2, vec2,
};

use super::{colors, layout};

pub fn field_label(ui: &mut Ui, label: &str) {
    ui.label(RichText::new(label).size(12.0).strong());
}

pub fn singleline(ui: &mut Ui, value: &mut String) {
    let layout = layout();
    let width = ui.available_width();
    let frame = input_frame();

    ui.allocate_ui_with_layout(
        vec2(width, layout.input_height),
        eframe::egui::Layout::left_to_right(Align::Center),
        |ui| {
            frame.show(ui, |ui| {
                ui.add_sized(
                    Vec2::new(ui.available_width(), layout.input_height),
                    TextEdit::singleline(value)
                        .frame(false)
                        .font(FontId::proportional(layout.input_font_size))
                        .horizontal_align(Align::LEFT)
                        .vertical_align(Align::Center)
                        .margin(Margin::ZERO),
                );
            });
        },
    );
}

pub fn singleline_hint(ui: &mut Ui, value: &mut String, hint: &str) {
    let layout = layout();
    let width = ui.available_width();
    let frame = input_frame();

    ui.allocate_ui_with_layout(
        vec2(width, layout.input_height),
        eframe::egui::Layout::left_to_right(Align::Center),
        |ui| {
            frame.show(ui, |ui| {
                ui.add_sized(
                    Vec2::new(ui.available_width(), layout.input_height),
                    TextEdit::singleline(value)
                        .frame(false)
                        .hint_text(hint)
                        .font(FontId::proportional(layout.input_font_size))
                        .horizontal_align(Align::LEFT)
                        .vertical_align(Align::Center)
                        .margin(Margin::ZERO),
                );
            });
        },
    );
}

pub fn multiline(ui: &mut Ui, value: &mut String, rows: usize) {
    let frame = input_frame();

    frame.show(ui, |ui| {
        ui.add(
            TextEdit::multiline(value)
                .frame(false)
                .desired_width(f32::INFINITY)
                .desired_rows(rows)
                .font(FontId::proportional(layout().input_font_size))
                .margin(Margin::ZERO),
        );
    });
}

fn input_frame() -> Frame {
    let colors = colors();
    let layout = layout();

    Frame::new()
        .fill(colors.background)
        .stroke(eframe::egui::Stroke::new(1.0, colors.border))
        .corner_radius(CornerRadius::same(layout.button_radius))
        .inner_margin(Margin::symmetric(layout.input_padding_x, 0))
}
