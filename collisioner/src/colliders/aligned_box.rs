use crate::colliders::{Bounded, Collides, PointCollider};
use crate::common::Vector3;

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
        assert!(
            size.x() >= 0.0,
            "Size cannot be negative, x was '{}'",
            size.x()
        );
        assert!(
            size.y() >= 0.0,
            "Size cannot be negative, y was '{}'",
            size.y()
        );
        assert!(
            size.z() >= 0.0,
            "Size cannot be negative, z was '{}'",
            size.z()
        );

        Self { position, size }
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn size(&self) -> Vector3 {
        self.size
    }
}

impl Bounded for AlignedBoxCollider {
    fn min(&self) -> Vector3 {
        self.position()
    }

    fn max(&self) -> Vector3 {
        self.position() + self.size()
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[should_panic]
    fn invalid_size_x() {
        let _ = AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(-1.0, 0.0, 0.0));
    }

    #[test]
    #[should_panic]
    fn invalid_size_y() {
        let _ = AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(0.0, -1.0, 0.0));
    }

    #[test]
    #[should_panic]
    fn invalid_size_z() {
        let _ = AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn box_bounding_regular() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(1.0, 2.0, 4.0), Vector3::new(3.0, 2.0, 1.0));

        assert_eq!(Vector3::new(1.0, 2.0, 4.0), box1.min());
        assert_eq!(Vector3::new(4.0, 4.0, 5.0), box1.max());
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
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let point = PointCollider::new(Vector3::new(1.0, 1.0, 1.0));

        assert!(box1.collides_with(&point));
    }

    #[test]
    fn box_point_edge_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 2.0));
        let point = PointCollider::new(Vector3::new(1.0, 1.0, 1.0));

        assert!(box1.collides_with(&point));
    }

    #[test]
    fn box_point_face_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(2.0, 1.0, 1.0));

        assert!(box1.collides_with(&point));
    }

    #[test]
    fn box_point_inside_collide() {
        let box1 =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(1.0, 1.0, 1.0));

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
}
