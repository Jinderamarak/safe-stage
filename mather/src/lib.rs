//! # Math library
//! Small collection of basic mather types and functions.

pub mod asserts;
mod axis;
mod quaternion;
mod vector;

pub use axis::Axis;
pub use quaternion::Quaternion;
pub use vector::Vector3;
