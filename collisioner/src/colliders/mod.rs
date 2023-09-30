/// Module for all the collider primitives
mod aligned;
pub use aligned::{AlignedBox, AlignedBoxCollision};

mod point;
pub use point::PointCollision;
