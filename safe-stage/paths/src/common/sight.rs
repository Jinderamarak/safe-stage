use collisions::common::Collides;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;
use maths::NaNExtension;
use models::immovable::Immovable;
use models::{movable::Movable, position::sixaxis::SixAxis};
use rayon::prelude::*;

/// Check if there is a line of sight between two coordinates
/// by using linear interpolation with fixed step.
#[deprecated]
pub fn line_of_sight(
    from: &SixAxis,
    to: &SixAxis,
    movable: &dyn Movable<SixAxis>,
    immovable: &dyn Collides<ColliderGroup<PrimaryCollider>>,
    move_step: f64,
    rotate_step: f64,
) -> bool {
    let diff_pos = (to.pos - from.pos).abs();
    let diff_rot = from.shortest_rotation(to);

    let move_steps = diff_pos.x().max(diff_pos.y()).max(diff_pos.z()) / move_step;
    let rotate_steps = diff_rot.x().max(diff_rot.y()).max(diff_rot.z()) / rotate_step;

    let steps = move_steps.max(rotate_steps).max(1.0) as usize;
    for i in 0..=steps {
        let t = (i as f64 / steps as f64).map_nan(0.0);
        #[allow(deprecated)]
        let c = from.lerp(to, t, t);
        if immovable.collides_with(&movable.move_to(&c)) {
            return false;
        }
    }

    true
}

/// Checks if there is a line of sight between two coordinates by moving in a given step.
///
/// Parallel version available with [line_of_sight_par].
pub fn line_of_sight_step(
    from: &SixAxis,
    to: &SixAxis,
    movable: &dyn Movable<SixAxis>,
    immovable: &dyn Collides<ColliderGroup<PrimaryCollider>>,
    step: &SixAxis,
) -> bool {
    let max_steps = from.stepping(to, step);
    (0..=max_steps).all(|i| {
        let t = (i as f64 / max_steps as f64).map_nan(0.0);
        let state = from.lerp_t(to, t);
        !immovable.collides_with(&movable.move_to(&state))
    })
}

/// Checks if there is a line of sight between two coordinates by moving in a given step.
///
/// **Runs in parallel using Rayon.**
///
/// Single-threaded version available with [line_of_sight_step].
pub fn line_of_sight_step_par(
    from: &SixAxis,
    to: &SixAxis,
    movable: &dyn Movable<SixAxis>,
    immovable: &Immovable,
    step: &SixAxis,
) -> bool {
    let max_steps = from.stepping(to, step);
    (0..=max_steps).into_par_iter().all(|i| {
        let t = (i as f64 / max_steps as f64).map_nan(0.0);
        let state = from.lerp_t(to, t);
        !immovable.collides_with(&movable.move_to(&state))
    })
}
