use crate::colliders::{AlignedBoxCollider, Bounded, Collides};
use crate::common::Vector3;

/// # Point Collider
/// Basic primitive for representing single point.
/// Only alias for `Vector3`.
///
/// ## Example
/// ```
/// use collisioner::colliders::PointCollider;
/// use collisioner::colliders::Collides;
/// use collisioner::common::Vector3;
///
/// let point1 = PointCollider::new(Vector3::new(0.0, 0.0, 0.0));
/// let point2 = PointCollider::new(Vector3::new(1.0, 1.0, 1.0));
/// let point3 = PointCollider::new(Vector3::new(1.0, 1.0, 1.0));
///
/// assert!(!point1.collides_with(&point2));
/// assert!(point2.collides_with(&point3));
/// assert!(!point1.collides_with(&point3));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PointCollider {
    position: Vector3,
}

impl PointCollider {
    pub fn new(position: Vector3) -> Self {
        Self { position }
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }
}

impl From<Vector3> for PointCollider {
    fn from(position: Vector3) -> Self {
        Self::new(position)
    }
}

impl Bounded for PointCollider {
    fn min(&self) -> Vector3 {
        self.position()
    }

    fn max(&self) -> Vector3 {
        self.position()
    }
}

impl Collides<Self> for PointCollider {
    fn collides_with(&self, other: &Self) -> bool {
        self.position() == other.position()
    }
}

impl Collides<AlignedBoxCollider> for PointCollider {
    fn collides_with(&self, other: &AlignedBoxCollider) -> bool {
        other.collides_with(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn bounding_volume() {
        let point = PointCollider::new(Vector3::new(0.0, 0.0, 0.0));

        assert_eq!(point.position(), point.min());
        assert_eq!(point.position(), point.max());
    }

    #[test]
    fn same_points_collide() {
        let point = PointCollider::new(Vector3::new(0.0, 0.0, 0.0));
        let other = PointCollider::new(Vector3::new(0.0, 0.0, 0.0));

        assert!(point.collides_with(&other));
    }

    #[test]
    fn different_points_dont_collide() {
        let point = PointCollider::new(Vector3::new(0.0, 0.0, 0.0));
        let other = PointCollider::new(Vector3::new(1.0, 1.0, 1.0));

        assert!(!point.collides_with(&other));
    }
}
