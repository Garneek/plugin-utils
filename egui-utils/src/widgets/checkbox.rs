use nih_plug::params::Param;
use nih_plug::prelude::ParamSetter;

use nih_plug_egui::egui::Color32;
use nih_plug_egui::egui::CornerRadius;
use nih_plug_egui::egui::Id;
use nih_plug_egui::egui::Rect;
use nih_plug_egui::egui::Response;
use nih_plug_egui::egui::Sense;
use nih_plug_egui::egui::Stroke;
use nih_plug_egui::egui::Ui;
use nih_plug_egui::egui::Vec2;
use nih_plug_egui::egui::Widget;

// Add some icon support

use super::WidgetStyle;

const PADDING: f32 = 6_f32;
const COLOR_ANIMATION_TIME: f32 = 0.045_f32;
const POSITION_ANIMATION_TIME: f32 = 0.045_f32;

struct ParamHandler<'a, P: Param> {
    param: &'a P,
    param_setter: &'a ParamSetter<'a>,
}

impl<'a, P: Param> ParamHandler<'a, P> {
    fn new(param: &'a P, param_setter: &'a ParamSetter) -> Self {
        Self {
            param,
            param_setter,
        }
    }

    fn handle_response(&self, response: &mut Response) -> bool {
        if response.double_clicked() {
            self.param_setter
                .set_parameter(self.param, self.param.default_plain_value());
        } else if response.clicked() {
            self.param_setter.begin_set_parameter(self.param);
            self.param_setter.set_parameter_normalized(
                self.param,
                if self.param.modulated_normalized_value() == 1.0 {
                    0.0
                } else {
                    1.0
                },
            );
            self.param_setter.end_set_parameter(self.param);
        }

        self.param.modulated_normalized_value() == 1.0
    }
}

/// Layout for the ParamCheckbox
#[derive(Clone, Copy)]
pub enum CheckboxLayout {
    /// Square checkbox
    Square,
    /// Rectangular checkbox with `y/x = f32` value
    Rect(f32),
    /// Vertical switch with animated switching
    VerticalSwitch,
    /// Horizontal switch with animated switching
    HorizontalSwitch,
}

/// Checkbox for a boolean parameter, or other param that takes either `1_f32` or `0_f32`
/// as a value
pub struct ParamCheckbox<'a, P: Param> {
    param_handler: ParamHandler<'a, P>,
    hover_text: String,
    layout: CheckboxLayout,
    size: f32,
    line_width: f32,
    line_color: Color32,
    off_color: Color32,
    on_color: Color32,
    background_color: Color32,
    highlight_color: Color32,
    show_hover_text: bool,
}

impl<'a, P: Param> ParamCheckbox<'a, P> {
    /// Create a new `ParamCheckbox` for a param with a preset
    pub fn for_param(
        param: &'a P,
        param_setter: &'a ParamSetter,
        style: &WidgetStyle,
        layout: CheckboxLayout,
    ) -> Self {
        Self {
            param_handler: ParamHandler::new(param, param_setter),
            hover_text: String::new(),
            layout,
            size: style.element_size,
            line_width: style.line_width,
            line_color: style.line_color,
            off_color: style.element_main_color,
            on_color: style.element_accent_color,
            background_color: style.background_color,
            highlight_color: style.highlight_color,
            show_hover_text: style.show_hover_text,
        }
    }

    /// Set content of hover text
    pub fn set_hover_text(mut self, text: String) -> Self {
        self.hover_text = text;
        self
    }

    // Draw the component based on a response
    fn draw(&mut self, ui: &mut Ui, val: bool, response: &mut Response, hover: bool) {
        // background
        ui.painter().rect_filled(
            response.rect.shrink(PADDING),
            CornerRadius::same(self.line_width as u8),
            self.background_color,
        );

        let mut rect = response.rect.shrink(PADDING * 1.5_f32 + self.line_width);
        let id = Id::new((rect.center().x * 10000_f32 + rect.center().y) as i64);

        // Modify the rect based on the layout
        rect = match self.layout {
            CheckboxLayout::VerticalSwitch => self.vert_switch(ui, rect, val, id),
            CheckboxLayout::HorizontalSwitch => self.hor_switch(ui, rect, val, id),
            _ => rect,
        };

        // Draw a handle rect onto the prepared rect
        self.draw_handle_rect(ui, rect, val, id);

        // hover highlight
        if hover {
            ui.painter().rect_filled(
                response.rect.shrink(PADDING * 1.1_f32 + self.line_width),
                CornerRadius::same(self.line_width as u8),
                self.highlight_color.linear_multiply(0.0075_f32),
            );
        }

        let stroke_outline = Stroke::new(self.line_width, self.line_color);

        // outline
        ui.painter().rect_stroke(
            response.rect.shrink(PADDING),
            CornerRadius::same(self.line_width as u8),
            stroke_outline,
            nih_plug_egui::egui::StrokeKind::Middle,
        );
    }

    fn draw_handle_rect(&self, ui: &mut Ui, rect: Rect, val: bool, id: Id) {
        ui.painter().rect_filled(
            rect,
            CornerRadius::same(self.line_width as u8),
            // Interpolate between on and off colors
            self.off_color.gamma_multiply(0.5_f32).lerp_to_gamma(
                self.on_color,
                ui.ctx()
                    .animate_bool_with_time(id, val, COLOR_ANIMATION_TIME),
            ),
        );
    }

    fn vert_switch(&self, ui: &mut Ui, mut rect: Rect, val: bool, id: Id) -> Rect {
        let t = rect.height() - rect.width();

        // Interpolate between base position and the end position
        rect = rect.translate(Vec2 {
            x: 0_f32,
            y: t * ui
                .ctx()
                .animate_bool_with_time(id, val, POSITION_ANIMATION_TIME),
        });

        rect.set_height(rect.width());
        rect
    }

    fn hor_switch(&self, ui: &mut Ui, mut rect: Rect, val: bool, id: Id) -> Rect {
        let t = rect.width() - rect.height();

        // Interpolate between base position and the end position
        rect = rect.translate(Vec2 {
            x: t * ui
                .ctx()
                .animate_bool_with_time(id, val, POSITION_ANIMATION_TIME),
            y: 0_f32,
        });

        rect.set_width(rect.height());
        rect
    }
}

impl<'a, P: Param> Widget for ParamCheckbox<'a, P> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let size = match self.layout {
            CheckboxLayout::Square => {
                Vec2::new(self.size + PADDING * 2_f32, self.size + PADDING * 2_f32)
            }
            CheckboxLayout::Rect(r) => Vec2::new(
                self.size + PADDING * 2_f32,
                (self.size + PADDING * 2_f32) * r,
            ),
            CheckboxLayout::VerticalSwitch => Vec2::new(
                self.size + PADDING * 2_f32,
                (self.size + PADDING * 2_f32) * 1.5_f32,
            ),
            CheckboxLayout::HorizontalSwitch => Vec2::new(
                (self.size + PADDING * 2_f32) * 1.5_f32,
                self.size + PADDING * 2_f32,
            ),
        };
        let mut response = ui.allocate_response(size, Sense::click());
        let val = self.param_handler.handle_response(&mut response);

        let hover = response.hovered();

        self.draw(ui, val, &mut response, hover);

        if self.show_hover_text {
            ui.allocate_rect(response.rect, Sense::hover())
                .on_hover_text_at_pointer(self.hover_text);
        }

        response
    }
}
