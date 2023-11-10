//! # Basic Colliders
//! Module containing all the primitives for collisions.
//! Prefer using `Collider` over the primitives.
//!
//! ## Implementation hierarchy for colliders
//! **Oriented Box** > **Aligned Box** > **Sphere** > **Point**
//! - hierarchy based on the collider complexity
//! - implementations are sorted based on this hierarchy in ascending order
//! - every collider implements collision detection for itself and for all colliders with same or lower complexity
//! - every collider takes collision detection for all colliders with higher complexity

mod aligned_box;
mod bounded;
mod collider;
mod collides;
mod oriented_box;
mod point;
mod projectable;
mod sphere;

pub use collider::Collider;

pub use aligned_box::AlignedBoxCollider;
pub use oriented_box::OrientedBoxCollider;
pub use point::PointCollider;
pub use sphere::SphereCollider;

pub use bounded::Bounded;
pub use collides::Collides;
pub use projectable::Projectable;
