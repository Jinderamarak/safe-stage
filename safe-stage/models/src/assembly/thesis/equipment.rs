use crate::loader::load_stl_from_bytes;
use crate::parts::equipment::Equipment;
use collisions::complex::group::ColliderGroup;
use collisions::{collider_group, PrimaryCollider};

const DETECTOR_ALPHA: &[u8] = include_bytes!("./models/Detector Alpha.stl");
const DETECTOR_BETA: &[u8] = include_bytes!("./models/Detector Beta.stl");

pub struct ThesisDetectorAlpha(PrimaryCollider);

impl Default for ThesisDetectorAlpha {
    fn default() -> Self {
        ThesisDetectorAlpha(PrimaryCollider::build(
            &load_stl_from_bytes(DETECTOR_ALPHA).unwrap(),
        ))
    }
}

impl Equipment for ThesisDetectorAlpha {
    fn collider(&self) -> ColliderGroup<PrimaryCollider> {
        collider_group!(self.0.clone())
    }
}

pub struct ThesisDetectorBeta(PrimaryCollider);

impl Default for ThesisDetectorBeta {
    fn default() -> Self {
        ThesisDetectorBeta(PrimaryCollider::build(
            &load_stl_from_bytes(DETECTOR_BETA).unwrap(),
        ))
    }
}

impl Equipment for ThesisDetectorBeta {
    fn collider(&self) -> ColliderGroup<PrimaryCollider> {
        collider_group!(self.0.clone())
    }
}
