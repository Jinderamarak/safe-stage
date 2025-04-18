use crate::common::timing::timed;
use crate::deferred::pathing::rotation_point_par::SafeRotationPointParallelStrategy;
use crate::eager::pathing::a_star_with_los::AStar3DSpaceWithLoSStrategy;
use crate::eager::space::sampled_space_3d::sample_grid_space_3d_par;
use crate::eager::space::space_3d::Grid3DSpace;
use crate::path::PathResult;
use crate::postprocess::smooth_par::smooth_path_par;
use crate::resolver::stage::StagePathResolver;
use crate::resolver::{PathResolver, StateUpdateError};
use crate::strategy::PathStrategy;
use collisions::common::Collides;
use maths::Vector3;
use models::immovable::Immovable;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;
use std::thread;

/// # Down Rotate Find Resolver
/// Path resolver intended for a stage.
/// Resolver the path by combining [SafeRotationPointParallelStrategy],
/// [AStar3DSpaceWithLoSStrategy] and [smooth_path_par].
///
/// **Runs in parallel using Rayon.**
pub struct DownRotateFindResolver {
    safe_rotation: SafeRotationPointParallelStrategy,
    move_speed: Vector3,
    sample_min: Vector3,
    sample_max: Vector3,
    sample_step: Vector3,
    sample_space: Option<Grid3DSpace>,
    sample_epsilon: Vector3,
    los_step: Vector3,
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
        los_step: Vector3,
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
            los_step,
            smoothing_step,
        }
    }
}

impl StagePathResolver for DownRotateFindResolver {}

impl PathResolver<SixAxis> for DownRotateFindResolver {
    fn update_state(
        &mut self,
        new: &SixAxis,
        movable: &dyn Movable<SixAxis>,
        immovable: &Immovable,
    ) -> Result<(), StateUpdateError> {
        if immovable.collides_with(&movable.move_to(new)) {
            return Err(StateUpdateError::InvalidState);
        }

        let (sampled, time_to_resample) = timed!({
            Some(sample_grid_space_3d_par(
                &self.sample_min,
                &self.sample_max,
                movable,
                immovable,
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
        movable: &dyn Movable<SixAxis>,
        immovable: &Immovable,
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
                            movable,
                            immovable,
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
            (_, Some(space)) | (Some(space), _) => AStar3DSpaceWithLoSStrategy::new(
                space,
                self.move_speed,
                self.sample_epsilon,
                self.los_step,
            ),
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
