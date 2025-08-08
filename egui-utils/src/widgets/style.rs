use nih_plug_egui::egui::Color32;

pub mod ferra_color {
    use nih_plug_egui::egui::Color32;

    // Ferra color palette `https://github.com/casperstorm/ferra`
    #[allow(dead_code)]
    pub(crate) const FERRA_NIGHT: Color32 = Color32::from_rgb(42, 41, 45);
    #[allow(dead_code)]
    pub(crate) const FERRA_ASH: Color32 = Color32::from_rgb(55, 53, 57);
    #[allow(dead_code)]
    pub(crate) const FERRA_UMBER: Color32 = Color32::from_rgb(77, 66, 75);
    #[allow(dead_code)]
    pub(crate) const FERRA_BARK: Color32 = Color32::from_rgb(111, 93, 99);
    #[allow(dead_code)]
    pub(crate) const FERRA_MIST: Color32 = Color32::from_rgb(209, 209, 224);
    #[allow(dead_code)]
    pub(crate) const FERRA_SAGE: Color32 = Color32::from_rgb(177, 182, 149);
    #[allow(dead_code)]
    pub(crate) const FERRA_BLUSH: Color32 = Color32::from_rgb(254, 205, 178);
    #[allow(dead_code)]
    pub(crate) const FERRA_CORAL: Color32 = Color32::from_rgb(255, 160, 122);
    #[allow(dead_code)]
    pub(crate) const FERRA_ROSE: Color32 = Color32::from_rgb(246, 182, 201);
    #[allow(dead_code)]
    pub(crate) const FERRA_EMBER: Color32 = Color32::from_rgb(224, 107, 117);
    #[allow(dead_code)]
    pub(crate) const FERRA_HONEY: Color32 = Color32::from_rgb(245, 215, 110);
}

use ferra_color::*;

pub struct WidgetStyle {
    pub element_size: f32,
    pub line_color: Color32,
    pub line_width: f32,
    pub text_color: Color32,
    pub text_size: f32,
    pub element_main_color: Color32,
    pub element_accent_color: Color32,
    pub show_hover_text: bool,
    pub outline: bool,
    pub padding: f32,
    pub show_label: bool,
    pub background_radius: f32,
    pub background_opacity: f32,
    pub background_color: Color32,
    pub highlight_color: Color32,
}

impl WidgetStyle {
    pub const fn const_default() -> Self {
        Self {
            element_size: 64_f32,
            line_color: FERRA_BLUSH,
            line_width: 4_f32,
            text_color: FERRA_BLUSH,
            text_size: 16_f32,
            element_main_color: FERRA_BARK,
            element_accent_color: FERRA_ROSE,
            show_hover_text: false,
            outline: false,
            padding: 8_f32,
            show_label: true,
            background_radius: 8_f32,
            background_opacity: 0.95_f32,
            background_color: FERRA_ASH,
            highlight_color: FERRA_MIST,
        }
    }
}

impl Default for WidgetStyle {
    fn default() -> Self {
        Self::const_default()
    }
}
