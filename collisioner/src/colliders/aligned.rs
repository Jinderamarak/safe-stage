use crate::colliders::point::PointCollision;
use crate::common::vector::Vector3;

#[derive(Debug, Clone, PartialEq)]
pub struct AlignedBox {
    position: Vector3,
    size: Vector3,
}

impl AlignedBox {
    pub fn new(position: Vector3, size: Vector3) -> Self {
        Self { position, size }
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn size(&self) -> Vector3 {
        self.size
    }
}

pub trait AlignedBoxCollision {
    fn collides(&self, other: &AlignedBox) -> bool;
}

impl AlignedBoxCollision for AlignedBox {
    fn collides(&self, other: &AlignedBox) -> bool {
        let self_min = self.position();
        let self_max = self.position() + self.size();
        let other_min = other.position();
        let other_max = other.position() + other.size();

        self_min.x() <= other_max.x()
            && self_max.x() >= other_min.x()
            && self_min.y() <= other_max.y()
            && self_max.y() >= other_min.y()
            && self_min.z() <= other_max.z()
            && self_max.z() >= other_min.z()
    }
}

impl PointCollision for AlignedBox {
    fn collides(&self, point: &Vector3) -> bool {
        let self_min = self.position();
        let self_max = self.position() + self.size();

        self_min.x() <= point.x()
            && self_max.x() >= point.x()
            && self_min.y() <= point.y()
            && self_max.y() >= point.y()
            && self_min.z() <= point.z()
            && self_max.z() >= point.z()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boxes_corner_collide() {
        let box1 = AlignedBox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let box2 = AlignedBox::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(AlignedBoxCollision::collides(&box1, &box2));
    }

    #[test]
    fn test_boxes_edge_collide() {
        let box1 = AlignedBox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let box2 = AlignedBox::new(Vector3::new(1.0, 1.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(AlignedBoxCollision::collides(&box1, &box2));
    }

    #[test]
    fn test_boxes_face_collide() {
        let box1 = AlignedBox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let box2 = AlignedBox::new(Vector3::new(1.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(AlignedBoxCollision::collides(&box1, &box2));
    }

    #[test]
    fn test_boxes_overlap_collide() {
        let box1 = AlignedBox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let box2 = AlignedBox::new(Vector3::new(0.5, 0.5, 0.5), Vector3::new(1.0, 1.0, 1.0));

        assert!(AlignedBoxCollision::collides(&box1, &box2));
    }

    #[test]
    fn test_boxes_inside_collide() {
        let box1 = AlignedBox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let box2 = AlignedBox::new(Vector3::new(0.5, 0.5, 0.5), Vector3::new(1.0, 1.0, 1.0));

        assert!(AlignedBoxCollision::collides(&box1, &box2));
    }

    #[test]
    fn test_boxes_outside_dont_collide() {
        let box1 = AlignedBox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let box2 = AlignedBox::new(Vector3::new(2.0, 2.0, 2.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(!AlignedBoxCollision::collides(&box1, &box2));
    }

    #[test]
    fn test_boxes_close_dont_collide() {
        let box1 = AlignedBox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let box2 = AlignedBox::new(Vector3::new(2.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(!AlignedBoxCollision::collides(&box1, &box2));
    }

    #[test]
    fn test_box_point_corner_collide() {
        let box1 = AlignedBox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let point = Vector3::new(1.0, 1.0, 1.0);

        assert!(PointCollision::collides(&box1, &point));
    }

    #[test]
    fn test_box_point_edge_collide() {
        let box1 = AlignedBox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 2.0));
        let point = Vector3::new(1.0, 1.0, 1.0);

        assert!(PointCollision::collides(&box1, &point));
    }

    #[test]
    fn test_box_point_face_collide() {
        let box1 = AlignedBox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = Vector3::new(2.0, 1.0, 1.0);

        assert!(PointCollision::collides(&box1, &point));
    }

    #[test]
    fn test_box_point_inside_collide() {
        let box1 = AlignedBox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = Vector3::new(1.0, 1.0, 1.0);

        assert!(PointCollision::collides(&box1, &point));
    }

    #[test]
    fn test_box_point_outside_dont_collide() {
        let box1 = AlignedBox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let point = Vector3::new(2.0, 2.0, 2.0);

        assert!(!PointCollision::collides(&box1, &point));
    }

    #[test]
    fn test_box_point_close_dont_collide() {
        let box1 = AlignedBox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = Vector3::new(3.0, 1.0, 1.0);

        assert!(!PointCollision::collides(&box1, &point));
    }
}
