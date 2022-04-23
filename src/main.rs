#[macro_use]
extern crate lazy_static;

mod model;
mod rounded_corners;
mod update;
mod sin_rect;
mod util;
mod view;

fn main() {
    nannou::app(model::Model::new).update(update::update).run();
}
