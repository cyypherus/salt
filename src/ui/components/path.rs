//! Path component for Salt UI
//!
//! This module provides a path component for Salt applications.

use crate::ui::color::Color;
use crate::ui::gesture::callbacks::{OnClick, OnDrag, OnHover};
use crate::ui::gesture::{DragPhase, Point};
use crate::ui::HitTestable;
use std::rc::Rc;

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

/// Builder for creating path elements
#[derive(Clone)]
pub struct PathBuilder<T: ?Sized> {
    /// List of path commands
    pub commands: Vec<PathCommand>,
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
    /// Calculated bounds (min_x, min_y, max_x, max_y)
    pub bounds: Option<(f32, f32, f32, f32)>,
    /// Current x position
    pub current_x: f32,
    /// Current y position
    pub current_y: f32,
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

    /// Move to a point (M)
    pub fn move_to(mut self, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::MoveTo(x, y));
        self.update_bounds(x, y);
        self
    }

    /// Line to a point (L)
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

    /// Cubic bezier curve (C)
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

    /// Close path (Z)
    pub fn close_path(mut self) -> Self {
        self.commands.push(PathCommand::ClosePath);
        self
    }

    /// Helper for creating a rectangle path
    pub fn rect(self, x: f32, y: f32, width: f32, height: f32) -> Self {
        self.move_to(x, y)
            .line_to(x + width, y)
            .line_to(x + width, y + height)
            .line_to(x, y + height)
            .close_path()
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

/// Create a new path builder with default properties
pub fn path<T>() -> PathBuilder<T> {
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
