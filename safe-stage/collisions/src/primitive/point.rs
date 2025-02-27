use crate::collides_group_impl;
use crate::common::{Bounded, Collides, Projectable, Rotation, Transformation, Translation};
use crate::primitive::{AlignedBoxCollider, OrientedBoxCollider, SphereCollider};
use maths::{Quaternion, Vector3};

/// # Point Collider
/// Collision primitive for representing single point.
///
/// ## Example
/// ```
/// use collisions::primitive::PointCollider;
/// use collisions::common::Collides;
/// use maths::Vector3;
///
/// let point1 = PointCollider::new(Vector3::new(0.0, 0.0, 0.0));
/// let point2 = PointCollider::new(Vector3::new(1.0, 1.0, 1.0));
/// let point3 = PointCollider::new(Vector3::new(1.0, 1.0, 1.0));
///
/// assert!(!point1.collides_with(&point2));
/// assert!(point2.collides_with(&point3));
/// assert!(!point1.collides_with(&point3));
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct PointCollider {
    position: Vector3,
}

impl PointCollider {
    /// Creates a new `PointCollider` with `position`.
    pub fn new(position: Vector3) -> Self {
        Self { position }
    }

    /// Returns the position of the point.
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

impl Projectable for PointCollider {
    fn project(&self, axis: &Vector3) -> (f64, f64) {
        let projection = self.position().dot(axis);
        (projection, projection)
    }
}

impl Rotation for PointCollider {
    fn rotate(&self, _: &Quaternion) -> Self {
        self.clone()
    }

    fn rotate_around(&self, rotation: &Quaternion, pivot: &Vector3) -> Self {
        Self::new(self.position.rotate_around(rotation, pivot))
    }
}

impl Translation for PointCollider {
    fn translate(&self, translation: &Vector3) -> Self {
        Self::new(self.position() + translation)
    }
}

impl Transformation for PointCollider {
    fn transform(&self, rotation: &Quaternion, pivot: &Vector3, translation: &Vector3) -> Self {
        let position = self.position.rotate_around(rotation, pivot) + translation;
        PointCollider::new(position)
    }
}

impl Collides<Self> for PointCollider {
    fn collides_with(&self, other: &Self) -> bool {
        self.position() == other.position()
    }
}

impl Collides<SphereCollider> for PointCollider {
    fn collides_with(&self, other: &SphereCollider) -> bool {
        other.collides_with(self)
    }
}

impl Collides<AlignedBoxCollider> for PointCollider {
    fn collides_with(&self, other: &AlignedBoxCollider) -> bool {
        other.collides_with(self)
    }
}

impl Collides<OrientedBoxCollider> for PointCollider {
    fn collides_with(&self, other: &OrientedBoxCollider) -> bool {
        other.collides_with(self)
    }
}

collides_group_impl!(
    PointCollider, PointCollider
    PointCollider, SphereCollider
    PointCollider, AlignedBoxCollider
    PointCollider, OrientedBoxCollider
);

#[cfg(test)]
mod tests {
    use super::*;
    use maths::asserts::assert_vectors;

    #[test]
    fn bounds_regular() {
        let point = PointCollider::new(Vector3::new(0.0, 0.0, 0.0));

        assert_eq!(point.position(), point.min());
        assert_eq!(point.position(), point.max());
    }

    #[test]
    fn rotation() {
        let point = PointCollider::new(Vector3::new(1.0, 2.0, 3.0));
        let rotation = Quaternion::from_euler(&Vector3::new(
            10.0_f64.to_radians(),
            20.0_f64.to_radians(),
            30.0_f64.to_radians(),
        ));
        let rotated = point.rotate(&rotation);

        assert_eq!(point.position(), rotated.position());
    }

    #[test]
    fn pivot_rotation() {
        let point = PointCollider::new(Vector3::new(0.0, 0.0, 0.0));
        let pivot = Vector3::new(1.0, 1.0, 1.0);
        let rotation = Quaternion::from_euler(&Vector3::new(
            0.0_f64.to_radians(),
            0.0_f64.to_radians(),
            90.0_f64.to_radians(),
        ));
        let rotated = point.rotate_around(&rotation, &pivot);

        assert_vectors(Vector3::new(2.0, 0.0, 0.0), rotated.position());
    }

    #[test]
    fn translation() {
        let point = PointCollider::new(Vector3::new(1.0, 2.0, 3.0));
        let translation = Vector3::new(1.0, 2.0, 3.0);
        let translated = point.translate(&translation);

        let expected = Vector3::new(2.0, 4.0, 6.0);
        assert_eq!(expected, translated.position());
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
