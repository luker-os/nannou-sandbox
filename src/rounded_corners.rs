use nannou::prelude::*;

use crate::util::point2::point_towards;

pub fn rounded_rect(reference: &Rect, radius: &f32) -> Vec<Point2> {
    let mut points = vec![];

    let tl = reference.top_left();
    let tr = reference.top_right();
    let bl = reference.bottom_left();
    let br = reference.bottom_right();

    points.extend(rounded_corner(tl, bl, tr, radius));
    points.extend(rounded_corner(tr, tl, br, radius));
    points.extend(rounded_corner(br, tr, bl, radius));
    points.extend(rounded_corner(bl, br, tl, radius));

    if points.len() > 0 {
        points.push(points.get(0).unwrap().clone());
    } else {
        // no points were created due to small radius. return the original reference rect's corners.
        return vec![tl, tr, br, bl, tl];
    }

    points
}

/// Return a vec of points representing a rounded corner.
pub fn rounded_corner(anchor: Point2, before: Point2, after: Point2, radius: &f32) -> Vec<Point2> {
    // Find points to start curving from.
    let p1 = point_towards(anchor, before, &radius);
    let p2 = point_towards(anchor, after, &radius);

    let radius = *radius;

    let v1 = anchor - p1;
    let v2 = anchor - p2;

    //Angle between vector 1 and vector 2 divided by 2
    let angle = v1.angle_between(v2) / 2f32;

    let tan = angle.tan().abs();
    let segment = radius / tan;

    let length1 = v1.distance(Point2::ZERO);
    let length2 = v2.distance(Point2::ZERO);

    // Points of intersection are calculated by the proportion between
    // the coordinates of the vector, length of vector and the length of the segment.
    let p1_cross = anchor - v1 * segment / length1;
    let p2_cross = anchor - v2 * segment / length2;

    // Calculation of the coordinates of the circle
    // center by the addition of angular vectors.
    let d = anchor * 2f32 - p1_cross - p2_cross;

    let length = d.distance(Point2::ZERO);
    let segment = Point2::new(segment, radius).distance(Point2::ZERO);

    let circle_point = anchor - d * segment / length;

    // start_angle and end_angle of arc

    let start_angle = (p1_cross.y - circle_point.y).atan2(p1_cross.x - circle_point.x);
    let end_angle = (p2_cross.y - circle_point.y).atan2(p2_cross.x - circle_point.x);

    // Sweep angle
    let mut sweep_angle = end_angle - start_angle;

    if sweep_angle > f32::PI() {
        sweep_angle = f32::PI() - sweep_angle;
    }

    let degree_factor = 180f32 / f32::PI();
    let points_count = (sweep_angle * degree_factor).abs().round() as i32;
    let sign = sweep_angle.signum();

    let mut points = vec![];

    // control iteration direction so that points are tracked in a continuous order.
    let range;
    if sign > 0f32 {
        range = (0..points_count).rev().collect::<Vec<i32>>();
    } else {
        range = (0..points_count).collect::<Vec<i32>>();
    }

    // Collect points on the curve
    for i in range {
        let point_x =
            circle_point.x + (start_angle + sign * i as f32 / degree_factor).cos() * radius;

        let point_y =
            circle_point.y + (start_angle + sign * i as f32 / degree_factor).sin() * radius;

        points.push(Point2::new(point_x, point_y));
    }
    points
}
