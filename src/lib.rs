//! Salt: A Rust SVG rendering library for web applications
//!
//! Salt provides a simple interface for creating SVG-based web applications
//! using Rust, with WebAssembly as the compilation target.

pub mod ui;
use std::fmt;

pub use crate::ui::{Color, DragPhase, Point, TextAlign};
use ui::AppCtx;
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
pub trait AppCore {
    /// Create a new instance of the application
    fn new() -> Self;

    /// Handle a mouse event
    ///
    /// Return true if the application state changed and a re-render is needed.
    fn handle_event(&mut self, event: MouseEvent) -> bool;

    /// Render the application to SVG
    fn render(&mut self, dimensions: Dimensions) -> String;

    /// Initialize the app with any setup required
    fn init(&mut self) {}
}

/// A higher-level trait for interactive Salt applications
///
/// This trait provides a more streamlined way to build Salt applications
/// by handling most of the common interaction patterns automatically.
/// Applications only need to implement the view method.
pub trait App {
    type State;
    /// Create a new instance of the application
    fn new() -> Self;

    /// Define the view for the application
    ///
    /// This method should use the View and UI components to build the interface.
    fn view(&mut self, dimensions: Dimensions);

    /// Get a mutable reference to the application state
    fn state(&mut self) -> (&mut AppCtx<Self::State>, &mut Self::State);
}

/// Implementation of App for any type that implements InteractiveApp
///
/// This provides all the gesture and event handling automatically.
impl<T: App> AppCore for T {
    fn new() -> Self {
        <T as App>::new()
    }

    fn handle_event(&mut self, event: MouseEvent) -> bool {
        let x = event.x as f32;
        let y = event.y as f32;
        let (ctx, state) = self.state();
        let view = &mut ctx.view;

        // Handle mouse down event
        if event.event_type == EventType::MouseDown {
            // Hit test the view to check if any interactive elements were clicked
            if let Some(idx) = view.hit_test(x, y) {
                let mut shapes = Vec::new();
                std::mem::swap(&mut shapes, &mut view.shapes);

                // Store drag start position and the element that received mouse down
                ctx.gestures.drag.start_x = Some(x);
                ctx.gestures.drag.start_y = Some(y);
                ctx.gestures.drag.dragging_shape_idx = Some(idx);
                ctx.gestures.drag.mouse_down_idx = Some(idx);

                // Call the on_drag handler with start phase
                if let (Some(start_x), Some(start_y)) =
                    (ctx.gestures.drag.start_x, ctx.gestures.drag.start_y)
                {
                    shapes[idx].on_drag(
                        state,
                        ui::gesture::DragPhase::Start,
                        ui::gesture::Point::new(start_x, start_y),
                        ui::gesture::Point::new(x, y),
                    );
                }

                std::mem::swap(&mut shapes, &mut view.shapes);
                return true;
            }
            return false;
        }

        // Handle mouse up event
        if event.event_type == EventType::MouseUp {
            // Check if we released on the same shape that we started on (click behavior)
            let current_idx = view.hit_test(x, y);
            let drag = &ctx.gestures.drag;

            if let (Some(drag_idx), Some(start_x), Some(start_y), Some(down_idx)) = (
                drag.dragging_shape_idx,
                drag.start_x,
                drag.start_y,
                drag.mouse_down_idx,
            ) {
                let mut shapes = Vec::new();
                std::mem::swap(&mut shapes, &mut view.shapes);

                // Notify the shape of drag end
                shapes[drag_idx].on_drag(
                    state,
                    ui::gesture::DragPhase::End,
                    ui::gesture::Point::new(start_x, start_y),
                    ui::gesture::Point::new(x, y),
                );

                // If mouse up is on the same element as mouse down, trigger click
                if current_idx == Some(down_idx) {
                    shapes[down_idx].on_click(state);
                }

                std::mem::swap(&mut shapes, &mut view.shapes);
            }

            ctx.reset_interaction();

            return true;
        }

        // Handle mouse move event
        if event.event_type == EventType::MouseMove {
            // Handle hover effect
            let hover_idx = view.hit_test(x, y);
            let current_hover_idx = ctx.gestures.hover.hover_shape_idx;

            // Always handle hover effects, even during drags
            if hover_idx != current_hover_idx {
                let mut shapes = Vec::new();
                std::mem::swap(&mut shapes, &mut view.shapes);

                if let Some(idx) = current_hover_idx {
                    shapes[idx].on_hover(state, false, ui::gesture::Point::new(x, y));
                }

                // Call on_hover for the new shape
                if let Some(idx) = hover_idx {
                    shapes[idx].on_hover(state, true, ui::gesture::Point::new(x, y));
                }

                std::mem::swap(&mut shapes, &mut view.shapes);
                ctx.gestures.hover.hover_shape_idx = hover_idx;

                // Return true to indicate we processed a hover event
                return true;
            }

            // Handle dragging
            let drag = &ctx.gestures.drag;
            if let (Some(idx), Some(start_x), Some(start_y)) =
                (drag.dragging_shape_idx, drag.start_x, drag.start_y)
            {
                let mut shapes = Vec::new();
                std::mem::swap(&mut shapes, &mut view.shapes);
                shapes[idx].on_drag(
                    state,
                    ui::gesture::DragPhase::Move,
                    ui::gesture::Point::new(start_x, start_y),
                    ui::gesture::Point::new(x, y),
                );
                std::mem::swap(&mut shapes, &mut view.shapes);

                return true;
            }
        }

        false
    }

    fn render(&mut self, dimensions: Dimensions) -> String {
        self.state().0.set_dimensions(dimensions);
        self.state().0.clear();
        self.view(dimensions);
        self.state().0.view.render(dimensions)
    }
}

/// State for tracking drag operations
#[derive(Default, Clone, Debug)]
pub struct DragState {
    /// X coordinate where drag started
    pub start_x: Option<f32>,
    /// Y coordinate where drag started
    pub start_y: Option<f32>,
    /// Index of shape being dragged
    pub dragging_shape_idx: Option<usize>,
    /// Index of shape that received mouse down
    pub mouse_down_idx: Option<usize>,
}

/// State for tracking hover operations
#[derive(Default, Clone, Debug)]
pub struct HoverState {
    /// Index of shape being hovered
    pub hover_shape_idx: Option<usize>,
}

// Macro for exporting a specific App implementation
#[macro_export]
macro_rules! salt_app {
    ($app_type:ty) => {
        use $crate::wasm_bindgen::prelude::*;
        use $crate::web_sys::console;
        use $crate::AppCore;

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
                    app: <$app_type as $crate::App>::new(),
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
