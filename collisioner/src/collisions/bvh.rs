use crate::colliders::{Bounded, Collider};
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

        let (min, max) = objects
            .iter()
            .map(Self::collider_to_point)
            .minmax_by(|a, b| a.partial_cmp(b).unwrap())
            .into_option()
            .unwrap();

        let diff = max - min;
        let axis = if diff.x() >= diff.y() && diff.x() >= diff.z() {
            Axis::X
        } else if diff.y() >= diff.x() && diff.y() >= diff.z() {
            Axis::Y
        } else {
            Axis::Z
        };

        let mut ordered = objects
            .iter()
            .map(|c| (c, Self::collider_to_point(c).get(axis)))
            .collect::<Vec<(&Collider, f64)>>();

        ordered.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let ordered = ordered
            .into_iter()
            .map(|(c, _)| c.clone())
            .collect::<Vec<Collider>>();

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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

    fn cbox(x: f64, y: f64, z: f64, w: f64, h: f64, d: f64) -> Collider {
        Collider::aligned_box(x, y, z, w, h, d)
    }

    fn cpoint(x: f64, y: f64, z: f64) -> Collider {
        Collider::point(x, y, z)
    }
}
