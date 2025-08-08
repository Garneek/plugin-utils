extern crate nih_plug;
extern crate nih_plug_egui;

extern crate egui_utils;

use egui_utils::*;
use nih_plug::prelude::ParamSetter;
use nih_plug_egui::egui::Rounding;
use nih_plug_egui::egui::Shadow;
use nih_plug_egui::egui::Ui;

use std::sync::Arc;

use nih_plug::editor::Editor;

use nih_plug_egui::create_egui_editor;
use nih_plug_egui::egui;
use nih_plug_egui::egui::CentralPanel;
use nih_plug_egui::EguiState;

use super::params::EguiUtilsExampleParams;

mod style;
use style::*;

const WIDGET_STYLE: WidgetStyle = WidgetStyle::const_default();

fn knobs(ui: &mut Ui, params: Arc<EguiUtilsExampleParams>, param_setter: &ParamSetter) {
    // ui.add(ArcKnob::for_param(
    //     &params.percent,
    //     param_setter,
    //     &KNOB_PRESET_1,
    // ));
}

fn switches(ui: &mut Ui, params: Arc<EguiUtilsExampleParams>, param_setter: &ParamSetter) {
    ui.add(ParamCheckbox::for_param(
        &params.bool,
        param_setter,
        &WIDGET_STYLE,
        CheckboxLayout::Square,
    ));
}

pub(crate) fn default_state() -> Arc<EguiState> {
    EguiState::from_size(512, 1024)
}

pub(crate) fn create(
    params: Arc<EguiUtilsExampleParams>,
    state: Arc<EguiState>,
) -> Option<Box<dyn Editor>> {
    let frame = egui::Frame {
        inner_margin: 16_f32.into(),
        outer_margin: 0_f32.into(),
        rounding: Rounding::ZERO,
        shadow: Shadow::NONE,
        fill: FERRA_NIGHT,
        stroke: egui::Stroke::NONE,
    };

    create_egui_editor(
        state,
        (),
        |_, _| {},
        move |cx, setter, _user_state| {
            CentralPanel::default().frame(frame).show(cx, |ui| {
                ui.vertical(|ui| {
                    knobs(ui, params.clone(), setter);
                    ui.add_space(16_f32);
                    switches(ui, params.clone(), setter);
                    ui.add_space(16_f32);
                });
            });
        },
    )
}
