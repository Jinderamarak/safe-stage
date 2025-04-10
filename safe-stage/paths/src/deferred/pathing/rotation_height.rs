use crate::path::PathResult;
use crate::strategy::PathStrategy;
use collisions::common::Collides;
use maths::{NaNExtension, Vector3};
use models::immovable::Immovable;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;

/// # Safe Rotation Height Strategy
/// Moves the stage down with the given step until it reaches a height, where it can safely rotate.
/// Then it rotates the stage to the target rotation.
///
/// Parallel version available with [SafeRotationHeightParallelStrategy].
pub struct SafeRotationHeightStrategy {
    height_min: f64,
    height_step: f64,
    rotation_step: Vector3,
}

impl SafeRotationHeightStrategy {
    pub fn new(height_min: f64, height_step: f64, rotation_step: Vector3) -> Self {
        Self {
            height_min,
            height_step,
            rotation_step,
        }
    }
}

impl PathStrategy<SixAxis> for SafeRotationHeightStrategy {
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

        let rotation_steps = SixAxis::from_rotation(from.rot).stepping(
            &SixAxis::from_rotation(to.rot),
            &SixAxis::from_rotation(self.rotation_step),
        );

        let pos_step = Vector3::new(0.0, 0.0, self.height_step);

        let mut current_pos = from.pos;
        while current_pos.z() >= self.height_min {
            let mut collided = false;
            for i in 0..=rotation_steps {
                let t = (i as f64 / rotation_steps as f64).map_nan(0.0);
                let rot = from.rot.lerp(&to.rot, t);
                let state = SixAxis {
                    pos: current_pos,
                    rot,
                };

                if immovable.collides_with(&movable.move_to(&state)) {
                    if i == 0 {
                        return PathResult::UnreachableEnd(None);
                    }
                    collided = true;
                    break;
                }
            }

            if !collided {
                let lowered_state = SixAxis {
                    pos: current_pos,
                    rot: from.rot,
                };
                let with_rotation = SixAxis {
                    pos: current_pos,
                    rot: to.rot,
                };
                return PathResult::Path(vec![*from, lowered_state, with_rotation]);
            }

            current_pos = current_pos - pos_step;
        }

        current_pos = Vector3::new(from.pos.x(), from.pos.y(), self.height_min);
        let lowered_state = SixAxis {
            pos: current_pos,
            rot: from.rot,
        };

        for i in 0..=rotation_steps {
            let t = (i as f64 / rotation_steps as f64).map_nan(0.0);
            let rot = from.rot.lerp(&to.rot, t);
            let state = SixAxis {
                pos: current_pos,
                rot,
            };

            if immovable.collides_with(&movable.move_to(&state)) {
                if i == 0 {
                    return PathResult::UnreachableEnd(None);
                }

                let previous_t = ((i - 1) as f64 / rotation_steps as f64).map_nan(0.0);
                let previous = from.rot.lerp(&to.rot, previous_t);
                let max_rotation = SixAxis {
                    pos: current_pos,
                    rot: previous,
                };

                return PathResult::UnreachableEnd(Some(vec![*from, lowered_state, max_rotation]));
            }
        }

        let with_rotation = SixAxis {
            pos: current_pos,
            rot: to.rot,
        };
        PathResult::Path(vec![*from, lowered_state, with_rotation])
    }
}
