use collisions::common::Collides;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;
use maths::Vector3;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;
use rand::random;
use rayon::prelude::*;

pub fn random_samples<M, I>(
    min: &SixAxis,
    max: &SixAxis,
    immovable: &I,
    movable: &M,
    count: usize,
) -> Vec<SixAxis>
where
    M: Movable<SixAxis>,
    I: Collides<ColliderGroup<PrimaryCollider>>,
{
    let range = max - min;
    let mut samples = Vec::with_capacity(count);
    while samples.len() < count {
        let sample = next_sample(min, &range);
        let at_sample = movable.move_to(&sample);
        if !immovable.collides_with(&at_sample) {
            samples.push(sample);
        }
    }

    samples.shrink_to_fit();
    samples
}

pub fn random_samples_par<M, I>(
    min: &SixAxis,
    max: &SixAxis,
    immovable: &I,
    movable: &M,
    count: usize,
) -> Vec<SixAxis>
where
    M: Movable<SixAxis> + Sync,
    I: Sync + Collides<ColliderGroup<PrimaryCollider>>,
{
    let range = max - min;
    (0..count)
        .into_par_iter()
        .map(|_| loop {
            let sample = next_sample(min, &range);
            let at_sample = movable.move_to(&sample);
            if !immovable.collides_with(&at_sample) {
                return sample;
            }
        })
        .collect()
}

fn next_sample(min: &SixAxis, range: &SixAxis) -> SixAxis {
    let x = random::<f64>() * range.pos.x() + min.pos.x();
    let y = random::<f64>() * range.pos.y() + min.pos.y();
    let z = random::<f64>() * range.pos.z() + min.pos.z();
    let rx = random::<f64>() * range.rot.x() + min.rot.x();
    let ry = random::<f64>() * range.rot.y() + min.rot.y();
    let rz = random::<f64>() * range.rot.z() + min.rot.z();
    SixAxis {
        pos: Vector3::new(x, y, z),
        rot: Vector3::new(rx, ry, rz),
    }
}
