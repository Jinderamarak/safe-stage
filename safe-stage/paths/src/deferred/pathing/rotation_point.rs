use crate::path::PathResult;
use crate::strategy::PathStrategy;
use collisions::common::Collides;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;
use maths::{NaNExtension, Vector3};
use models::movable::Movable;
use models::position::sixaxis::SixAxis;

/// # Safe Rotation Point Strategy
/// Moves the stage towards a safe point by the given step until the rotation can be safely done.
/// Then it rotates the stage to the target rotation.
///
/// Parallel version available with [SafeRotationPointParallelStrategy].
pub struct SafeRotationPointStrategy {
    tend_point: Vector3,
    move_step: Vector3,
    rotation_step: Vector3,
}

impl SafeRotationPointStrategy {
    pub fn new(tend_point: Vector3, move_step: Vector3, rotation_step: Vector3) -> Self {
        Self {
            tend_point,
            move_step,
            rotation_step,
        }
    }
}

impl PathStrategy<SixAxis> for SafeRotationPointStrategy {
    fn find_path<M, I>(
        &self,
        from: &SixAxis,
        to: &SixAxis,
        movable: &M,
        immovable: &I,
    ) -> PathResult<SixAxis>
    where
        M: Movable<SixAxis> + Sync,
        I: Collides<ColliderGroup<PrimaryCollider>> + Sync + Send,
    {
        if immovable.collides_with(&movable.move_to(from)) {
            return PathResult::InvalidStart(*from);
        }

        let rotation_steps = vector_stepping(&from.rot, &to.rot, &self.rotation_step);
        let move_steps = vector_stepping(&from.pos, &self.tend_point, &self.move_step);

        let first = (0..=move_steps)
            .map(|i| {
                let t = (i as f64 / move_steps as f64).map_nan(0.0);
                let pos = from.pos.lerp(&self.tend_point, t);
                for j in 0..=rotation_steps {
                    let t = (j as f64 / rotation_steps as f64).map_nan(0.0);
                    let rot = from.rot.lerp(&to.rot, t);
                    let state = SixAxis { pos, rot };

                    if immovable.collides_with(&movable.move_to(&state)) {
                        if j == 0 {
                            return (i, Some(false));
                        }
                        return (i, None);
                    }
                }

                (i, Some(true))
            })
            .find(|(_, valid)| valid.is_some());

        if let Some((i, Some(true))) = first {
            let t = (i as f64 / move_steps as f64).map_nan(0.0);
            let pos = from.pos.lerp(&self.tend_point, t);
            let lowered_state = SixAxis { pos, rot: from.rot };
            let with_rotation = SixAxis { pos, rot: to.rot };

            PathResult::Path(vec![*from, lowered_state, with_rotation])
        } else {
            PathResult::UnreachableEnd(None)
        }
    }
}

fn vector_stepping(from: &Vector3, to: &Vector3, step: &Vector3) -> usize {
    let from = SixAxis::from_position(*from);
    let to = SixAxis::from_position(*to);
    let step = SixAxis::from_position(*step);
    from.stepping(&to, &step)
}
