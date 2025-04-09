use crate::path::PathResult;
use crate::strategy::PathStrategy;
use maths::NaNExtension;
use models::collider::ModelCollider;
use models::movable::Movable;
use models::position::linear::LinearState;
use models::position::sixaxis::SixAxis;

/// # Linear Pathfinding Strategy
/// Moves in a straight line from start to end with a fixed step size.
///
/// Parallel version available with [LinearParallelStrategy].
pub struct LinearStrategy<P> {
    step_size: P,
}

impl<P> LinearStrategy<P> {
    pub fn new(step_size: P) -> Self {
        Self { step_size }
    }
}

impl PathStrategy<SixAxis> for LinearStrategy<SixAxis> {
    fn find_path(
        &self,
        from: &SixAxis,
        to: &SixAxis,
        movable: &dyn Movable<SixAxis>,
        immovable: &dyn ModelCollider,
    ) -> PathResult<SixAxis> {
        if immovable.collides_with(&movable.move_to(from)) {
            return PathResult::InvalidStart(*from);
        }

        let max_steps = from.stepping(to, &self.step_size);
        for i in 1..=max_steps {
            let t = (i as f64 / max_steps as f64).map_nan(0.0);
            let state = from.lerp_t(to, t);

            if immovable.collides_with(&movable.move_to(&state)) {
                if i == 1 {
                    return PathResult::UnreachableEnd(Some(vec![*from]));
                }

                let previous_t = ((i - 1) as f64 / max_steps as f64).map_nan(0.0);
                let previous = from.lerp_t(to, previous_t);
                return PathResult::UnreachableEnd(Some(vec![*from, previous]));
            }
        }

        PathResult::Path(vec![*from, *to])
    }
}

impl PathStrategy<LinearState> for LinearStrategy<LinearState> {
    fn find_path(
        &self,
        from: &LinearState,
        to: &LinearState,
        movable: &dyn Movable<LinearState>,
        immovable: &dyn ModelCollider,
    ) -> PathResult<LinearState> {
        if immovable.collides_with(&movable.move_to(from)) {
            return PathResult::InvalidStart(*from);
        }

        let steps = ((to.as_relative() - from.as_relative()).abs()
            / self.step_size.as_relative().ceil()) as usize;
        for i in 1..=steps {
            let t = (i as f64 / steps as f64).map_nan(0.0);
            let state = from.lerp(to, t);

            if immovable.collides_with(&movable.move_to(&state)) {
                if i == 1 {
                    return PathResult::UnreachableEnd(Some(vec![*from]));
                }

                let previous_t = ((i - 1) as f64 / steps as f64).map_nan(0.0);
                let previous = from.lerp(to, previous_t);
                return PathResult::UnreachableEnd(Some(vec![*from, previous]));
            }
        }

        PathResult::Path(vec![*from, *to])
    }
}
