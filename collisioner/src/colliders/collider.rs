use crate::colliders::{AlignedBoxCollider, Bounded, Collides, OrientedBoxCollider, PointCollider};
use crate::common::Vector3;

/// # Collider
/// Enum representing all possible collider primitives.
/// ## Example
/// ```
/// use collisioner::common::Vector3;
/// use collisioner::colliders::{Collider, PointCollider, AlignedBoxCollider, Collides};
///
/// let point = PointCollider::new(Vector3::new(0.0, 0.0, 0.0));
/// let collider_point = Collider::from(point);
///
/// let aligned_box = AlignedBoxCollider::new(Vector3::new(-1.0, -1.0, -1.0), Vector3::new(2.0, 2.0, 2.0));
/// let collider_box = Collider::from(aligned_box);
///
/// assert!(collider_point.collides_with(&collider_box));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Collider {
    Point(PointCollider),
    AlignedBox(AlignedBoxCollider),
    OrientedBox(OrientedBoxCollider),
}

impl From<PointCollider> for Collider {
    fn from(point: PointCollider) -> Self {
        Self::Point(point)
    }
}

impl From<AlignedBoxCollider> for Collider {
    fn from(aligned_box: AlignedBoxCollider) -> Self {
        Self::AlignedBox(aligned_box)
    }
}

impl From<OrientedBoxCollider> for Collider {
    fn from(oriented_box: OrientedBoxCollider) -> Self {
        Self::OrientedBox(oriented_box)
    }
}

impl Collides<Collider> for Collider {
    fn collides_with(&self, other: &Collider) -> bool {
        match (self, other) {
            (Collider::Point(point), Collider::Point(other_point)) => {
                point.collides_with(other_point)
            }
            (Collider::Point(point), Collider::AlignedBox(aligned_box)) => {
                point.collides_with(aligned_box)
            }
            (Collider::Point(point), Collider::OrientedBox(oriented_box)) => {
                point.collides_with(oriented_box)
            }
            (Collider::AlignedBox(aligned_box), Collider::Point(point)) => {
                aligned_box.collides_with(point)
            }
            (Collider::AlignedBox(aligned_box), Collider::AlignedBox(other_aligned_box)) => {
                aligned_box.collides_with(other_aligned_box)
            }
            (Collider::AlignedBox(aligned_box), Collider::OrientedBox(oriented_box)) => {
                aligned_box.collides_with(oriented_box)
            }
            (Collider::OrientedBox(oriented_box), Collider::Point(point)) => {
                oriented_box.collides_with(point)
            }
            (Collider::OrientedBox(oriented_box), Collider::AlignedBox(aligned_box)) => {
                oriented_box.collides_with(aligned_box)
            }
            (Collider::OrientedBox(oriented_box), Collider::OrientedBox(other_oriented_box)) => {
                oriented_box.collides_with(other_oriented_box)
            }
        }
    }
}

impl Bounded for Collider {
    fn min(&self) -> Vector3 {
        match self {
            Collider::Point(point) => point.min(),
            Collider::AlignedBox(aligned_box) => aligned_box.min(),
            Collider::OrientedBox(oriented_box) => oriented_box.min(),
        }
    }

    fn max(&self) -> Vector3 {
        match self {
            Collider::Point(point) => point.max(),
            Collider::AlignedBox(aligned_box) => aligned_box.max(),
            Collider::OrientedBox(oriented_box) => oriented_box.max(),
        }
    }
}
