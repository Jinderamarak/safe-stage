//! # Complex collision structures
//! Composite data structures for accelerating collision detection.
//! Differences:
//!  - [BvhTree] - boxes for bounding volumes, slower transformations, tighter bounds,
//!     any `Collider` composites
//!  - [BvhSphere] - spheres for bounding volumes, faster transformations, looser bounds,
//!     only triangular meshes

mod bvh;
pub mod bvh_sphere_linear;
pub mod bvh_sphere_recursive;
pub mod group;
pub(crate) mod tree;

pub use bvh::BvhTree;

#[cfg(all(feature = "bvh-linear", feature = "bvh-recursive"))]
compile_error!("features `bvh-linear` and `bvh-recursive` are mutually exclusive, pick preferred");

#[cfg(all(not(feature = "bvh-linear"), not(feature = "bvh-recursive")))]
compile_error!("one of features `bvh-linear` and `bvh-recursive` must be picked");

#[cfg(all(feature = "bvh-recursive", not(feature = "bvh-linear")))]
pub use bvh_sphere_recursive::BvhSphereRecursive as BvhSphere;

#[cfg(all(feature = "bvh-linear", not(feature = "bvh-recursive")))]
pub use bvh_sphere_linear::BvhSphereLinear as BvhSphere;
