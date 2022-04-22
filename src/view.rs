use nannou::prelude::*;

use crate::{model::Model, rounded_corners::rounded_rect};

pub fn view(app: &App, _model: &Model, frame: Frame) {
    let grey = Rgb::from_components((0.5f32, 0.5f32, 0.5f32));
    let draw = app.draw();
    draw.background().color(grey);

    let rows = 4;
    let columns = 5;

    let window = app.window_rect();
    let height = window.h();
    let width = window.w();

    let x_div = width as f32 / columns as f32;
    let y_div = height as f32 / rows as f32;

    for r in 0..rows {
        let r = r as f32;
        for c in 0..columns {
            let c = c as f32;

            let x = x_div / 2 as f32 + x_div * c;
            let x = x - width / 2f32;

            let y = y_div / 2 as f32 + y_div * r;
            let y = y - height / 2f32;

            let grey = ((app.time + r + c) / 3f32).sin() / 2f32 + 0.5f32;

            let grey = lin_srgb(grey, grey, grey);

            let size = (app.time + r * 3f32 + c * 3f32).sin() * 25f32 + 200f32;
            let radius = (app.time + r * 5f32 + c * 5f32).sin() * size / 2f32 + size / 2f32;
            let points = rounded_rect(&Rect::from_x_y_w_h(x, y, size, size), &(radius / 2f32));

            draw.polygon().points(points).color(grey);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
