extern crate nih_plug;
extern crate nih_plug_egui;

extern crate egui_utils;

use egui_utils::*;
use nih_plug::prelude::ParamSetter;
use nih_plug_egui::egui::CornerRadius;
use nih_plug_egui::egui::Shadow;
use nih_plug_egui::egui::Ui;

use std::sync::Arc;

use nih_plug::editor::Editor;

use nih_plug_egui::create_egui_editor;
use nih_plug_egui::egui;
use nih_plug_egui::egui::CentralPanel;
use nih_plug_egui::EguiState;

use super::params::EguiUtilsExampleParams;

const WIDGET_STYLE: WidgetStyle = WidgetStyle::const_default().set_outline(false);
const MARIGINS: f32 = 0_f32;

fn knobs(ui: &mut Ui, params: Arc<EguiUtilsExampleParams>, param_setter: &ParamSetter) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.add(
                ArcKnob::new(
                    &params.percent,
                    param_setter,
                    KnobLayout::Square,
                    &WIDGET_STYLE.set_show_label(false),
                )
                .set_hover_text("Hover text".to_string()),
            );
            ui.add_space(MARIGINS);
            ui.add(ArcKnob::new(
                &params.percent,
                param_setter,
                KnobLayout::Square,
                &WIDGET_STYLE
                    .set_size(96_f32)
                    .set_line_width(6_f32)
                    .set_show_label(false),
            ));
            ui.add_space(MARIGINS);
            ui.add(ArcKnob::new(
                &params.percent,
                param_setter,
                KnobLayout::Square,
                &WIDGET_STYLE
                    .set_size(128_f32)
                    .set_line_width(7_f32)
                    .set_show_label(false),
            ));
        });

        ui.horizontal(|ui| {
            ui.add(ArcKnob::new(
                &params.percent,
                param_setter,
                KnobLayout::Vertical,
                &WIDGET_STYLE,
            ));

            ui.add_space(MARIGINS);
            ui.add(ArcKnob::new(
                &params.percent,
                param_setter,
                KnobLayout::Vertical,
                &WIDGET_STYLE
                    .set_size(96_f32)
                    .set_line_width(6_f32)
                    .set_text_size(20_f32),
            ));
            ui.add_space(MARIGINS);
            ui.add(ArcKnob::new(
                &params.percent,
                param_setter,
                KnobLayout::Vertical,
                &WIDGET_STYLE
                    .set_size(128_f32)
                    .set_line_width(7_f32)
                    .set_text_size(24_f32),
            ));
        });
        ui.add_space(MARIGINS);
        ui.vertical(|ui| {
            ui.add(ArcKnob::new(
                &params.percent,
                param_setter,
                KnobLayout::Horizontal,
                &WIDGET_STYLE,
            ));

            ui.add_space(MARIGINS);
            ui.add(ArcKnob::new(
                &params.percent,
                param_setter,
                KnobLayout::Horizontal,
                &WIDGET_STYLE
                    .set_size(96_f32)
                    .set_line_width(6_f32)
                    .set_text_size(32_f32),
            ));
            ui.add_space(MARIGINS);
            ui.add(ArcKnob::new(
                &params.percent,
                param_setter,
                KnobLayout::Horizontal,
                &WIDGET_STYLE
                    .set_size(128_f32)
                    .set_line_width(7_f32)
                    .set_text_size(40_f32),
            ));
        });
    });
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
    EguiState::from_size(512, 720)
}

pub(crate) fn create(
    params: Arc<EguiUtilsExampleParams>,
    state: Arc<EguiState>,
) -> Option<Box<dyn Editor>> {
    let frame = egui::Frame {
        inner_margin: 16_f32.into(),
        outer_margin: 0_f32.into(),
        corner_radius: CornerRadius::ZERO,
        shadow: Shadow::NONE,
        // fill: Color32::WHITE,
        fill: ferra_color::FERRA_NIGHT,
        stroke: egui::Stroke::NONE,
    };

    create_egui_editor(
        state,
        (),
        |cx, _| {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "futura".to_string(),
                Arc::new(egui::FontData::from_static(include_bytes!(
                    "../resources/FuturaCondensed.ttf"
                ))),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "futura".to_string());
            cx.set_fonts(fonts);
        },
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
