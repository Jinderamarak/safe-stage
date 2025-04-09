use crate::common::reconstruct::reconstruct_path;
#[allow(deprecated)]
use crate::common::sight::line_of_sight;
use crate::neighbors::NeighborStrategy;
use crate::{common::heapstate::MinHeapState, path::PathResult, strategy::PathStrategy};
use models::collider::ModelCollider;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;
use std::collections::{BinaryHeap, HashMap, HashSet};

/// # Theta* Pathfinding Strategy
pub struct ThetaStarStrategy<N> {
    move_step: f64,
    move_cost: f64,
    rotate_step: f64,
    rotate_cost: f64,
    neighbor_strategy: N,
}

impl<N> ThetaStarStrategy<N>
where
    N: NeighborStrategy<SixAxis>,
{
    pub fn new(
        move_step: f64,
        move_cost: f64,
        rotate_step: f64,
        rotate_cost: f64,
        neighbor_strategy: N,
    ) -> ThetaStarStrategy<N> {
        ThetaStarStrategy {
            move_step,
            move_cost,
            rotate_step,
            rotate_cost,
            neighbor_strategy,
        }
    }

    fn heuristic(&self, from: &SixAxis, to: &SixAxis) -> f64 {
        #[allow(deprecated)]
        let (pos, rot) = from.manhattan_distances(to);
        pos * self.move_cost + rot * self.rotate_cost
    }

    #[allow(clippy::too_many_arguments)]
    fn update_vertex(
        &self,
        s: &SixAxis,
        sn: &SixAxis,
        end: &SixAxis,
        gscore: &mut HashMap<SixAxis, f64>,
        parent: &mut HashMap<SixAxis, SixAxis>,
        open: &mut BinaryHeap<MinHeapState<f64, SixAxis>>,
        movable: &dyn Movable<SixAxis>,
        immovable: &dyn ModelCollider,
    ) {
        #[allow(deprecated)]
        if line_of_sight(
            &parent[s],
            sn,
            movable,
            immovable,
            self.move_step,
            self.rotate_step,
        ) {
            let d = self.heuristic(&parent[s], sn);
            if gscore[&parent[s]] + d < gscore[sn] {
                gscore.insert(*sn, gscore[&parent[s]] + d);
                parent.insert(*sn, parent[s]);
                open.push(MinHeapState {
                    weight: gscore[sn] + self.heuristic(sn, end),
                    state: *sn,
                });
            }
        } else {
            let d = self.heuristic(s, sn);
            if gscore[s] + d < gscore[sn] {
                gscore.insert(*sn, gscore[s] + d);
                parent.insert(*sn, *s);
                open.push(MinHeapState {
                    weight: gscore[sn] + self.heuristic(sn, end),
                    state: *sn,
                });
            }
        }
    }
}

impl<N> PathStrategy<SixAxis> for ThetaStarStrategy<N>
where
    N: NeighborStrategy<SixAxis>,
{
    fn find_path(
        &self,
        start: &SixAxis,
        end: &SixAxis,
        movable: &dyn Movable<SixAxis>,
        immovable: &dyn ModelCollider,
    ) -> PathResult<SixAxis> {
        if immovable.collides_with(&movable.move_to(start)) {
            return PathResult::InvalidStart(*start);
        }

        let mut gscore: HashMap<SixAxis, f64> = HashMap::new();
        let mut parent: HashMap<SixAxis, SixAxis> = HashMap::new();
        let mut open: BinaryHeap<MinHeapState<f64, SixAxis>> = BinaryHeap::new();
        let mut closed: HashSet<SixAxis> = HashSet::new();

        gscore.insert(*start, 0.0);
        parent.insert(*start, *start);
        open.push(MinHeapState {
            weight: 0.0,
            state: *start,
        });

        while let Some(heap) = open.pop() {
            let s = heap.state;

            #[allow(deprecated)]
            if s.close_to_pos_rot(end, self.move_step, self.rotate_step) {
                if immovable.collides_with(&movable.move_to(end)) {
                    return PathResult::UnreachableEnd(Some(reconstruct_path(&parent, &s)));
                }

                if s != *end {
                    parent.insert(*end, s);
                }

                let path = reconstruct_path(&parent, end);
                return PathResult::Path(path);
            }

            closed.insert(s);
            for sn in self.neighbor_strategy.neighbors(&s) {
                if closed.contains(&sn) {
                    continue;
                }

                if immovable.collides_with(&movable.move_to(&sn)) {
                    continue;
                }

                if open.iter().all(|x| x.state != sn) {
                    gscore.insert(sn, f64::INFINITY);
                }

                self.update_vertex(
                    &s,
                    &sn,
                    end,
                    &mut gscore,
                    &mut parent,
                    &mut open,
                    movable,
                    immovable,
                )
            }
        }

        PathResult::UnreachableEnd(None)
    }
}
