use crate::common::{Bounded, Collides, Rotation, Transformation, Translation, Treeable};
use crate::complex::tree::RecursiveTree;
use crate::primitive::TriangleCollider;
use itertools::Itertools;
use maths::{Axis, Quaternion, Vector3};
use std::sync::Arc;

type Tree<T> = RecursiveTree<T, TriangleCollider>;

/// # Recursive Volume Hierarchy of Triangles
/// A tree structure of triangle mesh with generic collider as bounding volumes.
///
/// Uses recursive binary tree for storing nodes.
///
/// When used with [AlignedBoxCollider], faster than other BVHs.
#[derive(Debug)]
pub struct BvhRecursive<T>(pub(crate) Arc<Tree<T>>);

impl<T> Clone for BvhRecursive<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> BvhRecursive<T>
where
    T: Clone + Treeable + Bounded,
{
    /// Builds a BVH from a list of triangles.
    ///
    /// **Expects a non-empty list of triangles.**
    pub fn build(triangles: &[TriangleCollider]) -> Self {
        assert!(!triangles.is_empty());

        let leaves = into_leaves(triangles);
        BvhRecursive(Arc::new(build_from_leaves(leaves)))
    }

    /// Concatenates two BVHs into one.
    pub fn concat(self, other: BvhRecursive<T>) -> BvhRecursive<T> {
        let bounding = self.0.key().bound_children(other.0.key());
        let left = RecursiveTree::clone(&self.0);
        let right = RecursiveTree::clone(&other.0);
        let node = left.concat(right, bounding);
        BvhRecursive(Arc::new(node))
    }

    /// Returns a list of triangles in the BVH.
    pub fn triangle_buffer(&self) -> Vec<Vector3> {
        node_triangles(&self.0)
    }
}

fn node_triangles<T>(node: &Tree<T>) -> Vec<Vector3> {
    match node {
        RecursiveTree::Leaf(_, t) => {
            let (a, b, c) = t.points();
            vec![*a, *b, *c]
        }
        RecursiveTree::Branch(_, l, r) => {
            let mut triangles = node_triangles(l);
            triangles.extend(node_triangles(r));
            triangles
        }
    }
}

impl<T> Collides<Self> for BvhRecursive<T>
where
    T: Collides<T>,
{
    fn collides_with(&self, other: &Self) -> bool {
        collides_with_node(&self.0, &other.0)
    }
}

fn collides_with_node<T>(left: &Tree<T>, right: &Tree<T>) -> bool
where
    T: Collides<T>,
{
    match (left, right) {
        (RecursiveTree::Leaf(_, t1), RecursiveTree::Leaf(_, t2)) => t1.collides_with(t2),
        (leaf @ RecursiveTree::Leaf(s1, _), RecursiveTree::Branch(s2, l, r))
        | (RecursiveTree::Branch(s1, l, r), leaf @ RecursiveTree::Leaf(s2, _)) => {
            s1.collides_with(s2) && (collides_with_node(l, leaf) || collides_with_node(r, leaf))
        }
        (RecursiveTree::Branch(s1, l1, r1), RecursiveTree::Branch(s2, l2, r2)) => {
            s1.collides_with(s2)
                && (collides_with_node(l1, l2)
                    || collides_with_node(l1, r2)
                    || collides_with_node(r1, l2)
                    || collides_with_node(r1, r2))
        }
    }
}

impl<T> Rotation for BvhRecursive<T>
where
    T: Treeable + Bounded,
{
    fn rotate(&self, rotation: &Quaternion) -> Self {
        Self(Arc::new(rotate_around_node(
            &self.0,
            rotation,
            &self.0.key().center(),
        )))
    }

    fn rotate_around(&self, rotation: &Quaternion, pivot: &Vector3) -> Self {
        Self(Arc::new(rotate_around_node(&self.0, rotation, pivot)))
    }
}

fn rotate_around_node<T>(node: &Tree<T>, rotation: &Quaternion, pivot: &Vector3) -> Tree<T>
where
    T: Treeable,
{
    match node {
        RecursiveTree::Branch(_, l, r) => {
            let left = rotate_around_node(l, rotation, pivot);
            let right = rotate_around_node(r, rotation, pivot);
            let bounding = left.key().bound_children(right.key());
            RecursiveTree::Branch(bounding, Box::new(left), Box::new(right))
        }
        RecursiveTree::Leaf(_, t) => {
            let triangle = t.rotate_around(rotation, pivot);
            let bound = T::bound_triangle(&triangle);
            RecursiveTree::Leaf(bound, triangle)
        }
    }
}

impl<T> Translation for BvhRecursive<T>
where
    T: Translation,
{
    fn translate(&self, translation: &Vector3) -> Self {
        Self(Arc::new(translate_node(&self.0, translation)))
    }
}

fn translate_node<T>(node: &Tree<T>, translation: &Vector3) -> Tree<T>
where
    T: Translation,
{
    match node {
        RecursiveTree::Branch(s, l, r) => RecursiveTree::Branch(
            s.translate(translation),
            Box::new(translate_node(l, translation)),
            Box::new(translate_node(r, translation)),
        ),
        RecursiveTree::Leaf(s, t) => {
            RecursiveTree::Leaf(s.translate(translation), t.translate(translation))
        }
    }
}

impl<T> Transformation for BvhRecursive<T>
where
    T: Treeable,
{
    fn transform(&self, rotation: &Quaternion, pivot: &Vector3, translation: &Vector3) -> Self {
        Self(Arc::new(transform_node(
            &self.0,
            rotation,
            pivot,
            translation,
        )))
    }
}

fn transform_node<T>(
    node: &Tree<T>,
    rotation: &Quaternion,
    pivot: &Vector3,
    translation: &Vector3,
) -> Tree<T>
where
    T: Treeable,
{
    match node {
        RecursiveTree::Branch(_, l, r) => {
            let left = transform_node(l, rotation, pivot, translation);
            let right = transform_node(r, rotation, pivot, translation);
            let bounding = left.key().bound_children(right.key());
            RecursiveTree::Branch(bounding, Box::new(left), Box::new(right))
        }
        RecursiveTree::Leaf(_, t) => {
            let t = t.transform(rotation, pivot, translation);
            let s = T::bound_triangle(&t);
            RecursiveTree::Leaf(s, t)
        }
    }
}

fn build_from_leaves<T>(mut leaves: Vec<Tree<T>>) -> Tree<T>
where
    T: Treeable + Bounded + Clone,
{
    if leaves.len() == 1 {
        return leaves.pop().unwrap();
    }

    if leaves.len() == 2 {
        let left = leaves.pop().unwrap();
        let right = leaves.pop().unwrap();
        return RecursiveTree::Branch(
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

fn into_leaves<T>(triangles: &[TriangleCollider]) -> Vec<Tree<T>>
where
    T: Treeable,
{
    triangles
        .iter()
        .map(|t| RecursiveTree::Leaf(T::bound_triangle(t), t.clone()))
        .collect()
}

fn split_by_axis<T>(leaves: &[Tree<T>], axis: Axis) -> (Vec<Tree<T>>, Vec<Tree<T>>)
where
    T: Bounded + Clone,
{
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

fn longest_axis<T>(leaves: &[Tree<T>]) -> Axis
where
    T: Bounded,
{
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
