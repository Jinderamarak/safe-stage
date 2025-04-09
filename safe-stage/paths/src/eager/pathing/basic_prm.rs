use crate::{common::heapstate::MinHeapState, path::PathResult, strategy::PathStrategy};
use maths::Vector3;
use models::collider::ModelCollider;
use models::{movable::Movable, position::sixaxis::SixAxis};
use std::collections::{BinaryHeap, HashMap};

type Graph = HashMap<usize, Vec<usize>>;

/// # Basic Probabilistic Roadmap Strategy
/// Implementation of a Probabilistic Road-Map pathfinding.
pub struct BasicPrmStrategy {
    samples: Vec<SixAxis>,
    graph: Graph,
}

impl BasicPrmStrategy {
    pub fn new(samples: Vec<SixAxis>, graph: Graph) -> BasicPrmStrategy {
        BasicPrmStrategy { samples, graph }
    }

    pub fn samples(&self) -> &Vec<SixAxis> {
        &self.samples
    }

    fn heuristic(from: &SixAxis, to: &SixAxis) -> f64 {
        let pos = 1.0;
        let rot = 1_f64.to_radians();
        let speed = SixAxis {
            pos: Vector3::new(pos, pos, pos),
            rot: Vector3::new(rot, rot, rot),
        };

        from.time_to(to, &speed)
    }

    fn reconstruct_path(
        &self,
        came_from: &HashMap<usize, usize>,
        current: usize,
        from: &SixAxis,
        to: &SixAxis,
    ) -> Vec<SixAxis> {
        let mut total_path = vec![*to, self.samples[current]];
        let mut current = current;
        while let Some(next) = came_from.get(&current) {
            total_path.push(self.samples[*next]);
            current = *next;
        }

        total_path.push(*from);
        total_path
    }
}

impl PathStrategy<SixAxis> for BasicPrmStrategy {
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

        let mut closest_start = 0;
        #[allow(deprecated)]
        let closest_start_dist = self.samples[0].euclidean_distances(from);
        let mut closest_start_dist = closest_start_dist.0 + closest_start_dist.1;

        let mut closest_end = 0;
        #[allow(deprecated)]
        let closest_end_dist = self.samples[0].euclidean_distances(to);
        let mut closest_end_dist = closest_end_dist.0 + closest_end_dist.1;

        for (i, sample) in self.samples.iter().enumerate() {
            #[allow(deprecated)]
            let dist = sample.euclidean_distances(from);
            let dist = dist.0 + dist.1;
            if dist < closest_start_dist {
                closest_start = i;
                closest_start_dist = dist;
            }

            #[allow(deprecated)]
            let dist = sample.euclidean_distances(to);
            let dist = dist.0 + dist.1;
            if dist < closest_end_dist {
                closest_end = i;
                closest_end_dist = dist;
            }
        }

        let mut open_set = BinaryHeap::new();
        open_set.push(MinHeapState {
            weight: 0.0,
            state: closest_start,
        });

        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        g_score.insert(closest_start, 0.0);

        let mut f_score = HashMap::new();
        f_score.insert(
            closest_start,
            Self::heuristic(&self.samples[closest_start], &self.samples[closest_end]),
        );

        while let Some(current) = open_set.pop() {
            let current = current.state;

            if current == closest_end {
                return PathResult::Path(self.reconstruct_path(&came_from, current, from, to));
            }

            for neighbor in &self.graph[&current] {
                let tentative_g_score = g_score[&current] + 1.0;
                if let Some(g) = g_score.get(neighbor) {
                    if tentative_g_score >= *g {
                        continue;
                    }
                }

                came_from.insert(*neighbor, current);
                g_score.insert(*neighbor, tentative_g_score);
                f_score.insert(
                    *neighbor,
                    tentative_g_score
                        + Self::heuristic(&self.samples[*neighbor], &self.samples[closest_end]),
                );
                open_set.push(MinHeapState {
                    weight: tentative_g_score
                        + Self::heuristic(&self.samples[*neighbor], &self.samples[closest_end]),
                    state: *neighbor,
                });
            }
        }

        PathResult::UnreachableEnd(None)
    }
}
