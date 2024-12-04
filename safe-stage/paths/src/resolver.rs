pub mod retract;
pub mod stage;

use crate::path::PathResult;
use collisions::common::Collides;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;
use models::movable::Movable;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StateUpdateError {
    #[error("Invalid state")]
    InvalidState,
}

pub trait PathResolver<P, M, I>
where
    M: Movable<P> + Sync,
    I: Collides<ColliderGroup<PrimaryCollider>> + Sync + Send,
{
    fn update_state(&mut self, new: &P, movable: &M, immovable: &I)
        -> Result<(), StateUpdateError>;

    fn resolve_path(&self, from: &P, to: &P, movable: &M, immovable: &I) -> PathResult<P>;
}

pub struct DynamicMovable<P>(pub Arc<dyn Movable<P> + Send + Sync>);

impl<P> Movable<P> for DynamicMovable<P> {
    fn move_to(&self, position: &P) -> ColliderGroup<PrimaryCollider> {
        self.0.move_to(position)
    }
}

pub struct DynamicImmovable(pub Arc<dyn Collides<ColliderGroup<PrimaryCollider>> + Send + Sync>);

impl Collides<ColliderGroup<PrimaryCollider>> for DynamicImmovable {
    fn collides_with(&self, other: &ColliderGroup<PrimaryCollider>) -> bool {
        self.0.collides_with(other)
    }
}
