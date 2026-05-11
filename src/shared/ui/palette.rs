use eframe::egui::Color32;

#[derive(Clone, Copy)]
struct Palette {
    neutral_500: Color32,
    neutral_400: Color32,
    neutral_300: Color32,
    neutral_200: Color32,
    neutral_100: Color32,
    white: Color32,
    brand_300: Color32,
    brand_200: Color32,
    brand_100: Color32,
}

#[derive(Clone, Copy)]
pub struct Colors {
    pub background: Color32,
    pub surface: Color32,
    pub surface_elevated: Color32,
    pub border: Color32,
    pub text_primary: Color32,
    pub text_muted: Color32,
    pub accent: Color32,
    pub accent_hover: Color32,
    pub accent_active: Color32,
}

fn palette() -> Palette {
    Palette {
        neutral_500: Color32::from_rgb(0x0F, 0x0F, 0x0F),
        neutral_400: Color32::from_rgb(0x14, 0x14, 0x14),
        neutral_300: Color32::from_rgb(0x23, 0x23, 0x24),
        neutral_200: Color32::from_rgb(0x50, 0x50, 0x50),
        neutral_100: Color32::from_rgb(0xA6, 0xA6, 0xA6),
        white: Color32::from_rgb(0xFF, 0xFF, 0xFF),
        brand_300: Color32::from_rgb(0x00, 0x55, 0xCC),
        brand_200: Color32::from_rgb(0x00, 0x63, 0xF2),
        brand_100: Color32::from_rgb(0x5C, 0xA0, 0xFF),
    }
}

pub fn colors() -> Colors {
    let palette = palette();

    Colors {
        background: palette.neutral_500,
        surface: palette.neutral_400,
        surface_elevated: palette.neutral_300,
        border: palette.neutral_200,
        text_primary: palette.white,
        text_muted: palette.neutral_100,
        accent: palette.brand_200,
        accent_hover: palette.brand_300,
        accent_active: palette.brand_100,
    }
}
