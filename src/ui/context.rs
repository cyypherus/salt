//! Context for Salt UI applications
//!
//! This module provides a context that encapsulates the state needed by Salt applications.

use crate::{ui::view::View, Dimensions, DragState, HoverState};

#[derive(Default, Clone, Debug)]
pub struct GestureState {
    pub drag: DragState,
    pub hover: HoverState,
}

pub struct AppCtx<T: ?Sized> {
    pub view: View<T>,
    pub gestures: GestureState,
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
        self.gestures.drag.dragging_shape_id = None;
        self.gestures.drag.mouse_down_id = None;
    }

    /// Get the current dimensions
    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }
}
