use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;

/// # Equipment
pub trait Equipment: Send + Sync {
    /// Get the full representation of the equipment.
    fn collider(&self) -> ColliderGroup<PrimaryCollider>;
}
