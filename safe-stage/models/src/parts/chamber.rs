use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;

/// # Chamber
pub trait Chamber: Send + Sync {
    /// Get the full representation of the chamber.
    fn full(&self) -> ColliderGroup<PrimaryCollider>;
    /// Get the representation of the chamber with less obstructive parts.
    fn less_obstructive(&self) -> ColliderGroup<PrimaryCollider>;
    /// Get the representation of the chamber only with non-obstructive parts.
    fn non_obstructive(&self) -> ColliderGroup<PrimaryCollider>;
}
