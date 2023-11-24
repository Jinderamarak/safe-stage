use crate::math::Vector3;

/// Objects bounding volume
pub trait Bounded {
    fn min(&self) -> Vector3;
    fn max(&self) -> Vector3;
}
