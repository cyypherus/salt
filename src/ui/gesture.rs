//! Gesture handling for Salt UI components
//!
//! This module provides types and utilities for handling gestures in Salt applications.

/// Represents the phase of a drag interaction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragPhase {
    /// Drag has just started (initial contact)
    Start,
    /// Drag is in progress (continued movement)
    Move,
    /// Drag has ended (contact released)
    End,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    /// Create a new point with the given coordinates
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Calculate the distance to another point
    pub fn distance(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate the distance squared (more efficient when only comparing distances)
    pub fn distance_squared(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

/// Gesture types that can be handled by UI components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureType {
    /// A tap or click (press and release in same location)
    Tap,
    /// A drag operation (press, move, release)
    Drag,
    /// A hover (pointer moving over an element without pressing)
    Hover,
}

/// Type definitions for gesture callbacks
pub mod callbacks {
    use super::{DragPhase, Point};
    use std::rc::Rc;

    /// Callback type for click/tap gestures
    pub type OnClick<T> = Option<Rc<dyn Fn(&mut T)>>;

    /// Callback type for hover gestures
    pub type OnHover<T> = Option<Rc<dyn Fn(&mut T, bool, Point)>>;

    /// Callback type for drag gestures
    pub type OnDrag<T> = Option<Rc<dyn Fn(&mut T, DragPhase, Point, Point)>>;
}
