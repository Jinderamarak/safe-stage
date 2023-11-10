use crate::colliders::{
    Bounded, Collides, OrientedBoxCollider, PointCollider, Projectable, SphereCollider,
};
use crate::common::Vector3;
use itertools::Itertools;

/// # Axis Aligned Box Collider
/// Basic primitive for collision detection of boxes.
///
/// ## Example
/// ```
/// use collisioner::colliders::AlignedBoxCollider;
/// use collisioner::colliders::Collides;
/// use collisioner::common::Vector3;
///
/// let box1 = AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
/// let box2 = AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 2.0, 2.0));
/// assert!(box1.collides_with(&box2));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct AlignedBoxCollider {
    position: Vector3,
    size: Vector3,
}

impl AlignedBoxCollider {
    pub fn new(position: Vector3, size: Vector3) -> Self {
        Self {
            position,
            size: Vector3::new(size.x().abs(), size.y().abs(), size.z().abs()),
        }
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn size(&self) -> Vector3 {
        self.size
    }

    fn corners(&self) -> [Vector3; 8] {
        let half_size = self.size() / 2.0;
        let negative_pos = self.position - half_size;
        let positive_pos = self.position + half_size;

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
        self.position - self.size / 2.0
    }

    fn max(&self) -> Vector3 {
        self.position + self.size / 2.0
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
        let center = other.position();

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
    use crate::tests::asserts::assert_vector;

    #[test]
    fn box_bounding_regular() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(1.0, 2.0, 4.0), Vector3::new(3.0, 2.0, 1.0));

        assert_vector(Vector3::new(-0.5, 1.0, 3.5), box1.min());
        assert_vector(Vector3::new(2.5, 3.0, 4.5), box1.max());
    }

    #[test]
    fn boxes_corner_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let box2 =
            AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(box1.collides_with(&box2));
    }

    #[test]
    fn boxes_edge_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let box2 =
            AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(box1.collides_with(&box2));
    }

    #[test]
    fn boxes_face_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let box2 =
            AlignedBoxCollider::new(Vector3::new(1.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(box1.collides_with(&box2));
    }

    #[test]
    fn boxes_overlap_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let box2 =
            AlignedBoxCollider::new(Vector3::new(0.5, 0.5, 0.5), Vector3::new(1.0, 1.0, 1.0));

        assert!(box1.collides_with(&box2));
    }

    #[test]
    fn boxes_inside_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let box2 =
            AlignedBoxCollider::new(Vector3::new(0.5, 0.5, 0.5), Vector3::new(1.0, 1.0, 1.0));

        assert!(box1.collides_with(&box2));
    }

    #[test]
    fn boxes_outside_dont_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let box2 =
            AlignedBoxCollider::new(Vector3::new(2.0, 2.0, 2.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(!box1.collides_with(&box2));
    }

    #[test]
    fn boxes_close_dont_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let box2 =
            AlignedBoxCollider::new(Vector3::new(2.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(!box1.collides_with(&box2));
    }

    #[test]
    fn box_point_corner_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(1.0, 1.0, 1.0));

        assert!(box1.collides_with(&point));
    }

    #[test]
    fn box_point_edge_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(1.0, 1.0, 0.0));

        assert!(box1.collides_with(&point));
    }

    #[test]
    fn box_point_face_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(1.0, 0.0, 0.0));

        assert!(box1.collides_with(&point));
    }

    #[test]
    fn box_point_inside_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(0.0, 0.0, 0.0));

        assert!(box1.collides_with(&point));
    }

    #[test]
    fn box_point_outside_dont_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let point = PointCollider::new(Vector3::new(2.0, 2.0, 2.0));

        assert!(!box1.collides_with(&point));
    }

    #[test]
    fn box_point_close_dont_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(3.0, 1.0, 1.0));

        assert!(!box1.collides_with(&point));
    }

    #[test]
    fn aabb_sphere_corner_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(2.0, 2.0, 2.0), 1.733);

        assert!(aabb.collides_with(&sphere));
        assert!(sphere.collides_with(&aabb));
    }

    #[test]
    fn aabb_sphere_corner_dont_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(2.0, 2.0, 2.0), 1.73);

        assert!(!aabb.collides_with(&sphere));
        assert!(!sphere.collides_with(&aabb));
    }

    #[test]
    fn aabb_sphere_edge_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(1.70, 1.70, 0.0), 1.0);

        assert!(aabb.collides_with(&sphere));
        assert!(sphere.collides_with(&aabb));
    }

    #[test]
    fn aabb_sphere_edge_dont_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(1.71, 1.71, 0.0), 1.0);

        assert!(!aabb.collides_with(&sphere));
        assert!(!sphere.collides_with(&aabb));
    }

    #[test]
    fn aabb_sphere_face_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(1.0, 0.0, 0.0), 1.0);

        assert!(aabb.collides_with(&sphere));
        assert!(sphere.collides_with(&aabb));
    }

    #[test]
    fn aabb_sphere_inside_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 1.0);

        assert!(aabb.collides_with(&sphere));
        assert!(sphere.collides_with(&aabb));
    }
}
