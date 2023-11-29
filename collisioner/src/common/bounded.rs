use crate::math::Vector3;

/// # Object bounding volume
/// A bounding box encasing a collider.
pub trait Bounded {
    /// Returns the minimum point of the bounding box.
    fn min(&self) -> Vector3;

    /// Returns the maximum point of the bounding box.
    fn max(&self) -> Vector3;
}
