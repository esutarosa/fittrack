use eframe::egui::{Ui, vec2};

use super::{Layout, centered_button};
use crate::shared::i18n::AppLanguage;

pub fn language_toggle(ui: &mut Ui, language: &mut AppLanguage, layout: Layout) {
    ui.horizontal(|ui| {
        for item in AppLanguage::ALL {
            if centered_button(
                ui,
                item.short_label(),
                vec2(52.0, layout.input_height),
                *language == item,
                layout.button_radius,
            )
            .clicked()
            {
                *language = item;
            }
        }
    });
}
