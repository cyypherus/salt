//! Rectangle component for Salt UI
//!
//! This module provides a rectangle component for Salt applications.

use crate::ui::color::Color;
use crate::ui::gesture::callbacks::{OnClick, OnDrag, OnHover};
use crate::ui::gesture::{DragPhase, Point};
use crate::ui::HitTestable;
use std::rc::Rc;

/// Builder for creating rectangle elements
#[derive(Clone)]
pub struct RectBuilder<T: ?Sized> {
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
    /// Click callback
    pub on_click: OnClick<T>,
    /// Hover callback
    pub on_hover: OnHover<T>,
    /// Drag callback
    pub on_drag: OnDrag<T>,
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

impl<T> RectBuilder<T> {
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

/// Create a new rectangle builder with default properties
pub fn rect<T>() -> RectBuilder<T> {
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
