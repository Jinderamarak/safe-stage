use crate::common::timing::timed;
use crate::lazy::pathing::rotation_point_par::SafeRotationPointParallelStrategy;
use crate::path::PathResult;
use crate::postprocess::smooth_par::smooth_path_par;
use crate::precompute::pathing::a_star::AStar3DSpaceStrategy;
use crate::precompute::space::sampled_space_3d::sample_grid_space_3d_par;
use crate::precompute::space::space_3d::Grid3DSpace;
use crate::resolver::stage::StagePathResolver;
use crate::resolver::{PathResolver, StateUpdateError};
use crate::strategy::PathStrategy;
use collisions::common::Collides;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;
use maths::Vector3;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;
use std::thread;

pub struct DownRotateFindResolver {
    safe_rotation: SafeRotationPointParallelStrategy,
    move_speed: Vector3,
    sample_min: Vector3,
    sample_max: Vector3,
    sample_step: Vector3,
    sample_space: Option<Grid3DSpace>,
    sample_epsilon: Vector3,
    smoothing_step: SixAxis,
}

impl DownRotateFindResolver {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        down_point: Vector3,
        down_step: SixAxis,
        move_speed: Vector3,
        sample_min: Vector3,
        sample_max: Vector3,
        sample_step: Vector3,
        sample_epsilon: Vector3,
        smoothing_step: SixAxis,
    ) -> Self {
        let safe_rotation =
            SafeRotationPointParallelStrategy::new(down_point, down_step.pos, down_step.rot);
        let sample_space = None;
        Self {
            safe_rotation,
            move_speed,
            sample_min,
            sample_max,
            sample_step,
            sample_space,
            sample_epsilon,
            smoothing_step,
        }
    }
}

impl StagePathResolver for DownRotateFindResolver {}

impl<M, I> PathResolver<SixAxis, M, I> for DownRotateFindResolver
where
    M: Movable<SixAxis> + Sync,
    I: Collides<ColliderGroup<PrimaryCollider>> + Sync + Send,
{
    fn update_state(
        &mut self,
        new: &SixAxis,
        movable: &M,
        immovable: &I,
    ) -> Result<(), StateUpdateError> {
        if immovable.collides_with(&movable.move_to(new)) {
            return Err(StateUpdateError::InvalidState);
        }

        let (sampled, time_to_resample) = timed!({
            Some(sample_grid_space_3d_par(
                &self.sample_min,
                &self.sample_max,
                immovable,
                movable,
                &self.sample_step,
                &new.rot,
            ))
        });

        self.sample_space = sampled;
        log::info!("Resampled space in {} ms", time_to_resample.as_millis());

        Ok(())
    }

    fn resolve_path(
        &self,
        from: &SixAxis,
        to: &SixAxis,
        movable: &M,
        immovable: &I,
    ) -> PathResult<SixAxis> {
        let mut start = *from;
        let mut prepath = Vec::new();
        let mut resampled = None;
        let diff = to - &start;
        if diff.rot.len2() != 0.0 {
            log::debug!("Rotation changed, resampling space and going to tend point");

            let (resample, down) = thread::scope(|s| {
                let resample = s.spawn(|| {
                    let (result, time_to_resampled) = timed!({
                        sample_grid_space_3d_par(
                            &self.sample_min,
                            &self.sample_max,
                            immovable,
                            movable,
                            &self.sample_step,
                            &start.rot,
                        )
                    });
                    log::info!("Resampled space in {} ms", time_to_resampled.as_millis());
                    result
                });
                let down = s.spawn(|| {
                    let (result, time_to_position) =
                        timed!({ self.safe_rotation.find_path(&start, to, movable, immovable) });
                    log::info!(
                        "Found path to safe rotation in {} ms",
                        time_to_position.as_millis()
                    );
                    result
                });

                (resample.join().unwrap(), down.join().unwrap())
            });

            resampled = Some(resample);
            prepath = match down {
                PathResult::Path(path) => path,
                other => return other,
            };
            start = *prepath.last().unwrap();
            log::debug!("New start point: {start:?}");
        }

        let strategy = match (&self.sample_space, &resampled) {
            (_, Some(space)) | (Some(space), _) => {
                AStar3DSpaceStrategy::new(space, self.move_speed, self.sample_epsilon)
            }
            (None, None) => {
                unreachable!("Resolver was not properly initialized by updating its state!");
            }
        };

        let (path, time_to_path) = timed!({ strategy.find_path(&start, to, movable, immovable) });
        log::info!("Found rough path in {} ms", time_to_path.as_millis());
        log::debug!("Rough path has {} nodes", path.nodes());

        let path = match path {
            PathResult::Path(mut path) => {
                prepath.append(&mut path);
                PathResult::Path(prepath)
            }
            PathResult::UnreachableEnd(Some(mut hint)) => {
                prepath.append(&mut hint);
                PathResult::UnreachableEnd(Some(prepath))
            }
            other => return other,
        };

        let (smoothed, time_to_smooth) =
            timed!({ smooth_path_par(path, movable, immovable, &self.smoothing_step) });
        log::info!("Smoothed path in {} ms", time_to_smooth.as_millis());
        log::debug!("Smoothed path has {} nodes", smoothed.nodes());

        smoothed
    }
}
