use crate::colliders::{AlignedBox, Point};

pub enum Collider {
    Point(Point),
    AlignedBox(AlignedBox),
}

impl From<Point> for Collider {
    fn from(point: Point) -> Self {
        Self::Point(point)
    }
}

impl From<AlignedBox> for Collider {
    fn from(aligned_box: AlignedBox) -> Self {
        Self::AlignedBox(aligned_box)
    }
}
