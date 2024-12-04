use crate::common::timing::timed;
use crate::lazy::pathing::linear_par::LinearParallelStrategy;
use crate::path::PathResult;
use crate::resolver::stage::StagePathResolver;
use crate::resolver::{PathResolver, StateUpdateError};
use crate::strategy::PathStrategy;
use collisions::common::Collides;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;

pub struct StageLinearResolver {
    strategy: LinearParallelStrategy<SixAxis>,
}

impl StageLinearResolver {
    pub fn new(step_size: SixAxis) -> Self {
        Self {
            strategy: LinearParallelStrategy::new(step_size),
        }
    }
}

impl StagePathResolver for StageLinearResolver {}

impl<M, I> PathResolver<SixAxis, M, I> for StageLinearResolver
where
    M: Movable<SixAxis> + Sync,
    I: Collides<ColliderGroup<PrimaryCollider>> + Sync + Send,
{
    fn update_state(
        &mut self,
        new: &SixAxis,
        movable: &M,
        immovable: &I,
    ) -> Result<(), StateUpdateError> {
        if immovable.collides_with(&movable.move_to(new)) {
            return Err(StateUpdateError::InvalidState);
        }

        Ok(())
    }

    fn resolve_path(
        &self,
        from: &SixAxis,
        to: &SixAxis,
        movable: &M,
        immovable: &I,
    ) -> PathResult<SixAxis> {
        let (path, time_to_path) =
            timed!({ self.strategy.find_path(from, to, movable, immovable) });
        log::info!("Linear path in {} ms", time_to_path.as_millis());
        path
    }
}
