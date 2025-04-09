use crate::movable::Movable;
use crate::parts::holder::Holder;
use crate::position::sixaxis::SixAxis;
use std::sync::Arc;

/// # Stage
pub trait Stage: Movable<SixAxis> {
    /// Get the Stage as [Arc] reference.
    fn as_movable(&self) -> Arc<dyn Movable<SixAxis>>;
    /// Swap the attached holder with the given one.
    fn swap_holder(&mut self, holder: Option<Box<dyn Holder>>);
    /// Get the active holder.
    fn active_holder(&self) -> Option<&dyn Holder>;
    /// Get the mutable reference to the active holder.
    fn active_holder_mut(&mut self) -> Option<&mut (dyn Holder + 'static)>;
}
