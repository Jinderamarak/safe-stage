use crate::colliders::{Bounded, Collider, Collides};
use crate::common::{Axis, Vector3};
use itertools::Itertools;

/// # Bounding Volume Hierarchy
/// A tree structure that allows for efficient collision detection.
#[derive(Debug, PartialEq)]
pub enum BvhTree {
    Branch(Collider, Option<Box<BvhTree>>, Option<Box<BvhTree>>),
    Leaf(Collider),
}

impl BvhTree {
    pub fn build(objects: &[Collider]) -> Option<BvhTree> {
        if objects.is_empty() {
            return None;
        }

        if objects.len() == 1 {
            return Some(BvhTree::Leaf(objects[0].clone()));
        }

        let axis = Self::longest_axis(objects);
        let ordered = Self::order_by_axis(objects, axis);

        let half = ordered.len() / 2;

        let left = Self::build(&ordered[..half]);
        let right = Self::build(&ordered[half..]);

        let bounding = Self::colliders_to_bounding(objects);

        Some(BvhTree::Branch(
            bounding,
            left.map(Box::new),
            right.map(Box::new),
        ))
    }

    fn longest_axis(colliders: &[Collider]) -> Axis {
        let (min, max) = colliders
            .iter()
            .map(Self::collider_to_point)
            .minmax_by(|a, b| a.partial_cmp(b).unwrap())
            .into_option()
            .unwrap();

        let diff = max - min;
        if diff.x() >= diff.y() && diff.x() >= diff.z() {
            Axis::X
        } else if diff.y() >= diff.x() && diff.y() >= diff.z() {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    fn order_by_axis(colliders: &[Collider], axis: Axis) -> Vec<Collider> {
        let mut ordered = colliders
            .iter()
            .map(|c| (c, Self::collider_to_point(c).get(axis)))
            .collect::<Vec<(&Collider, f64)>>();

        ordered.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        ordered
            .into_iter()
            .map(|(c, _)| c.clone())
            .collect::<Vec<Collider>>()
    }

    fn collider_to_point(c: &Collider) -> Vector3 {
        let min = c.min();
        let max = c.max();
        min + ((max - min) / 2.0)
    }

    fn colliders_to_bounding(colliders: &[Collider]) -> Collider {
        let min = colliders
            .iter()
            .map(|c| c.min())
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let max = colliders
            .iter()
            .map(|c| c.max())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let size = max - min;
        let pos = min + size / 2.0;
        Collider::aligned_box(pos.x(), pos.y(), pos.z(), size.x(), size.y(), size.z())
    }

    fn nodes_collide(a: &Option<Box<BvhTree>>, b: &Option<Box<BvhTree>>) -> bool {
        match (a, b) {
            (Some(a), Some(b)) => a.collides_with(b),
            _ => false,
        }
    }
}

impl Collides<Self> for BvhTree {
    fn collides_with(&self, other: &BvhTree) -> bool {
        match (self, other) {
            (Self::Leaf(c1), Self::Leaf(c2)) => c1.collides_with(c2),
            (leaf @ Self::Leaf(c1), Self::Branch(c2, left, right))
            | (Self::Branch(c1, left, right), leaf @ Self::Leaf(c2)) => {
                c1.collides_with(c2)
                    && (left
                        .as_ref()
                        .map(|l| l.collides_with(leaf))
                        .unwrap_or(false)
                        || right
                            .as_ref()
                            .map(|r| r.collides_with(leaf))
                            .unwrap_or(false))
            }
            (Self::Branch(c1, left1, right1), Self::Branch(c2, left2, right2)) => {
                c1.collides_with(c2)
                    && (Self::nodes_collide(left1, left2)
                        || Self::nodes_collide(left1, right2)
                        || Self::nodes_collide(right1, left2)
                        || Self::nodes_collide(right1, right2))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn cbox(x: f64, y: f64, z: f64, w: f64, h: f64, d: f64) -> Collider {
        Collider::aligned_box(x, y, z, w, h, d)
    }

    fn cpoint(x: f64, y: f64, z: f64) -> Collider {
        Collider::point(x, y, z)
    }

    #[test]
    fn build_bhv_correct() {
        let objects = vec![
            cpoint(1.0, 1.0, 1.0),
            cpoint(2.0, 2.0, 2.0),
            cpoint(3.0, 3.0, 3.0),
            cpoint(4.0, 4.0, 4.0),
        ];

        let tree = BvhTree::build(&objects[..]);

        assert!(tree.is_some());
        assert_eq!(
            BvhTree::Branch(
                cbox(2.5, 2.5, 2.5, 3.0, 3.0, 3.0),
                Some(Box::new(BvhTree::Branch(
                    cbox(1.5, 1.5, 1.5, 1.0, 1.0, 1.0),
                    Some(Box::new(BvhTree::Leaf(cpoint(1.0, 1.0, 1.0)))),
                    Some(Box::new(BvhTree::Leaf(cpoint(2.0, 2.0, 2.0)))),
                ))),
                Some(Box::new(BvhTree::Branch(
                    cbox(3.5, 3.5, 3.5, 1.0, 1.0, 1.0),
                    Some(Box::new(BvhTree::Leaf(cpoint(3.0, 3.0, 3.0)))),
                    Some(Box::new(BvhTree::Leaf(cpoint(4.0, 4.0, 4.0)))),
                ))),
            ),
            tree.unwrap()
        );
    }

    #[test]
    fn build_bhv_complex() {
        let mut objects = vec![
            cbox(0.0, 0.0, 0.0, 1.0, 1.0, 1.0),
            cbox(2.0, 0.0, 0.0, 1.0, 1.0, 1.0),
            cbox(0.0, 2.0, 0.0, 1.0, 1.0, 1.0),
            cbox(2.0, 2.0, 0.0, 1.0, 1.0, 1.0),
            cbox(0.0, 0.0, 2.0, 1.0, 1.0, 1.0),
            cbox(2.0, 0.0, 2.0, 1.0, 1.0, 1.0),
            cbox(0.0, 2.0, 2.0, 1.0, 1.0, 1.0),
            cbox(2.0, 2.0, 2.0, 1.0, 1.0, 1.0),
        ];

        let tree = BvhTree::build(&mut objects[..]);

        assert!(tree.is_some());
        assert_eq!(
            BvhTree::Branch(
                cbox(1.0, 1.0, 1.0, 3.0, 3.0, 3.0),
                Some(Box::new(BvhTree::Branch(
                    cbox(0.0, 1.0, 1.0, 1.0, 3.0, 3.0),
                    Some(Box::new(BvhTree::Branch(
                        cbox(0.0, 0.0, 1.0, 1.0, 1.0, 3.0),
                        Some(Box::new(BvhTree::Leaf(cbox(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)))),
                        Some(Box::new(BvhTree::Leaf(cbox(0.0, 0.0, 2.0, 1.0, 1.0, 1.0)))),
                    ))),
                    Some(Box::new(BvhTree::Branch(
                        cbox(0.0, 2.0, 1.0, 1.0, 1.0, 3.0),
                        Some(Box::new(BvhTree::Leaf(cbox(0.0, 2.0, 0.0, 1.0, 1.0, 1.0)))),
                        Some(Box::new(BvhTree::Leaf(cbox(0.0, 2.0, 2.0, 1.0, 1.0, 1.0)))),
                    ))),
                ))),
                Some(Box::new(BvhTree::Branch(
                    cbox(2.0, 1.0, 1.0, 1.0, 3.0, 3.0),
                    Some(Box::new(BvhTree::Branch(
                        cbox(2.0, 0.0, 1.0, 1.0, 1.0, 3.0),
                        Some(Box::new(BvhTree::Leaf(cbox(2.0, 0.0, 0.0, 1.0, 1.0, 1.0)))),
                        Some(Box::new(BvhTree::Leaf(cbox(2.0, 0.0, 2.0, 1.0, 1.0, 1.0)))),
                    ))),
                    Some(Box::new(BvhTree::Branch(
                        cbox(2.0, 2.0, 1.0, 1.0, 1.0, 3.0),
                        Some(Box::new(BvhTree::Leaf(cbox(2.0, 2.0, 0.0, 1.0, 1.0, 1.0)))),
                        Some(Box::new(BvhTree::Leaf(cbox(2.0, 2.0, 2.0, 1.0, 1.0, 1.0)))),
                    ))),
                ))),
            ),
            tree.unwrap()
        );
    }

    #[test]
    fn longest_axis() {
        let objects = vec![
            cpoint(1.0, 1.0, 1.0),
            cpoint(2.0, 2.0, 2.0),
            cpoint(3.0, 3.0, 3.0),
            cpoint(4.0, 4.0, 4.0),
        ];

        assert_eq!(Axis::X, BvhTree::longest_axis(&objects[..]));
    }

    #[test]
    fn order_by_axis() {
        let objects = vec![
            cpoint(1.0, 1.0, 1.0),
            cpoint(2.0, 2.0, 2.0),
            cpoint(3.0, 3.0, 3.0),
            cpoint(4.0, 4.0, 4.0),
        ];

        let ordered = BvhTree::order_by_axis(&objects[..], Axis::X);

        assert_eq!(objects[0], ordered[0]);
        assert_eq!(objects[1], ordered[1]);
        assert_eq!(objects[2], ordered[2]);
        assert_eq!(objects[3], ordered[3]);
    }

    #[test]
    fn collider_to_point() {
        let c = cbox(1.0, 1.0, 1.0, 2.0, 2.0, 2.0);
        let p = BvhTree::collider_to_point(&c);

        assert_eq!(Vector3::new(1.0, 1.0, 1.0), p);
    }

    #[test]
    fn colliders_to_bounding() {
        let colliders = vec![
            cbox(1.0, 1.0, 1.0, 2.0, 2.0, 2.0),
            cbox(2.0, 2.0, 2.0, 3.0, 3.0, 3.0),
            cbox(3.0, 3.0, 3.0, 4.0, 4.0, 4.0),
            cbox(4.0, 4.0, 4.0, 5.0, 5.0, 5.0),
        ];

        let bounding = BvhTree::colliders_to_bounding(&colliders[..]);

        assert_eq!(cbox(3.25, 3.25, 3.25, 6.5, 6.5, 6.5), bounding);
    }

    #[test]
    fn nodes_collide_some() {
        let a = Some(Box::new(BvhTree::Leaf(cpoint(1.0, 1.0, 1.0))));
        let b = Some(Box::new(BvhTree::Leaf(cpoint(1.0, 1.0, 1.0))));

        assert!(BvhTree::nodes_collide(&a, &b));
    }

    #[test]
    fn nodes_collide_some_none() {
        let a = Some(Box::new(BvhTree::Leaf(cpoint(1.0, 1.0, 1.0))));
        let b = None;

        assert!(!BvhTree::nodes_collide(&a, &b));
    }

    #[test]
    fn leafs_collide() {
        let a = BvhTree::Leaf(cpoint(1.0, 1.0, 1.0));
        let b = BvhTree::Leaf(cpoint(1.0, 1.0, 1.0));

        assert!(a.collides_with(&b));
    }

    #[test]
    fn branches_collide() {
        let leaf_a = BvhTree::Leaf(cpoint(1.0, 1.0, 1.0));
        let branch_a = BvhTree::Branch(
            cbox(1.0, 1.0, 1.0, 1.0, 1.0, 1.0),
            None,
            Some(Box::new(leaf_a)),
        );
        let leaf_b = BvhTree::Leaf(cpoint(1.0, 1.0, 1.0));
        let branch_b = BvhTree::Branch(
            cbox(1.0, 1.0, 1.0, 1.0, 1.0, 1.0),
            Some(Box::new(leaf_b)),
            None,
        );

        assert!(branch_a.collides_with(&branch_b));
    }
}
