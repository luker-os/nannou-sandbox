use nannou::color::encoding::Linear;
use nannou::color::IntoLinSrgba;
use nannou::draw::properties::ColorScalar;
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

            let grey = lin_srgb(grey, grey, grey);

            // draw.rect().color(grey).w(size).h(size).x_y(x, y);
            // let half_size = size / 2f32;
            // let points = vec![
            //     (pt2(-half_size, half_size), grey),
            //     (pt2(half_size, half_size), grey),
            //     (pt2(half_size, -half_size), grey),
            //     (pt2(-half_size, -half_size), grey),
            // ];

            // let size = app.time.sin() * 25f32 + 100f32;
            let radius = app.time.sin() * size / 2f32 + size / 2f32;
            draw_rounded_rect(
                &draw,
                &Rect::from_x_y_w_h(x, y, size, size),
                &(radius / 2f32),
                grey,
            );

            // draw.polygon().points_colored(points);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

pub fn draw_rounded_rect<C>(draw: &Draw, reference: &Rect, radius: &f32, color: C)
where
    C: IntoLinSrgba<ColorScalar> + Clone,
{
    let mut points = vec![];
    let a = reference.top_left();
    let b = reference.top_right();
    let c = reference.bottom_left();
    let d = reference.bottom_right();

    // top left
    let p1 = border_loc(a, c, radius);
    let p2 = border_loc(a, b, radius);
    draw.ellipse().w(5f32).h(5f32).x_y(p1.x, p1.y);
    draw.ellipse().w(5f32).h(5f32).x_y(p2.x, p2.y);

    draw.ellipse().w(5f32).h(5f32).x_y(a.x, a.y);
    draw.ellipse().w(5f32).h(5f32).x_y(b.x, b.y);
    draw.ellipse().w(5f32).h(5f32).x_y(c.x, c.y);
    draw.ellipse().w(5f32).h(5f32).x_y(d.x, d.y);

    let start = p1.clone();

    let curve: Vec<_> = rounded_corner(a, p1, p2, &radius)
        .into_iter()
        .map(|point| (point, color.clone()))
        .collect();

    points.extend(curve);

    // top right
    let p1 = border_loc(b, a, radius);
    let p2 = border_loc(b, d, radius);

    let curve: Vec<_> = rounded_corner(b, p1, p2, &radius)
        .into_iter()
        .map(|point| (point, color.clone()))
        .collect();
    points.extend(curve);

    // bottom right
    let p1 = border_loc(d, b, radius);
    let p2 = border_loc(d, c, radius);

    let curve: Vec<_> = rounded_corner(d, p1, p2, &radius)
        .into_iter()
        .map(|point| (point, color.clone()))
        .collect();
    points.extend(curve);

    // bottom left
    let p1 = border_loc(c, d, radius);
    let p2 = border_loc(c, a, radius);

    let curve: Vec<_> = rounded_corner(c, p1, p2, &radius)
        .into_iter()
        .map(|point| (point, color.clone()))
        .collect();
    points.extend(curve);

    points.push((start, color.clone()));
    draw.polygon().points_colored(points);
}

pub fn border_loc(anchor: Point2, towards: Point2, distance: &f32) -> Point2 {
    let total_distance = anchor.distance(towards);
    let x = anchor.x - (distance * (anchor.x - towards.x)) / total_distance;
    let y = anchor.y - (distance * (anchor.y - towards.y)) / total_distance;

    Point2::new(x, y)
}

pub fn rounded_corner(angularPoint: Point2, p1: Point2, p2: Point2, radius: &f32) -> Vec<Point2> {
    let mut radius = *radius;

    //Vector 1
    let dx1 = angularPoint.x - p1.x;
    let dy1 = angularPoint.y - p1.y;

    //Vector 1
    let dx2 = angularPoint.x - p2.x;
    let dy2 = angularPoint.y - p2.y;

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
    let p1Cross = proportion_point(angularPoint, segment, length1, dx1, dy1);
    let p2Cross = proportion_point(angularPoint, segment, length2, dx2, dy2);

    // Calculation of the coordinates of the circle
    // center by the addition of angular vectors.
    let dx = angularPoint.x * 2f32 - p1Cross.x - p2Cross.x;
    let dy = angularPoint.y * 2f32 - p1Cross.y - p2Cross.y;

    let L = length(dx, dy);
    let d = length(segment, radius);

    let circlePoint = proportion_point(angularPoint, d, L, dx, dy);

    //StartAngle and EndAngle of arc
    let mut startAngle = (p1Cross.y - circlePoint.y).atan2(p1Cross.x - circlePoint.x);
    let endAngle = (p2Cross.y - circlePoint.y).atan2(p2Cross.x - circlePoint.x);

    //Sweep angle
    let mut sweepAngle = endAngle - startAngle;

    //Some additional checks
    if sweepAngle < 0f32 {
        startAngle = endAngle;
        sweepAngle = -sweepAngle;
    }

    if sweepAngle > f32::PI() {
        sweepAngle = f32::PI() - sweepAngle;
    }

    // --------

    let degreeFactor = 180f32 / f32::PI();

    let pointsCount = (sweepAngle * degreeFactor).abs().round() as i32;
    let sign = sweepAngle.signum();

    let mut points = vec![];

    let range;

    if sign > 0f32 {
        range = (0..pointsCount).rev().collect::<Vec<i32>>();
    } else {
        range = (0..pointsCount).collect::<Vec<i32>>();
    }

    for i in range {
        let pointX = circlePoint.x + (startAngle + sign * i as f32 / degreeFactor).cos() * radius;

        let pointY = (circlePoint.y + (startAngle + sign * i as f32 / degreeFactor).sin() * radius);

        points.push(Point2::new(pointX, pointY));
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

#[test]
fn debug_test() {
    let size = 100f32;
    let radius = &40f32;

    let reference = Rect::from_x_y_w_h(0f32, 0f32, size, size);

    let a = reference.top_left();
    let b = reference.top_right();
    let c = reference.bottom_left();
    let d = reference.bottom_right();

    let p1 = border_loc(a, c, radius);
    let p2 = border_loc(a, b, radius);

    let curve: Vec<_> = rounded_corner(a, p1, p2, &radius)
        .into_iter()
        .map(|point| (point, BLACK))
        .collect();

    // top right
    let p1 = border_loc(b, a, radius);
    let p2 = border_loc(b, d, radius);

    let curve: Vec<_> = rounded_corner(b, p1, p2, &radius)
        .into_iter()
        .map(|point| (point, RED))
        .collect();

    // bottom right
    let p1 = border_loc(d, b, radius);
    let p2 = border_loc(d, c, radius);

    let curve: Vec<_> = rounded_corner(d, p1, p2, &radius)
        .into_iter()
        .map(|point| (point, BLACK))
        .collect();

    // bottom left
    let p1 = border_loc(c, d, radius);
    let p2 = border_loc(c, a, radius);

    let curve: Vec<_> = rounded_corner(c, p1, p2, &radius)
        .into_iter()
        .map(|point| (point, BLACK))
        .collect();
}
