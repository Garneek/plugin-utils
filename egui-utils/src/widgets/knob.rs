use std::f32::consts::TAU;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

use lazy_static::lazy_static;

use nih_plug::params::Param;
use nih_plug::prelude::ParamSetter;

use nih_plug_egui::egui;
use nih_plug_egui::egui::epaint::Vec2;
use nih_plug_egui::egui::Align2;
use nih_plug_egui::egui::CornerRadius;
use nih_plug_egui::egui::FontId;
use nih_plug_egui::egui::Painter;
use nih_plug_egui::egui::Pos2;
use nih_plug_egui::egui::Rect;
use nih_plug_egui::egui::Response;
use nih_plug_egui::egui::Sense;
use nih_plug_egui::egui::Shape;
use nih_plug_egui::egui::Stroke;
use nih_plug_egui::egui::Ui;
use nih_plug_egui::egui::Widget;

use super::WidgetStyle;

pub struct ArcKnob<'a, P: Param> {
    slider: SliderRegion<'a, P>,
    layout: KnobLayout,
    style: &'a WidgetStyle,
    hover_text: String,
    label_text: String,
}

impl<'a, P: Param> ArcKnob<'a, P> {
    pub fn new(
        param: &'a P,
        param_setter: &'a ParamSetter,
        layout: KnobLayout,
        style: &'a WidgetStyle,
    ) -> Self {
        Self {
            slider: SliderRegion::new(param, param_setter),
            layout,
            style,
            hover_text: String::new(),
            label_text: String::new(),
        }
    }

    pub fn set_hover_text(mut self, t: String) -> Self {
        self.hover_text = t;
        self
    }

    pub fn set_label_text(mut self, t: String) -> Self {
        self.label_text = t;
        self
    }
}

impl<'a, P: Param> Widget for ArcKnob<'a, P> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut resp = ui.allocate_response(self.get_desired_size(), Sense::click_and_drag());
        let val = self.slider.handle_response(ui, &mut resp);

        let rect = resp.rect.shrink(self.style.padding);
        let painter = ui.painter_at(rect);
        ui.painter().rect_filled(
            rect,
            CornerRadius::same(self.style.background_radius),
            self.style
                .background_color
                .gamma_multiply(self.style.background_opacity),
        );

        if self.style.outline {
            ui.painter().rect_stroke(
                rect,
                CornerRadius::same(self.style.background_radius),
                Stroke::new(2_f32, self.style.line_color),
                egui::StrokeKind::Middle,
            );
        }

        self.draw_knob(
            painter,
            self.get_knob_center(&rect),
            self.style.element_size * 0.5_f32 - self.style.line_width * 2_f32,
            val,
        );

        if self.style.show_label {
            let rect = self.get_label_rect(rect);
            let painter = ui.painter_at(rect);

            painter.rect_filled(
                rect.shrink(self.style.text_size / 8_f32),
                CornerRadius::same(self.style.background_radius),
                self.style.highlight_color.gamma_multiply(0.075),
            );

            let label_pos = rect.center();
            painter.text(
                label_pos,
                Align2::CENTER_CENTER,
                if self.label_text.is_empty() {
                    self.slider.param.name().to_owned()
                } else {
                    self.label_text
                },
                FontId::proportional(self.style.text_size),
                self.style.text_color,
            );
        }

        if self.style.show_hover_text {
            ui.allocate_rect(rect, Sense::hover())
                .on_hover_text_at_pointer(if self.hover_text.is_empty() {
                    self.slider.get_string()
                } else {
                    self.hover_text
                });
        }

        resp
    }
}

#[derive(Clone, Copy)]
pub enum KnobLayout {
    Square,
    Vertical,
    Horizontal,
}

impl Default for KnobLayout {
    fn default() -> Self {
        Self::Vertical
    }
}

impl<'a, P: Param> ArcKnob<'a, P> {
    #[inline]
    fn draw_knob(&self, painter: Painter, center: Pos2, radius: f32, v: f32) {
        let lw = self.style.line_width;

        // BG
        let bg_path = Shape::closed_line(
            get_arc_points(0.625, -0.75, center, radius + lw / 2_f32, 1_f32, 0.025_f32),
            Stroke::new(lw, self.style.line_color.gamma_multiply(0.6_f32)),
        );
        painter.add(bg_path);

        // Arc points
        let points = get_arc_points(0.625, -0.75, center, radius + lw / 2_f32, v, 0.015_f32);
        let (start, end) = (points[0], points[points.len() - 1]);

        let arc_stroke = Stroke::new(lw, self.style.element_accent_color);

        // Edges of the knob
        let points_edge = vec![
            start,
            points[1],
            center,
            if v == 1_f32 {
                points[points.len() - 2]
            } else {
                let mut p = center;
                p.y = start.y;
                p
            },
            if v == 1_f32 { end } else { start },
        ];
        painter.add(Shape::closed_line(points_edge, arc_stroke));

        // Circle
        let circle = Shape::circle_filled(center, radius, self.style.element_main_color);
        painter.add(circle);

        // Arc
        let arc_path = Shape::line(points.clone(), arc_stroke);
        painter.add(arc_path);

        // Circle BG for text
        if self.style.show_value {
            let circle = Shape::circle_filled(
                center,
                radius / 1.2_f32,
                self.style.shadow_color.gamma_multiply(0.25_f32),
            );
            painter.add(circle);

            // If font size for values is > 0, then set it as the value, otherwise scale by the
            // inverse of this value
            let font_size = if self.style.text_value_size <= 0_f32 {
                self.style.element_size / -self.style.text_value_size
            } else {
                self.style.text_value_size
            };

            painter.text(
                center,
                Align2::CENTER_CENTER,
                self.slider.get_string(),
                FontId::proportional(font_size),
                self.style.text_color,
            );
        }

        // End marker
        let unit_vec = get_arc_direction(0.625, -0.75, v);
        painter.add(Shape::line_segment(
            [
                unit_vec * (radius / 1.2_f32) + center.to_vec2(),
                unit_vec * (radius + lw) + center.to_vec2(),
            ],
            Stroke::new(lw, self.style.highlight_color),
        ));
    }

    #[inline]
    fn get_desired_size(&self) -> Vec2 {
        match self.layout {
            KnobLayout::Square => Vec2::new(
                self.style.element_size * 1_f32 + self.style.padding * 2_f32,
                self.style.element_size * 1_f32 + self.style.padding * 2_f32,
            ),
            KnobLayout::Vertical => Vec2::new(
                self.style.element_size * 1_f32 + self.style.padding * 2_f32,
                self.style.element_size * 1_f32 + self.style.text_size * 2_f32,
            ),
            KnobLayout::Horizontal => Vec2::new(
                self.style.element_size * 2.5_f32 + self.style.padding * 2_f32,
                self.style.element_size * 1_f32 + self.style.padding * 2_f32,
            ),
        }
    }

    #[inline]
    fn get_knob_center(&self, rect: &Rect) -> Pos2 {
        let mut p = match self.layout {
            KnobLayout::Square => rect.center(),
            KnobLayout::Vertical => {
                let mut p = rect.center_top();
                p.y += self.style.element_size * 0.5_f32 + self.style.padding;
                p
            }
            KnobLayout::Horizontal => {
                let mut p = rect.left_center();
                p.x += self.style.element_size * 0.5_f32 + self.style.padding;
                p
            }
        };
        p.y += self.style.line_width / 2_f32;
        p
    }

    #[inline]
    fn get_label_rect(&self, rect: Rect) -> Rect {
        match self.layout {
            KnobLayout::Square => rect,
            KnobLayout::Vertical => {
                rect.with_min_y(rect.min.y + self.style.element_size * 0.95_f32)
            }
            KnobLayout::Horizontal => {
                rect.with_min_x(rect.min.x + self.style.element_size * 0.95_f32)
            }
        }
    }
}

#[inline]
fn get_arc_direction(start: f32, end: f32, value: f32) -> Pos2 {
    let angle = (start + lerp(0.0, end, value)) * TAU;
    let (y, x) = angle.sin_cos();
    Pos2::new(x, -y)
}

#[inline]
fn get_arc_points(
    start: f32,
    end: f32,
    center: Pos2,
    radius: f32,
    value: f32,
    max_arc_distance: f32,
) -> Vec<Pos2> {
    let start_turns = start;
    let arc_length = lerp(0.0, end, value);
    let end_turns = start_turns + arc_length;

    // Calculate the number of points.  Avoid division by zero.
    let points = (if arc_length.abs() > 0.0 {
        (arc_length.abs() / max_arc_distance).ceil() as usize
    } else {
        1 // Ensure at least one point even for zero-length arcs.
    })
    .max(1);

    // Pre-calculate TAU multipliers and center offset
    let start_angle = start_turns * TAU;
    let end_angle = end_turns * TAU;
    let center_vec = center.to_vec2();

    // Use a pre-allocated vector for better performance
    let mut points_vec = Vec::with_capacity(points + 1); // +1 because of the <= in the original range

    // Iterate and calculate points
    for i in 0..=points {
        let t = i as f32 / points as f32; // Simplified division, no need for points - 1
        let angle = lerp(start_angle, end_angle, t);

        // Use direct calculation rather than calling cos/sin separately.
        let (y, x) = angle.sin_cos(); // More efficient

        points_vec.push(Pos2::new(radius * x, -radius * y) + center_vec);
    }

    points_vec
}

pub fn lerp<T>(start: T, end: T, t: f32) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<f32, Output = T> + Copy,
{
    (end - start) * t.clamp(0.0, 1.0) + start
}

//
const GRANULAR_DRAG_MULTIPLIER: f32 = 0.00035;
const NORMAL_DRAG_MULTIPLIER: f32 = 0.005;

lazy_static! {
    static ref DRAG_NORMALIZED_START_VALUE_MEMORY_ID: egui::Id = egui::Id::new((file!(), 0));
    static ref DRAG_AMOUNT_MEMORY_ID: egui::Id = egui::Id::new((file!(), 1));
    static ref VALUE_ENTRY_MEMORY_ID: egui::Id = egui::Id::new((file!(), 2));
}

struct SliderRegion<'a, P: Param> {
    param: &'a P,
    param_setter: &'a ParamSetter<'a>,
}

impl<'a, P: Param> SliderRegion<'a, P> {
    fn new(param: &'a P, param_setter: &'a ParamSetter) -> Self {
        SliderRegion {
            param,
            param_setter,
        }
    }

    fn set_normalized_value(&self, normalized: f32) {
        // This snaps to the nearest plain value if the parameter is stepped in some way.
        // TODO: As an optimization, we could add a `const CONTINUOUS: bool` to the parameter to
        //       avoid this normalized->plain->normalized conversion for parameters that don't need
        //       it
        let value = self.param.preview_plain(normalized);
        if value != self.plain_value() {
            self.param_setter.set_parameter(self.param, value);
        }
    }

    fn plain_value(&self) -> P::Plain {
        self.param.modulated_plain_value()
    }

    fn normalized_value(&self) -> f32 {
        self.param.modulated_normalized_value()
    }

    fn get_drag_normalized_start_value_memory(ui: &Ui) -> f32 {
        ui.memory(|mem| mem.data.get_temp(*DRAG_NORMALIZED_START_VALUE_MEMORY_ID))
            .unwrap_or(0.5)
    }

    fn set_drag_normalized_start_value_memory(ui: &Ui, amount: f32) {
        ui.memory_mut(|mem| {
            mem.data
                .insert_temp(*DRAG_NORMALIZED_START_VALUE_MEMORY_ID, amount)
        });
    }

    fn get_drag_amount_memory(ui: &Ui) -> f32 {
        ui.memory(|mem| mem.data.get_temp(*DRAG_AMOUNT_MEMORY_ID))
            .unwrap_or(0.0)
    }

    fn set_drag_amount_memory(ui: &Ui, amount: f32) {
        ui.memory_mut(|mem| mem.data.insert_temp(*DRAG_AMOUNT_MEMORY_ID, amount));
    }

    /// Begin and end drag still need to be called when using this..
    fn reset_param(&self) {
        self.param_setter
            .set_parameter(self.param, self.param.default_plain_value());
    }

    fn drag(&self, ui: &Ui, drag_delta: Vec2, multiplier: f32) {
        // Remember the intial position when we started with the granular drag. This value gets
        // reset whenever we have a normal itneraction with the slider.
        let start_value = if Self::get_drag_amount_memory(ui) == 0.0 {
            Self::set_drag_normalized_start_value_memory(ui, self.normalized_value());
            self.normalized_value()
        } else {
            Self::get_drag_normalized_start_value_memory(ui)
        };

        let total_drag_distance = -drag_delta.y + Self::get_drag_amount_memory(ui);
        Self::set_drag_amount_memory(ui, total_drag_distance);

        self.set_normalized_value(
            (start_value + (total_drag_distance * multiplier)).clamp(0.0, 1.0),
        );
    }

    // Handle the input for a given response. Returns an f32 containing the normalized value of
    // the parameter.
    fn handle_response(&self, ui: &Ui, response: &mut Response) -> f32 {
        // This has been replaced with the ParamSlider/CustomParamSlider structure and supporting
        // functions (above) since that was still working in egui 0.22

        if response.drag_started() {
            // When beginning a drag or dragging normally, reset the memory used to keep track of
            // our granular drag
            self.param_setter.begin_set_parameter(self.param);
            Self::set_drag_amount_memory(ui, 0.0);
        }
        if let Some(_clicked_pos) = response.interact_pointer_pos() {
            if ui.input(|mem| mem.modifiers.command) {
                // Like double clicking, Ctrl+Click should reset the parameter
                self.reset_param();
                response.mark_changed();
            } else if ui.input(|mem| mem.modifiers.shift) {
                // And shift dragging should switch to a more granular input method
                self.drag(ui, response.drag_delta(), GRANULAR_DRAG_MULTIPLIER);
                response.mark_changed();
            } else {
                self.drag(ui, response.drag_delta(), NORMAL_DRAG_MULTIPLIER);
                response.mark_changed();
                //Self::set_drag_amount_memory(ui, 0.0);
            }
        }
        if response.double_clicked() {
            self.reset_param();
            response.mark_changed();
        }
        if response.drag_stopped() {
            self.param_setter.end_set_parameter(self.param);
            Self::set_drag_amount_memory(ui, 0.0);
        }
        self.normalized_value()
    }

    fn get_string(&self) -> String {
        self.param.to_string()
    }
}
