use collisions::common::Collides;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;
use maths::Vector3;
use models::immovable::Immovable;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;
use rayon::prelude::*;

pub fn grid_samples<M, I>(
    min: &SixAxis,
    max: &SixAxis,
    immovable: &I,
    movable: &M,
    step: &SixAxis,
) -> Vec<SixAxis>
where
    M: Movable<SixAxis>,
    I: Collides<ColliderGroup<PrimaryCollider>>,
{
    let diff = max - min;
    let (x, y, z, rx, ry, rz) = (
        range(diff.pos.x(), step.pos.x()),
        range(diff.pos.y(), step.pos.y()),
        range(diff.pos.z(), step.pos.z()),
        range(diff.rot.x(), step.rot.x()),
        range(diff.rot.y(), step.rot.y()),
        range(diff.rot.z(), step.rot.z()),
    );

    let mut samples = Vec::new();
    for x in 0..x {
        for y in 0..y {
            for z in 0..z {
                for rx in 0..rx {
                    for ry in 0..ry {
                        for rz in 0..rz {
                            let sample = SixAxis {
                                pos: Vector3::new(
                                    min.pos.x() + x as f64 * step.pos.x(),
                                    min.pos.y() + y as f64 * step.pos.y(),
                                    min.pos.z() + z as f64 * step.pos.z(),
                                ),
                                rot: Vector3::new(
                                    min.rot.x() + rx as f64 * step.rot.x(),
                                    min.rot.y() + ry as f64 * step.rot.y(),
                                    min.rot.z() + rz as f64 * step.rot.z(),
                                ),
                            };

                            let at_sample = movable.move_to(&sample);
                            if !immovable.collides_with(&at_sample) {
                                samples.push(sample);
                            }
                        }
                    }
                }
            }
        }
    }

    samples.shrink_to_fit();
    samples
}

pub fn grid_samples_par(
    min: &SixAxis,
    max: &SixAxis,
    movable: &dyn Movable<SixAxis>,
    immovable: &Immovable,
    step: &SixAxis,
) -> Vec<SixAxis> {
    let diff = max - min;
    let (x, y, z, rx, ry, rz) = (
        range(diff.pos.x(), step.pos.x()),
        range(diff.pos.y(), step.pos.y()),
        range(diff.pos.z(), step.pos.z()),
        range(diff.rot.x(), step.rot.x()),
        range(diff.rot.y(), step.rot.y()),
        range(diff.rot.z(), step.rot.z()),
    );

    (0..x)
        .into_par_iter()
        .flat_map(|x| {
            (0..y).into_par_iter().flat_map(move |y| {
                (0..z).into_par_iter().flat_map(move |z| {
                    (0..rx).into_par_iter().flat_map(move |rx| {
                        (0..ry).into_par_iter().flat_map(move |ry| {
                            (0..rz).into_par_iter().filter_map(move |rz| {
                                let sample = SixAxis {
                                    pos: Vector3::new(
                                        min.pos.x() + x as f64 * step.pos.x(),
                                        min.pos.y() + y as f64 * step.pos.y(),
                                        min.pos.z() + z as f64 * step.pos.z(),
                                    ),
                                    rot: Vector3::new(
                                        min.rot.x() + rx as f64 * step.rot.x(),
                                        min.rot.y() + ry as f64 * step.rot.y(),
                                        min.rot.z() + rz as f64 * step.rot.z(),
                                    ),
                                };

                                let at_sample = movable.move_to(&sample);
                                if !immovable.collides_with(&at_sample) {
                                    Some(sample)
                                } else {
                                    None
                                }
                            })
                        })
                    })
                })
            })
        })
        .collect()
}

fn range(diff: f64, step: f64) -> usize {
    if step == 0.0 {
        1
    } else {
        (diff / step).ceil() as usize
    }
}
