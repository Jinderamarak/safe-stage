use crate::common::sight::line_of_sight_step_par;
use crate::path::PathResult;
use models::immovable::Immovable;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;

/// Smooths the path by removing intermediate nodes if there is a line of sight to a next node.
///
/// **Runs in parallel using Rayon.**
///
/// Single-threaded version available with [smooth_path].
pub fn smooth_path_par(
    path: PathResult<SixAxis>,
    movable: &dyn Movable<SixAxis>,
    immovable: &Immovable,
    step: &SixAxis,
) -> PathResult<SixAxis> {
    path.map(|p| smooth_path_nodes_par(p, movable, immovable, step))
}

fn smooth_path_nodes_par(
    path: &[SixAxis],
    movable: &dyn Movable<SixAxis>,
    immovable: &Immovable,
    step: &SixAxis,
) -> Vec<SixAxis> {
    let mut smooth = vec![path[0]];
    let mut k = 0;
    for i in 1..path.len() - 1 {
        if !line_of_sight_step_par(&path[k], &path[i + 1], movable, immovable, step) {
            smooth.push(path[i]);
            k = i;
        }
    }

    smooth.push(path[path.len() - 1]);
    smooth.shrink_to_fit();
    smooth
}
