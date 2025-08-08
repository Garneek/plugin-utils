extern crate egui_utils;

use egui_utils::{CheckboxLayout, KnobPreset};
use nih_plug_egui::egui::Color32;

// Ferra color palette

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

#[allow(dead_code)]
pub const KNOB_PRESET_1: KnobPreset = KnobPreset {
    radius: Some(32_f32),
    line_color: Some(FERRA_ROSE),
    center_size: None,
    line_width: None,
    knob_color: Some(FERRA_BARK),
    center_to_line_space: None,
    hover_text: Some(true),
    show_center_value: None,
    text_size: Some(14_f32),
    outline: Some(true),
    padding: None,
    show_label: None,
    swap_label_and_value: None,
    text_color_override: Some(FERRA_ROSE),
    readable_box: None,
    background_radius: None,
    background_opacity: None,
    background_color: Some(FERRA_UMBER),
    layout: Some(egui_utils::KnobLayout::Vertical),
    arc_start: None,
    arc_end: None,
};
