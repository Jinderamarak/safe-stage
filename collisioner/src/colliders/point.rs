use crate::colliders::aligned::{AlignedBox, AlignedBoxCollision};
use crate::common::vector::Vector3 as Point;

pub trait PointCollision {
    fn collides(&self, other: &Point) -> bool;
}

impl PointCollision for Point {
    fn collides(&self, other: &Point) -> bool {
        self == other
    }
}

impl AlignedBoxCollision for Point {
    fn collides(&self, other: &AlignedBox) -> bool {
        PointCollision::collides(other, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_points_collide() {
        let point = Point::new(0.0, 0.0, 0.0);
        let other = Point::new(0.0, 0.0, 0.0);

        assert!(PointCollision::collides(&point, &other));
    }
    
    #[test]
    fn test_different_points_dont_collide() {
        let point = Point::new(0.0, 0.0, 0.0);
        let other = Point::new(1.0, 1.0, 1.0);

        assert!(!PointCollision::collides(&point, &other));
    }
}
