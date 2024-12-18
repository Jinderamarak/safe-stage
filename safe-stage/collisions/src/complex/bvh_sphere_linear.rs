use crate::collides_group_impl;
use crate::common::{Collides, Rotation, Transformation, Translation, Treeable};
use crate::complex::bvh_recursive::BvhRecursive;
use crate::complex::tree::{LinearNode, LinearTree, LinearTreeNode, LinearTreePtr, RecursiveTree};
use crate::primitive::{SphereCollider, TriangleCollider};
use maths::{Quaternion, Vector3};
use std::fmt::Debug;

#[cfg(feature = "rayon-bvh-linear")]
use rayon::prelude::*;

/// # Linear Spherical Volume Hierarchy of Triangles
/// A tree structure of triangle mesh with spheres as bounding volumes.
///
/// Uses binary tree in linear memory for storing nodes.
///
/// Has faster transformation, but slower collision detection than [BvhSphereRecursive].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct BvhSphereLinear(pub(crate) LinearTree<SphereCollider, TriangleCollider>);

impl BvhSphereLinear {
    /// Builds a BVH from a list of triangles.
    ///
    /// **Expects a non-empty list of triangles.**
    pub fn build(triangles: &[TriangleCollider]) -> Self {
        let naive = BvhRecursive::build(triangles);
        let tree = RecursiveTree::clone(&naive.0);
        let linear = LinearTree::from(tree);
        BvhSphereLinear(linear)
    }

    /// Concatenates two BVHs into one.
    pub fn concat(self, other: BvhSphereLinear) -> BvhSphereLinear {
        let left = self.0.key(&self.0.root());
        let right = other.0.key(&other.0.root());
        let bounding = left.bound_children(right);
        BvhSphereLinear(self.0.concat(&other.0, bounding))
    }

    /// Returns a list of triangles in the BVH.
    pub fn triangle_buffer(&self) -> Vec<Vector3> {
        #[cfg(feature = "rayon-bvh-linear")]
        let data_iter = self.0.raw().par_iter();

        #[cfg(not(feature = "rayon-bvh-linear"))]
        let data_iter = self.0.raw().iter();

        data_iter
            .filter_map(|n| n.as_ref())
            .filter_map(|n| n.1.as_ref())
            .flat_map(|t| {
                let (a, b, c) = t.points();
                vec![*a, *b, *c]
            })
            .collect()
    }
}

impl Collides<Self> for BvhSphereLinear {
    fn collides_with(&self, other: &Self) -> bool {
        collides_with_node(&self.0, self.0.root(), &other.0, other.0.root())
    }
}

collides_group_impl!(BvhSphereLinear, BvhSphereLinear);

fn collides_with_node(
    left_tree: &LinearTree<SphereCollider, TriangleCollider>,
    left: LinearTreePtr,
    right_tree: &LinearTree<SphereCollider, TriangleCollider>,
    right: LinearTreePtr,
) -> bool {
    let s1 = left_tree.key(&left);
    let s2 = right_tree.key(&right);
    match (left_tree.value(&left), right_tree.value(&right)) {
        (LinearTreeNode::Value(t1), LinearTreeNode::Value(t2)) => t1.collides_with(t2),
        (LinearTreeNode::Branch(l1, r1), LinearTreeNode::Branch(l2, r2)) => {
            collides_with_node(left_tree, l1, right_tree, l2)
                || collides_with_node(left_tree, l1, right_tree, r2)
                || collides_with_node(left_tree, r1, right_tree, l2)
                || collides_with_node(left_tree, r1, right_tree, r2)
        }
        (LinearTreeNode::Value(_), LinearTreeNode::Branch(l, r)) => {
            s1.collides_with(s2)
                && (collides_with_node(left_tree, left, right_tree, l)
                    || collides_with_node(left_tree, left, right_tree, r))
        }
        (LinearTreeNode::Branch(l, r), LinearTreeNode::Value(_)) => {
            s1.collides_with(s2)
                && (collides_with_node(left_tree, l, right_tree, right)
                    || collides_with_node(left_tree, r, right_tree, right))
        }
    }
}

impl Rotation for BvhSphereLinear {
    fn rotate(&self, rotation: &Quaternion) -> Self {
        #[cfg(feature = "rayon-bvh-linear")]
        let data_iter = self.0.raw().par_iter();

        #[cfg(not(feature = "rayon-bvh-linear"))]
        let data_iter = self.0.raw().iter();

        let data = data_iter
            .map(|n| {
                n.as_ref().map(|n| {
                    LinearNode(
                        n.0.rotate(rotation),
                        n.1.as_ref().map(|v| v.rotate(rotation)),
                    )
                })
            })
            .collect::<Vec<_>>();

        BvhSphereLinear(LinearTree::from_raw(data))
    }
    fn rotate_around(&self, rotation: &Quaternion, pivot: &Vector3) -> Self {
        #[cfg(feature = "rayon-bvh-linear")]
        let data_iter = self.0.raw().par_iter();

        #[cfg(not(feature = "rayon-bvh-linear"))]
        let data_iter = self.0.raw().iter();

        let data = data_iter
            .map(|n| {
                n.as_ref().map(|n| {
                    LinearNode(
                        n.0.rotate_around(rotation, pivot),
                        n.1.as_ref().map(|v| v.rotate_around(rotation, pivot)),
                    )
                })
            })
            .collect::<Vec<_>>();

        BvhSphereLinear(LinearTree::from_raw(data))
    }
}

impl Translation for BvhSphereLinear {
    fn translate(&self, translation: &Vector3) -> Self {
        #[cfg(feature = "rayon-bvh-linear")]
        let data_iter = self.0.raw().par_iter();

        #[cfg(not(feature = "rayon-bvh-linear"))]
        let data_iter = self.0.raw().iter();

        let data = data_iter
            .map(|n| {
                n.as_ref().map(|n| {
                    LinearNode(
                        n.0.translate(translation),
                        n.1.as_ref().map(|v| v.translate(translation)),
                    )
                })
            })
            .collect::<Vec<_>>();

        BvhSphereLinear(LinearTree::from_raw(data))
    }
}

impl Transformation for BvhSphereLinear {
    fn transform(&self, rotation: &Quaternion, pivot: &Vector3, translation: &Vector3) -> Self {
        #[cfg(feature = "rayon-bvh-linear")]
        let data_iter = self.0.raw().par_iter();

        #[cfg(not(feature = "rayon-bvh-linear"))]
        let data_iter = self.0.raw().iter();

        let data = data_iter
            .map(|n| {
                n.as_ref().map(|n| {
                    LinearNode(
                        n.0.transform(rotation, pivot, translation),
                        n.1.as_ref()
                            .map(|v| v.transform(rotation, pivot, translation)),
                    )
                })
            })
            .collect::<Vec<_>>();

        BvhSphereLinear(LinearTree::from_raw(data))
    }
}
