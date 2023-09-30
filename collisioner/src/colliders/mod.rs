//! # Colliders
//! Module containing all the primitives for collisions

mod aligned;
pub use aligned::{AlignedBox, AlignedBoxCollision};

mod point;
pub use point::PointCollision;
