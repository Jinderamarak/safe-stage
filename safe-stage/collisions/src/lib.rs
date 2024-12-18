//! # Collisions
//! Implementations of traits relevant to colliders, primitive colliders and composite colliders.

use crate::primitive::AlignedBoxCollider;

pub mod common;
pub mod complex;
pub mod primitive;

///  BVH with AABB is currently the most performant implementation for this use-case.
pub type PrimaryCollider = complex::bvh_recursive::BvhRecursive<AlignedBoxCollider>;
