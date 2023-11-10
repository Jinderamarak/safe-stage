use crate::colliders::{
    AlignedBoxCollider, Bounded, Collides, OrientedBoxCollider, PointCollider, Projectable,
};
use crate::common::Vector3;

#[derive(Debug, Clone, PartialEq)]
pub struct SphereCollider {
    position: Vector3,
    radius: f64,
}

impl SphereCollider {
    pub fn new(position: Vector3, radius: f64) -> Self {
        Self { position, radius }
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Bounded for SphereCollider {
    fn min(&self) -> Vector3 {
        self.position - Vector3::new(self.radius, self.radius, self.radius)
    }

    fn max(&self) -> Vector3 {
        self.position + Vector3::new(self.radius, self.radius, self.radius)
    }
}

impl Projectable for SphereCollider {
    fn project(&self, axis: Vector3) -> (f64, f64) {
        let projection = self.position().dot(axis);
        (projection - self.radius(), projection + self.radius())
    }
}

impl Collides<Self> for SphereCollider {
    fn collides_with(&self, other: &Self) -> bool {
        let distance = (self.position - other.position).len();
        distance <= self.radius() + other.radius()
    }
}

impl Collides<PointCollider> for SphereCollider {
    fn collides_with(&self, other: &PointCollider) -> bool {
        let distance = (self.position - other.position()).len();
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
    use crate::tests::asserts::*;

    #[test]
    fn bounding_regular() {
        let sphere = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 1.0);

        assert_vector(Vector3::new(-1.0, -1.0, -1.0), sphere.min());
        assert_vector(Vector3::new(1.0, 1.0, 1.0), sphere.max());
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
