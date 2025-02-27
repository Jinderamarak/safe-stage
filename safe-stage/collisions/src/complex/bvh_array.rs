use crate::common::{Bounded, Collides, Rotation, Transformation, Translation, Treeable};
use crate::primitive::{AlignedBoxCollider, TriangleCollider};
use itertools::Itertools;
use maths::{Axis, Quaternion, Vector3};
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
struct BoxIdx(usize);
#[derive(Debug, Clone, Copy)]
struct TriIdx(usize);
#[derive(Debug, Clone, Copy)]
struct NodeIdx(usize);

#[derive(Clone)]
enum BvhNode {
    Branch(BoxIdx, NodeIdx, NodeIdx),
    Leaf(BoxIdx, TriIdx),
}

impl BvhNode {
    #[inline]
    fn get_box_idx(&self) -> &BoxIdx {
        match self {
            BvhNode::Branch(idx, ..) => idx,
            BvhNode::Leaf(idx, ..) => idx,
        }
    }
}

pub struct BvhTreeArr {
    root: NodeIdx,
    nodes: Arc<[BvhNode]>,
    boxes: Arc<[AlignedBoxCollider]>,
    triangles: Arc<[TriangleCollider]>,
}

impl BvhTreeArr {
    pub fn build(triangles: &[TriangleCollider]) -> Self {
        let triangles = triangles
            .iter()
            .cloned()
            .collect::<Arc<[TriangleCollider]>>();
        let mut nodes = Vec::new();
        let mut boxes = Vec::new();
        let indexes = make_leaves(&triangles, &mut nodes, &mut boxes);
        let root = make_subtree(&mut nodes, &mut boxes, &indexes);

        BvhTreeArr {
            root: NodeIdx(root),
            nodes: Arc::from(nodes),
            boxes: Arc::from(boxes),
            triangles,
        }
    }

    pub fn concat(self, other: BvhTreeArr) -> Self {
        let node_offset = self.nodes.len();
        let box_offset = self.boxes.len();
        let tri_offset = self.triangles.len();

        let self_box = &self.boxes[self.root.0];
        let other_box = &self.boxes[other.root.0];
        let bounding = self_box.bound_children(other_box);

        let root = NodeIdx(self.nodes.len() + other.nodes.len());
        let nodes = self
            .nodes
            .iter()
            .cloned()
            .chain(other.nodes.iter().map(|n| match n {
                BvhNode::Branch(BoxIdx(bi), NodeIdx(li), NodeIdx(ri)) => BvhNode::Branch(
                    BoxIdx(*bi + box_offset),
                    NodeIdx(*li + node_offset),
                    NodeIdx(*ri + node_offset),
                ),
                BvhNode::Leaf(BoxIdx(bi), TriIdx(ti)) => {
                    BvhNode::Leaf(BoxIdx(*bi + box_offset), TriIdx(*ti + tri_offset))
                }
            }))
            .chain([BvhNode::Branch(
                BoxIdx(self.boxes.len() + other.boxes.len()),
                self.root,
                NodeIdx(self.root.0 + node_offset),
            )])
            .collect();

        let boxes = self
            .boxes
            .iter()
            .cloned()
            .chain(other.boxes.iter().cloned())
            .chain([bounding])
            .collect();

        let triangles = self
            .triangles
            .iter()
            .cloned()
            .chain(other.triangles.iter().cloned())
            .collect();

        BvhTreeArr {
            root,
            nodes,
            boxes,
            triangles,
        }
    }

    #[inline]
    pub fn triangle_buffer(&self) -> Vec<Vector3> {
        self.triangles
            .iter()
            .map(|t| {
                let (a, b, c) = t.points();
                [*a, *b, *c]
            })
            .flatten()
            .collect()
    }

    #[inline]
    fn get_node_box(&self, NodeIdx(node): NodeIdx) -> &BoxIdx {
        &self.nodes[node].get_box_idx()
    }

    fn collide_nodes(&self, other: &BvhTreeArr, self_idx: NodeIdx, other_idx: NodeIdx) -> bool {
        let self_node = &self.nodes[self_idx.0];
        let other_node = &other.nodes[other_idx.0];
        match (self_node, other_node) {
            (BvhNode::Leaf(_, TriIdx(self_tx)), BvhNode::Leaf(_, TriIdx(other_tx))) => {
                self.triangles[*self_tx].collides_with(&other.triangles[*other_tx])
            }
            (
                BvhNode::Leaf(BoxIdx(self_bx), _),
                BvhNode::Branch(BoxIdx(other_bx), other_lx, other_rx),
            ) => {
                self.boxes[*self_bx].collides_with(&other.boxes[*other_bx])
                    && (self.collide_nodes(other, self_idx, *other_lx)
                        || self.collide_nodes(other, self_idx, *other_rx))
            }
            (
                BvhNode::Branch(BoxIdx(self_bx), self_lx, self_rx),
                BvhNode::Leaf(BoxIdx(other_bx), _),
            ) => {
                self.boxes[*self_bx].collides_with(&other.boxes[*other_bx])
                    && (self.collide_nodes(other, *self_lx, other_idx)
                        || self.collide_nodes(other, *self_rx, other_idx))
            }
            (
                BvhNode::Branch(BoxIdx(self_bx), self_lx, self_rx),
                BvhNode::Branch(BoxIdx(other_bx), other_lx, other_rx),
            ) => {
                self.boxes[*self_bx].collides_with(&other.boxes[*other_bx])
                    && (self.collide_nodes(other, *self_lx, *other_lx)
                        || self.collide_nodes(other, *self_lx, *other_rx)
                        || self.collide_nodes(other, *self_rx, *other_lx)
                        || self.collide_nodes(other, *self_rx, *other_rx))
            }
        }
    }

    fn updated_boxes(
        &self,
        node_idx: NodeIdx,
        new_triangles: &[TriangleCollider],
        new_boxes: &mut [AlignedBoxCollider],
    ) -> &BoxIdx {
        let node = &self.nodes[node_idx.0];
        match node {
            BvhNode::Leaf(box_idx, tri_idx) => {
                new_boxes[box_idx.0] =
                    AlignedBoxCollider::bound_triangle(&new_triangles[tri_idx.0]);
                box_idx
            }
            BvhNode::Branch(box_idx, left_idx, right_idx) => {
                let left_bx = self.updated_boxes(*left_idx, new_triangles, new_boxes);
                let right_bx = self.updated_boxes(*right_idx, new_triangles, new_boxes);
                new_boxes[box_idx.0] = new_boxes[left_bx.0].bound_children(&new_boxes[right_bx.0]);
                box_idx
            }
        }
    }
}

impl Clone for BvhTreeArr {
    fn clone(&self) -> Self {
        BvhTreeArr {
            root: self.root,
            nodes: self.nodes.clone(),
            boxes: self.boxes.clone(),
            triangles: self.triangles.clone(),
        }
    }
}

impl Collides<Self> for BvhTreeArr {
    #[inline]
    fn collides_with(&self, other: &Self) -> bool {
        self.collide_nodes(other, self.root, other.root)
    }
}

impl Rotation for BvhTreeArr {
    #[inline]
    fn rotate(&self, rotation: &Quaternion) -> Self {
        let BoxIdx(box_idx) = self.get_node_box(self.root);
        let root_box = &self.boxes[*box_idx];
        self.rotate_around(rotation, &root_box.center())
    }

    fn rotate_around(&self, rotation: &Quaternion, pivot: &Vector3) -> Self {
        let triangles = self
            .triangles
            .iter()
            .map(|t| t.rotate_around(rotation, pivot))
            .collect::<Arc<[TriangleCollider]>>();

        let mut new_boxes = Box::<[AlignedBoxCollider]>::from(self.boxes.as_ref());
        let _ = self.updated_boxes(self.root, &triangles, &mut new_boxes);
        let boxes = Arc::<[AlignedBoxCollider]>::from(new_boxes);

        BvhTreeArr {
            root: self.root,
            nodes: self.nodes.clone(),
            boxes,
            triangles,
        }
    }
}

impl Translation for BvhTreeArr {
    fn translate(&self, translation: &Vector3) -> Self {
        let boxes = self
            .boxes
            .iter()
            .map(|b| b.translate(translation))
            .collect();

        let triangles = self
            .triangles
            .iter()
            .map(|t| t.translate(translation))
            .collect();

        BvhTreeArr {
            root: self.root,
            nodes: self.nodes.clone(),
            boxes,
            triangles,
        }
    }
}

impl Transformation for BvhTreeArr {
    fn transform(&self, rotation: &Quaternion, pivot: &Vector3, translation: &Vector3) -> Self {
        let triangles = self
            .triangles
            .iter()
            .map(|t| t.transform(rotation, pivot, translation))
            .collect::<Arc<[TriangleCollider]>>();

        let mut new_boxes = Box::<[AlignedBoxCollider]>::from(self.boxes.as_ref());
        let _ = self.updated_boxes(self.root, &triangles, &mut new_boxes);
        let boxes = Arc::<[AlignedBoxCollider]>::from(new_boxes);

        BvhTreeArr {
            root: self.root,
            nodes: self.nodes.clone(),
            boxes,
            triangles,
        }
    }
}

fn make_subtree(
    nodes: &mut Vec<BvhNode>,
    boxes: &mut Vec<AlignedBoxCollider>,
    indexes: &[usize],
) -> usize {
    if indexes.len() == 1 {
        return indexes[0];
    }

    let axis = longest_axis(&boxes, indexes);
    let (left, right) = split_by_axis(boxes, indexes, axis);

    let left_idx = make_subtree(nodes, boxes, &left);
    let right_idx = make_subtree(nodes, boxes, &right);

    let left = &boxes[nodes[left_idx].get_box_idx().0];
    let right = &boxes[nodes[right_idx].get_box_idx().0];

    let bounding = left.bound_children(right);
    boxes.push(bounding);

    nodes.push(BvhNode::Branch(
        BoxIdx(boxes.len() - 1),
        NodeIdx(left_idx),
        NodeIdx(right_idx),
    ));
    nodes.len() - 1
}

fn make_leaves(
    triangles: &[TriangleCollider],
    nodes: &mut Vec<BvhNode>,
    boxes: &mut Vec<AlignedBoxCollider>,
) -> Vec<usize> {
    let mut tri_indexes = Vec::with_capacity(triangles.len());
    for (idx, tri) in triangles.iter().enumerate() {
        boxes.push(AlignedBoxCollider::bound_triangle(tri));
        nodes.push(BvhNode::Leaf(BoxIdx(boxes.len() - 1), TriIdx(idx)));
        tri_indexes.push(idx);
    }

    tri_indexes
}

fn split_by_axis(
    boxes: &[AlignedBoxCollider],
    indexes: &[usize],
    axis: Axis,
) -> (Vec<usize>, Vec<usize>) {
    let mut ordered = indexes
        .iter()
        .map(|i| (*i, boxes[*i].center().get(axis)))
        .collect::<Vec<_>>();

    ordered.sort_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).expect("incomparable values"));

    let half = ordered.len().div_ceil(2);

    ordered
        .chunks(half)
        .map(|chunk| chunk.iter().map(|(i, _)| *i).collect::<Vec<_>>())
        .collect_tuple()
        .expect("half was not in fact half")
}

fn longest_axis(boxes: &[AlignedBoxCollider], indexes: &[usize]) -> Axis {
    let (min, max) = indexes
        .iter()
        .map(|i| boxes[*i].center())
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
