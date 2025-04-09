use crate::loader::load_stl_from_bytes;
use crate::movable::Movable;
use crate::parts::retract::Retract;
use crate::position::linear::LinearState;
use collisions::common::Translation;
use collisions::complex::group::ColliderGroup;
use collisions::{collider_group, PrimaryCollider};
use maths::Vector3;
use std::sync::Arc;

const ENTRY: &[u8] = include_bytes!("./models/Retract Entry.stl");
const ARM: &[u8] = include_bytes!("./models/Retract Arm.stl");

const INSERTED_POSITION: Vector3 = Vector3::new(0.0, 0.0, 0.0);
const RETRACTED_POSITION: Vector3 = Vector3::new(80e-3, 0.0, 0.0);

#[derive(Clone)]
pub struct ThesisRetract {
    entry: PrimaryCollider,
    inserted: PrimaryCollider,
    retracted: PrimaryCollider,
}

impl Retract for ThesisRetract {
    fn as_movable(&self) -> Arc<dyn Movable<LinearState> + Send + Sync> {
        Arc::new(self.clone())
    }
}

impl Default for ThesisRetract {
    fn default() -> Self {
        let entry = PrimaryCollider::build(&load_stl_from_bytes(ENTRY).unwrap());
        let arm = PrimaryCollider::build(&load_stl_from_bytes(ARM).unwrap());

        let inserted = arm.translate(&INSERTED_POSITION);
        let retracted = arm.translate(&RETRACTED_POSITION);

        ThesisRetract {
            entry,
            inserted,
            retracted,
        }
    }
}

impl Movable<LinearState> for ThesisRetract {
    fn move_to(&self, position: &LinearState) -> ColliderGroup<PrimaryCollider> {
        match position {
            LinearState::None => collider_group!(self.entry.clone(), self.retracted.clone()),
            LinearState::Full => collider_group!(self.entry.clone(), self.inserted.clone()),
            LinearState::Partial(p) => {
                let position = RETRACTED_POSITION.lerp(&INSERTED_POSITION, *p) - INSERTED_POSITION;
                let arm = self.inserted.translate(&position);
                collider_group!(self.entry.clone(), arm)
            }
        }
    }
}
