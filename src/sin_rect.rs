use nannou::{color, prelude::*};

use crate::{
    rounded_corners::rounded_rect,
    util::{
        grid::{grid_point, grid_points},
        point2::shift,
        wave::Sin,
        window::{scale_point, window_divided},
    },
};

pub struct SinRect {
    radius: Sin,
    size: Sin,
    value: Sin,
    center: Point2,
}

impl SinRect {
    pub fn draw(&self, draw: &Draw, x: f32) {
        draw.polyline()
            .points(self.get_points(x))
            .color(color::gray(self.value.get(x)));

        draw.polyline()
            .points(shift(self.get_points(x), Point2::new(10f32, 5f32)))
            .color(color::gray(self.value.get(x) + 0.05f32));
    }

    pub fn get_points(&self, x: f32) -> Vec<Point2> {
        rounded_rect(
            &Rect::from_xy_wh(self.center, Point2::ONE * self.size.get(x)),
            &(self.radius.get(x) / 2f32),
        )
    }

    pub fn screen_grid_rects(app: &App, rows: i32, cols: i32) -> Vec<SinRect> {
        let div = window_divided(app, rows, cols);

        grid_points(rows, cols)
            .into_iter()
            .map(|point| {
                let center = grid_point(div, point, scale_point(&app));

                let value = Sin::with_range(point.x + point.y, 80f32, 0f32, 1f32);
                let size = Sin::with_range(point.x * 3f32 + point.y * 3f32, 40f32, 150f32, 160f32);
                let radius = Sin::with_range(point.x * 5f32 + point.y * 5f32, 10f32, 0f32, 160f32);

                Self {
                    center,
                    value,
                    size,
                    radius,
                }
            })
            .collect()
    }
}
