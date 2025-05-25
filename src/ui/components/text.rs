//! Text component for Salt UI
//!
//! This module provides a text component for Salt applications.

use crate::ui::color::Color;
use crate::ui::gesture::callbacks::{OnClick, OnDrag, OnHover};
use crate::ui::gesture::{DragPhase, Point};
use crate::ui::HitTestable;
use crate::ui::TextAlign;
use std::rc::Rc;

/// Builder for creating text elements
#[derive(Clone)]
pub struct TextBuilder<T: ?Sized> {
    /// X-coordinate
    pub x: f32,
    /// Y-coordinate
    pub y: f32,
    /// Text content
    pub text: String,
    /// Font family
    pub font_family: String,
    /// Font size
    pub font_size: f32,
    /// Fill color
    pub fill: Color,
    /// Text anchor (alignment)
    pub text_anchor: String,
    /// Click callback
    pub on_click: OnClick<T>,
    /// Hover callback
    pub on_hover: OnHover<T>,
    /// Drag callback
    pub on_drag: OnDrag<T>,
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

impl<T> TextBuilder<T> {
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

    /// Set the text content
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    /// Set the font family
    pub fn font_family(mut self, font_family: impl Into<String>) -> Self {
        self.font_family = font_family.into();
        self
    }

    /// Set the font size
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set the fill color
    pub fn fill(mut self, fill: Color) -> Self {
        self.fill = fill;
        self
    }

    /// Set the text alignment
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

    /// Set the click callback
    pub fn on_click(mut self, callback: impl Fn(&mut T) + 'static) -> Self {
        self.on_click = Some(Rc::new(callback));
        self
    }

    /// Set the hover callback
    pub fn on_hover(mut self, callback: impl Fn(&mut T, bool, Point) + 'static) -> Self {
        self.on_hover = Some(Rc::new(callback));
        self
    }

    /// Set the drag callback
    pub fn on_drag(mut self, callback: impl Fn(&mut T, DragPhase, Point, Point) + 'static) -> Self {
        self.on_drag = Some(Rc::new(callback));
        self
    }
}

/// Create a new text builder with default properties
pub fn text<T>() -> TextBuilder<T> {
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
