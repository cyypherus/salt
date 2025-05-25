//! UI components for Salt framework
//!
//! This module provides the basic components for building UI interfaces in Salt applications.

mod path;
mod rect;
mod text;

pub use path::{path, PathBuilder, PathCommand};
pub use rect::{rect, RectBuilder};
pub use text::{text, TextBuilder};
