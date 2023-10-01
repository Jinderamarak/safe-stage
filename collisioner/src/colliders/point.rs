use crate::colliders::{AlignedBox, Collides};

/// # Point
/// Basic primitive for representing single point.
/// Only alias for `Vector3`.
///
/// ## Example
/// ```
/// use collisioner::colliders::Point;
/// use collisioner::colliders::Collides;
///
/// let point1 = Point::new(0.0, 0.0, 0.0);
/// let point2 = Point::new(1.0, 1.0, 1.0);
/// let point3 = Point::new(1.0, 1.0, 1.0);
///
/// assert!(!point1.collides_with(&point2));
/// assert!(point2.collides_with(&point3));
/// assert!(!point1.collides_with(&point3));
/// ```
pub use crate::common::Vector3 as Point;

impl Collides<Self> for Point {
    fn collides_with(&self, other: &Self) -> bool {
        self == other
    }
}

impl Collides<AlignedBox> for Point {
    fn collides_with(&self, other: &AlignedBox) -> bool {
        other.collides_with(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_points_collide() {
        let point = Point::new(0.0, 0.0, 0.0);
        let other = Point::new(0.0, 0.0, 0.0);

        assert!(point.collides_with(&other));
    }

    #[test]
    fn test_different_points_dont_collide() {
        let point = Point::new(0.0, 0.0, 0.0);
        let other = Point::new(1.0, 1.0, 1.0);

        assert!(!point.collides_with(&other));
    }
}
