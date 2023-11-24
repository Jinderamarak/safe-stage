use crate::math::{Quaternion, Vector3};

/// Rotation of specific object
pub trait Rotation<T = Self> {
    fn rotate(&self, rotation: Quaternion) -> T;
    fn rotate_around(&self, rotation: Quaternion, pivot: Vector3) -> T;
}
