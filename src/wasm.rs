use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::{App, Dimensions, EventType, MouseEvent};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// The main entry point for connecting a Rust App to WASM
pub fn export_app<T: App + 'static>() {
    // App will be instantiated when the JS constructor is called
    console::log_1(&"Salt app registered with WASM".into());
}

/// The WASM exported class that wraps a Rust App implementation
#[wasm_bindgen]
pub struct SaltApp {
    app: Box<dyn App>,
}

#[wasm_bindgen]
impl SaltApp {
    /// Handle a mouse event from JavaScript
    pub fn handle_mouse_event(&mut self, event_type: &str, x: f64, y: f64) -> bool {
        let event = MouseEvent {
            event_type: EventType::from(event_type),
            x,
            y,
        };

        self.app.handle_event(event)
    }

    /// Render the app as an SVG string
    pub fn render_svg(&self, width: u32, height: u32) -> String {
        let dimensions = Dimensions { width, height };
        self.app.render(dimensions)
    }
}

impl Default for SaltApp {
    fn default() -> Self {
        SaltApp {
            app: Box::new(crate::DefaultApp::new()),
        }
    }
}

// Macro for exporting a specific App implementation
#[macro_export]
macro_rules! salt_app {
    ($app_type:ty) => {
        #[wasm_bindgen]
        pub fn create_app() -> $crate::wasm::SaltApp {
            $crate::wasm::SaltApp {
                app: Box::new(<$app_type>::new()),
            }
        }
    };
}
