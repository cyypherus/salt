use backer::{models::Area, nodes::*, Layout};
use color::HueDirection;
use salt::{
    id, salt_app,
    ui::{
        components::{path, rect, text},
        gesture::DragPhase,
        AppCtx, Color,
    },
    App, Dimensions,
};
use wasm_bindgen::prelude::*;

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

struct DrawingApp {
    state: DrawingAppState,
    ctx: AppCtx<DrawingAppState>,
}

// Drawing application that tracks points when dragging the mouse
pub struct DrawingAppState {
    // Collection of strokes for the user's drawing
    strokes: Vec<Stroke>,
    // Current drawing state
    is_drawing: bool,
    // Currently selected color
    current_color: Palette,
    hovered_color: Option<Palette>,
}

impl App for DrawingApp {
    type State = DrawingAppState;
    fn new() -> Self {
        Self {
            state: DrawingAppState {
                strokes: Vec::new(),
                is_drawing: false,
                current_color: Palette::Black,
                hovered_color: None,
            },
            ctx: AppCtx::new(),
        }
    }

    fn view(&mut self, dimensions: Dimensions) {
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
                            draw(clear_button),
                            group(
                                colors
                                    .into_iter()
                                    .enumerate()
                                    .map(move |(i, color)| {
                                        draw(move |area, app: &mut DrawingApp| {
                                            color_button(id!(i as u64), color, area, app);
                                        })
                                    })
                                    .collect(),
                            ),
                        ],
                    )
                    .height(80.),
                    draw(canvas).pad(20.),
                ]),
            ])
            .pad(20.)
        }))
        .draw(
            Area::new(0., 0., dimensions.width as f32, dimensions.height as f32),
            self,
        );
    }

    fn state(&mut self) -> (&mut AppCtx<Self::State>, &mut Self::State) {
        (&mut self.ctx, &mut self.state)
    }
}

fn clear_button(area: Area, app: &mut DrawingApp) {
    app.ctx.view.push(
        rect()
            .fill(Color::WHITE)
            .stroke(Color::BLACK)
            .stroke_width(1.0)
            .x(area.x)
            .y(area.y)
            .width(area.width)
            .height(area.height)
            .finish(id!())
            .on_click(|state: &mut DrawingAppState| {
                state.strokes.clear();
            }),
    );
    app.ctx.view.push(
        text()
            .fill(Color::BLACK)
            .x(area.x + area.width / 2.0)
            .y(area.y + area.height / 2.0)
            .text_align(salt::ui::TextAlign::Center)
            .text("Clear")
            .font_size(20.)
            .finish(id!())
            .on_click(|state: &mut DrawingAppState| {
                state.strokes.clear();
            }),
    );
}

fn color_button(id: u64, color: Palette, area: Area, app: &mut DrawingApp) {
    app.ctx.view.push(
        rect()
            .corner_radius(5.0)
            .fill(color.color().lerp(
                Color::WHITE,
                if Some(color) == app.state.hovered_color {
                    0.2
                } else {
                    0.0
                },
                HueDirection::Shorter,
            ))
            .stroke(if color == app.state.current_color {
                Color::BLACK
            } else {
                Color::TRANSPARENT
            })
            .stroke_width(if color == app.state.current_color {
                5.0
            } else {
                1.0
            })
            .x(area.x)
            .y(area.y)
            .width(area.width)
            .height(area.height)
            .finish(id!(id))
            .on_click(move |state: &mut DrawingAppState| {
                state.current_color = color;
            })
            .on_hover(move |state: &mut DrawingAppState, hovered, _| {
                if hovered {
                    state.hovered_color = Some(color);
                } else if state.hovered_color == Some(color) {
                    state.hovered_color = None;
                }
            }),
    );
}

fn canvas(area: Area, app: &mut DrawingApp) {
    app.ctx.view.push(
        rect()
            .fill(Color::WHITE)
            .x(area.x)
            .y(area.y)
            .width(area.width)
            .height(area.height)
            .finish(id!())
            .on_drag(
                move |state: &mut DrawingAppState, phase, _, current| match phase {
                    DragPhase::Start => {
                        state.is_drawing = true;
                        state.strokes.push(Stroke {
                            points: Vec::new(),
                            color: state.current_color,
                        });
                    }
                    DragPhase::Move => {
                        if let Some(stroke) = state.strokes.last_mut() {
                            stroke.points.push((current.x as f64, current.y as f64));
                        }
                    }
                    DragPhase::End => {
                        if let Some(stroke) = state.strokes.last_mut() {
                            stroke.points.push((current.x as f64, current.y as f64));
                        }
                        state.is_drawing = false;
                    }
                },
            ),
    );
    for stroke in &app.state.strokes {
        if stroke.points.is_empty() {
            continue;
        }
        let mut path = path().move_to(stroke.points[0].0 as f32, stroke.points[0].1 as f32);
        for point in &stroke.points {
            path = path.line_to(point.0 as f32, point.1 as f32);
        }
        app.ctx.view.push(
            path.stroke_width(10.)
                .fill(Color::TRANSPARENT)
                .stroke(stroke.color.color())
                .finish(id!()),
        );
    }
}

salt_app!(DrawingApp);
