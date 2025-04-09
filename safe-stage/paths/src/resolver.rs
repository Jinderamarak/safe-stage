pub mod retract;
pub mod stage;

use crate::path::PathResult;
use models::collider::ModelCollider;
use models::movable::Movable;
use thiserror::Error;

/// # State Update Error
/// Error of a resolver state update.
#[derive(Error, Debug)]
pub enum StateUpdateError {
    #[error("Invalid state")]
    InvalidState,
}

/// # Path Resolver
/// Resolves a path between two points and holds state to do it faster.
pub trait PathResolver<P> {
    fn update_state(
        &mut self,
        new: &P,
        movable: &dyn Movable<P>,
        immovable: &dyn ModelCollider,
    ) -> Result<(), StateUpdateError>;

    fn resolve_path(
        &self,
        from: &P,
        to: &P,
        movable: &dyn Movable<P>,
        immovable: &dyn ModelCollider,
    ) -> PathResult<P>;
}
