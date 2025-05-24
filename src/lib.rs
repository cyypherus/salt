//! Salt: A Rust SVG rendering library for web applications
//!
//! Salt provides a simple interface for creating SVG-based web applications
//! using Rust, with WebAssembly as the compilation target.

mod wasm;

use std::fmt;

pub use wasm_bindgen;
pub use web_sys;

/// Event types that can be handled by Salt applications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Click,
    MouseDown,
    MouseUp,
    MouseMove,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::Click => write!(f, "click"),
            EventType::MouseDown => write!(f, "mousedown"),
            EventType::MouseUp => write!(f, "mouseup"),
            EventType::MouseMove => write!(f, "mousemove"),
        }
    }
}

impl From<&str> for EventType {
    fn from(s: &str) -> Self {
        match s {
            "click" => EventType::Click,
            "mousedown" => EventType::MouseDown,
            "mouseup" => EventType::MouseUp,
            "mousemove" => EventType::MouseMove,
            _ => EventType::Click, // Default to Click for unknown events
        }
    }
}

/// Mouse event data
#[derive(Debug, Clone, Copy)]
pub struct MouseEvent {
    /// Type of mouse event
    pub event_type: EventType,
    /// X coordinate relative to the application container
    pub x: f64,
    /// Y coordinate relative to the application container
    pub y: f64,
}

/// Dimensions of the rendering surface
#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
}

/// Core trait for Salt applications
///
/// Implement this trait to create a Salt application that can be
/// rendered as SVG and respond to user interactions.
pub trait App {
    /// Create a new instance of the application
    fn new() -> Self
    where
        Self: Sized;

    /// Handle a mouse event
    ///
    /// Return true if the application state changed and a re-render is needed.
    fn handle_event(&mut self, event: MouseEvent) -> bool;

    /// Render the application to SVG
    fn render(&self, dimensions: Dimensions) -> String;

    /// Initialize the app with any setup required
    fn init(&mut self) {}
}
