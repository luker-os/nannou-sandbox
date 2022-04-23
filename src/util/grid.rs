use nannou::prelude::*;

pub fn grid_points(rows: i32, cols: i32) -> Vec<Point2> {
    let mut points = vec![];
    for r in 0..rows {
        for c in 0..cols {
            points.push(Point2::new(c as f32, r as f32));
        }
    }

    points
}

pub fn grid_point(grid_div: Point2, grid_point: Point2, scale: Point2) -> Point2 {
    (grid_div / 2f32 + grid_div * grid_point) - scale / 2f32
}
