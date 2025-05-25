//! Path component for Salt UI
//!
//! This module provides a path component for Salt applications.

use crate::ui::{color::Color, Shape, ShapeType};

#[derive(Clone, Debug)]
pub enum PathCommand {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    CurveTo(f32, f32, f32, f32, f32, f32),
    ClosePath,
}

#[derive(Clone)]
pub struct PathBuilder {
    pub commands: Vec<PathCommand>,
    pub fill: Color,
    pub stroke: Color,
    pub stroke_width: f32,
    pub bounds: Option<(f32, f32, f32, f32)>,
    pub current_x: f32,
    pub current_y: f32,
}

impl PathBuilder {
    pub fn hit_test_shape(&self, x: f32, y: f32) -> bool {
        if let Some((min_x, min_y, max_x, max_y)) = self.bounds {
            let half_stroke = self.stroke_width / 2.0;

            x >= min_x - half_stroke
                && x <= max_x + half_stroke
                && y >= min_y - half_stroke
                && y <= max_y + half_stroke
        } else {
            false
        }
    }

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

    pub fn move_to(mut self, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::MoveTo(x, y));
        self.update_bounds(x, y);
        self
    }

    pub fn line_to(mut self, x: f32, y: f32) -> Self {
        if self.commands.is_empty() {
            self.commands
                .push(PathCommand::MoveTo(self.current_x, self.current_y));
        }
        self.commands.push(PathCommand::LineTo(x, y));
        self.update_bounds(x, y);
        self
    }

    pub fn curve_to(mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) -> Self {
        if self.commands.is_empty() {
            self.commands
                .push(PathCommand::MoveTo(self.current_x, self.current_y));
        }
        self.commands
            .push(PathCommand::CurveTo(x1, y1, x2, y2, x, y));
        self.update_bounds(x1, y1);
        self.update_bounds(x2, y2);
        self.update_bounds(x, y);
        self
    }

    pub fn close_path(mut self) -> Self {
        self.commands.push(PathCommand::ClosePath);
        self
    }

    pub fn rect(self, x: f32, y: f32, width: f32, height: f32) -> Self {
        self.move_to(x, y)
            .line_to(x + width, y)
            .line_to(x + width, y + height)
            .line_to(x, y + height)
            .close_path()
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

    pub fn finish<T>(self, id: u64) -> Shape<T> {
        Shape::new(id, ShapeType::Path(self))
    }
}

pub fn path() -> PathBuilder {
    PathBuilder {
        commands: Vec::new(),
        fill: Color::BLACK,
        stroke: Color::TRANSPARENT,
        stroke_width: 0.0,
        bounds: None,
        current_x: 0.0,
        current_y: 0.0,
    }
}
