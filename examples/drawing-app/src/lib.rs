use salt::{salt_app, App, Dimensions, EventType, MouseEvent};
use wasm_bindgen::prelude::*;

// Drawing application that tracks points when dragging the mouse
pub struct DrawingApp {
    // Points for the user's drawing
    points: Vec<(f64, f64)>,
    // Current drawing state
    is_drawing: bool,
}

impl App for DrawingApp {
    fn new() -> Self {
        Self {
            points: Vec::new(),
            is_drawing: false,
        }
    }

    fn handle_event(&mut self, event: MouseEvent) -> bool {
        match event.event_type {
            EventType::MouseDown => {
                // Start drawing and add the first point
                self.is_drawing = true;
                self.points.push((event.x, event.y));
                true
            }
            EventType::MouseMove => {
                // Add points while drawing
                if self.is_drawing {
                    self.points.push((event.x, event.y));
                    true
                } else {
                    false
                }
            }
            EventType::MouseUp => {
                // Stop drawing
                self.is_drawing = false;
                true
            }
            _ => false,
        }
    }
    fn render(&self, dimensions: Dimensions) -> String {
        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
            dimensions.width, dimensions.height, dimensions.width, dimensions.height
        );

        // Background
        svg.push_str(&format!(
            r#"<rect x="0" y="0" width="{}" height="{}" fill="white" />"#,
            dimensions.width, dimensions.height
        ));

        // Instructions
        svg.push_str(r#"<text x="10" y="20" font-family="sans-serif" font-size="14" fill="black">Click and drag to draw</text>"#);

        // Draw the user's sketch as a path
        if !self.points.is_empty() {
            svg.push_str(r#"<path d=""#);

            // Start at the first point
            if let Some((first_x, first_y)) = self.points.first() {
                svg.push_str(&format!("M {},{} ", first_x, first_y));

                // Add line segments to each subsequent point
                for (x, y) in &self.points[1..] {
                    svg.push_str(&format!("L {},{} ", x, y));
                }
            }

            svg.push_str(r#"" stroke="black" stroke-width="2" fill="none" />"#);
        }

        svg.push_str("</svg>");
        svg
    }
}

// Export our app using the salt_app macro
salt_app!(DrawingApp);
