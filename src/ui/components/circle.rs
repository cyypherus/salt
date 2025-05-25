//! Circle component for Salt UI
//!
//! This module provides a circle component for Salt applications.

use crate::ui::color::Color;
use crate::ui::gesture::callbacks::{OnClick, OnDrag, OnHover};
use crate::ui::gesture::{DragPhase, Point};
use crate::ui::HitTestable;
use std::rc::Rc;

/// Builder for creating circle elements
#[derive(Clone)]
pub struct CircleBuilder<T: ?Sized> {
    /// Center x-coordinate
    pub cx: f32,
    /// Center y-coordinate
    pub cy: f32,
    /// Radius
    pub r: f32,
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

impl<T> CircleBuilder<T> {
    /// Set the center x-coordinate
    pub fn cx(mut self, cx: f32) -> Self {
        self.cx = cx;
        self
    }

    /// Set the center y-coordinate
    pub fn cy(mut self, cy: f32) -> Self {
        self.cy = cy;
        self
    }

    /// Set the radius
    pub fn r(mut self, r: f32) -> Self {
        self.r = r;
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

/// Create a new circle builder with default properties
pub fn circle<T>() -> CircleBuilder<T> {
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
