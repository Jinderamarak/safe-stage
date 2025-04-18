use crate::movable::Movable;
use crate::position::linear::LinearState;
use std::sync::Arc;

/// # Retractable Device
pub trait Retract: Movable<LinearState> {
    /// Get the Retractable Device as [Arc] reference.
    fn as_movable(&self) -> Arc<dyn Movable<LinearState>>;
}
