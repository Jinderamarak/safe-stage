use crate::immovable::Immovable;
use collisions::PrimaryCollider;

/// # Stage Holder
pub trait Holder: Send + Sync {
    /// Clone the holder into box.
    fn cloned(&self) -> Box<dyn Holder>;
    /// Get the full representation of the holder.
    fn collider(&self) -> Immovable;
    /// Swap the attached sample with the given one.
    fn swap_sample(&mut self, sample: Option<PrimaryCollider>);
}
