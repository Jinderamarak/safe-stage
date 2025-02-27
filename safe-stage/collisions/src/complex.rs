//! # Complex collision structures
//! Composite data structures for accelerating collision detection.
//!
//! Differences:
//!  - [BvhTree] - boxes for bounding volumes, slower transformations, tighter bounds,
//!     any `Collider` composites, during rotation, AABBs are converted to OBBs
//!  - [BvhRecursive] - generic bounding volume hierarchy,
//!     recommended to use with [AlignedBoxCollider]
//!  - [BvhSphereLinear] - bounding sphere hierarchy, implemented with Vec as node storage,
//!     very fast transformations slower collision detection
//! - [BvhSphereRecursive] - bounding sphere hierarchy, variant of [BvhRecursive] but optimized
//!     for [SphereCollider] as the bounding shape

use crate::collides_group_impl;
use crate::complex::bvh_array::BvhTreeArr;
use crate::complex::bvh_recursive::BvhRecursive;
use crate::primitive::AlignedBoxCollider;

pub mod bvh;
pub mod bvh_array;
pub mod bvh_recursive;
pub mod bvh_sphere_linear;
pub mod bvh_sphere_recursive;
pub mod group;
pub(crate) mod tree;

collides_group_impl!(BvhTreeArr, BvhTreeArr);

collides_group_impl!(
    BvhRecursive<AlignedBoxCollider>,
    BvhRecursive<AlignedBoxCollider>
);
