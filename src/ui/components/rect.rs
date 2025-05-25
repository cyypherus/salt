//! Rectangle component for Salt UI
//!
//! This module provides a rectangle component for Salt applications.

use crate::ui::{color::Color, Shape, ShapeType};

/// Builder for creating rectangle elements
#[derive(Clone)]
pub struct RectBuilder {
    /// X-coordinate of top-left corner
    pub x: f32,
    /// Y-coordinate of top-left corner
    pub y: f32,
    /// Width of rectangle
    pub width: f32,
    /// Height of rectangle
    pub height: f32,
    /// Fill color
    pub fill: Color,
    /// Stroke color
    pub stroke: Color,
    /// Stroke width
    pub stroke_width: f32,
    /// Corner radius for all corners
    pub corner_radius: f32,
}

impl RectBuilder {
    /// Test if a point is within the rectangle shape (for hit testing)
    pub fn hit_test_shape(&self, x: f32, y: f32) -> bool {
        // Simple bounds test for rectangle
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

impl RectBuilder {
    /// Set the x-coordinate
    pub fn x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    /// Set the y-coordinate
    pub fn y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    /// Set the width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set the fill color
    pub fn fill(mut self, fill: Color) -> Self {
        self.fill = fill;
        self
    }

    /// Set the stroke color
    pub fn stroke(mut self, stroke: Color) -> Self {
        self.stroke = stroke;
        self
    }

    /// Set the stroke width
    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    /// Set the corner radius for all corners
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn finish<T>(self, id: u64) -> Shape<T> {
        Shape::new(id, ShapeType::Rect(self))
    }
}

/// Create a new rectangle builder with default properties
pub fn rect() -> RectBuilder {
    RectBuilder {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
        fill: Color::BLACK,
        stroke: Color::TRANSPARENT,
        stroke_width: 0.0,
        corner_radius: 0.0,
    }
}
