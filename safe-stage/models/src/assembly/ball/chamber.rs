use crate::loader::load_stl_from_bytes;
use collisions::complex::BvhSphere;

const DATA: &[u8] = include_bytes!("./models/BallArena.stl");

pub struct BallChamber {
    tree: BvhSphere,
}

impl BallChamber {
    pub fn collider(&self) -> &BvhSphere {
        &self.tree
    }
}

impl Default for BallChamber {
    fn default() -> Self {
        let triangles = load_stl_from_bytes(DATA).unwrap();
        let tree = BvhSphere::build(&triangles);
        BallChamber { tree }
    }
}
