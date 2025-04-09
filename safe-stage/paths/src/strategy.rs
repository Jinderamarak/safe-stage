use crate::path::PathResult;
use models::collider::ModelCollider;
use models::movable::Movable;

/// # Path strategy
/// A path strategy that can find a path from one point to another.
pub trait PathStrategy<P> {
    /// Try finding a path from one point to another.
    fn find_path(
        &self,
        from: &P,
        to: &P,
        movable: &dyn Movable<P>,
        immovable: &dyn ModelCollider,
    ) -> PathResult<P>;
}
