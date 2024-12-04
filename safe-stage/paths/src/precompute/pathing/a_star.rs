use crate::common::heapstate::MinHeapState;
use crate::common::reconstruct::reconstruct_path;
use crate::path::PathResult;
use crate::precompute::space::space_3d::Grid3DSpace;
use crate::strategy::PathStrategy;
use collisions::common::Collides;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;
use maths::Vector3;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;
use std::collections::{BinaryHeap, HashMap};

type GridPoint = (usize, usize, usize);

pub struct AStar3DSpaceStrategy<'a> {
    space: &'a Grid3DSpace,
    speed: Vector3,
    epsilon: Vector3,
}

impl<'a> AStar3DSpaceStrategy<'a> {
    pub fn new(space: &'a Grid3DSpace, speed: Vector3, epsilon: Vector3) -> Self {
        Self {
            space,
            speed,
            epsilon,
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
}

impl PathStrategy<SixAxis> for AStar3DSpaceStrategy<'_> {
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

        let grid_start = if let Some(start) = self
            .space
            .around_on_grid(&from.pos)
            .into_iter()
            .find(|(x, y, z)| !self.space.is_occupied(*x, *y, *z))
        {
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
