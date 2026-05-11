#[derive(Clone, Copy)]
pub struct Layout {
    pub sidebar_width: f32,
    pub page_padding: f32,
    pub section_gap: f32,
    pub card_padding: i8,
    pub nav_item_height: f32,
    pub input_height: f32,
    pub input_padding_x: i8,
    pub input_font_size: f32,
    pub auth_card_width: f32,
    pub auth_card_height: f32,
    pub icon_button_size: f32,
    pub button_radius: u8,
    pub card_radius: u8,
    pub window_radius: u8,
}

pub fn layout() -> Layout {
    Layout {
        sidebar_width: 264.0,
        page_padding: 24.0,
        section_gap: 16.0,
        card_padding: 16,
        nav_item_height: 40.0,
        input_height: 36.0,
        input_padding_x: 12,
        input_font_size: 14.0,
        auth_card_width: 420.0,
        auth_card_height: 476.0,
        icon_button_size: 24.0,
        button_radius: 10,
        card_radius: 12,
        window_radius: 12,
    }
}
