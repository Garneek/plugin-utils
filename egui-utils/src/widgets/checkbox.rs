use nih_plug::params::Param;
use nih_plug::prelude::ParamSetter;

use nih_plug_egui::egui::epaint::PathShape;
use nih_plug_egui::egui::Color32;
use nih_plug_egui::egui::Id;
use nih_plug_egui::egui::Rect;
use nih_plug_egui::egui::Response;
use nih_plug_egui::egui::Rounding;
use nih_plug_egui::egui::Sense;
use nih_plug_egui::egui::Shape;
use nih_plug_egui::egui::Stroke;
use nih_plug_egui::egui::Ui;
use nih_plug_egui::egui::Vec2;
use nih_plug_egui::egui::Widget;

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
            self.param_setter.set_parameter_normalized(
                self.param,
                if self.param.modulated_normalized_value() == 1.0 {
                    0.0
                } else {
                    1.0
                },
            );
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

/// `ParamCheckbox` preset. Set it as constant, if a value is set as `None` default value is used
pub struct ParamCheckboxPreset {
    /// Layout, default: `CheckboxLayout::Square`
    pub layout: Option<CheckboxLayout>,
    /// Checkbox size. For `Square` and `Rect` it is the width of the widget. For the `HorizontalSwitch`
    /// and `VerticalSwitch` it is the shorter side. Default: `16_f32`
    pub size: Option<f32>,
    /// Width of the outline of the widget. Default: `size * 0.075`
    pub line_width: Option<f32>,
    /// Outline color. Default: `BLACK`
    pub line_color: Option<Color32>,
    /// Color of the handle in `HorizontalSwitch` and `VerticalSwitch` on the off position.
    /// Default: `PLACEHOLDER`
    pub off_color: Option<Color32>,
    /// Color of the handle in the on position. Default: `LIGHT_BLUE`
    pub on_color: Option<Color32>,
    /// Background color. Default: `LIGHT_GRAY`
    pub background_color: Option<Color32>,
    /// Color applied on hover with low opacity. Default: `WHITE`
    pub highlight_color: Option<Color32>,
    /// Wheter to show a hover text or not, you need to set the hover text with
    /// `ParamCheckbox::set_hover_text`. Default: `false`
    pub show_hover_text: Option<bool>,
}

impl Default for ParamCheckboxPreset {
    fn default() -> Self {
        Self {
            layout: None,
            size: None,
            line_width: None,
            line_color: None,
            off_color: None,
            on_color: None,
            background_color: None,
            highlight_color: None,
            show_hover_text: None,
        }
    }
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
        preset: &ParamCheckboxPreset,
    ) -> Self {
        let mut temp = Self {
            param_handler: ParamHandler::new(param, param_setter),
            hover_text: String::new(),
            layout: preset.layout.unwrap_or(CheckboxLayout::Square),
            size: preset.size.unwrap_or(16_f32),
            line_width: 1_f32,
            line_color: preset.line_color.unwrap_or(Color32::BLACK),
            off_color: preset.off_color.unwrap_or(Color32::PLACEHOLDER),
            on_color: preset.on_color.unwrap_or(Color32::LIGHT_BLUE),
            background_color: preset.background_color.unwrap_or(Color32::LIGHT_GRAY),
            highlight_color: preset.highlight_color.unwrap_or(Color32::WHITE),
            show_hover_text: preset.show_hover_text.unwrap_or(false),
        };
        temp.line_width = preset.line_width.unwrap_or(temp.size * 0.075_f32);
        temp
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
            Rounding::same(self.line_width),
            self.background_color,
        );

        let mut rect = response.rect.shrink(PADDING * 1.1_f32 + self.line_width);
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
                Rounding::same(self.line_width),
                self.highlight_color.linear_multiply(0.0075_f32),
            );
        }

        let stroke_outline = Stroke::new(self.line_width, self.line_color);

        // outline
        ui.painter().rect_stroke(
            response.rect.shrink(PADDING),
            Rounding::same(self.line_width),
            stroke_outline,
        );
    }

    fn draw_handle_rect(&self, ui: &mut Ui, rect: Rect, val: bool, id: Id) {
        ui.painter().rect_filled(
            rect,
            Rounding::same(self.line_width),
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
