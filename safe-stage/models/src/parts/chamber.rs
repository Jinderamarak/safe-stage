use crate::immovable::Immovable;

/// # Chamber
pub trait Chamber: Send + Sync {
    /// Get the full representation of the chamber.
    fn full(&self) -> Immovable;
    /// Get the representation of the chamber with less obstructive parts.
    fn less_obstructive(&self) -> Immovable;
    /// Get the representation of the chamber only with non-obstructive parts.
    fn non_obstructive(&self) -> Immovable;
}
