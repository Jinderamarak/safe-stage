use crate::path::PathResult;
use crate::strategy::PathStrategy;
use maths::{NaNExtension, Vector3};
use models::collider::ModelCollider;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;
use rayon::prelude::*;

/// # Parallel Safe Rotation Height Strategy
/// Moves the stage down with the given step until it reaches a height, where it can safely rotate.
/// Then it rotates the stage to the target rotation.
///
/// **Runs in parallel using Rayon.**
///
/// Single-threaded version available with [SafeRotationHeightStrategy].
pub struct SafeRotationHeightParallelStrategy {
    height_min: f64,
    height_step: f64,
    rotation_step: Vector3,
}

impl SafeRotationHeightParallelStrategy {
    pub fn new(height_min: f64, height_step: f64, rotation_step: Vector3) -> Self {
        Self {
            height_min,
            height_step,
            rotation_step,
        }
    }
}

impl PathStrategy<SixAxis> for SafeRotationHeightParallelStrategy {
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

        let rotation_steps = SixAxis::from_rotation(from.rot).stepping(
            &SixAxis::from_rotation(to.rot),
            &SixAxis::from_rotation(self.rotation_step),
        );

        let (height_steps, height_step) =
            height_stepping(from.pos.z(), self.height_min, self.height_step);

        let first = (0..=height_steps)
            .into_par_iter()
            .map(|i| {
                let h = (from.pos.z() - i as f64 * height_step).max(self.height_min);
                for i in 0..=rotation_steps {
                    let t = (i as f64 / rotation_steps as f64).map_nan(0.0);
                    let rot = from.rot.lerp(&to.rot, t);
                    let state = SixAxis {
                        pos: Vector3::new(from.pos.x(), from.pos.y(), h),
                        rot,
                    };

                    if immovable.collides_with(&movable.move_to(&state)) {
                        if i == 0 {
                            return (i, Some(false));
                        }
                        return (i, None);
                    }
                }

                (i, Some(true))
            })
            .find_first(|(_, valid)| valid.is_some());

        if let Some((i, Some(true))) = first {
            let height = from.pos.z() - i as f64 * height_step;
            let lowered_state = SixAxis {
                pos: Vector3::new(from.pos.x(), from.pos.y(), height),
                rot: from.rot,
            };
            let with_rotation = SixAxis {
                pos: Vector3::new(from.pos.x(), from.pos.y(), height),
                rot: to.rot,
            };

            PathResult::Path(vec![*from, lowered_state, with_rotation])
        } else {
            PathResult::UnreachableEnd(None)
        }
    }
}

fn height_stepping(from: f64, min: f64, step: f64) -> (usize, f64) {
    let diff = from - min;
    let steps = (diff / step).ceil() as usize;
    let step = diff / steps as f64;
    (steps, step)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exclusive_height_stepping() {
        let from = 48.0;
        let min = 10.0;
        let step = 5.0;

        let (steps, step) = height_stepping(from, min, step);
        let last_height = from - steps as f64 * step;
        assert_eq!(min, last_height);
    }
}
