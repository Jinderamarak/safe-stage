//! # Math Library
//! Small collection of basic math types and functions.
//! All functions operate with angles in radians.

pub mod asserts;
mod axis;
mod nan;
mod quaternion;
mod vector2;
mod vector3;

pub use axis::Axis;
pub use nan::NaNExtension;
pub use quaternion::Quaternion;
pub use vector2::Vector2;
pub use vector3::Vector3;
