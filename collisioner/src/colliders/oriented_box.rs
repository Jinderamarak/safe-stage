use crate::colliders::{AlignedBoxCollider, Collides};
use crate::common::{Quaternion, Vector3};
use itertools::Itertools;

/// # Oriented Bounding Box Collider
/// Advanced primitive for collision detection of boxes with rotation.
///
/// ## Example
/// ```
/// use assert_float_eq::*;
/// use collisioner::colliders::OrientedBoxCollider;
/// use collisioner::common::{Quaternion, Vector3};
///
/// let position = Vector3::new(0.0, 0.0, 0.0);
/// let size = Vector3::new(2.0, 2.0, 2.0);
/// let rotation = Vector3::new(0.0, 0.0, 90.0_f64.to_radians());
///
/// let obb = OrientedBoxCollider::new(position, size, Quaternion::from_euler(rotation));
///
/// assert_eq!(Vector3::new(0.0, 0.0, 0.0), obb.position());
/// assert_eq!(Vector3::new(2.0, 2.0, 2.0), obb.size());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct OrientedBoxCollider {
    position: Vector3,
    size: Vector3,
    rotation: Quaternion,
}

impl OrientedBoxCollider {
    pub fn new(position: Vector3, size: Vector3, rotation: Quaternion) -> Self {
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

        Self {
            position,
            size,
            rotation,
        }
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn size(&self) -> Vector3 {
        self.size
    }

    pub fn rotation(&self) -> Quaternion {
        self.rotation
    }

    pub fn rotate_around(&self, pivot: Vector3, rotation: Quaternion) -> Self {
        let new_position = (self.position - pivot).rotate(rotation) + pivot;
        let new_rotation = self.rotation * rotation;

        Self::new(new_position, self.size, new_rotation)
    }

    fn corners(&self) -> [Vector3; 8] {
        let half_size = (self.size() / 2.0).rotate(self.rotation);
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

    fn project(&self, axis: Vector3) -> (f64, f64) {
        self.corners()
            .iter()
            .map(|corner| axis.dot(*corner))
            .minmax()
            .into_option()
            .unwrap()
    }

    fn intersects(&self, other: &OrientedBoxCollider, axis: Vector3) -> bool {
        let (self_min, self_max) = self.project(axis);
        let (other_min, other_max) = other.project(axis);

        self_max >= other_min && self_min <= other_max
    }
}

impl From<AlignedBoxCollider> for OrientedBoxCollider {
    fn from(value: AlignedBoxCollider) -> Self {
        OrientedBoxCollider::new(
            value.position(),
            value.size(),
            Quaternion::new(0.0, 0.0, 0.0, 0.0),
        )
    }
}

impl Collides<OrientedBoxCollider> for OrientedBoxCollider {
    fn collides_with(&self, other: &OrientedBoxCollider) -> bool {
        let self_ax = Vector3::new(1.0, 0.0, 0.0).rotate(self.rotation);
        let self_ay = Vector3::new(0.0, 1.0, 0.0).rotate(self.rotation);
        let self_az = Vector3::new(0.0, 0.0, 1.0).rotate(self.rotation);

        let other_ax = Vector3::new(1.0, 0.0, 0.0).rotate(other.rotation);
        let other_ay = Vector3::new(0.0, 1.0, 0.0).rotate(other.rotation);
        let other_az = Vector3::new(0.0, 0.0, 1.0).rotate(other.rotation);

        let axes = [
            self_ax,
            self_ay,
            self_az,
            other_ax,
            other_ay,
            other_az,
            self_ax.cross(other_ax),
            self_ax.cross(other_ay),
            self_ax.cross(other_az),
            self_ay.cross(other_ax),
            self_ay.cross(other_ay),
            self_ay.cross(other_az),
            self_az.cross(other_ax),
            self_az.cross(other_ay),
            self_az.cross(other_az),
        ];

        axes.iter().all(|axis| self.intersects(other, *axis))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::asserts::*;

    #[test]
    #[should_panic]
    fn invalid_size_x() {
        let _ = OrientedBoxCollider::new(
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(-1.0, 0.0, 0.0),
            Quaternion::new(0.0, 0.0, 0.0, 0.0),
        );
    }

    #[test]
    #[should_panic]
    fn invalid_size_y() {
        let _ = OrientedBoxCollider::new(
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(0.0, -1.0, 0.0),
            Quaternion::new(0.0, 0.0, 0.0, 0.0),
        );
    }

    #[test]
    #[should_panic]
    fn invalid_size_z() {
        let _ = OrientedBoxCollider::new(
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(0.0, 0.0, -1.0),
            Quaternion::new(0.0, 0.0, 0.0, 0.0),
        );
    }

    #[test]
    fn rotate_around_pivot() {
        let collider = OrientedBoxCollider::new(
            Vector3::new(5.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::from_euler(Vector3::new(
                0.0_f64.to_radians(),
                0.0_f64.to_radians(),
                45.0_f64.to_radians(),
            )),
        );

        let rotated = collider.rotate_around(
            Vector3::new(0.0, 0.0, 0.0),
            Quaternion::from_euler(Vector3::new(
                0.0_f64.to_radians(),
                0.0_f64.to_radians(),
                90.0_f64.to_radians(),
            )),
        );

        let position = Vector3::new(0.0, 5.0, 0.0);
        let rotation = Quaternion::from_euler(Vector3::new(
            0.0_f64.to_radians(),
            0.0_f64.to_radians(),
            135.0_f64.to_radians(),
        ));

        assert_vector(position, rotated.position());
        assert_quaternion(rotation, rotated.rotation());
    }

    #[test]
    fn corners() {
        let collider = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(2.0, 4.0, 6.0),
            Quaternion::from_euler(Vector3::new(
                0.0_f64.to_radians(),
                0.0_f64.to_radians(),
                90.0_f64.to_radians(),
            )),
        );

        let actual_corners = collider.corners();
        let expected_corners = [
            Vector3::new(-2.0, 1.0, 3.0),
            Vector3::new(2.0, 1.0, 3.0),
            Vector3::new(2.0, -1.0, 3.0),
            Vector3::new(-2.0, -1.0, 3.0),
            Vector3::new(-2.0, 1.0, -3.0),
            Vector3::new(2.0, 1.0, -3.0),
            Vector3::new(2.0, -1.0, -3.0),
            Vector3::new(-2.0, -1.0, -3.0),
        ];

        for expected in &expected_corners {
            let mut found = false;
            for actual in &actual_corners {
                if (expected.x() - actual.x()).abs() <= f64::EPSILON * 2.0
                    && (expected.y() - actual.y()).abs() <= f64::EPSILON * 2.0
                    && (expected.z() - actual.z()).abs() <= f64::EPSILON * 2.0
                {
                    found = true;
                    break;
                }
            }

            assert!(found, "Missing corner {:?}", expected);
        }
    }

    #[test]
    fn boxes_corner_corner_collide() {
        let box1 = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::new(0.0, 0.0, 0.0, 0.0),
        );
        let box2 = OrientedBoxCollider::new(
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::new(0.0, 0.0, 0.0, 0.0),
        );

        assert!(box1.collides_with(&box2));
        assert!(box2.collides_with(&box1));
    }

    #[test]
    fn boxes_edge_edge_collide() {
        let box1 = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::new(0.0, 0.0, 0.0, 0.0),
        );
        let box2 = OrientedBoxCollider::new(
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::new(0.0, 0.0, 0.0, 0.0),
        );

        assert!(box1.collides_with(&box2));
        assert!(box2.collides_with(&box1));
    }

    #[test]
    fn boxes_face_face_collide() {
        let box1 = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::new(0.0, 0.0, 0.0, 0.0),
        );
        let box2 = OrientedBoxCollider::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::new(0.0, 0.0, 0.0, 0.0),
        );

        assert!(box1.collides_with(&box2));
        assert!(box2.collides_with(&box1));
    }

    #[test]
    fn boxes_face_edge_collide() {
        let box1 = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::from_euler(Vector3::new(
                0.0_f64.to_radians(),
                0.0_f64.to_radians(),
                45.0_f64.to_radians(),
            )),
        );
        let box2 = OrientedBoxCollider::new(
            Vector3::new(0.7, 0.7, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::new(0.0, 0.0, 0.0, 0.0),
        );

        assert!(box1.collides_with(&box2));
        assert!(box2.collides_with(&box1));
    }

    #[test]
    fn boxes_face_edge_close_dont_collide() {
        let box1 = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::from_euler(Vector3::new(
                0.0_f64.to_radians(),
                0.0_f64.to_radians(),
                45.0_f64.to_radians(),
            )),
        );
        let box2 = OrientedBoxCollider::new(
            Vector3::new(0.71, 0.71, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::new(0.0, 0.0, 0.0, 0.0),
        );

        assert!(!box1.collides_with(&box2));
    }
}
