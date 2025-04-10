use crate::path::PathResult;
use crate::strategy::PathStrategy;
use collisions::common::Collides;
use maths::NaNExtension;
use models::immovable::Immovable;
use models::movable::Movable;
use models::position::linear::LinearState;
use models::position::sixaxis::SixAxis;
use rayon::prelude::*;

/// # Parallel Linear Pathfinding Strategy
/// Moves in a straight line from start to end with fixed step.
///
/// **Runs in parallel using Rayon.**
///
/// Single-threaded version available with [LinearStrategy].
pub struct LinearParallelStrategy<P> {
    step_size: P,
}

impl<P> LinearParallelStrategy<P> {
    pub fn new(step_size: P) -> Self {
        Self { step_size }
    }
}

impl PathStrategy<SixAxis> for LinearParallelStrategy<SixAxis> {
    fn find_path(
        &self,
        from: &SixAxis,
        to: &SixAxis,
        movable: &dyn Movable<SixAxis>,
        immovable: &Immovable,
    ) -> PathResult<SixAxis> {
        if immovable.collides_with(&movable.move_to(from)) {
            return PathResult::InvalidStart(*from);
        }

        let max_steps = from.stepping(to, &self.step_size);
        let first = (1..=max_steps).into_par_iter().find_first(|i| {
            let t = (*i as f64 / max_steps as f64).map_nan(0.0);
            let state = from.lerp_t(to, t);
            immovable.collides_with(&movable.move_to(&state))
        });

        if let Some(i) = first {
            if i == 1 {
                return PathResult::UnreachableEnd(Some(vec![*from]));
            }

            let previous_t = ((i - 1) as f64 / max_steps as f64).map_nan(0.0);
            let previous = from.lerp_t(to, previous_t);
            return PathResult::UnreachableEnd(Some(vec![*from, previous]));
        }

        PathResult::Path(vec![*from, *to])
    }
}

impl PathStrategy<LinearState> for LinearParallelStrategy<LinearState> {
    fn find_path(
        &self,
        from: &LinearState,
        to: &LinearState,
        movable: &dyn Movable<LinearState>,
        immovable: &Immovable,
    ) -> PathResult<LinearState> {
        if immovable.collides_with(&movable.move_to(from)) {
            return PathResult::InvalidStart(*from);
        }

        let steps = ((to.as_relative() - from.as_relative()).abs() / self.step_size.as_relative())
            .ceil() as usize;

        let first = (1..=steps).into_par_iter().find_first(|i| {
            let t = (*i as f64 / steps as f64).map_nan(0.0);
            let state = from.lerp(to, t);
            immovable.collides_with(&movable.move_to(&state))
        });

        if let Some(i) = first {
            if i == 1 {
                return PathResult::UnreachableEnd(Some(vec![*from]));
            }

            let previous_t = ((i - 1) as f64 / steps as f64).map_nan(0.0);
            let previous = from.lerp(to, previous_t);
            return PathResult::UnreachableEnd(Some(vec![*from, previous]));
        }

        PathResult::Path(vec![*from, *to])
    }
}
