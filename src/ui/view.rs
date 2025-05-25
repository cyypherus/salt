//! View module for Salt framework
//!
//! This module provides the View component for rendering shapes in Salt applications.

use crate::ui::components::{PathBuilder, RectBuilder, TextBuilder};
use crate::ui::gesture::{DragPhase, Point};
use crate::Dimensions;

/// Trait for hit testing shapes
pub trait HitTestable {
    /// Check if the point (x, y) is within the shape
    fn hit_test(&self, x: f32, y: f32) -> bool;
}

/// Represents an SVG shape
#[derive(Clone)]
pub struct Shape<T: ?Sized> {
    /// Unique identifier for the shape
    pub id: u64,
    /// The actual shape data
    pub shape_type: ShapeType<T>,
}

/// Shape types that can be rendered
#[derive(Clone)]
pub enum ShapeType<T: ?Sized> {
    /// Rectangle shape
    Rect(RectBuilder<T>),
    /// Text shape
    Text(TextBuilder<T>),
    /// Path shape
    Path(PathBuilder<T>),
}

impl<T> Shape<T> {
    /// Create a new shape with the given ID and type
    pub fn new(id: u64, shape_type: ShapeType<T>) -> Self {
        Self { id, shape_type }
    }
    
    /// Execute the on_click callback if present
    pub fn on_click(&mut self, state: &mut T) {
        match &mut self.shape_type {
            ShapeType::Rect(builder) => builder.on_click.as_ref().map(|func| func(state)),
            ShapeType::Text(builder) => builder.on_click.as_ref().map(|func| func(state)),
            ShapeType::Path(builder) => builder.on_click.as_ref().map(|func| func(state)),
        };
    }

    /// Execute the on_hover callback if present
    pub fn on_hover(&mut self, state: &mut T, hovered: bool, point: Point) {
        match &mut self.shape_type {
            ShapeType::Rect(builder) => builder
                .on_hover
                .as_ref()
                .map(|func| func(state, hovered, point)),
            ShapeType::Text(builder) => builder
                .on_hover
                .as_ref()
                .map(|func| func(state, hovered, point)),
            ShapeType::Path(builder) => builder
                .on_hover
                .as_ref()
                .map(|func| func(state, hovered, point)),
        };
    }

    /// Execute the on_drag callback if present
    pub fn on_drag(&mut self, state: &mut T, phase: DragPhase, start: Point, current: Point) {
        match &mut self.shape_type {
            ShapeType::Rect(builder) => builder
                .on_drag
                .as_ref()
                .map(|func| func(state, phase, start, current)),
            ShapeType::Text(builder) => builder
                .on_drag
                .as_ref()
                .map(|func| func(state, phase, start, current)),
            ShapeType::Path(builder) => builder
                .on_drag
                .as_ref()
                .map(|func| func(state, phase, start, current)),
        };
    }
    
    /// Test if a point hits this shape
    pub fn hit_test(&self, x: f32, y: f32) -> bool {
        match &self.shape_type {
            ShapeType::Rect(rect) => rect.hit_test(x, y),
            ShapeType::Text(text) => text.hit_test(x, y),
            ShapeType::Path(path) => path.hit_test(x, y),
        }
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

    /// Add a rectangle to the view with a unique ID
    pub fn rect(&mut self, id: u64, builder: RectBuilder<T>) -> &mut Self {
        self.shapes.push(Shape::new(id, ShapeType::Rect(builder)));
        self
    }

    /// Add text to the view with a unique ID
    pub fn text(&mut self, id: u64, builder: TextBuilder<T>) -> &mut Self {
        self.shapes.push(Shape::new(id, ShapeType::Text(builder)));
        self
    }

    /// Add a path to the view with a unique ID
    pub fn path(&mut self, id: u64, builder: PathBuilder<T>) -> &mut Self {
        self.shapes.push(Shape::new(id, ShapeType::Path(builder)));
        self
    }

    /// Test if a point hits any shape in the view
    /// Returns the index of the hit shape if found, in reverse order (top to bottom)
    pub fn hit_test(&self, x: f32, y: f32) -> Option<usize> {
        for (idx, shape) in self.shapes.iter().enumerate().rev() {
            if shape.hit_test(x, y) {
                return Some(idx);
            }
        }
        None
    }
    
    /// Test if a point hits any shape in the view
    /// Returns the index and ID of the hit shape if found, in reverse order (top to bottom)
    pub fn hit_test_with_id(&self, x: f32, y: f32) -> Option<(usize, u64)> {
        for (idx, shape) in self.shapes.iter().enumerate().rev() {
            if shape.hit_test(x, y) {
                return Some((idx, shape.id));
            }
        }
        None
    }
    
    /// Find the index of a shape by its ID
    pub fn find_shape_by_id(&self, id: u64) -> Option<usize> {
        self.shapes.iter().position(|shape| shape.id == id)
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
            match &shape.shape_type {
                ShapeType::Rect(rect) => {
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
                ShapeType::Text(text) => {
                    svg.push_str(&format!(
                        r#"<text x="{}" y="{}" font-family="{}" font-size="{}" fill="{:x}" text-anchor="{}">{}</text>"#,
                        text.x, text.y, text.font_family, text.font_size,
                        text.fill.to_rgba8(), text.text_anchor, text.text
                    ));
                }
                ShapeType::Path(path) => {
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
