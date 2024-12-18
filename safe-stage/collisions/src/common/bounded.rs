use maths::Vector3;

/// # Object bounding volume
/// A bounding box encasing a collider.
pub trait Bounded {
    /// Returns the minimum point of the bounding box.
    fn min(&self) -> Vector3;

    /// Returns the maximum point of the bounding box.
    fn max(&self) -> Vector3;

    /// Returns the center of the bounding box.
    fn center(&self) -> Vector3 {
        (self.min() + self.max()) / 2.0
    }
}
