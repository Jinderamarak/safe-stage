use crate::immovable::Immovable;
use crate::loader::load_stl_from_bytes;
use crate::movable::Movable;
use crate::parts::holder::Holder;
use crate::parts::stage::Stage;
use crate::position::sixaxis::SixAxis;
use collisions::common::{Rotation, Transformation};
use collisions::complex::group::ColliderGroup;
use collisions::{collider_group, PrimaryCollider};
use maths::{Quaternion, Vector3};
use std::sync::{Arc, LazyLock};

const BASE: &[u8] = include_bytes!("./models/Base.stl");
const TILTER: &[u8] = include_bytes!("./models/Tilter.stl");

const ROTATION_PIVOT: Vector3 = Vector3::new(0.0, 0.0, 12.5e-3);
static BASE_MODEL_ROTATION: LazyLock<Quaternion> =
    LazyLock::new(|| Quaternion::from_euler(&Vector3::new(0.0, 0.0, 90_f64.to_radians())));

// Add small offset to make 0/0/0 not collide with the chamber
const STAGE_POSITION: Vector3 = Vector3::new(0.0, 0.0, -62.5e-3 + 1e-12);

pub struct ThesisStage {
    base: PrimaryCollider,
    tilter: PrimaryCollider,
    holder: Option<Box<dyn Holder>>,
}

impl Clone for ThesisStage {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            tilter: self.tilter.clone(),
            holder: self.holder.as_ref().map(|h| h.cloned()),
        }
    }
}

impl Stage for ThesisStage {
    fn as_movable(&self) -> Arc<dyn Movable<SixAxis>> {
        Arc::new(self.clone())
    }
    fn swap_holder(&mut self, holder: Option<Box<dyn Holder>>) {
        self.holder = holder;
    }

    fn active_holder(&self) -> Option<&dyn Holder> {
        self.holder.as_deref()
    }

    fn active_holder_mut(&mut self) -> Option<&mut (dyn Holder + 'static)> {
        self.holder.as_deref_mut()
    }
}

impl Default for ThesisStage {
    fn default() -> Self {
        let base = PrimaryCollider::build(&load_stl_from_bytes(BASE).unwrap())
            .rotate_around(&BASE_MODEL_ROTATION, &Vector3::ZERO);
        let tilter = PrimaryCollider::build(&load_stl_from_bytes(TILTER).unwrap())
            .rotate_around(&BASE_MODEL_ROTATION, &Vector3::ZERO);
        ThesisStage {
            base,
            tilter,
            holder: None,
        }
    }
}

impl Movable<SixAxis> for ThesisStage {
    fn move_to(&self, coords: &SixAxis) -> Immovable {
        let offset = coords.pos + STAGE_POSITION;
        let tilt = Quaternion::from_euler(&Vector3::new(0.0, coords.rot.y(), 0.0));
        let rotation = Quaternion::from_euler(&Vector3::new(0.0, 0.0, coords.rot.z()));

        let base = self
            .base
            .transform(&Quaternion::IDENTITY, &Vector3::ZERO, &offset);
        let tilter = self.tilter.transform(&(tilt), &ROTATION_PIVOT, &offset);

        if let Some(holder) = self.holder.as_ref() {
            let holder = ColliderGroup(
                holder
                    .collider()
                    .0
                    .into_iter()
                    .map(|i| i.transform(&(tilt * rotation), &ROTATION_PIVOT, &offset))
                    .collect(),
            );

            collider_group!(base, tilter).extended(holder)
        } else {
            collider_group!(base, tilter)
        }
    }
}
