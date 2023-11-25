//! # Common functionality
//! Common traits for functionality that is shared between colliders.

mod bounded;
mod collides;
mod projectable;
mod rotation;

pub use bounded::Bounded;
pub use collides::Collides;
pub use projectable::Projectable;
pub use rotation::Rotation;
