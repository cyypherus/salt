use std::rc::Rc;

use color::{AlphaColor, LinearSrgb};
use salt::Dimensions;

pub(crate) type Color = AlphaColor<LinearSrgb>;

pub(crate) enum DragPhase {
    Start,
    Move,
    End,
}

pub(crate) enum Shape<T> {
    Circle(CircleBuilder<T>),
    Rect(RectBuilder<T>),
    Text(TextBuilder<T>),
    Path(PathBuilder<T>),
}

impl<T> Shape<T> {
    pub(crate) fn on_click(&mut self, state: &mut T) {
        match self {
            Shape::Circle(builder) => builder.on_click.as_ref().map(|func| func(state)),
            Shape::Rect(builder) => builder.on_click.as_ref().map(|func| func(state)),
            Shape::Text(builder) => builder.on_click.as_ref().map(|func| func(state)),
            Shape::Path(builder) => builder.on_click.as_ref().map(|func| func(state)),
        };
    }

    pub(crate) fn on_hover(&mut self, state: &mut T, hovered: bool, point: Point) {
        match self {
            Shape::Circle(builder) => builder
                .on_hover
                .as_ref()
                .map(|func| func(state, hovered, point)),
            Shape::Rect(builder) => builder
                .on_hover
                .as_ref()
                .map(|func| func(state, hovered, point)),
            Shape::Text(builder) => builder
                .on_hover
                .as_ref()
                .map(|func| func(state, hovered, point)),
            Shape::Path(builder) => builder
                .on_hover
                .as_ref()
                .map(|func| func(state, hovered, point)),
        };
    }

    pub(crate) fn on_drag(
        &mut self,
        state: &mut T,
        phase: DragPhase,
        start: Point,
        current: Point,
    ) {
        match self {
            Shape::Circle(builder) => builder
                .on_drag
                .as_ref()
                .map(|func| func(state, phase, start, current)),
            Shape::Rect(builder) => builder
                .on_drag
                .as_ref()
                .map(|func| func(state, phase, start, current)),
            Shape::Text(builder) => builder
                .on_drag
                .as_ref()
                .map(|func| func(state, phase, start, current)),
            Shape::Path(builder) => builder
                .on_drag
                .as_ref()
                .map(|func| func(state, phase, start, current)),
        };
    }
}

// Trait for hit testing shapes
trait HitTestable {
    fn hit_test(&self, x: f32, y: f32) -> bool;
}

pub(crate) struct View<T> {
    pub(crate) shapes: Vec<Shape<T>>,
}

pub(crate) struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Point {
    pub(crate) fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TextAlign {
    Left,
    Center,
    Right,
}

type OnClick<T> = Option<Rc<dyn Fn(&mut T)>>;
type OnHover<T> = Option<Rc<dyn Fn(&mut T, bool, Point)>>;
type OnDrag<T> = Option<Rc<dyn Fn(&mut T, DragPhase, Point, Point)>>;

#[derive(Clone)]
pub struct CircleBuilder<T> {
    cx: f32,
    cy: f32,
    r: f32,
    fill: Color,
    stroke: Color,
    stroke_width: f32,
    on_click: OnClick<T>,
    on_hover: OnHover<T>,
    on_drag: OnDrag<T>,
}

#[derive(Clone)]
pub struct RectBuilder<T> {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    fill: Color,
    stroke: Color,
    stroke_width: f32,
    on_click: OnClick<T>,
    on_hover: OnHover<T>,
    on_drag: OnDrag<T>,
}

#[derive(Clone)]
pub struct TextBuilder<T> {
    x: f32,
    y: f32,
    text: String,
    font_family: String,
    font_size: f32,
    fill: Color,
    text_anchor: String,
    on_click: OnClick<T>,
    on_hover: OnHover<T>,
    on_drag: OnDrag<T>,
}

/// Represents an SVG path command
#[derive(Clone, Debug)]
pub enum PathCommand {
    /// Move to absolute coordinates (M)
    MoveTo(f32, f32),
    /// Line to absolute coordinates (L)
    LineTo(f32, f32),
    /// Cubic bezier curve to absolute coordinates (C)
    CurveTo(f32, f32, f32, f32, f32, f32),
    /// Close path (Z)
    ClosePath,
}

#[derive(Clone)]
pub struct PathBuilder<T> {
    commands: Vec<PathCommand>,
    fill: Color,
    stroke: Color,
    stroke_width: f32,
    on_click: OnClick<T>,
    on_hover: OnHover<T>,
    on_drag: OnDrag<T>,
    bounds: Option<(f32, f32, f32, f32)>, // (min_x, min_y, max_x, max_y)
    current_x: f32,
    current_y: f32,
}
pub(crate) fn circle<T>() -> CircleBuilder<T> {
    CircleBuilder {
        cx: 0.0,
        cy: 0.0,
        r: 10.0,
        fill: Color::BLACK,
        stroke: Color::BLACK,
        stroke_width: 1.0,
        on_click: None,
        on_hover: None,
        on_drag: None,
    }
}

pub(crate) fn rect<T>() -> RectBuilder<T> {
    RectBuilder {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
        fill: Color::TRANSPARENT,
        stroke: Color::BLACK,
        stroke_width: 1.0,
        on_click: None,
        on_hover: None,
        on_drag: None,
    }
}

pub(crate) fn path<T>() -> PathBuilder<T> {
    PathBuilder {
        commands: Vec::new(),
        fill: Color::TRANSPARENT,
        stroke: Color::BLACK,
        stroke_width: 1.0,
        on_click: None,
        on_hover: None,
        on_drag: None,
        bounds: None,
        current_x: 0.0,
        current_y: 0.0,
    }
}
pub(crate) fn text<T>() -> TextBuilder<T> {
    TextBuilder {
        x: 0.0,
        y: 0.0,
        text: "".to_string(),
        font_family: "sans-serif".to_string(),
        font_size: 12.0,
        fill: Color::BLACK,
        text_anchor: "start".to_string(),
        on_click: None,
        on_hover: None,
        on_drag: None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gesture {
    Tap,
    Drag,
    Hover,
}

impl<App> View<App> {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    // Circle methods
    pub fn circle(&mut self, builder: CircleBuilder<App>) -> &mut Self {
        self.shapes.push(Shape::Circle::<App>(builder));
        self
    }

    // Rectangle methods
    pub fn rect(&mut self, builder: RectBuilder<App>) -> &mut Self {
        self.shapes.push(Shape::Rect::<App>(builder));
        self
    }

    // Text methods
    pub fn text(&mut self, builder: TextBuilder<App>) -> &mut Self {
        self.shapes.push(Shape::Text::<App>(builder));
        self
    }

    // Path methods
    pub fn path(&mut self, builder: PathBuilder<App>) -> &mut Self {
        self.shapes.push(Shape::Path::<App>(builder));
        self
    }

    // Hit test the view
    pub fn hit_test(&self, x: f32, y: f32) -> Option<usize> {
        for (idx, shape) in self.shapes.iter().enumerate().rev() {
            if match shape {
                Shape::Circle(circle) => circle.hit_test(x, y),
                Shape::Rect(rect) => rect.hit_test(x, y),
                Shape::Text(text) => text.hit_test(x, y),
                Shape::Path(path) => path.hit_test(x, y),
            } {
                return Some(idx);
            }
        }
        None
    }

    // Render the view to SVG
    pub fn render(&self, dimensions: Dimensions) -> String {
        // Initialize SVG with header and viewport
        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 {} {}">"#,
            dimensions.width, dimensions.height,
        );

        // Add shapes to the SVG
        for shape in &self.shapes {
            match shape {
                Shape::Circle(circle) => {
                    svg.push_str(&format!(
                        r#"<circle cx="{}" cy="{}" r="{}" fill="{:x}" stroke="{:x}" stroke-width="{}" />"#,
                        circle.cx, circle.cy, circle.r, circle.fill.to_rgba8(), circle.stroke.to_rgba8(), circle.stroke_width
                    ));
                }
                Shape::Rect(rect) => {
                    let mut rect_str = format!(
                        r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{:x}" stroke="{:x}" stroke-width="{}" "#,
                        rect.x,
                        rect.y,
                        rect.width,
                        rect.height,
                        rect.fill.to_rgba8(),
                        rect.stroke.to_rgba8(),
                        rect.stroke_width
                    );

                    rect_str.push_str("/>");
                    svg.push_str(&rect_str);
                }
                Shape::Text(text) => {
                    svg.push_str(&format!(
                        r#"<text x="{}" y="{}" font-family="{}" font-size="{}" fill="{:x}" text-anchor="{}">{}</text>"#,
                        text.x, text.y, text.font_family, text.font_size,
                        text.fill.to_rgba8(), text.text_anchor, text.text
                    ));
                }
                Shape::Path(path) => {
                    let path_data = path.commands.iter().fold(String::new(), |mut acc, cmd| {
                        match cmd {
                            PathCommand::MoveTo(x, y) => acc.push_str(&format!("M {},{} ", x, y)),
                            PathCommand::LineTo(x, y) => acc.push_str(&format!("L {},{} ", x, y)),
                            PathCommand::CurveTo(x1, y1, x2, y2, x, y) => {
                                acc.push_str(&format!("C {},{} {},{} {},{} ", x1, y1, x2, y2, x, y))
                            }
                            PathCommand::ClosePath => acc.push_str("Z "),
                        }
                        acc
                    });

                    svg.push_str(&format!(
                        r#"<path d="{}" fill="{:x}" stroke="{:x}" stroke-width="{}" />"#,
                        path_data.trim(),
                        path.fill.to_rgba8(),
                        path.stroke.to_rgba8(),
                        path.stroke_width
                    ));
                }
            }
        }

        // Close the SVG tag
        svg.push_str("</svg>");

        svg
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }
}
// Hit testing implementations
impl<T> HitTestable for CircleBuilder<T> {
    fn hit_test(&self, x: f32, y: f32) -> bool {
        if self.on_drag.is_none() && self.on_click.is_none() && self.on_hover.is_none() {
            return false;
        }
        // Simple bounding box test for circle
        let left = self.cx - self.r;
        let right = self.cx + self.r;
        let top = self.cy - self.r;
        let bottom = self.cy + self.r;

        x >= left && x <= right && y >= top && y <= bottom
    }
}

impl<T> HitTestable for RectBuilder<T> {
    fn hit_test(&self, x: f32, y: f32) -> bool {
        if self.on_drag.is_none() && self.on_click.is_none() && self.on_hover.is_none() {
            return false;
        }
        // Simple bounds test for rectangle
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

impl<T> HitTestable for TextBuilder<T> {
    fn hit_test(&self, x: f32, y: f32) -> bool {
        if self.on_drag.is_none() && self.on_click.is_none() && self.on_hover.is_none() {
            return false;
        }
        // Simple bounding box for text
        let text_width = self.text.len() as f32 * self.font_size * 0.6;
        let text_height = self.font_size * 1.2;

        let (left, right) = match self.text_anchor.as_str() {
            "middle" => (self.x - text_width / 2.0, self.x + text_width / 2.0),
            "end" => (self.x - text_width, self.x),
            _ => (self.x, self.x + text_width), // start or default
        };

        let top = self.y - text_height;
        let bottom = self.y;

        x >= left && x <= right && y >= top && y <= bottom
    }
}

impl<T> HitTestable for PathBuilder<T> {
    fn hit_test(&self, x: f32, y: f32) -> bool {
        if self.on_drag.is_none() && self.on_click.is_none() && self.on_hover.is_none() {
            return false;
        }
        // Use the calculated bounds for hit testing
        if let Some((min_x, min_y, max_x, max_y)) = self.bounds {
            // Add stroke width to make the bounding box a bit larger
            let half_stroke = self.stroke_width / 2.0;

            x >= min_x - half_stroke
                && x <= max_x + half_stroke
                && y >= min_y - half_stroke
                && y <= max_y + half_stroke
        } else {
            false
        }
    }
}

// Builder implementations
impl<T> CircleBuilder<T> {
    pub fn cx(mut self, cx: f32) -> Self {
        self.cx = cx;
        self
    }

    pub fn cy(mut self, cy: f32) -> Self {
        self.cy = cy;
        self
    }

    pub fn r(mut self, r: f32) -> Self {
        self.r = r;
        self
    }

    pub fn fill(mut self, fill: Color) -> Self {
        self.fill = fill;
        self
    }

    pub fn stroke(mut self, stroke: Color) -> Self {
        self.stroke = stroke;
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn on_click(mut self, callback: impl Fn(&mut T) + 'static) -> Self {
        self.on_click = Some(Rc::new(callback));
        self
    }

    pub fn on_hover(mut self, callback: impl Fn(&mut T, bool, Point) + 'static) -> Self {
        self.on_hover = Some(Rc::new(callback));
        self
    }

    pub fn on_drag(mut self, callback: impl Fn(&mut T, DragPhase, Point, Point) + 'static) -> Self {
        self.on_drag = Some(Rc::new(callback));
        self
    }
}

impl<T> RectBuilder<T> {
    pub fn x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn fill(mut self, fill: Color) -> Self {
        self.fill = fill;
        self
    }

    pub fn stroke(mut self, stroke: Color) -> Self {
        self.stroke = stroke;
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn on_click(mut self, callback: impl Fn(&mut T) + 'static) -> Self {
        self.on_click = Some(Rc::new(callback));
        self
    }

    pub fn on_hover(mut self, callback: impl Fn(&mut T, bool, Point) + 'static) -> Self {
        self.on_hover = Some(Rc::new(callback));
        self
    }

    pub fn on_drag(mut self, callback: impl Fn(&mut T, DragPhase, Point, Point) + 'static) -> Self {
        self.on_drag = Some(Rc::new(callback));
        self
    }
}

impl<T> TextBuilder<T> {
    pub fn x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn font_family(mut self, font_family: impl Into<String>) -> Self {
        self.font_family = font_family.into();
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn fill(mut self, fill: Color) -> Self {
        self.fill = fill;
        self
    }

    pub fn text_align(mut self, align: TextAlign) -> Self {
        // Convert alignment to text-anchor for SVG
        let anchor = match align {
            TextAlign::Left => "start",
            TextAlign::Center => "middle",
            TextAlign::Right => "end",
        };
        self.text_anchor = anchor.to_string();
        self
    }

    pub fn on_click(mut self, callback: impl Fn(&mut T) + 'static) -> Self {
        self.on_click = Some(Rc::new(callback));
        self
    }

    pub fn on_hover(mut self, callback: impl Fn(&mut T, bool, Point) + 'static) -> Self {
        self.on_hover = Some(Rc::new(callback));
        self
    }

    pub fn on_drag(mut self, callback: impl Fn(&mut T, DragPhase, Point, Point) + 'static) -> Self {
        self.on_drag = Some(Rc::new(callback));
        self
    }
}

impl<T> PathBuilder<T> {
    // Update bounds with a new point
    fn update_bounds(&mut self, x: f32, y: f32) {
        match self.bounds {
            Some((min_x, min_y, max_x, max_y)) => {
                self.bounds = Some((min_x.min(x), min_y.min(y), max_x.max(x), max_y.max(y)));
            }
            None => {
                self.bounds = Some((x, y, x, y));
            }
        }
        self.current_x = x;
        self.current_y = y;
    }

    // Move to a point (M)
    pub fn move_to(mut self, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::MoveTo(x, y));
        self.update_bounds(x, y);
        self
    }

    // Line to a point (L)
    pub fn line_to(mut self, x: f32, y: f32) -> Self {
        // If there are no commands yet, implicitly start at (0,0)
        if self.commands.is_empty() {
            self.commands
                .push(PathCommand::MoveTo(self.current_x, self.current_y));
        }
        self.commands.push(PathCommand::LineTo(x, y));
        self.update_bounds(x, y);
        self
    }

    // Cubic bezier curve (C)
    pub fn curve_to(mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) -> Self {
        // If there are no commands yet, implicitly start at (0,0)
        if self.commands.is_empty() {
            self.commands
                .push(PathCommand::MoveTo(self.current_x, self.current_y));
        }
        self.commands
            .push(PathCommand::CurveTo(x1, y1, x2, y2, x, y));
        // Update bounds with control points and end point
        self.update_bounds(x1, y1);
        self.update_bounds(x2, y2);
        self.update_bounds(x, y);
        self
    }

    // Close path (Z)
    pub fn close_path(mut self) -> Self {
        self.commands.push(PathCommand::ClosePath);
        self
    }

    // Helper for creating a rectangle path
    pub fn rect(self, x: f32, y: f32, width: f32, height: f32) -> Self {
        self.move_to(x, y)
            .line_to(x + width, y)
            .line_to(x + width, y + height)
            .line_to(x, y + height)
            .close_path()
    }

    pub fn fill(mut self, fill: Color) -> Self {
        self.fill = fill;
        self
    }

    pub fn stroke(mut self, stroke: Color) -> Self {
        self.stroke = stroke;
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn on_click(mut self, callback: impl Fn(&mut T) + 'static) -> Self {
        self.on_click = Some(Rc::new(callback));
        self
    }

    pub fn on_hover(mut self, callback: impl Fn(&mut T, bool, Point) + 'static) -> Self {
        self.on_hover = Some(Rc::new(callback));
        self
    }

    pub fn on_drag(mut self, callback: impl Fn(&mut T, DragPhase, Point, Point) + 'static) -> Self {
        self.on_drag = Some(Rc::new(callback));
        self
    }
}
