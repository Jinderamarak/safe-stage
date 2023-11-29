use crate::math::{Quaternion, Vector3};

/// # Object rotation
/// A rotation of an object.
pub trait Rotation<T = Self> {
    /// Returns object rotated by given rotation around the objects center.
    fn rotate(&self, rotation: Quaternion) -> T;

    /// Returns object rotated by given rotation around the given pivot.
    fn rotate_around(&self, rotation: Quaternion, pivot: Vector3) -> T;
}
