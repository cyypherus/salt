//! View module for Salt framework
//!
//! This module provides the View component for rendering shapes in Salt applications.

use crate::ui::components::{CircleBuilder, PathBuilder, RectBuilder, TextBuilder};
use crate::ui::gesture::{DragPhase, Point};
use crate::Dimensions;

/// Trait for hit testing shapes
pub trait HitTestable {
    /// Check if the point (x, y) is within the shape
    fn hit_test(&self, x: f32, y: f32) -> bool;
}

/// Represents an SVG shape
#[derive(Clone)]
pub enum Shape<T: ?Sized> {
    /// Circle shape
    Circle(CircleBuilder<T>),
    /// Rectangle shape
    Rect(RectBuilder<T>),
    /// Text shape
    Text(TextBuilder<T>),
    /// Path shape
    Path(PathBuilder<T>),
}

impl<T> Shape<T> {
    /// Execute the on_click callback if present
    pub fn on_click(&mut self, state: &mut T) {
        match self {
            Shape::Circle(builder) => builder.on_click.as_ref().map(|func| func(state)),
            Shape::Rect(builder) => builder.on_click.as_ref().map(|func| func(state)),
            Shape::Text(builder) => builder.on_click.as_ref().map(|func| func(state)),
            Shape::Path(builder) => builder.on_click.as_ref().map(|func| func(state)),
        };
    }

    /// Execute the on_hover callback if present
    pub fn on_hover(&mut self, state: &mut T, hovered: bool, point: Point) {
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

    /// Execute the on_drag callback if present
    pub fn on_drag(&mut self, state: &mut T, phase: DragPhase, start: Point, current: Point) {
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

/// Text alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    /// Left alignment (SVG text-anchor: start)
    Left,
    /// Center alignment (SVG text-anchor: middle)
    Center,
    /// Right alignment (SVG text-anchor: end)
    Right,
}

/// Main view component for rendering shapes and handling interactions
#[derive(Clone)]
pub struct View<T: ?Sized> {
    /// Collection of shapes in the view
    pub shapes: Vec<Shape<T>>,
}

impl<T> View<T> {
    /// Create a new empty view
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    /// Add a circle to the view
    pub fn circle(&mut self, builder: CircleBuilder<T>) -> &mut Self {
        self.shapes.push(Shape::Circle(builder));
        self
    }

    /// Add a rectangle to the view
    pub fn rect(&mut self, builder: RectBuilder<T>) -> &mut Self {
        self.shapes.push(Shape::Rect(builder));
        self
    }

    /// Add text to the view
    pub fn text(&mut self, builder: TextBuilder<T>) -> &mut Self {
        self.shapes.push(Shape::Text(builder));
        self
    }

    /// Add a path to the view
    pub fn path(&mut self, builder: PathBuilder<T>) -> &mut Self {
        self.shapes.push(Shape::Path(builder));
        self
    }

    /// Test if a point hits any shape in the view
    /// Returns the index of the hit shape if found, in reverse order (top to bottom)
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

    /// Render the view to SVG
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
                            crate::ui::components::PathCommand::MoveTo(x, y) => {
                                acc.push_str(&format!("M {},{} ", x, y))
                            }
                            crate::ui::components::PathCommand::LineTo(x, y) => {
                                acc.push_str(&format!("L {},{} ", x, y))
                            }
                            crate::ui::components::PathCommand::CurveTo(x1, y1, x2, y2, x, y) => {
                                acc.push_str(&format!("C {},{} {},{} {},{} ", x1, y1, x2, y2, x, y))
                            }
                            crate::ui::components::PathCommand::ClosePath => acc.push_str("Z "),
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

    /// Clear all shapes from the view
    pub fn clear(&mut self) {
        self.shapes.clear();
    }
}

impl<T> Default for View<T> {
    fn default() -> Self {
        Self::new()
    }
}
