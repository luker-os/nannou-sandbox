use nannou::prelude::*;

use crate::view::view;

pub struct Model {
    _window: window::Id,
}

impl Model {
    pub fn new(app: &App) -> Model {
        let _window = app.new_window().view(view).fullscreen().build().unwrap();
        Model { _window }
    }
}
