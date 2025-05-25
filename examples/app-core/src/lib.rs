use salt::{salt_app, Dimensions, EventType, MouseEvent};
use wasm_bindgen::prelude::*;

pub struct CounterApp {
    count: i32,
    btn_hover: bool,
    btn_clicked: bool,
}

impl AppCore for CounterApp {
    fn new() -> Self {
        Self {
            count: 0,
            btn_hover: false,
            btn_clicked: false,
        }
    }

    fn handle_event(&mut self, event: MouseEvent) -> bool {
        match event.event_type {
            EventType::MouseDown => {
                // Check if the click is within the button area
                if is_point_in_rect(event.x as f32, event.y as f32, 250.0, 150.0, 300.0, 100.0) {
                    self.btn_clicked = true;
                    return true;
                }
            }
            EventType::MouseUp => {
                if self.btn_clicked {
                    // Only increment counter if mouse up is also over button (complete click)
                    if is_point_in_rect(event.x as f32, event.y as f32, 250.0, 150.0, 300.0, 100.0)
                    {
                        self.count += 1;
                    }
                    self.btn_clicked = false;
                    return true;
                }
            }
            EventType::MouseMove => {
                // Check for hover state
                let hover_now =
                    is_point_in_rect(event.x as f32, event.y as f32, 250.0, 150.0, 300.0, 100.0);

                if hover_now != self.btn_hover {
                    self.btn_hover = hover_now;
                    return true;
                }
            }
            _ => {}
        }

        false
    }

    fn render(&mut self, dimensions: Dimensions) -> String {
        // Create SVG manually
        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 {} {}">"#,
            dimensions.width, dimensions.height,
        );

        // Background
        svg.push_str(r#"<rect x="0" y="0" width="100%" height="100%" fill="white" />"#);

        // Title
        svg.push_str(
            &format!(
                r#"<text x="400" y="80" font-family="Arial" font-size="32" text-anchor="middle" fill="black">Counter: {}</text>"#,
                self.count
            )
        );

        // Button with hover effect
        let btn_color = if self.btn_hover {
            if self.btn_clicked {
                "#4466cc"
            } else {
                "#5588ee"
            }
        } else {
            "#6699ff"
        };

        svg.push_str(
            &format!(
                r#"<rect x="250" y="150" width="300" height="100" rx="10" ry="10" fill="{}" stroke="navy" stroke-width="2" />"#,
                btn_color
            )
        );

        // Button text
        svg.push_str(
            r#"<text x="400" y="210" font-family="Arial" font-size="28" text-anchor="middle" fill="white">Increment</text>"#
        );

        // Close SVG tag
        svg.push_str("</svg>");

        svg
    }
}

fn is_point_in_rect(
    x: f32,
    y: f32,
    rect_x: f32,
    rect_y: f32,
    rect_width: f32,
    rect_height: f32,
) -> bool {
    x >= rect_x && x <= rect_x + rect_width && y >= rect_y && y <= rect_y + rect_height
}

salt_app!(CounterApp);
