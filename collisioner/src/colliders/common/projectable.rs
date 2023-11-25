use crate::math::Vector3;

/// # Object projection
/// A projection of an object onto an axis.
pub trait Projectable {
    /// Returns the minimum and maximum points of the projection on a given axis.
    fn project(&self, axis: Vector3) -> (f64, f64);

    /// Returns true if the projection intersects with the other projection.
    fn intersects(&self, other: &impl Projectable, axis: Vector3) -> bool {
        let (self_min, self_max) = self.project(axis);
        let (other_min, other_max) = other.project(axis);

        self_max >= other_min && self_min <= other_max
    }
}
