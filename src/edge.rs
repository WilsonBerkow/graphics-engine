use point::{Color, Point);

struct EdgeMeta {
    count: usize,
    color: Color,
}

struct EdgeList {
    points: Vec<Point>,
    meta: Vec<EdgeMeta>
}

