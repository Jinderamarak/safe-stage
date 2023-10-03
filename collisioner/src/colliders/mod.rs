//! # Colliders
//! Module containing all the primitives for collisions

mod collides;
pub use collides::Collides;

mod bounded;
pub use bounded::Bounded;

mod point;
pub use point::PointCollider;

mod aligned_box;
pub use aligned_box::AlignedBoxCollider;

mod collider;
pub use collider::Collider;
