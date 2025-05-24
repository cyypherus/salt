// Macro for exporting a specific App implementation
#[macro_export]
macro_rules! salt_app {
    ($app_type:ty) => {
        use $crate::wasm_bindgen::prelude::*;
        use $crate::web_sys::console;

        #[wasm_bindgen]
        pub struct SaltApp {
            app: $app_type,
        }

        #[wasm_bindgen]
        impl SaltApp {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                console::log_1(&"Creating custom SaltApp".into());
                Self {
                    app: <$app_type>::new(),
                }
            }

            pub fn handle_mouse_event(&mut self, event_type: &str, x: f64, y: f64) -> bool {
                let event = $crate::MouseEvent {
                    event_type: $crate::EventType::from(event_type),
                    x,
                    y,
                };

                self.app.handle_event(event)
            }

            pub fn render_svg(&mut self, width: u32, height: u32) -> String {
                let dimensions = $crate::Dimensions { width, height };
                self.app.render(dimensions)
            }
        }

        #[wasm_bindgen(start)]
        pub fn start() -> Result<(), JsValue> {
            console::log_1(&"Custom Salt app initialized".into());
            Ok(())
        }
    };
}
