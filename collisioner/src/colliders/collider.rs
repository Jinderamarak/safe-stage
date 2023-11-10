use crate::colliders::{
    AlignedBoxCollider, Bounded, Collides, OrientedBoxCollider, PointCollider, SphereCollider,
};
use crate::common::{Quaternion, Vector3};

/// # Generic Collider
/// Enum representing all possible collider primitives.
/// ## Example
/// ```
/// use collisioner::common::Vector3;
/// use collisioner::colliders::{Collider, PointCollider, AlignedBoxCollider, Collides};
///
/// let point = PointCollider::new(Vector3::new(0.0, 0.0, 0.0));
/// let collider_point = Collider::from(point);
///
/// let collider_box = Collider::aligned_box(-1.0, -1.0, -1.0, 2.0, 2.0, 2.0);
///
/// assert!(collider_point.collides_with(&collider_box));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Collider {
    Point(PointCollider),
    Sphere(SphereCollider),
    AlignedBox(AlignedBoxCollider),
    OrientedBox(OrientedBoxCollider),
}

impl Collider {
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self::Point(PointCollider::new(Vector3::new(x, y, z)))
    }

    pub fn sphere(x: f64, y: f64, z: f64, r: f64) -> Self {
        Self::Sphere(SphereCollider::new(Vector3::new(x, y, z), r))
    }

    pub fn aligned_box(x: f64, y: f64, z: f64, w: f64, h: f64, d: f64) -> Self {
        Self::AlignedBox(AlignedBoxCollider::new(
            Vector3::new(x, y, z),
            Vector3::new(w, h, d),
        ))
    }

    pub fn oriented_box(
        x: f64,
        y: f64,
        z: f64,
        w: f64,
        h: f64,
        d: f64,
        rx: f64,
        ry: f64,
        rz: f64,
    ) -> Self {
        Self::OrientedBox(OrientedBoxCollider::new(
            Vector3::new(x, y, z),
            Vector3::new(w, h, d),
            Quaternion::from_euler(Vector3::new(rx, ry, rz)),
        ))
    }
}

impl From<PointCollider> for Collider {
    fn from(point: PointCollider) -> Self {
        Self::Point(point)
    }
}

impl From<SphereCollider> for Collider {
    fn from(sphere: SphereCollider) -> Self {
        Self::Sphere(sphere)
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
            (Collider::Point(point), Collider::Sphere(sphere)) => point.collides_with(sphere),
            (Collider::Point(point), Collider::AlignedBox(aligned_box)) => {
                point.collides_with(aligned_box)
            }
            (Collider::Point(point), Collider::OrientedBox(oriented_box)) => {
                point.collides_with(oriented_box)
            }
            (Collider::Sphere(sphere), Collider::Point(point)) => sphere.collides_with(point),
            (Collider::Sphere(sphere), Collider::Sphere(other_sphere)) => {
                sphere.collides_with(other_sphere)
            }
            (Collider::Sphere(sphere), Collider::AlignedBox(aligned_box)) => {
                sphere.collides_with(aligned_box)
            }
            (Collider::Sphere(sphere), Collider::OrientedBox(oriented_box)) => {
                sphere.collides_with(oriented_box)
            }
            (Collider::AlignedBox(aligned_box), Collider::Point(point)) => {
                aligned_box.collides_with(point)
            }
            (Collider::AlignedBox(aligned_box), Collider::Sphere(sphere)) => {
                aligned_box.collides_with(sphere)
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
            (Collider::OrientedBox(oriented_box), Collider::Sphere(sphere)) => {
                oriented_box.collides_with(sphere)
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
            Collider::Sphere(sphere) => sphere.min(),
            Collider::AlignedBox(aligned_box) => aligned_box.min(),
            Collider::OrientedBox(oriented_box) => oriented_box.min(),
        }
    }

    fn max(&self) -> Vector3 {
        match self {
            Collider::Point(point) => point.max(),
            Collider::Sphere(sphere) => sphere.max(),
            Collider::AlignedBox(aligned_box) => aligned_box.max(),
            Collider::OrientedBox(oriented_box) => oriented_box.max(),
        }
    }
}
