use backer::{models::Area, nodes::*, Layout};
use color::HueDirection;
use salt::{salt_app, App, Dimensions, EventType, MouseEvent};
use wasm_bindgen::prelude::*;

mod view;

use view::*;

// A single stroke with its color
struct Stroke {
    points: Vec<(f64, f64)>,
    color: Palette,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Palette {
    Black,
    Red,
    Green,
    Blue,
    Purple,
}

impl Palette {
    fn color(&self) -> Color {
        match self {
            Palette::Black => Color::new([0., 0., 0., 1.]),
            Palette::Red => Color::new([1., 0., 0., 1.]),
            Palette::Green => Color::new([0., 1., 0., 1.]),
            Palette::Blue => Color::new([0., 0., 1., 1.]),
            Palette::Purple => Color::new([0.5, 0., 0.5, 1.]),
        }
    }
}

// Drawing application that tracks points when dragging the mouse
pub struct DrawingApp {
    // Collection of strokes for the user's drawing
    strokes: Vec<Stroke>,
    // Current drawing state
    is_drawing: bool,
    // Currently selected color
    current_color: Palette,
    hovered_color: Option<Palette>,

    // Was a button clicked
    button_clicked: Option<String>,
    // View for rendering
    view: View<DrawingApp>,
    // Mouse interaction state
    drag_start_x: Option<f32>,
    drag_start_y: Option<f32>,
    hover_shape_idx: Option<usize>,
    dragging_shape_idx: Option<usize>,
    // Store which element received the mouse down for click detection
    mouse_down_idx: Option<usize>,
}

impl App for DrawingApp {
    fn new() -> Self {
        Self {
            strokes: Vec::new(),
            is_drawing: false,
            current_color: Palette::Black,
            hovered_color: None,
            button_clicked: None,
            view: View::new(),
            drag_start_x: None,
            drag_start_y: None,
            hover_shape_idx: None,
            dragging_shape_idx: None,
            mouse_down_idx: None,
        }
    }

    fn handle_event(&mut self, event: MouseEvent) -> bool {
        let x = event.x as f32;
        let y = event.y as f32;

        // Handle mouse down event
        if event.event_type == EventType::MouseDown {
            // Hit test the view to check if any interactive elements were clicked
            if let Some(idx) = self.view.hit_test(x, y) {
                let mut shapes = Vec::new();
                std::mem::swap(&mut shapes, &mut self.view.shapes);

                // Store drag start position and the element that received mouse down
                self.drag_start_x = Some(x);
                self.drag_start_y = Some(y);
                self.dragging_shape_idx = Some(idx);
                self.mouse_down_idx = Some(idx);
                // Call the on_drag handler with start phase,
                // on_click will be called on mouse up if it's still the same element
                if let (Some(start_x), Some(start_y)) = (self.drag_start_x, self.drag_start_y) {
                    shapes[idx].on_drag(
                        self,
                        DragPhase::Start,
                        Point::new(start_x, start_y),
                        Point::new(x, y),
                    );
                }

                std::mem::swap(&mut shapes, &mut self.view.shapes);
                return true;
            }

            return false;
        }

        // Handle mouse up event
        if event.event_type == EventType::MouseUp {
            self.button_clicked = None;
            self.is_drawing = false;

            // Check if we released on the same shape that we started on (click behavior)
            let current_idx = self.view.hit_test(x, y);

            if let (Some(drag_idx), Some(start_x), Some(start_y), Some(down_idx)) = (
                self.dragging_shape_idx,
                self.drag_start_x,
                self.drag_start_y,
                self.mouse_down_idx,
            ) {
                let mut shapes = Vec::new();
                std::mem::swap(&mut shapes, &mut self.view.shapes);

                // Notify the shape of drag end
                shapes[drag_idx].on_drag(
                    self,
                    DragPhase::End,
                    Point::new(start_x, start_y),
                    Point::new(x, y),
                );

                // If mouse up is on the same element as mouse down, trigger click
                // We always send the click event regardless of whether we've been dragging
                // This ensures that dragging and clicking are not mutually exclusive
                if current_idx == Some(down_idx) {
                    shapes[down_idx].on_click(self);
                }

                std::mem::swap(&mut shapes, &mut self.view.shapes);
            }

            // Reset all interaction state
            self.drag_start_x = None;
            self.drag_start_y = None;
            self.dragging_shape_idx = None;
            self.mouse_down_idx = None;

            return true;
        }

        // Handle mouse move event
        if event.event_type == EventType::MouseMove {
            // Handle hover effect
            let hover_idx = self.view.hit_test(x, y);

            // Always handle hover effects, even during drags
            // This ensures hover and drag are not mutually exclusive
            if hover_idx != self.hover_shape_idx {
                let mut shapes = Vec::new();
                std::mem::swap(&mut shapes, &mut self.view.shapes);

                if let Some(idx) = self.hover_shape_idx {
                    shapes[idx].on_hover(self, false, Point::new(x, y));
                }

                // Call on_hover for the new shape
                if let Some(idx) = hover_idx {
                    shapes[idx].on_hover(self, true, Point::new(x, y));
                }

                std::mem::swap(&mut shapes, &mut self.view.shapes);
                self.hover_shape_idx = hover_idx;

                // Return true to indicate we processed a hover event
                return true;
            }

            // Handle dragging
            if let (Some(idx), Some(start_x), Some(start_y)) = (
                self.dragging_shape_idx,
                self.drag_start_x,
                self.drag_start_y,
            ) {
                let mut shapes = Vec::new();
                std::mem::swap(&mut shapes, &mut self.view.shapes);
                shapes[idx].on_drag(
                    self,
                    DragPhase::Move,
                    Point::new(start_x, start_y),
                    Point::new(x, y),
                );
                std::mem::swap(&mut shapes, &mut self.view.shapes);

                return true;
            }
        }

        false
    }

    fn render(&mut self, dimensions: Dimensions) -> String {
        self.view.clear();
        view(self, dimensions);
        self.view.render(dimensions)
    }
}

fn view(app: &mut DrawingApp, dimensions: Dimensions) {
    let colors = [
        Palette::Black,
        Palette::Red,
        Palette::Green,
        Palette::Blue,
        Palette::Purple,
    ];
    Layout::new(dynamic(|_: &mut DrawingApp| {
        stack(vec![
            //>
            column(vec![
                //>
                row_spaced(
                    10.,
                    vec![
                        draw(|area, app: &mut DrawingApp| {
                            app.view.rect(
                                rect()
                                    .fill(Color::WHITE)
                                    .stroke(Color::BLACK)
                                    .stroke_width(1.0)
                                    .x(area.x)
                                    .y(area.y)
                                    .width(area.width)
                                    .height(area.height)
                                    .on_click(|app: &mut DrawingApp| {
                                        app.strokes.clear();
                                    }),
                            );
                            app.view.text(
                                text()
                                    .fill(Color::BLACK)
                                    .x(area.x + area.width / 2.0)
                                    .y(area.y + area.height / 2.0)
                                    .text_align(TextAlign::Center)
                                    .text("Clear")
                                    .font_size(20.)
                                    .on_click(|app: &mut DrawingApp| {
                                        app.strokes.clear();
                                    }),
                            );
                        }),
                        group(
                            colors
                                .into_iter()
                                .map(move |color| {
                                    draw(move |area, app: &mut DrawingApp| {
                                        app.view.rect(
                                            rect()
                                                .fill(color.color().lerp(
                                                    Color::WHITE,
                                                    if Some(color) == app.hovered_color {
                                                        0.2
                                                    } else {
                                                        0.0
                                                    },
                                                    HueDirection::Shorter,
                                                ))
                                                .stroke(if color == app.current_color {
                                                    Color::BLACK
                                                } else {
                                                    Color::TRANSPARENT
                                                })
                                                .stroke_width(if color == app.current_color {
                                                    5.0
                                                } else {
                                                    1.0
                                                })
                                                .x(area.x)
                                                .y(area.y)
                                                .width(area.width)
                                                .height(area.height)
                                                .on_click(move |app: &mut DrawingApp| {
                                                    app.current_color = color;
                                                })
                                                .on_hover(
                                                    move |app: &mut DrawingApp, hovered, _| {
                                                        if hovered {
                                                            app.hovered_color = Some(color);
                                                        } else if app.hovered_color == Some(color) {
                                                            app.hovered_color = None;
                                                        }
                                                    },
                                                ),
                                        );
                                    })
                                })
                                .collect(),
                        ),
                    ],
                )
                .height(80.),
                draw(|area, app: &mut DrawingApp| {
                    app.view.rect(
                        rect()
                            .fill(Color::WHITE)
                            .x(area.x)
                            .y(area.y)
                            .width(area.width)
                            .height(area.height)
                            .on_drag(move |app: &mut DrawingApp, phase, _, current| match phase {
                                DragPhase::Start => {
                                    app.is_drawing = true;
                                    app.strokes.push(Stroke {
                                        points: Vec::new(),
                                        color: app.current_color,
                                    });
                                }
                                DragPhase::Move => {
                                    if let Some(stroke) = app.strokes.last_mut() {
                                        stroke.points.push((current.x as f64, current.y as f64));
                                    }
                                }
                                DragPhase::End => {
                                    if let Some(stroke) = app.strokes.last_mut() {
                                        stroke.points.push((current.x as f64, current.y as f64));
                                    }
                                    app.is_drawing = false;
                                }
                            }),
                    );
                    for stroke in &app.strokes {
                        if stroke.points.is_empty() {
                            continue;
                        }
                        let mut path =
                            path().move_to(stroke.points[0].0 as f32, stroke.points[0].1 as f32);
                        for point in &stroke.points {
                            path = path.line_to(point.0 as f32, point.1 as f32);
                        }
                        app.view
                            .path(path.stroke_width(10.).stroke(stroke.color.color()));
                    }
                })
                .pad(20.),
            ]),
        ])
    }))
    .draw(
        Area::new(0., 0., dimensions.width as f32, dimensions.height as f32),
        app,
    );
}

// Export our app using the salt_app macro
salt_app!(DrawingApp);
