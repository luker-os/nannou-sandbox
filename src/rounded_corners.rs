use nannou::prelude::*;

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

    points
}

pub fn border_loc(anchor: Point2, towards: Point2, distance: &f32) -> Point2 {
    let total_distance = anchor.distance(towards);
    let x = anchor.x - (distance * (anchor.x - towards.x)) / total_distance;
    let y = anchor.y - (distance * (anchor.y - towards.y)) / total_distance;

    Point2::new(x, y)
}

pub fn rounded_corner(
    angular_point: Point2,
    before: Point2,
    after: Point2,
    radius: &f32,
) -> Vec<Point2> {
    let p1 = border_loc(angular_point, before, &radius);
    let p2 = border_loc(angular_point, after, &radius);

    let mut radius = *radius;

    //Vector 1
    let dx1 = angular_point.x - p1.x;
    let dy1 = angular_point.y - p1.y;

    //Vector 1
    let dx2 = angular_point.x - p2.x;
    let dy2 = angular_point.y - p2.y;

    //Angle between vector 1 and vector 2 divided by 2
    let angle = (dy1.atan2(dx1) - dy2.atan2(dx2)) / 2f32;

    let tan = angle.tan().abs();
    let mut segment = radius / tan;

    //Check the segment
    let length1 = length(dx1, dy1);
    let length2 = length(dx2, dy2);

    let len = length1.min(length2);

    if segment > len {
        segment = len;
        radius = len * tan;
    }

    // Points of intersection are calculated by the proportion between
    // the coordinates of the vector, length of vector and the length of the segment.
    let p1_cross = proportion_point(angular_point, segment, length1, dx1, dy1);
    let p2_cross = proportion_point(angular_point, segment, length2, dx2, dy2);

    // Calculation of the coordinates of the circle
    // center by the addition of angular vectors.
    let dx = angular_point.x * 2f32 - p1_cross.x - p2_cross.x;
    let dy = angular_point.y * 2f32 - p1_cross.y - p2_cross.y;

    let l = length(dx, dy);
    let d = length(segment, radius);

    let circle_point = proportion_point(angular_point, d, l, dx, dy);

    //start_angle and end_angle of arc
    let mut start_angle = (p1_cross.y - circle_point.y).atan2(p1_cross.x - circle_point.x);
    let end_angle = (p2_cross.y - circle_point.y).atan2(p2_cross.x - circle_point.x);

    //Sweep angle
    let mut sweep_angle = end_angle - start_angle;

    //Some additional checks
    if sweep_angle < 0f32 {
        start_angle = end_angle;
        sweep_angle = -sweep_angle;
    }

    if sweep_angle > f32::PI() {
        sweep_angle = f32::PI() - sweep_angle;
    }

    let degree_factor = 180f32 / f32::PI();

    let points_count = (sweep_angle * degree_factor).abs().round() as i32;
    let sign = sweep_angle.signum();

    let mut points = vec![];

    let range;

    if sign > 0f32 {
        range = (0..points_count).rev().collect::<Vec<i32>>();
    } else {
        range = (0..points_count).collect::<Vec<i32>>();
    }

    for i in range {
        let point_x =
            circle_point.x + (start_angle + sign * i as f32 / degree_factor).cos() * radius;

        let point_y =
            circle_point.y + (start_angle + sign * i as f32 / degree_factor).sin() * radius;

        points.push(Point2::new(point_x, point_y));
    }
    points

    // --------------------------------
    // let vec1 = corner - a;
    // let vec2 = corner - b;

    // let angle = vec1.angle_between(vec2) / 2f32;

    // let tan = angle.tan().abs();

    // // let segment = radius / tan;
    // // Skipping some code that will probably bite me...

    // let segment = corner.distance(a).min(corner.distance(b));
    // let radius = segment * tan;

    // let p1_cross = proportion_point(corner, segment, corner.distance(a), vec1);
    // let p2_cross = proportion_point(corner, segment, corner.distance(a), vec2);

    // let vector = corner * 2f32 - p1_cross - p2_cross;

    // let length =
}

pub fn length(dx: f32, dy: f32) -> f32 {
    (dx * dx + dy * dy).sqrt()
}

pub fn proportion_point(point: Point2, segment: f32, length: f32, dx: f32, dy: f32) -> Point2 {
    let factor = segment / length;
    Point2::new(point.x - dx * factor, point.y - dy * factor)
}

#[test]
fn it_test_border_loc() {
    let anchor = Point2::new(0f32, 100f32);
    let towards = Point2::new(0f32, 0f32);
    let distance = 75f32;

    let loc = border_loc(anchor, towards, &distance);

    assert_eq!(loc, Point2::new(0f32, 25f32));

    let anchor = Point2::new(0f32, -100f32);
    let towards = Point2::new(0f32, 0f32);
    let distance = 75f32;

    let loc = border_loc(anchor, towards, &distance);

    assert_eq!(loc, Point2::new(0f32, -25f32));

    let anchor = Point2::new(100f32, 0f32);
    let towards = Point2::new(0f32, 0f32);
    let distance = 75f32;

    let loc = border_loc(anchor, towards, &distance);

    assert_eq!(loc, Point2::new(25f32, 0f32));

    let anchor = Point2::new(-100f32, 0f32);
    let towards = Point2::new(0f32, 0f32);
    let distance = 75f32;

    let loc = border_loc(anchor, towards, &distance);

    assert_eq!(loc, Point2::new(-25f32, 0f32));

    let anchor = Point2::new(-100f32, -100f32);
    let towards = Point2::new(-100f32, 100f32);
    let distance = 75f32;

    let loc = border_loc(anchor, towards, &distance);

    assert_eq!(loc, Point2::new(-100f32, -25f32));
}
