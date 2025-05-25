//! Context for Salt UI applications
//!
//! This module provides a context that encapsulates the state needed by Salt applications.

use crate::{ui::view::View, Dimensions, DragState, HoverState};

/// Gesture state for interactive applications
#[derive(Default, Clone, Debug)]
pub struct GestureState {
    /// Drag gesture state
    pub drag: DragState,
    /// Hover gesture state
    pub hover: HoverState,
}

/// Context for Salt applications
///
/// This struct encapsulates the state needed by Salt applications,
/// including the view and the gesture tracking state.
pub struct AppCtx<T: ?Sized> {
    /// The view for rendering
    pub view: View<T>,
    /// Gesture tracking state
    pub gestures: GestureState,
    /// Current dimensions
    pub dimensions: Dimensions,
}

impl<T> Default for AppCtx<T> {
    fn default() -> Self {
        Self {
            view: View::new(),
            gestures: GestureState::default(),
            dimensions: Dimensions {
                width: 0,
                height: 0,
            },
        }
    }
}

impl<T> AppCtx<T> {
    /// Create a new context
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the dimensions
    pub fn set_dimensions(&mut self, dimensions: Dimensions) {
        self.dimensions = dimensions;
    }

    /// Clear the view
    pub fn clear(&mut self) {
        self.view.clear();
    }

    /// Reset all interaction state
    pub fn reset_interaction(&mut self) {
        self.gestures.drag.start_x = None;
        self.gestures.drag.start_y = None;
        self.gestures.drag.dragging_shape_idx = None;
        self.gestures.drag.mouse_down_idx = None;
    }

    /// Get the current dimensions
    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }
}
