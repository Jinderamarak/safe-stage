use crate::colliders::{Bounded, Collides, PointCollider};
use crate::common::{Quaternion, Vector3};

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
/// let corners = obb.corners();
///
/// for corner in &corners {
///    assert_float_absolute_eq!(corner.x().abs(), 1.0);
/// }
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

    pub fn corners(&self) -> [Vector3; 8] {
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

    pub fn rotate_around(&self, pivot: Vector3, rotation: Quaternion) -> Self {
        let new_position = (self.position - pivot).rotate(rotation) + pivot;
        let new_rotation = self.rotation * rotation;

        Self::new(new_position, self.size, new_rotation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;
    use pretty_assertions::assert_eq;

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
}
