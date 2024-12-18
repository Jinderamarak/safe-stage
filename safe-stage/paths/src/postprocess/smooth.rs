use crate::common::sight::line_of_sight_step;
use crate::path::PathResult;
use collisions::common::Collides;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;

/// Smooths the path by removing intermediate nodes if there is a line of sight to a next node.
///
/// Parallel version available with [smooth_path_par].
pub fn smooth_path<M, I>(
    path: PathResult<SixAxis>,
    movable: &M,
    immovable: &I,
    step: &SixAxis,
) -> PathResult<SixAxis>
where
    M: Movable<SixAxis>,
    I: Collides<ColliderGroup<PrimaryCollider>>,
{
    path.map(|p| smooth_path_nodes(p, movable, immovable, step))
}

fn smooth_path_nodes<M, I>(
    path: &[SixAxis],
    movable: &M,
    immovable: &I,
    step: &SixAxis,
) -> Vec<SixAxis>
where
    M: Movable<SixAxis>,
    I: Collides<ColliderGroup<PrimaryCollider>>,
{
    let mut smooth = vec![path[0]];
    let mut k = 0;
    for i in 1..path.len() - 1 {
        if !line_of_sight_step(&path[k], &path[i + 1], movable, immovable, step) {
            smooth.push(path[i]);
            k = i;
        }
    }

    smooth.push(path[path.len() - 1]);
    smooth.shrink_to_fit();
    smooth
}
