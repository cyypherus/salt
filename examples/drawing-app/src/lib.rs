use backer::{nodes::*, Layout};
use salt::{salt_app, App, Dimensions, EventType, MouseEvent};
use wasm_bindgen::prelude::*;

// A single stroke with its color
struct Stroke {
    points: Vec<(f64, f64)>,
    color: String,
}

// Drawing application that tracks points when dragging the mouse
pub struct DrawingApp {
    // Collection of strokes for the user's drawing
    strokes: Vec<Stroke>,
    // Current drawing state
    is_drawing: bool,
    // Currently selected color
    current_color: String,
    // Was a button clicked
    button_clicked: Option<String>,
    view: Option<View>,
}

impl App for DrawingApp {
    fn new() -> Self {
        Self {
            strokes: Vec::new(),
            is_drawing: false,
            current_color: String::from("black"),
            button_clicked: None,
            view: None,
        }
    }

    fn handle_event(&mut self, event: MouseEvent) -> bool {
        // Check if a button was clicked
        if event.event_type == EventType::MouseDown {
            let button_height = 30.0;
            let clear_button_width = 60.0;
            let color_button_width = 30.0;
            let colors = ["black", "red", "green", "blue", "purple"];

            // Check for clear button click
            if event.y >= 30.0 && event.y <= 30.0 + button_height && event.x <= clear_button_width {
                self.strokes.clear();
                self.button_clicked = Some("clear".to_string());
                return true;
            }

            // Check for color button clicks
            let mut x_offset = clear_button_width + 10.0;
            for color in colors.iter() {
                if event.y >= 30.0
                    && event.y <= 30.0 + button_height
                    && event.x >= x_offset
                    && event.x <= x_offset + color_button_width
                {
                    self.current_color = color.to_string();
                    self.button_clicked = Some(color.to_string());
                    return true;
                }
                x_offset += color_button_width + 5.0;
            }
        }

        // Reset button clicked state on mouse up
        if event.event_type == EventType::MouseUp {
            self.button_clicked = None;
        }

        // Only continue with drawing if we're below the button area
        if event.y < 70.0 {
            return false;
        }

        match event.event_type {
            EventType::MouseDown => {
                // Start a new stroke and add the first point
                self.is_drawing = true;
                let new_stroke = Stroke {
                    points: vec![(event.x, event.y)],
                    color: self.current_color.clone(),
                };
                self.strokes.push(new_stroke);
                true
            }
            EventType::MouseMove => {
                // Add points to the current stroke while drawing
                if self.is_drawing {
                    if let Some(current_stroke) = self.strokes.last_mut() {
                        current_stroke.points.push((event.x, event.y));
                    }
                    true
                } else {
                    false
                }
            }
            EventType::MouseUp => {
                // Stop drawing
                self.is_drawing = false;
                true
            }
            _ => false,
        }
    }

    fn render(&self, dimensions: Dimensions) -> String {
        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
            dimensions.width, dimensions.height, dimensions.width, dimensions.height
        );

        // Background
        svg.push_str(&format!(
            r#"<rect x="0" y="0" width="{}" height="{}" fill="white" />"#,
            dimensions.width, dimensions.height
        ));

        // Instructions
        svg.push_str(r#"<text x="10" y="20" font-family="sans-serif" font-size="14" fill="black">Click and drag to draw</text>"#);

        // Clear button
        let button_y = 30.0;
        let button_height = 30.0;
        let clear_button_width = 60.0;

        // Clear button - highlight if clicked
        let clear_fill = if self.button_clicked.as_deref() == Some("clear") {
            "lightgray"
        } else {
            "white"
        };
        svg.push_str(&format!(
            r#"<rect x="0" y="{}" width="{}" height="{}" fill="{}" stroke="black" />"#,
            button_y, clear_button_width, button_height, clear_fill
        ));
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="sans-serif" font-size="12" text-anchor="middle" fill="black">Clear</text>"#,
            clear_button_width / 2.0, button_y + button_height / 2.0 + 5.0
        ));

        // Color buttons
        let colors = ["black", "red", "green", "blue", "purple"];
        let color_button_width = 30.0;
        let mut x_offset = clear_button_width + 10.0;

        for color in colors.iter() {
            // Highlight the selected color or if it was clicked
            let is_selected = &self.current_color == color;
            let is_clicked = self.button_clicked.as_deref() == Some(color);
            let stroke_width = if is_selected { "2" } else { "1" };
            let button_fill = if is_clicked { "lightgray" } else { color };

            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="black" stroke-width="{}" />"#,
                x_offset, button_y, color_button_width, button_height, button_fill, stroke_width
            ));

            x_offset += color_button_width + 5.0;
        }

        // UI separator line
        svg.push_str(&format!(
            r#"<line x1="0" y1="70" x2="{}" y2="70" stroke="lightgray" stroke-width="1" />"#,
            dimensions.width
        ));

        // Draw all strokes
        for stroke in &self.strokes {
            if stroke.points.is_empty() {
                continue;
            }

            svg.push_str(&format!(r#"<path d=""#));

            // Start at the first point
            if let Some((first_x, first_y)) = stroke.points.first() {
                svg.push_str(&format!("M {},{} ", first_x, first_y));

                // Add line segments to each subsequent point
                for (x, y) in &stroke.points[1..] {
                    svg.push_str(&format!("L {},{} ", x, y));
                }
            }

            svg.push_str(&format!(
                r#"" stroke="{}" stroke-width="2" fill="none" />"#,
                stroke.color
            ));
        }

        svg.push_str("</svg>");
        svg
    }
}

fn view(app: &mut DrawingApp) -> String {
    let base = View::new();
    Layout::new(dynamic(|_: &mut View| {
        stack(vec![
            //>
            column(vec![
                //>
                draw(|area, view: &mut View| {
                    view.rect(
                        rect()
                            .fill("white")
                            .stroke("black")
                            .stroke_width(1.0)
                            .x(area.x)
                            .y(area.y)
                            .width(area.width)
                            .height(area.height),
                    );
                }),
            ]),
        ])
    }));
    base.render()
}
// Define shape types for our views
enum Shape {
    Circle(CircleBuilder),
    Rect(RectBuilder),
    Text(TextBuilder),
    Line(LineBuilder),
}

// Trait for hit testing shapes
trait HitTestable {
    fn hit_test(&self, x: f32, y: f32) -> bool;
}

struct View {
    shapes: Vec<Shape>,
}

// Builder structs for shape configuration
pub struct CircleBuilder {
    cx: f32,
    cy: f32,
    r: f32,
    fill: String,
    stroke: String,
    stroke_width: f32,
}

pub struct RectBuilder {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    fill: String,
    stroke: String,
    stroke_width: f32,
    rx: Option<f32>,
    ry: Option<f32>,
}

pub struct TextBuilder {
    x: f32,
    y: f32,
    text: String,
    font_family: String,
    font_size: f32,
    fill: String,
    text_anchor: String,
}

pub struct LineBuilder {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    stroke: String,
    stroke_width: f32,
}

pub fn circle() -> CircleBuilder {
    CircleBuilder {
        cx: 0.0,
        cy: 0.0,
        r: 10.0,
        fill: "none".to_string(),
        stroke: "black".to_string(),
        stroke_width: 1.0,
    }
}

pub fn rect() -> RectBuilder {
    RectBuilder {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
        fill: "none".to_string(),
        stroke: "black".to_string(),
        stroke_width: 1.0,
        rx: None,
        ry: None,
    }
}

pub fn line() -> LineBuilder {
    LineBuilder {
        x1: 0.0,
        y1: 0.0,
        x2: 100.0,
        y2: 100.0,
        stroke: "black".to_string(),
        stroke_width: 1.0,
    }
}

pub fn text() -> TextBuilder {
    TextBuilder {
        x: 0.0,
        y: 0.0,
        text: "".to_string(),
        font_family: "sans-serif".to_string(),
        font_size: 12.0,
        fill: "black".to_string(),
        text_anchor: "start".to_string(),
    }
}

impl View {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    // Circle methods
    pub fn circle(&mut self, builder: CircleBuilder) -> &mut Self {
        self.shapes.push(Shape::Circle(builder));
        self
    }

    // Rectangle methods
    pub fn rect(&mut self, builder: RectBuilder) -> &mut Self {
        self.shapes.push(Shape::Rect(builder));
        self
    }

    // Text methods
    pub fn text(&mut self, builder: TextBuilder) -> &mut Self {
        self.shapes.push(Shape::Text(builder));
        self
    }

    // Line methods
    pub fn line(&mut self, builder: LineBuilder) -> &mut Self {
        self.shapes.push(Shape::Line(builder));
        self
    }

    // Hit test the view
    pub fn hit_test(&self, x: f32, y: f32) -> Option<usize> {
        for (idx, shape) in self.shapes.iter().enumerate().rev() {
            if match shape {
                Shape::Circle(circle) => circle.hit_test(x, y),
                Shape::Rect(rect) => rect.hit_test(x, y),
                Shape::Line(line) => line.hit_test(x, y),
                Shape::Text(text) => text.hit_test(x, y),
            } {
                return Some(idx);
            }
        }
        None
    }

    // Render the view to SVG
    pub fn render(&self) -> String {
        let mut svg = String::new();

        for shape in &self.shapes {
            match shape {
                Shape::Circle(circle) => {
                    svg.push_str(&format!(
                        r#"<circle cx="{}" cy="{}" r="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
                        circle.cx, circle.cy, circle.r, circle.fill, circle.stroke, circle.stroke_width
                    ));
                }
                Shape::Rect(rect) => {
                    let mut rect_str = format!(
                        r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" "#,
                        rect.x,
                        rect.y,
                        rect.width,
                        rect.height,
                        rect.fill,
                        rect.stroke,
                        rect.stroke_width
                    );

                    if let Some(rx) = rect.rx {
                        rect_str.push_str(&format!(r#"rx="{}" "#, rx));
                    }

                    if let Some(ry) = rect.ry {
                        rect_str.push_str(&format!(r#"ry="{}" "#, ry));
                    }

                    rect_str.push_str("/>");
                    svg.push_str(&rect_str);
                }
                Shape::Text(text) => {
                    svg.push_str(&format!(
                        r#"<text x="{}" y="{}" font-family="{}" font-size="{}" fill="{}" text-anchor="{}">{}</text>"#,
                        text.x, text.y, text.font_family, text.font_size,
                        text.fill, text.text_anchor, text.text
                    ));
                }
                Shape::Line(line) => {
                    svg.push_str(&format!(
                        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" />"#,
                        line.x1, line.y1, line.x2, line.y2, line.stroke, line.stroke_width
                    ));
                }
            }
        }

        svg
    }
}

// Hit testing implementations
impl HitTestable for CircleBuilder {
    fn hit_test(&self, x: f32, y: f32) -> bool {
        let dx = x - self.cx;
        let dy = y - self.cy;
        dx * dx + dy * dy <= self.r * self.r
    }
}

impl HitTestable for RectBuilder {
    fn hit_test(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

impl HitTestable for TextBuilder {
    fn hit_test(&self, x: f32, y: f32) -> bool {
        // Simple approximation for text hit testing
        // Assumes a rectangle based on the font size
        let text_width = self.text.len() as f32 * self.font_size * 0.6;
        let text_height = self.font_size * 1.2;

        let (left, right) = match self.text_anchor.as_str() {
            "middle" => (self.x - text_width / 2.0, self.x + text_width / 2.0),
            "end" => (self.x - text_width, self.x),
            _ => (self.x, self.x + text_width), // start or default
        };

        x >= left && x <= right && y >= self.y - text_height && y <= self.y
    }
}

impl HitTestable for LineBuilder {
    fn hit_test(&self, x: f32, y: f32) -> bool {
        // Distance from point to line segment
        let line_length_squared =
            (self.x2 - self.x1) * (self.x2 - self.x1) + (self.y2 - self.y1) * (self.y2 - self.y1);

        if line_length_squared == 0.0 {
            // Point
            let dx = x - self.x1;
            let dy = y - self.y1;
            return dx * dx + dy * dy <= (self.stroke_width * self.stroke_width) / 4.0;
        }

        let t = ((x - self.x1) * (self.x2 - self.x1) + (y - self.y1) * (self.y2 - self.y1))
            / line_length_squared;

        let t_clamped = t.clamp(0.0, 1.0);

        let closest_x = self.x1 + t_clamped * (self.x2 - self.x1);
        let closest_y = self.y1 + t_clamped * (self.y2 - self.y1);

        let dx = x - closest_x;
        let dy = y - closest_y;
        let distance_squared = dx * dx + dy * dy;

        distance_squared <= (self.stroke_width * self.stroke_width) / 4.0
    }
}

// Builder implementations
impl CircleBuilder {
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

    pub fn fill(mut self, fill: impl Into<String>) -> Self {
        self.fill = fill.into();
        self
    }

    pub fn stroke(mut self, stroke: impl Into<String>) -> Self {
        self.stroke = stroke.into();
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }
}

impl RectBuilder {
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

    pub fn fill(mut self, fill: impl Into<String>) -> Self {
        self.fill = fill.into();
        self
    }

    pub fn stroke(mut self, stroke: impl Into<String>) -> Self {
        self.stroke = stroke.into();
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn rx(mut self, rx: f32) -> Self {
        self.rx = Some(rx);
        self
    }

    pub fn ry(mut self, ry: f32) -> Self {
        self.ry = Some(ry);
        self
    }
}

impl TextBuilder {
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

    pub fn fill(mut self, fill: impl Into<String>) -> Self {
        self.fill = fill.into();
        self
    }

    pub fn text_anchor(mut self, anchor: impl Into<String>) -> Self {
        self.text_anchor = anchor.into();
        self
    }
}

impl LineBuilder {
    pub fn x1(mut self, x1: f32) -> Self {
        self.x1 = x1;
        self
    }

    pub fn y1(mut self, y1: f32) -> Self {
        self.y1 = y1;
        self
    }

    pub fn x2(mut self, x2: f32) -> Self {
        self.x2 = x2;
        self
    }

    pub fn y2(mut self, y2: f32) -> Self {
        self.y2 = y2;
        self
    }

    pub fn stroke(mut self, stroke: impl Into<String>) -> Self {
        self.stroke = stroke.into();
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }
}

// Export our app using the salt_app macro
salt_app!(DrawingApp);
