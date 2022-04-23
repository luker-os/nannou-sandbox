use nannou::prelude::*;

use crate::{sin_rect::SinRect, view::view};

static ROWS: i32 = 4;
static COLS: i32 = 5;

pub struct Model {
    _window: window::Id,
    pub sin_rects: Vec<SinRect>,
}

impl Model {
    pub fn new(app: &App) -> Model {
        let _window = app.new_window().view(view).fullscreen().build().unwrap();
        let sin_rects = SinRect::screen_grid_rects(app, ROWS, COLS);
        Model { _window, sin_rects }
    }
}
