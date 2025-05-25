//! Color utilities for Salt UI components
//!
//! This module provides color handling for Salt UI components.

/// Color type used throughout the UI components
pub type Color = color::AlphaColor<color::LinearSrgb>;

pub use color::*;

/// Direction for color interpolation in hue space
pub use color::HueDirection;
