use eframe::egui::ComboBox;

use crate::shared::i18n::AppLanguage;

pub fn language_toggle(ui: &mut eframe::egui::Ui, language: &mut AppLanguage) {
    ComboBox::from_id_salt("language_toggle")
        .selected_text(language.short_label())
        .width(76.0)
        .show_ui(ui, |ui| {
            for item in AppLanguage::ALL {
                ui.selectable_value(language, item, item.short_label());
            }
        });
}
