use crate::collides_group_impl;
use crate::common::{Collides, Rotation, Transformation, Translation};
use crate::complex::tree::RecursiveTree;
use crate::primitive::{SphereCollider, TriangleCollider};
use itertools::Itertools;
use maths::{Axis, Quaternion, Vector3};
use std::sync::Arc;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

type Node = RecursiveTree<SphereCollider, TriangleCollider>;

/// # Recursive Spherical Volume Hierarchy of Triangles
/// A tree structure of triangle mesh with spheres as bounding volumes.
///
/// Uses recursive binary tree for storing nodes.
///
/// Currently faster than `BvhSphereLinear`.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct BvhSphereRecursive(pub(crate) Arc<Node>);

impl Clone for BvhSphereRecursive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl BvhSphereRecursive {
    pub fn build(triangles: &[TriangleCollider]) -> Self {
        assert!(!triangles.is_empty());

        let leaves = into_leaves(triangles);
        BvhSphereRecursive(Arc::new(build_from_leaves(leaves)))
    }

    pub fn concat(self, other: BvhSphereRecursive) -> BvhSphereRecursive {
        let bounding = bounding_sphere(the_sphere(&self.0), the_sphere(&other.0));
        let left = Node::clone(&self.0);
        let right = Node::clone(&other.0);
        let node = left.concat(right, bounding);
        BvhSphereRecursive(Arc::new(node))
    }

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
    let pivot = the_sphere(node).center();
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

#[inline]
fn the_sphere(node: &Node) -> &SphereCollider {
    match node {
        Node::Leaf(s, _) => s,
        Node::Branch(s, _, _) => s,
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
            bounding_sphere(the_sphere(&left), the_sphere(&right)),
            Box::new(left),
            Box::new(right),
        );
    }

    let axis = longest_axis(&leaves);
    let (left, right) = split_by_axis(&leaves, axis);

    let left = build_from_leaves(left);
    let right = build_from_leaves(right);
    let bounding = bounding_sphere(the_sphere(&left), the_sphere(&right));

    left.concat(right, bounding)
}

fn into_leaves(triangles: &[TriangleCollider]) -> Vec<Node> {
    triangles
        .iter()
        .map(|t| Node::Leaf(triangle_circumsphere(t), t.clone()))
        .collect()
}

/// https://gamedev.stackexchange.com/questions/60630/how-do-i-find-the-circumcenter-of-a-triangle-in-3d
fn triangle_circumsphere(t: &TriangleCollider) -> SphereCollider {
    let (a, b, c) = t.points();

    let ac = *c - *a;
    let ab = *b - *a;
    let ab_ac = ab.cross(&ac);

    let offset =
        (ab_ac.cross(&ab) * ac.len2() + ac.cross(&ab_ac) * ab.len2()) / (2.0 * ab_ac.len2());

    let center = *a + offset;
    let radius = offset.len();

    SphereCollider::new(center, radius)
}

fn split_by_axis(leaves: &[Node], axis: Axis) -> (Vec<Node>, Vec<Node>) {
    let mut ordered = leaves
        .iter()
        .map(|c| (c, the_sphere(c).center().get(axis)))
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
        .map(|l| the_sphere(l).center())
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

pub(super) fn bounding_sphere(a: &SphereCollider, b: &SphereCollider) -> SphereCollider {
    let (a_center, b_center) = (a.center(), b.center());
    let (a_radius, b_radius) = (a.radius(), b.radius());

    let ab = b_center - a_center;
    let dist = ab.len();

    if dist + a_radius <= b_radius {
        return b.clone();
    } else if dist + b_radius <= a_radius {
        return a.clone();
    }

    let radius = (dist + a_radius + b_radius) / 2.0;
    let center = a_center + ab * ((radius - a_radius) / dist);

    SphereCollider::new(center, radius)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;
    use maths::Vector3;

    #[test]
    fn triangle_circumcircle() {
        let triangle = TriangleCollider::new(
            Vector3::new(1.0, 3.0, 0.0),
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(3.0, 1.0, 0.0),
        );

        let result = triangle_circumsphere(&triangle);
        let expected_center = Vector3::new(2.0, 2.0, 0.0);
        let expected_radius = std::f64::consts::SQRT_2;

        assert_eq!(expected_center, result.center());
        assert_float_absolute_eq!(expected_radius, result.radius());
    }

    #[test]
    fn bounding_sphere_1d() {
        let left = SphereCollider::new(Vector3::new(2.0, 0.0, 0.0), 4.0);
        let right = SphereCollider::new(Vector3::new(-2.0, 0.0, 0.0), 2.0);

        let expected = SphereCollider::new(Vector3::new(1.0, 0.0, 0.0), 5.0);
        let actual = bounding_sphere(&left, &right);

        assert_eq!(expected, actual);
    }

    #[test]
    fn bounding_sphere_2d() {
        let left = SphereCollider::new(Vector3::new(4.0, 3.0, 7.0), 1.0);
        let right = SphereCollider::new(Vector3::new(-4.0, -3.0, 7.0), 1.0);

        let expected = SphereCollider::new(Vector3::new(0.0, 0.0, 7.0), 6.0);
        let actual = bounding_sphere(&left, &right);

        assert_eq!(expected, actual);
    }
}
