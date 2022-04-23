use nannou::{color, prelude::*};

use crate::model::Model;

lazy_static! {
    static ref BACKGROUND_COLOR: Gray<f32> = color::gray(0.5f32);
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // Reset
    clear(&draw);

    // Create
    draw_rounded_rect_grid(app, &draw, model, &frame);

    // Commit
    draw.to_frame(app, &frame).unwrap();
}

fn draw_rounded_rect_grid(app: &App, draw: &Draw, model: &Model, _frame: &Frame) {
    for rect in &model.sin_rects {
        rect.draw(draw, app.time);
    }
}

fn clear(draw: &Draw) {
    draw.background().color(*BACKGROUND_COLOR);
}
