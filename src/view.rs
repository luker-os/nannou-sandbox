use nannou::{color, prelude::*};

use crate::{
    model::Model,
    rounded_corners::rounded_rect,
    util::{
        grid::{grid_point, grid_points},
        wave::Sin,
        window::{scale_point, window_divided},
    },
};

lazy_static! {
    static ref BACKGROUND_COLOR: Gray<f32> = color::gray(0.5f32);
}

static ROWS: i32 = 4;
static COLS: i32 = 5;

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // Reset
    clear(&draw);

    // Create
    draw_rounded_rect_grid(app, &draw, model, &frame);

    // Commit
    draw.to_frame(app, &frame).unwrap();
}

fn draw_rounded_rect_grid(app: &App, draw: &Draw, _model: &Model, _frame: &Frame) {
    let div = window_divided(app, ROWS, COLS);

    for point in grid_points(ROWS, COLS) {
        let center = grid_point(div, point, scale_point(&app));

        let value = Sin::with_range(point.x + point.y, 80f32, 0f32, 1f32).get(app.time);
        let size =
            Sin::with_range(point.x * 3f32 + point.y * 3f32, 5f32, 175f32, 225f32).get(app.time);
        let radius = Sin::with_range(point.x * 5f32 + point.y * 5f32, 10f32, 0f32, size / 2f32)
            .get(app.time);

        let points = rounded_rect(
            &Rect::from_xy_wh(center, Point2::ONE * size),
            &(radius / 2f32),
        );

        draw.polygon().points(points).color(color::gray(value));
    }
}

fn clear(draw: &Draw) {
    draw.background().color(*BACKGROUND_COLOR);
}
