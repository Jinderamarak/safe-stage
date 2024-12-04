use crate::loader::load_stl_from_bytes;
use crate::movable::Movable;
use crate::position::sixaxis::SixAxis;
use collisions::common::Transformation;
use collisions::complex::group::ColliderGroup;
use collisions::primitive::TriangleCollider;
use collisions::{collider_group, PrimaryCollider};
use maths::{Quaternion, Vector3};

const DATA: &[u8] = include_bytes!("./models/BallStage.stl");

pub struct BallStage {
    tree: PrimaryCollider,
}

impl Default for BallStage {
    fn default() -> Self {
        let triangles = ball_stage_triangles();
        let tree = PrimaryCollider::build(&triangles);
        BallStage { tree }
    }
}

pub fn ball_stage_triangles() -> Vec<TriangleCollider> {
    load_stl_from_bytes(DATA).unwrap()
}

impl Movable<SixAxis> for BallStage {
    fn move_to(&self, coords: &SixAxis) -> ColliderGroup<PrimaryCollider> {
        let rotation = Quaternion::from_euler(&coords.rot);
        let pivot = Vector3::ZERO;
        let movement = coords.pos;
        collider_group!(self.tree.transform(&rotation, &pivot, &movement))
    }
}
