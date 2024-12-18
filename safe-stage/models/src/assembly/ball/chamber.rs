use crate::loader::load_stl_from_bytes;
use collisions::PrimaryCollider;

const DATA: &[u8] = include_bytes!("./models/BallArena.stl");

pub struct BallChamber {
    tree: PrimaryCollider,
}

impl BallChamber {
    pub fn collider(&self) -> &PrimaryCollider {
        &self.tree
    }
}

impl Default for BallChamber {
    fn default() -> Self {
        let triangles = load_stl_from_bytes(DATA).unwrap();
        let tree = PrimaryCollider::build(&triangles);
        BallChamber { tree }
    }
}
