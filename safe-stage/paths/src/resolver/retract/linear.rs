use crate::common::timing::timed;
use crate::deferred::pathing::linear_par::LinearParallelStrategy;
use crate::path::PathResult;
use crate::resolver::retract::RetractPathResolver;
use crate::resolver::{PathResolver, StateUpdateError};
use crate::strategy::PathStrategy;
use collisions::common::Collides;
use models::immovable::Immovable;
use models::movable::Movable;
use models::position::linear::LinearState;

/// # Retract Linear Resolver
/// Path resolver intended for a retractable device.
/// Resolves the path using linear interpolation with fixed step.
///
/// **Runs in parallel using Rayon.**
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

impl PathResolver<LinearState> for RetractLinearResolver {
    fn update_state(
        &mut self,
        new: &LinearState,
        movable: &dyn Movable<LinearState>,
        immovable: &Immovable,
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
        movable: &dyn Movable<LinearState>,
        immovable: &Immovable,
    ) -> PathResult<LinearState> {
        let (path, time_to_path) =
            timed!({ self.strategy.find_path(from, to, movable, immovable) });
        log::info!("Linear path in {} ms", time_to_path.as_millis());
        path
    }
}
