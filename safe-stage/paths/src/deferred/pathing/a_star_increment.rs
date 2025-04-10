use crate::common::heapstate::MinHeapState;
use crate::common::reconstruct::reconstruct_path;
use crate::neighbors::NeighborStrategy;
use crate::path::PathResult;
use crate::strategy::PathStrategy;
use collisions::common::Collides;
use models::immovable::Immovable;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;
use std::collections::{BinaryHeap, HashMap};

/// # A* Incremental Pathfinding Strategy
/// Uses a previously found path as the input for the heuristic.
pub struct AStarIncrementStrategy<N> {
    previous: Vec<SixAxis>,
    speed: SixAxis,
    epsilon: SixAxis,
    neighbor_strategy: N,
}

impl<N> AStarIncrementStrategy<N>
where
    N: NeighborStrategy<SixAxis>,
{
    pub fn new(
        previous: Vec<SixAxis>,
        speed: SixAxis,
        epsilon: SixAxis,
        neighbor_strategy: N,
    ) -> AStarIncrementStrategy<N> {
        AStarIncrementStrategy {
            previous,
            speed,
            epsilon,
            neighbor_strategy,
        }
    }

    #[inline]
    fn heuristic(&self, from: &SixAxis, to: &SixAxis) -> f64 {
        let to = from.time_to(to, &self.speed);
        let path = from.time_to_path(&self.previous, &self.speed);
        to + path
    }
}

impl<N> PathStrategy<SixAxis> for AStarIncrementStrategy<N>
where
    N: NeighborStrategy<SixAxis>,
{
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

        let mut open_set = BinaryHeap::new();
        open_set.push(MinHeapState {
            weight: 0.0,
            state: *from,
        });

        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        g_score.insert(*from, 0.0);
        let mut f_score = HashMap::new();
        f_score.insert(*from, self.heuristic(from, to));

        while let Some(current) = open_set.pop() {
            let current = current.state;

            if current.close_to(to, &self.epsilon) {
                if immovable.collides_with(&movable.move_to(to)) {
                    return PathResult::UnreachableEnd(Some(reconstruct_path(
                        &came_from, &current,
                    )));
                }

                if current != *to {
                    came_from.insert(*to, current);
                }
                return PathResult::Path(reconstruct_path(&came_from, to));
            }

            for neighbor in self.neighbor_strategy.neighbors(&current) {
                if immovable.collides_with(&movable.move_to(&neighbor)) {
                    continue;
                }

                let tentative_g_score =
                    g_score.get(&current).unwrap() + current.time_to(&neighbor, &self.speed);
                if let Some(g) = g_score.get(&neighbor) {
                    if tentative_g_score >= *g {
                        continue;
                    }
                }

                let f = tentative_g_score + self.heuristic(&neighbor, to);

                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, f);
                open_set.push(MinHeapState {
                    weight: f,
                    state: neighbor,
                });
            }
        }

        let mut closest = *from;
        let mut best_score = f_score[&closest];
        for (coords, score) in f_score {
            if score < best_score {
                closest = coords;
                best_score = score;
            }
        }

        PathResult::UnreachableEnd(Some(reconstruct_path(&came_from, &closest)))
    }
}
