use crate::common::timing::timed;
use crate::deferred::pathing::linear_par::LinearParallelStrategy;
use crate::path::PathResult;
use crate::resolver::retract::RetractPathResolver;
use crate::resolver::{PathResolver, StateUpdateError};
use crate::strategy::PathStrategy;
use collisions::common::Collides;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;
use models::movable::Movable;
use models::position::linear::LinearState;

pub struct RetractLinearResolver {
    strategy: LinearParallelStrategy<LinearState>,
}

impl RetractLinearResolver {
    pub fn new(step_size: LinearState) -> Self {
        Self {
            strategy: LinearParallelStrategy::new(step_size),
        }
    }
}

impl RetractPathResolver for RetractLinearResolver {}

impl<M, I> PathResolver<LinearState, M, I> for RetractLinearResolver
where
    M: Movable<LinearState> + Sync,
    I: Collides<ColliderGroup<PrimaryCollider>> + Sync + Send,
{
    fn update_state(
        &mut self,
        new: &LinearState,
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
        from: &LinearState,
        to: &LinearState,
        movable: &M,
        immovable: &I,
    ) -> PathResult<LinearState> {
        let (path, time_to_path) =
            timed!({ self.strategy.find_path(from, to, movable, immovable) });
        log::info!("Linear path in {} ms", time_to_path.as_millis());
        path
    }
}
