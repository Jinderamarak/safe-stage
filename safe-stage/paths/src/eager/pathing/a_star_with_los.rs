use crate::common::heapstate::MinHeapState;
use crate::common::reconstruct::reconstruct_path;
use crate::common::sight::line_of_sight_step_par;
use crate::eager::space::space_3d::Grid3DSpace;
use crate::path::PathResult;
use crate::strategy::PathStrategy;
use collisions::common::Collides;
use maths::Vector3;
use models::immovable::Immovable;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;
use std::collections::{BinaryHeap, HashMap};

type GridPoint = (usize, usize, usize);

/// # A* 3D Space with Line of Sight Strategy
/// A* pathfinding strategy for a precomputed 3D grid space
/// with line of sight check between the neighbors.
///
/// The strategy uses the given speed to calculate the heuristic.
///
/// **Uses Rayon for parallel line of sight checks.**
pub struct AStar3DSpaceWithLoSStrategy<'a> {
    space: &'a Grid3DSpace,
    speed: Vector3,
    epsilon: Vector3,
    los_step: SixAxis,
}

impl<'a> AStar3DSpaceWithLoSStrategy<'a> {
    pub fn new(
        space: &'a Grid3DSpace,
        speed: Vector3,
        epsilon: Vector3,
        los_step: Vector3,
    ) -> Self {
        Self {
            space,
            speed,
            epsilon,
            los_step: SixAxis::from_position(los_step),
        }
    }

    #[inline]
    const fn with_speed(&self, v: &Vector3) -> Vector3 {
        Vector3::new(
            v.x() / self.speed.x(),
            v.y() / self.speed.y(),
            v.z() / self.speed.z(),
        )
    }

    fn grid_path_to_global_with_start(
        &self,
        path: &[GridPoint],
        from: &SixAxis,
        to: Option<&SixAxis>,
    ) -> Vec<SixAxis> {
        let start = [*from].into_iter().chain(path.iter().map(|grid| SixAxis {
            pos: self.space.grid_to_global(grid),
            rot: from.rot,
        }));

        if let Some(to) = to {
            start.chain([*to]).collect()
        } else {
            start.collect()
        }
    }

    fn heuristic(&self, from: &Vector3, to: &Vector3) -> f64 {
        let d = to - from;
        self.with_speed(&d).len()
    }

    #[inline]
    fn has_line_of_sight(
        &self,
        rot: &Vector3,
        from: &Vector3,
        to: &Vector3,
        movable: &dyn Movable<SixAxis>,
        immovable: &Immovable,
    ) -> bool {
        let sight_from = SixAxis {
            pos: *from,
            rot: *rot,
        };
        let sight_to = SixAxis {
            pos: *to,
            rot: *rot,
        };

        line_of_sight_step_par(&sight_from, &sight_to, movable, immovable, &self.los_step)
    }
}

impl PathStrategy<SixAxis> for AStar3DSpaceWithLoSStrategy<'_> {
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

        let rotation = from.rot;
        let grid_start = if let Some(start) =
            self.space
                .around_on_grid(&from.pos)
                .into_iter()
                .find(|(x, y, z)| {
                    let global = self.space.grid_to_global(&(*x, *y, *z));
                    !self.space.is_occupied(*x, *y, *z)
                        && self.has_line_of_sight(&rotation, &from.pos, &global, movable, immovable)
                }) {
            start
        } else {
            return PathResult::InvalidStart(*from);
        };

        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        let mut f_score = HashMap::new();

        open_set.push(MinHeapState {
            weight: 0.0,
            state: grid_start,
        });
        g_score.insert(grid_start, 0.0);
        f_score.insert(grid_start, self.heuristic(&from.pos, &to.pos));

        while let Some(MinHeapState { state: current, .. }) = open_set.pop() {
            let current_global = self.space.grid_to_global(&current);
            let diff = (to.pos - current_global).abs();

            if diff.x() < self.epsilon.x()
                && diff.y() < self.epsilon.y()
                && diff.z() < self.epsilon.z()
            {
                if immovable.collides_with(&movable.move_to(&into_sixaxis(&to.pos, from))) {
                    return PathResult::UnreachableEnd(Some(self.grid_path_to_global_with_start(
                        &reconstruct_path(&came_from, &current),
                        from,
                        None,
                    )));
                }

                return PathResult::Path(self.grid_path_to_global_with_start(
                    &reconstruct_path(&came_from, &current),
                    from,
                    Some(to),
                ));
            }

            for neighbor in self.space.neighbors_iter(current.0, current.1, current.2) {
                let neighbor_global = self.space.grid_to_global(&neighbor);

                if !self.has_line_of_sight(
                    &rotation,
                    &current_global,
                    &neighbor_global,
                    movable,
                    immovable,
                ) {
                    continue;
                }

                if immovable.collides_with(&movable.move_to(&into_sixaxis(&neighbor_global, from)))
                {
                    continue;
                }

                let neighbor_to_current = current_global - neighbor_global;
                let tentative_g_score =
                    g_score[&current] + self.with_speed(&neighbor_to_current).len();
                if let Some(g) = g_score.get(&neighbor) {
                    if tentative_g_score >= *g {
                        continue;
                    }
                }

                let tentative_f_score =
                    tentative_g_score + self.heuristic(&neighbor_global, &to.pos);

                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_f_score);
                open_set.push(MinHeapState {
                    weight: tentative_f_score,
                    state: neighbor,
                });
            }
        }

        let mut closest = grid_start;
        let mut closest_dist = f_score[&closest];
        for (grid, dist) in f_score {
            if dist < closest_dist {
                closest = grid;
                closest_dist = dist;
            }
        }

        PathResult::UnreachableEnd(Some(self.grid_path_to_global_with_start(
            &reconstruct_path(&came_from, &closest),
            from,
            None,
        )))
    }
}

#[inline]
const fn into_sixaxis(global: &Vector3, from: &SixAxis) -> SixAxis {
    SixAxis {
        pos: *global,
        rot: from.rot,
    }
}
