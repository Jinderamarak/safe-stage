//! # Collisioner
//! Library for collision detection and path planning

pub mod common;
pub mod complex;
pub mod primitive;

//  BvhSphere is the primary collider as it appears to be the most performant
pub use complex::BvhSphere as PrimaryCollider;
