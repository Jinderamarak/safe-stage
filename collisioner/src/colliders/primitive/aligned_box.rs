use crate::colliders::common::{Bounded, Collides, Projectable, Rotation};
use crate::colliders::primitive::{OrientedBoxCollider, PointCollider, SphereCollider};
use crate::math::{Quaternion, Vector3};
use itertools::Itertools;

/// # Axis Aligned Box Collider
/// Basic primitive for collision detection of boxes.
///
/// ## Example
/// ```
/// use collisioner::colliders::primitive::AlignedBoxCollider;
/// use collisioner::colliders::common::Collides;
/// use collisioner::math::Vector3;
///
/// let box1 = AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
/// let box2 = AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 2.0, 2.0));
/// assert!(box1.collides_with(&box2));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct AlignedBoxCollider {
    center: Vector3,
    size: Vector3,
}

impl AlignedBoxCollider {
    pub fn new(position: Vector3, size: Vector3) -> Self {
    pub fn new(center: Vector3, size: Vector3) -> Self {
        Self {
            center,
            size: Vector3::new(size.x().abs(), size.y().abs(), size.z().abs()),
        }
    }

    pub fn center(&self) -> Vector3 {
        self.center
    }

    pub fn size(&self) -> Vector3 {
        self.size
    }

    fn corners(&self) -> [Vector3; 8] {
        let half_size = self.size() / 2.0;
        let negative_pos = self.center - half_size;
        let positive_pos = self.center + half_size;

        [
            Vector3::new(negative_pos.x(), negative_pos.y(), negative_pos.z()),
            Vector3::new(positive_pos.x(), negative_pos.y(), negative_pos.z()),
            Vector3::new(negative_pos.x(), positive_pos.y(), negative_pos.z()),
            Vector3::new(positive_pos.x(), positive_pos.y(), negative_pos.z()),
            Vector3::new(negative_pos.x(), negative_pos.y(), positive_pos.z()),
            Vector3::new(positive_pos.x(), negative_pos.y(), positive_pos.z()),
            Vector3::new(negative_pos.x(), positive_pos.y(), positive_pos.z()),
            Vector3::new(positive_pos.x(), positive_pos.y(), positive_pos.z()),
        ]
    }
}

impl Bounded for AlignedBoxCollider {
    fn min(&self) -> Vector3 {
        self.center - self.size / 2.0
    }

    fn max(&self) -> Vector3 {
        self.center + self.size / 2.0
    }
}

impl Projectable for AlignedBoxCollider {
    fn project(&self, axis: Vector3) -> (f64, f64) {
        self.corners()
            .iter()
            .map(|corner| axis.dot(*corner))
            .minmax()
            .into_option()
            .unwrap()
    }
}

impl Rotation<OrientedBoxCollider> for AlignedBoxCollider {
    fn rotate(&self, rotation: Quaternion) -> OrientedBoxCollider {
        OrientedBoxCollider::from(self).rotate(rotation)
    }

    fn rotate_around(&self, rotation: Quaternion, point: Vector3) -> OrientedBoxCollider {
        OrientedBoxCollider::from(self).rotate_around(rotation, point)
    }
}

impl Collides<Self> for AlignedBoxCollider {
    fn collides_with(&self, other: &Self) -> bool {
        let self_min = self.min();
        let self_max = self.max();
        let other_min = other.min();
        let other_max = other.max();

        self_min.x() <= other_max.x()
            && self_max.x() >= other_min.x()
            && self_min.y() <= other_max.y()
            && self_max.y() >= other_min.y()
            && self_min.z() <= other_max.z()
            && self_max.z() >= other_min.z()
    }
}

impl Collides<PointCollider> for AlignedBoxCollider {
    fn collides_with(&self, point: &PointCollider) -> bool {
        let self_min = self.min();
        let self_max = self.max();
        let position = point.position();

        self_min.x() <= position.x()
            && self_max.x() >= position.x()
            && self_min.y() <= position.y()
            && self_max.y() >= position.y()
            && self_min.z() <= position.z()
            && self_max.z() >= position.z()
    }
}

impl Collides<SphereCollider> for AlignedBoxCollider {
    fn collides_with(&self, other: &SphereCollider) -> bool {
        let min = self.min();
        let max = self.max();
        let center = other.center();

        let distance_squared = (min.x().max(center.x()).min(max.x()) - center.x()).powi(2)
            + (min.y().max(center.y()).min(max.y()) - center.y()).powi(2)
            + (min.z().max(center.z()).min(max.z()) - center.z()).powi(2);

        distance_squared < other.radius().powi(2)
    }
}

impl Collides<OrientedBoxCollider> for AlignedBoxCollider {
    fn collides_with(&self, other: &OrientedBoxCollider) -> bool {
        other.collides_with(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::asserts::assert_vector;
    use pretty_assertions::assert_eq;

    #[test]
    fn bounds_regular() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 2.0, 4.0), Vector3::new(3.0, 2.0, 1.0));

        assert_vector(Vector3::new(-0.5, 1.0, 3.5), aabb.min());
        assert_vector(Vector3::new(2.5, 3.0, 4.5), aabb.max());
    }

    #[test]
    fn projection() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 2.0, 4.0), Vector3::new(3.0, 2.0, 1.0));

        assert_eq!((-0.5, 2.5), aabb.project(Vector3::new(1.0, 0.0, 0.0)));
        assert_eq!((1.0, 3.0), aabb.project(Vector3::new(0.0, 1.0, 0.0)));
        assert_eq!((3.5, 4.5), aabb.project(Vector3::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn rotation() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 2.0, 4.0));
        let rotation = Quaternion::from_euler(Vector3::new(0.0, 90.0_f64.to_radians(), 0.0));

        let rotated = aabb.rotate(rotation);

        assert_vector(Vector3::new(1.0, 1.0, 1.0), rotated.center());
        assert_vector(Vector3::new(-1.0, 0.0, 0.0), rotated.min());
        assert_vector(Vector3::new(3.0, 2.0, 2.0), rotated.max());
    }

    #[test]
    fn pivot_rotation() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(2.0, 2.0, 1.0), Vector3::new(2.0, 2.0, 4.0));
        let rotation = Quaternion::from_euler(Vector3::new(0.0, 90.0_f64.to_radians(), 0.0));

        let rotated = aabb.rotate_around(rotation, Vector3::new(1.0, 1.0, 1.0));

        assert_vector(Vector3::new(1.0, 2.0, 0.0), rotated.center());
        assert_vector(Vector3::new(-1.0, 1.0, -1.0), rotated.min());
        assert_vector(Vector3::new(3.0, 3.0, 1.0), rotated.max());
    }

    #[test]
    fn aabb_corner_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let other_aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(aabb.collides_with(&other_aabb));
    }

    #[test]
    fn aabb_edge_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let other_aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(aabb.collides_with(&other_aabb));
    }

    #[test]
    fn aabb_face_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let other_aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(aabb.collides_with(&other_aabb));
    }

    #[test]
    fn aabb_overlap_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let other_aabb =
            AlignedBoxCollider::new(Vector3::new(0.5, 0.5, 0.5), Vector3::new(1.0, 1.0, 1.0));

        assert!(aabb.collides_with(&other_aabb));
    }

    #[test]
    fn aabb_inside_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let other_aabb =
            AlignedBoxCollider::new(Vector3::new(0.5, 0.5, 0.5), Vector3::new(1.0, 1.0, 1.0));

        assert!(aabb.collides_with(&other_aabb));
    }

    #[test]
    fn aabb_outside_dont_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let other_aabb =
            AlignedBoxCollider::new(Vector3::new(2.0, 2.0, 2.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(!aabb.collides_with(&other_aabb));
    }

    #[test]
    fn aabb_close_dont_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let other_aabb =
            AlignedBoxCollider::new(Vector3::new(2.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(!aabb.collides_with(&other_aabb));
    }

    #[test]
    fn point_corner_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(1.0, 1.0, 1.0));

        assert!(aabb.collides_with(&point));
    }

    #[test]
    fn point_edge_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(1.0, 1.0, 0.0));

        assert!(aabb.collides_with(&point));
    }

    #[test]
    fn point_face_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(1.0, 0.0, 0.0));

        assert!(aabb.collides_with(&point));
    }

    #[test]
    fn point_inside_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(0.0, 0.0, 0.0));

        assert!(aabb.collides_with(&point));
    }

    #[test]
    fn point_outside_dont_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let point = PointCollider::new(Vector3::new(2.0, 2.0, 2.0));

        assert!(!aabb.collides_with(&point));
    }

    #[test]
    fn point_close_dont_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(3.0, 1.0, 1.0));

        assert!(!aabb.collides_with(&point));
    }

    #[test]
    fn sphere_corner_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(2.0, 2.0, 2.0), 1.733);

        assert!(aabb.collides_with(&sphere));
        assert!(sphere.collides_with(&aabb));
    }

    #[test]
    fn sphere_corner_dont_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(2.0, 2.0, 2.0), 1.73);

        assert!(!aabb.collides_with(&sphere));
        assert!(!sphere.collides_with(&aabb));
    }

    #[test]
    fn sphere_edge_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(1.70, 1.70, 0.0), 1.0);

        assert!(aabb.collides_with(&sphere));
        assert!(sphere.collides_with(&aabb));
    }

    #[test]
    fn sphere_edge_dont_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(1.71, 1.71, 0.0), 1.0);

        assert!(!aabb.collides_with(&sphere));
        assert!(!sphere.collides_with(&aabb));
    }

    #[test]
    fn sphere_face_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(1.0, 0.0, 0.0), 1.0);

        assert!(aabb.collides_with(&sphere));
        assert!(sphere.collides_with(&aabb));
    }

    #[test]
    fn sphere_inside_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 1.0);

        assert!(aabb.collides_with(&sphere));
        assert!(sphere.collides_with(&aabb));
    }
}
