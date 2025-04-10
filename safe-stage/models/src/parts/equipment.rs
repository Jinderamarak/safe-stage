use crate::immovable::Immovable;

/// # Equipment
pub trait Equipment: Send + Sync {
    /// Get the full representation of the equipment.
    fn collider(&self) -> Immovable;
}
