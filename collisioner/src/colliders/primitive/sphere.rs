use crate::colliders::common::{Bounded, Collides, Projectable, Rotation};
use crate::colliders::primitive::{AlignedBoxCollider, OrientedBoxCollider, PointCollider};
use crate::math::{Quaternion, Vector3};

/// # Sphere Collider
/// Primitive for representing sphere.
///
/// ## Example
/// ```
/// use collisioner::colliders::primitive::SphereCollider;
/// use collisioner::colliders::common::Collides;
/// use collisioner::math::Vector3;
///
/// let sphere1 = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 1.0);
/// let sphere2 = SphereCollider::new(Vector3::new(1.0, 1.0, 1.0), 1.0);
///
/// assert!(sphere1.collides_with(&sphere2));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SphereCollider {
    center: Vector3,
    radius: f64,
}

impl SphereCollider {
    pub fn new(center: Vector3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> Vector3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Bounded for SphereCollider {
    fn min(&self) -> Vector3 {
        self.center - Vector3::new(self.radius, self.radius, self.radius)
    }

    fn max(&self) -> Vector3 {
        self.center + Vector3::new(self.radius, self.radius, self.radius)
    }
}

impl Projectable for SphereCollider {
    fn project(&self, axis: Vector3) -> (f64, f64) {
        let projection = self.center().dot(axis);
        (projection - self.radius(), projection + self.radius())
    }
}

impl Rotation for SphereCollider {
    fn rotate(&self, _: Quaternion) -> Self {
        self.clone()
    }

    fn rotate_around(&self, rotation: Quaternion, pivot: Vector3) -> Self {
        Self::new(self.center.rotate_around(rotation, pivot), self.radius)
    }
}

impl Collides<Self> for SphereCollider {
    fn collides_with(&self, other: &Self) -> bool {
        let distance = (self.center - other.center).len();
        distance <= self.radius() + other.radius()
    }
}

impl Collides<PointCollider> for SphereCollider {
    fn collides_with(&self, other: &PointCollider) -> bool {
        let distance = (self.center - other.position()).len();
        distance <= self.radius()
    }
}

impl Collides<AlignedBoxCollider> for SphereCollider {
    fn collides_with(&self, other: &AlignedBoxCollider) -> bool {
        other.collides_with(self)
    }
}

impl Collides<OrientedBoxCollider> for SphereCollider {
    fn collides_with(&self, other: &OrientedBoxCollider) -> bool {
        other.collides_with(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::asserts::*;

    #[test]
    fn bounds_regular() {
        let sphere = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 1.0);

        assert_vector(Vector3::new(-1.0, -1.0, -1.0), sphere.min());
        assert_vector(Vector3::new(1.0, 1.0, 1.0), sphere.max());
    }

    #[test]
    fn projection() {
        let sphere = SphereCollider::new(Vector3::new(1.0, 2.0, 3.0), 3.0);

        assert_eq!((-2.0, 4.0), sphere.project(Vector3::new(1.0, 0.0, 0.0)));
        assert_eq!((-1.0, 5.0), sphere.project(Vector3::new(0.0, 1.0, 0.0)));
        assert_eq!((-0.0, 6.0), sphere.project(Vector3::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn rotation() {
        let sphere = SphereCollider::new(Vector3::new(1.0, 2.0, 3.0), 3.0);
        let rotation = Quaternion::from_euler(Vector3::new(
            10.0_f64.to_radians(),
            20.0_f64.to_radians(),
            30.0_f64.to_radians(),
        ));
        let rotated = sphere.rotate(rotation);

        assert_eq!(sphere.center(), rotated.center());
    }

    #[test]
    fn pivot_rotation() {
        let sphere = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 3.0);
        let pivot = Vector3::new(1.0, 1.0, 1.0);
        let rotation = Quaternion::from_euler(Vector3::new(
            0.0_f64.to_radians(),
            0.0_f64.to_radians(),
            90.0_f64.to_radians(),
        ));
        let rotated = sphere.rotate_around(rotation, pivot);

        assert_vector(Vector3::new(2.0, 0.0, 0.0), rotated.center());
    }

    #[test]
    fn point_inside_collide() {
        let sphere = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 1.0);
        let point = PointCollider::new(Vector3::new(0.0, 0.0, 0.0));

        assert!(sphere.collides_with(&point));
        assert!(point.collides_with(&sphere));
    }

    #[test]
    fn point_outside_dont_collide() {
        let sphere = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 1.0);
        let point = PointCollider::new(Vector3::new(2.0, 0.0, 0.0));

        assert!(!sphere.collides_with(&point));
        assert!(!point.collides_with(&sphere));
    }

    #[test]
    fn sphere_inside_collide() {
        let sphere = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 2.0);
        let other = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 1.0);

        assert!(sphere.collides_with(&other));
        assert!(other.collides_with(&sphere));
    }

    #[test]
    fn sphere_partially_collide() {
        let sphere = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 1.0);
        let other = SphereCollider::new(Vector3::new(1.0, 0.0, 0.0), 1.0);

        assert!(sphere.collides_with(&other));
        assert!(other.collides_with(&sphere));
    }

    #[test]
    fn sphere_touch_collide() {
        let sphere = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 1.0);
        let other = SphereCollider::new(Vector3::new(2.0, 0.0, 0.0), 1.0);

        assert!(sphere.collides_with(&other));
        assert!(other.collides_with(&sphere));
    }

    #[test]
    fn sphere_outside_dont_collide() {
        let sphere = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 1.0);
        let other = SphereCollider::new(Vector3::new(3.0, 0.0, 0.0), 1.0);

        assert!(!sphere.collides_with(&other));
        assert!(!other.collides_with(&sphere));
    }
}
