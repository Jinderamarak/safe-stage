use crate::eager::space::space_3d::Grid3DSpace;
use bitvec::vec::BitVec;
use collisions::common::Collides;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;
use maths::Vector3;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;
use rayon::prelude::*;

pub fn sample_grid_space_3d<M, I>(
    min: &Vector3,
    max: &Vector3,
    immovable: &I,
    movable: &M,
    step: &Vector3,
    rotation: &Vector3,
) -> Grid3DSpace
where
    M: Movable<SixAxis>,
    I: Collides<ColliderGroup<PrimaryCollider>>,
{
    let diff = max - min;
    let (x, y, z) = (
        range(diff.x(), step.x()),
        range(diff.y(), step.y()),
        range(diff.z(), step.z()),
    );

    let mut space = Grid3DSpace::new(x, y, z, *min, *max);
    for x in 0..x {
        for y in 0..y {
            for z in 0..z {
                let sample = SixAxis {
                    pos: space.grid_to_global(&(x, y, z)),
                    rot: *rotation,
                };

                let at_sample = movable.move_to(&sample);
                if immovable.collides_with(&at_sample) {
                    space.set_occupied(x, y, z);
                } else {
                    space.set_empty(x, y, z);
                }
            }
        }
    }

    space
}

pub fn sample_grid_space_3d_par<M, I>(
    min: &Vector3,
    max: &Vector3,
    immovable: &I,
    movable: &M,
    step: &Vector3,
    rotation: &Vector3,
) -> Grid3DSpace
where
    M: Movable<SixAxis> + Sync,
    I: Sync + Collides<ColliderGroup<PrimaryCollider>>,
{
    let diff = max - min;
    let (dx, dy, dz) = (
        range(diff.x(), step.x()),
        range(diff.y(), step.y()),
        range(diff.z(), step.z()),
    );

    //  Create space here for converting and initialize its data later
    let space = unsafe { Grid3DSpace::uninitialized(dx, dy, dz, *min, *max) };

    let len = dx * dy * dz;
    let samples = (0..len)
        .into_par_iter()
        .map(|i| {
            let x = i % dx;
            let y = (i / dx) % dy;
            let z = i / (dx * dy);

            let sample = SixAxis {
                pos: space.grid_to_global(&(x, y, z)),
                rot: *rotation,
            };

            let at_sample = movable.move_to(&sample);
            immovable.collides_with(&at_sample)
        })
        .collect::<Vec<_>>();

    let mut data = samples.into_iter().collect::<BitVec>();
    data.shrink_to_fit();

    space.with_data(data).unwrap()
}

fn range(diff: f64, step: f64) -> usize {
    if step == 0.0 {
        1
    } else {
        (diff / step).ceil() as usize + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_inclusive() {
        let start = 1.0;
        let end = 3.0;
        let step = 1.3;

        //  1.0, 2.3, 3.6 -> 3
        //  1.0, 2.0, 3.0 <-

        let expected = 3;
        let actual = range(end - start, step);
        assert_eq!(expected, actual);
    }

    #[test]
    fn range_inclusive_ceiling() {
        let start = 1.0;
        let end = 3.0;
        let step = 1.6;

        //  1.0, 2.6, 4.2 -> 3
        //  1.0, 2.0, 3.0 <-

        let expected = 3;
        let actual = range(end - start, step);
        assert_eq!(expected, actual);
    }
}
