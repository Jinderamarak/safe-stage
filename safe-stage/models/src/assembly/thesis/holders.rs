use crate::loader::load_stl_from_bytes;
use crate::parts::holder::Holder;
use collisions::common::{Rotation, Translation};
use collisions::complex::group::ColliderGroup;
use collisions::{collider_group, PrimaryCollider};
use maths::{Quaternion, Vector3};
use std::sync::LazyLock;

const CIRCLE: &[u8] = include_bytes!("./models/Holder Circle.stl");
static CIRCLE_MODEL_ROTATION: LazyLock<Quaternion> =
    LazyLock::new(|| Quaternion::from_euler(&Vector3::new(0.0, 0.0, 90_f64.to_radians())));
const CIRCLE_SAMPLE_OFFSET: Vector3 = Vector3::new(0.0, 0.0, 57.5e-3);

const SQUARE: &[u8] = include_bytes!("./models/Holder Square.stl");
static SQUARE_MODEL_ROTATION: LazyLock<Quaternion> =
    LazyLock::new(|| Quaternion::from_euler(&Vector3::new(0.0, 0.0, 90_f64.to_radians())));
const SQUARE_SAMPLE_OFFSET: Vector3 = Vector3::new(0.0, 0.0, 57.5e-3 - 5e-3);

macro_rules! thesis_holder_impl {
    ($name:ident, $source:ident, $rotation:ident, $offset:ident) => {
        #[derive(Clone)]
        pub struct $name {
            body: PrimaryCollider,
            sample: Option<PrimaryCollider>,
        }

        impl Default for $name {
            fn default() -> Self {
                let body = PrimaryCollider::build(&load_stl_from_bytes($source).unwrap())
                    .rotate_around(&$rotation, &Vector3::ZERO);
                let sample = None;
                Self { body, sample }
            }
        }

        impl Holder for $name {
            fn cloned(&self) -> Box<dyn Holder> {
                Box::new(self.clone())
            }

            fn collider(&self) -> ColliderGroup<PrimaryCollider> {
                if let Some(sample) = &self.sample {
                    collider_group!(self.body.clone(), sample.clone())
                } else {
                    collider_group!(self.body.clone())
                }
            }

            fn swap_sample(&mut self, sample: Option<PrimaryCollider>) {
                self.sample = sample.map(|s| s.translate(&$offset));
            }
        }
    };
}

thesis_holder_impl!(
    ThesisHolderCircle,
    CIRCLE,
    CIRCLE_MODEL_ROTATION,
    CIRCLE_SAMPLE_OFFSET
);
thesis_holder_impl!(
    ThesisHolderSquare,
    SQUARE,
    SQUARE_MODEL_ROTATION,
    SQUARE_SAMPLE_OFFSET
);
