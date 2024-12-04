use crate::loader::load_stl_from_bytes;
use crate::parts::chamber::Chamber;
use collisions::common::Translation;
use collisions::complex::group::ColliderGroup;
use collisions::{collider_group, PrimaryCollider};
use maths::Vector3;

const WALLS: &[u8] = include_bytes!("./models/Walls.stl");
const DOOR: &[u8] = include_bytes!("./models/Door.stl");
const POLE_PIECE: &[u8] = include_bytes!("./models/Pole Piece.stl");

const POLE_PIECE_OFFSET: Vector3 = Vector3::new(0.0, 0.0, 60.0e-3);
// const DOOR_HINGE: Vector3 = Vector3::new(-125e-3, -100e-3, 0.0);
// static DOOR_OPEN_ROTATION: LazyLock<Quaternion> =
//     LazyLock::new(|| Quaternion::from_euler(&Vector3::new(0.0, 0.0, -90_f64.to_radians())));

pub struct ThesisChamber {
    walls: PrimaryCollider,
    pole_piece: PrimaryCollider,
    door: PrimaryCollider,
}

impl Chamber for ThesisChamber {
    fn full(&self) -> ColliderGroup<PrimaryCollider> {
        collider_group!(
            self.walls.clone(),
            self.pole_piece.clone(),
            self.door.clone()
        )
    }

    fn without_walls(&self) -> ColliderGroup<PrimaryCollider> {
        collider_group!(self.pole_piece.clone(), self.door.clone())
    }

    fn only_walls(&self) -> ColliderGroup<PrimaryCollider> {
        collider_group!(self.walls.clone())
    }
}

impl Default for ThesisChamber {
    fn default() -> Self {
        let walls = load_stl_from_bytes(WALLS).unwrap();
        let walls = PrimaryCollider::build(&walls);

        let pole_piece = load_stl_from_bytes(POLE_PIECE).unwrap();
        let pole_piece = PrimaryCollider::build(&pole_piece).translate(&POLE_PIECE_OFFSET);

        let door = load_stl_from_bytes(DOOR).unwrap();
        let door = PrimaryCollider::build(&door);

        Self {
            walls,
            pole_piece,
            door,
        }
    }
}
