use crate::path::PathResult;
use collisions::common::Collides;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;
use models::movable::Movable;

/// # Path strategy
/// A path strategy that can find a path from one point to another.
pub trait PathStrategy<P> {
    /// Try finding a path from one point to another.
    /// Movable `M`  produces position `C` which immovable of type `I` must implement collision with.
    fn find_path<M, I>(&self, from: &P, to: &P, movable: &M, immovable: &I) -> PathResult<P>
    where
        M: Movable<P> + Sync,
        I: Collides<ColliderGroup<PrimaryCollider>> + Sync + Send;
}
