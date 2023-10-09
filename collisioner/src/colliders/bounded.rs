use crate::common::Vector3;

/// Defines objects bounding volume
pub trait Bounded {
    fn min(&self) -> Vector3;
    fn max(&self) -> Vector3;
}
