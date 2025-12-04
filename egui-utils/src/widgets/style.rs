use nih_plug_egui::egui::Color32;

pub mod ferra_color {
    use nih_plug_egui::egui::Color32;

    // Ferra color palette `https://github.com/casperstorm/ferra`
    #[allow(dead_code)]
    pub const FERRA_NIGHT: Color32 = Color32::from_rgb(42, 41, 45);
    #[allow(dead_code)]
    pub const FERRA_ASH: Color32 = Color32::from_rgb(55, 53, 57);
    #[allow(dead_code)]
    pub const FERRA_UMBER: Color32 = Color32::from_rgb(77, 66, 75);
    #[allow(dead_code)]
    pub const FERRA_BARK: Color32 = Color32::from_rgb(111, 93, 99);
    #[allow(dead_code)]
    pub const FERRA_MIST: Color32 = Color32::from_rgb(209, 209, 224);
    #[allow(dead_code)]
    pub const FERRA_SAGE: Color32 = Color32::from_rgb(177, 182, 149);
    #[allow(dead_code)]
    pub const FERRA_BLUSH: Color32 = Color32::from_rgb(254, 205, 178);
    #[allow(dead_code)]
    pub const FERRA_CORAL: Color32 = Color32::from_rgb(255, 160, 122);
    #[allow(dead_code)]
    pub const FERRA_ROSE: Color32 = Color32::from_rgb(246, 182, 201);
    #[allow(dead_code)]
    pub const FERRA_EMBER: Color32 = Color32::from_rgb(224, 107, 117);
    #[allow(dead_code)]
    pub const FERRA_HONEY: Color32 = Color32::from_rgb(245, 215, 110);
}

use ferra_color::*;

#[derive(Clone, Copy)]
pub struct WidgetStyle {
    pub element_size: f32,
    pub line_color: Color32,
    pub line_width: f32,
    pub text_color: Color32,
    pub text_size: f32,
    pub text_value_size: f32,
    pub element_main_color: Color32,
    pub element_accent_color: Color32,
    pub show_hover_text: bool,
    pub outline: bool,
    pub padding: f32,
    pub show_label: bool,
    pub show_value: bool,
    pub background_radius: u8,
    pub background_opacity: f32,
    pub background_color: Color32,
    pub highlight_color: Color32,
    pub shadow_color: Color32,
}

impl WidgetStyle {
    pub const fn const_default() -> Self {
        Self {
            element_size: 64_f32,
            line_color: FERRA_BLUSH,
            line_width: 4_f32,
            text_color: FERRA_BLUSH,
            text_size: 16_f32,
            text_value_size: -5_f32,
            element_main_color: FERRA_BARK,
            element_accent_color: FERRA_ROSE,
            show_hover_text: true,
            outline: false,
            padding: 1_f32,
            show_label: true,
            show_value: true,
            background_radius: 8,
            background_opacity: 0.95_f32,
            background_color: FERRA_ASH,
            highlight_color: FERRA_MIST,
            shadow_color: FERRA_NIGHT,
        }
    }
}

impl Default for WidgetStyle {
    fn default() -> Self {
        Self::const_default()
    }
}

impl WidgetStyle {
    pub const fn set_size(mut self, new_size: f32) -> Self {
        self.element_size = new_size;
        self
    }

    pub const fn set_line_width(mut self, new_w: f32) -> Self {
        self.line_width = new_w;
        self
    }

    pub const fn set_line_color(mut self, n: Color32) -> Self {
        self.line_color = n;
        self
    }

    pub const fn set_text_color(mut self, n: Color32) -> Self {
        self.text_color = n;
        self
    }

    pub const fn set_text_size(mut self, n: f32) -> Self {
        self.text_size = n;
        self
    }

    pub const fn set_value_text_size(mut self, n: f32) -> Self {
        self.text_value_size = n;
        self
    }

    pub const fn set_main_color(mut self, n: Color32) -> Self {
        self.element_main_color = n;
        self
    }

    pub const fn set_accent_color(mut self, n: Color32) -> Self {
        self.element_accent_color = n;
        self
    }

    pub const fn set_show_hover_text(mut self, n: bool) -> Self {
        self.show_hover_text = n;
        self
    }

    pub const fn set_outline(mut self, n: bool) -> Self {
        self.outline = n;
        self
    }

    pub const fn set_padding(mut self, n: f32) -> Self {
        self.padding = n;
        self
    }

    pub const fn set_show_label(mut self, n: bool) -> Self {
        self.show_label = n;
        self
    }

    pub const fn set_show_value(mut self, n: bool) -> Self {
        self.show_value = n;
        self
    }

    pub const fn set_background_radius(mut self, n: u8) -> Self {
        self.background_radius = n;
        self
    }

    pub const fn set_background_opacity(mut self, n: f32) -> Self {
        self.background_opacity = n;
        self
    }

    pub const fn set_background_color(mut self, n: Color32) -> Self {
        self.background_color = n;
        self
    }

    pub const fn set_highlight_color(mut self, n: Color32) -> Self {
        self.highlight_color = n;
        self
    }

    pub const fn set_shadow_color(mut self, n: Color32) -> Self {
        self.shadow_color = n;
        self
    }
}
