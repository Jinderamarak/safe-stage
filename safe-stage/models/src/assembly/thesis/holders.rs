use crate::loader::load_stl_from_bytes;
use crate::parts::holder::Holder;
use collisions::common::Translation;
use collisions::complex::group::ColliderGroup;
use collisions::{collider_group, PrimaryCollider};
use maths::Vector3;

const CIRCLE: &[u8] = include_bytes!("./models/Holder Circle.stl");
const CIRCLE_SAMPLE_OFFSET: Vector3 = Vector3::new(0.0, 0.0, 57.5e-3);

const SQUARE: &[u8] = include_bytes!("./models/Holder Square.stl");
const SQUARE_SAMPLE_OFFSET: Vector3 = Vector3::new(0.0, 0.0, 57.5e-3 - 5e-3);

macro_rules! thesis_holder_impl {
    ($name:ident, $source:ident, $offset:ident) => {
        #[derive(Clone)]
        pub struct $name {
            body: PrimaryCollider,
            sample: Option<PrimaryCollider>,
        }

        impl Default for $name {
            fn default() -> Self {
                let body = PrimaryCollider::build(&load_stl_from_bytes($source).unwrap());
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

thesis_holder_impl!(ThesisHolderCircle, CIRCLE, CIRCLE_SAMPLE_OFFSET);
thesis_holder_impl!(ThesisHolderSquare, SQUARE, SQUARE_SAMPLE_OFFSET);
