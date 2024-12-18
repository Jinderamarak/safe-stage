use maths::{Quaternion, Vector3};

/// # Object transformation
/// Applies both translation and rotation to an object in one step.
pub trait Transformation<T = Self> {
    /// Returns a new object rotated and then translated.
    fn transform(&self, rotation: &Quaternion, pivot: &Vector3, translation: &Vector3) -> T;
}
