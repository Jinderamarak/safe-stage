use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;

/// Entity can be moved to position `P` where it is represented as `C`.
pub trait Movable<P> {
    /// Get the representation of the entity at the given position.
    fn move_to(&self, position: &P) -> ColliderGroup<PrimaryCollider>;
}
