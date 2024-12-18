use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;

/// # Stage Holder
pub trait Holder: Send + Sync {
    /// Clone the holder into box.
    fn cloned(&self) -> Box<dyn Holder>;
    /// Get the full representation of the holder.
    fn collider(&self) -> ColliderGroup<PrimaryCollider>;
    /// Swap the attached sample with the given one.
    fn swap_sample(&mut self, sample: Option<PrimaryCollider>);
}
