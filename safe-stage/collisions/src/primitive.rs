//! # Primitive colliders
//! Module containing all the primitives for collisions.
//!
//! The [TriangleCollider] is a standalone collider, compared to the rest.
//!
//! Prefer using [Collider] over the primitives.
//!
//! ## Implementation hierarchy for colliders
//! **[Oriented Box]** > **[Aligned Box]** > **[Sphere]** > **[Point]**
//! - hierarchy based on the collider complexity
//! - implementations are sorted based on this hierarchy in ascending order
//! - every collider implements collision detection for itself and for all colliders with same or lower complexity
//! - every collider takes collision detection for all colliders with higher complexity

mod algo;
mod aligned_box;
mod collider;
mod oriented_box;
mod point;
mod sphere;
mod triangle;

pub use collider::Collider;

pub use aligned_box::AlignedBoxCollider;
pub use oriented_box::OrientedBoxCollider;
pub use point::PointCollider;
pub use sphere::SphereCollider;
pub use triangle::TriangleCollider;
