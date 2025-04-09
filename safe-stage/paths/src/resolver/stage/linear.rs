use crate::common::timing::timed;
use crate::deferred::pathing::linear_par::LinearParallelStrategy;
use crate::path::PathResult;
use crate::resolver::stage::StagePathResolver;
use crate::resolver::{PathResolver, StateUpdateError};
use crate::strategy::PathStrategy;
use models::collider::ModelCollider;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;

/// # Stage Linear Resolver
/// Path resolver intended for a stage.
/// Resolves the path using linear interpolation with fixed step.
///
/// **Runs in parallel using Rayon.**
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

impl PathResolver<SixAxis> for StageLinearResolver {
    fn update_state(
        &mut self,
        new: &SixAxis,
        movable: &dyn Movable<SixAxis>,
        immovable: &dyn ModelCollider,
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
        movable: &dyn Movable<SixAxis>,
        immovable: &dyn ModelCollider,
    ) -> PathResult<SixAxis> {
        let (path, time_to_path) =
            timed!({ self.strategy.find_path(from, to, movable, immovable) });
        log::info!("Linear path in {} ms", time_to_path.as_millis());
        path
    }
}
