use nannou::prelude::Point2;

/// Given a two points and a distance, returns a thir point that is `distance` away
/// from `anchor` in the direction of `towards`.
pub fn point_towards(anchor: Point2, towards: Point2, distance: &f32) -> Point2 {
    let total_distance = anchor.distance(towards);
    let x = anchor.x - (distance * (anchor.x - towards.x)) / total_distance;
    let y = anchor.y - (distance * (anchor.y - towards.y)) / total_distance;

    Point2::new(x, y)
}

pub fn shift(points: Vec<Point2>, offset: Point2) -> Vec<Point2> {
    points.into_iter().map(|point| point + offset).collect()
}
