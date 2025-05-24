//! Salt: A Rust SVG rendering library for web applications
//!
//! Salt provides a simple interface for creating SVG-based web applications
//! using Rust, with WebAssembly as the compilation target.

// Public API
mod wasm;
pub use wasm::export_app;

/// Event types that can be handled by Salt applications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Click,
    MouseDown,
    MouseUp,
    MouseMove,
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
}

// Default implementation
struct DefaultApp {
    clicks: Vec<(f64, f64)>,
}

impl App for DefaultApp {
    fn new() -> Self {
        Self { clicks: Vec::new() }
    }

    fn handle_event(&mut self, event: MouseEvent) -> bool {
        if event.event_type == EventType::Click {
            self.clicks.push((event.x, event.y));
            return true;
        }
        false
    }

    fn render(&self, dimensions: Dimensions) -> String {
        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
            dimensions.width, dimensions.height, dimensions.width, dimensions.height
        );

        // Add a background rectangle
        svg.push_str(&format!(
            r#"<rect x="0" y="0" width="{}" height="{}" fill="blue" />"#,
            dimensions.width, dimensions.height
        ));

        // Draw circles for each click
        for (x, y) in &self.clicks {
            svg.push_str(&format!(
                r#"<circle cx="{}" cy="{}" r="5" fill="red" />"#,
                x, y
            ));
        }

        svg.push_str("</svg>");
        svg
    }
}
