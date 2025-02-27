use crate::collides_group_impl;
use crate::common::{Collides, Rotation, Transformation, Translation, Treeable};
use crate::complex::tree::RecursiveTree;
use crate::primitive::{SphereCollider, TriangleCollider};
use itertools::Itertools;
use maths::{Axis, Quaternion, Vector3};
use std::sync::Arc;

type Node = RecursiveTree<SphereCollider, TriangleCollider>;

/// # Recursive Spherical Volume Hierarchy of Triangles
/// A tree structure of triangle mesh with spheres as bounding volumes.
///
/// Uses recursive binary tree for storing nodes.
///
/// Has slower transformation, but faster collision detection than [BvhSphereLinear].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct BvhSphereRecursive(pub(crate) Arc<Node>);

impl Clone for BvhSphereRecursive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl BvhSphereRecursive {
    /// Builds a BVH from a list of triangles.
    ///
    /// **Expects a non-empty list of triangles.**
    pub fn build(triangles: &[TriangleCollider]) -> Self {
        assert!(!triangles.is_empty());

        let leaves = into_leaves(triangles);
        BvhSphereRecursive(Arc::new(build_from_leaves(leaves)))
    }

    /// Concatenates two BVHs into one.
    pub fn concat(self, other: BvhSphereRecursive) -> BvhSphereRecursive {
        let bounding = self.0.key().bound_children(other.0.key());
        let left = Node::clone(&self.0);
        let right = Node::clone(&other.0);
        let node = left.concat(right, bounding);
        BvhSphereRecursive(Arc::new(node))
    }

    /// Returns a list of triangles in the BVH.
    pub fn triangle_buffer(&self) -> Vec<Vector3> {
        node_triangles(&self.0)
    }
}

fn node_triangles(node: &Node) -> Vec<Vector3> {
    match node {
        Node::Leaf(_, t) => {
            let (a, b, c) = t.points();
            vec![*a, *b, *c]
        }
        Node::Branch(_, l, r) => {
            let mut triangles = node_triangles(l);
            triangles.extend(node_triangles(r));
            triangles
        }
    }
}

impl Collides<Self> for BvhSphereRecursive {
    fn collides_with(&self, other: &Self) -> bool {
        collides_with_node(&self.0, &other.0)
    }
}

collides_group_impl!(BvhSphereRecursive, BvhSphereRecursive);

fn collides_with_node(left: &Node, right: &Node) -> bool {
    match (left, right) {
        (Node::Leaf(_, t1), Node::Leaf(_, t2)) => t1.collides_with(t2),
        (leaf @ Node::Leaf(s1, _), Node::Branch(s2, l, r))
        | (Node::Branch(s1, l, r), leaf @ Node::Leaf(s2, _)) => {
            s1.collides_with(s2) && (collides_with_node(l, leaf) || collides_with_node(r, leaf))
        }
        (Node::Branch(s1, l1, r1), Node::Branch(s2, l2, r2)) => {
            s1.collides_with(s2)
                && (collides_with_node(l1, l2)
                    || collides_with_node(l1, r2)
                    || collides_with_node(r1, l2)
                    || collides_with_node(r1, r2))
        }
    }
}

impl Rotation for BvhSphereRecursive {
    fn rotate(&self, rotation: &Quaternion) -> Self {
        Self(Arc::new(rotate_node(&self.0, rotation)))
    }

    fn rotate_around(&self, rotation: &Quaternion, pivot: &Vector3) -> Self {
        Self(Arc::new(rotate_around_node(&self.0, rotation, pivot)))
    }
}

fn rotate_node(node: &Node, rotation: &Quaternion) -> Node {
    let pivot = node.key().center();
    match node {
        Node::Branch(s, l, r) => Node::Branch(
            s.clone(),
            Box::new(rotate_around_node(l, rotation, &pivot)),
            Box::new(rotate_around_node(r, rotation, &pivot)),
        ),
        Node::Leaf(s, t) => Node::Leaf(s.clone(), t.rotate_around(rotation, &pivot)),
    }
}

fn rotate_around_node(node: &Node, rotation: &Quaternion, pivot: &Vector3) -> Node {
    match node {
        Node::Branch(s, l, r) => Node::Branch(
            s.rotate_around(rotation, pivot),
            Box::new(rotate_around_node(l, rotation, pivot)),
            Box::new(rotate_around_node(r, rotation, pivot)),
        ),
        Node::Leaf(s, t) => Node::Leaf(
            s.rotate_around(rotation, pivot),
            t.rotate_around(rotation, pivot),
        ),
    }
}

impl Translation for BvhSphereRecursive {
    fn translate(&self, translation: &Vector3) -> Self {
        Self(Arc::new(translate_node(&self.0, translation)))
    }
}

fn translate_node(node: &Node, translation: &Vector3) -> Node {
    match node {
        Node::Branch(s, l, r) => Node::Branch(
            s.translate(translation),
            Box::new(translate_node(l, translation)),
            Box::new(translate_node(r, translation)),
        ),
        Node::Leaf(s, t) => Node::Leaf(s.translate(translation), t.translate(translation)),
    }
}

impl Transformation for BvhSphereRecursive {
    fn transform(&self, rotation: &Quaternion, pivot: &Vector3, translation: &Vector3) -> Self {
        Self(Arc::new(transform_node(
            &self.0,
            rotation,
            pivot,
            translation,
        )))
    }
}

fn transform_node(
    node: &Node,
    rotation: &Quaternion,
    pivot: &Vector3,
    translation: &Vector3,
) -> Node {
    match node {
        Node::Branch(s, l, r) => Node::Branch(
            s.transform(rotation, pivot, translation),
            Box::new(transform_node(l, rotation, pivot, translation)),
            Box::new(transform_node(r, rotation, pivot, translation)),
        ),
        Node::Leaf(s, t) => Node::Leaf(
            s.transform(rotation, pivot, translation),
            t.transform(rotation, pivot, translation),
        ),
    }
}

fn build_from_leaves(mut leaves: Vec<Node>) -> Node {
    if leaves.len() == 1 {
        return leaves.pop().unwrap();
    }

    if leaves.len() == 2 {
        let left = leaves.pop().unwrap();
        let right = leaves.pop().unwrap();
        return Node::Branch(
            left.key().bound_children(right.key()),
            Box::new(left),
            Box::new(right),
        );
    }

    let axis = longest_axis(&leaves);
    let (left, right) = split_by_axis(&leaves, axis);

    let left = build_from_leaves(left);
    let right = build_from_leaves(right);
    let bounding = left.key().bound_children(right.key());

    left.concat(right, bounding)
}

fn into_leaves(triangles: &[TriangleCollider]) -> Vec<Node> {
    triangles
        .iter()
        .map(|t| Node::Leaf(SphereCollider::bound_triangle(t), t.clone()))
        .collect()
}

fn split_by_axis(leaves: &[Node], axis: Axis) -> (Vec<Node>, Vec<Node>) {
    let mut ordered = leaves
        .iter()
        .map(|c| (c, c.key().center().get(axis)))
        .collect::<Vec<_>>();

    ordered.sort_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).expect("incomparable values"));

    let half = ordered.len().div_ceil(2);

    ordered
        .chunks(half)
        .map(|chunk| chunk.iter().map(|(c, _)| (*c).clone()).collect::<Vec<_>>())
        .collect_tuple()
        .expect("half was not in fact half")
}

fn longest_axis(leaves: &[Node]) -> Axis {
    let (min, max) = leaves
        .iter()
        .map(|l| l.key().center())
        .minmax_by(|a, b| a.partial_cmp(b).expect("incomparable values"))
        .into_option()
        .expect("provided no colliders");

    let diff = max - min;
    if diff.x() >= diff.y() && diff.x() >= diff.z() {
        Axis::X
    } else if diff.y() >= diff.z() {
        Axis::Y
    } else {
        Axis::Z
    }
}
