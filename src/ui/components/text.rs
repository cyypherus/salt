//! Text component for Salt UI
//!
//! This module provides a text component for Salt applications.

use crate::ui::color::Color;
use crate::ui::{Shape, ShapeType, TextAlign};

#[derive(Clone)]
pub struct TextBuilder {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub font_family: String,
    pub font_size: f32,
    pub fill: Color,
    pub text_anchor: String,
}

impl TextBuilder {
    pub fn hit_test_shape(&self, x: f32, y: f32) -> bool {
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
        let anchor = match align {
            TextAlign::Left => "start",
            TextAlign::Center => "middle",
            TextAlign::Right => "end",
        };
        self.text_anchor = anchor.to_string();
        self
    }

    pub fn finish<T>(self, id: u64) -> Shape<T> {
        Shape::new(id, ShapeType::Text(self))
    }
}

pub fn text() -> TextBuilder {
    TextBuilder {
        x: 0.0,
        y: 0.0,
        text: "".to_string(),
        font_family: "sans-serif".to_string(),
        font_size: 12.0,
        fill: Color::BLACK,
        text_anchor: "start".to_string(),
    }
}
