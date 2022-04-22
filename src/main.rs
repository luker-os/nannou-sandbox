use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).fullscreen().build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let grey = Rgb::from_components((0.5f32, 0.5f32, 0.5f32));
    let draw = app.draw();
    draw.background().color(grey);

    let rows = 4;
    let columns = 5;
    let size = 200f32;

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

            let grey = app.time.sin() / 2f32 + 0.5f32;
            
            let grey = Rgb::from_components((grey, grey, grey));
            draw.rect().color(grey).w(size).h(size).x_y(x, y);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
