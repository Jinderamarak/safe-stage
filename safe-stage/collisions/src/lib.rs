//! # Collisions
//! Implementations of traits relevant to colliders, primitive colliders and composite colliders.

pub mod common;
pub mod complex;
pub mod primitive;

///  BVH with AABB is currently the most performant implementation for this use-case.
// pub type PrimaryCollider = complex::bvh_recursive::BvhRecursive<primitive::AlignedBoxCollider>;
pub type PrimaryCollider = complex::bvh_array::BvhTreeArr;
