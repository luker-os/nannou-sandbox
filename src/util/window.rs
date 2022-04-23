use nannou::prelude::*;

pub fn window_divided(app: &App, rows: i32, cols: i32) -> Point2 {
    let window = app.window_rect();
    let height = window.h();
    let width = window.w();

    let x_div = width as f32 / cols as f32;
    let y_div = height as f32 / rows as f32;

    Point2::new(x_div, y_div)
}

pub fn scale_point(app: &App) -> Point2 {
    let window = app.window_rect();
    Point2::new(window.w(), window.h())
}

// pub fn cart_offset(app: &App, Point2)
