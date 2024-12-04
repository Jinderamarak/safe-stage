use crate::neighbors::NeighborStrategy;
use collisions::common::Collides;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;
use std::collections::{HashSet, VecDeque};

pub fn flood_samples<M, I, N>(
    start: &SixAxis,
    immovable: &I,
    movable: &M,
    strategy: &N,
) -> Vec<SixAxis>
where
    M: Movable<SixAxis>,
    I: Collides<ColliderGroup<PrimaryCollider>>,
    N: NeighborStrategy<SixAxis>,
{
    let mut samples: Vec<SixAxis> = Vec::new();
    let mut closed: HashSet<SixAxis> = HashSet::new();
    let mut open: VecDeque<SixAxis> = VecDeque::new();

    open.push_back(*start);

    while let Some(state) = open.pop_front() {
        for neighbor in strategy.neighbors(&state) {
            if closed.contains(&neighbor) {
                continue;
            }
            if immovable.collides_with(&movable.move_to(&neighbor)) {
                continue;
            }

            samples.push(neighbor);
            closed.insert(neighbor);
            open.push_back(neighbor);
        }
    }

    samples.shrink_to_fit();
    samples
}
