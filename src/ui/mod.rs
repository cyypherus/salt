//! UI module for Salt framework
//!
//! This module provides UI components and utilities for building Salt applications.

pub mod color;
pub mod components;
pub mod context;
pub mod gesture;
pub mod view;

pub use color::Color;
pub use components::{circle, path, rect, text};
pub use context::{GestureState, AppCtx};
pub use gesture::{DragPhase, Point};
pub use view::{HitTestable, Shape, TextAlign, View};
