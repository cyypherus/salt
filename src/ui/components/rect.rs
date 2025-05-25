//! Rectangle component for Salt UI
//!
//! This module provides a rectangle component for Salt applications.

use crate::ui::{color::Color, Shape, ShapeType};

#[derive(Clone)]
pub struct RectBuilder {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub fill: Color,
    pub stroke: Color,
    pub stroke_width: f32,
    pub corner_radius: f32,
}

impl RectBuilder {
    pub fn hit_test_shape(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
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

    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn finish<T>(self, id: u64) -> Shape<T> {
        Shape::new(id, ShapeType::Rect(self))
    }
}

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
