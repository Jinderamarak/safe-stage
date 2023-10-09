use crate::colliders::{AlignedBoxCollider, Bounded, Collider};
use crate::common::{Axis, Vector3};
use itertools::Itertools;

/// # Bounding Volume Hierarchy
/// A tree structure that allows for efficient collision detection.
#[derive(Debug, PartialEq)]
pub enum BvhTree {
    Branch(
        AlignedBoxCollider,
        Option<Box<BvhTree>>,
        Option<Box<BvhTree>>,
    ),
    Leaf(Collider),
}

impl BvhTree {
    pub fn build(objects: &mut [Collider]) -> Option<BvhTree> {
        if objects.is_empty() {
            return None;
        }

        if objects.len() == 1 {
            return Some(BvhTree::Leaf(objects[0].clone()));
        }

        let (min, max) = objects
            .iter()
            .map(collider_to_point)
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
            .map(|c| (c, collider_to_point(c).get(axis)))
            .collect::<Vec<(&Collider, f64)>>();

        ordered.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let mut ordered = ordered
            .into_iter()
            .map(|(c, _)| c.clone())
            .collect::<Vec<Collider>>();

        let half = ordered.len() / 2;

        let left = Self::build(&mut ordered[..half]);
        let right = Self::build(&mut ordered[half..]);

        let bounding = self::colliders_to_bounding_box(objects);

        Some(BvhTree::Branch(
            bounding,
            left.map(Box::new),
            right.map(Box::new),
        ))
    }
}

fn collider_to_point(c: &Collider) -> Vector3 {
    let min = c.min();
    let max = c.max();
    min + ((max - min) / 2.0)
}

fn colliders_to_bounding_box(colliders: &[Collider]) -> AlignedBoxCollider {
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

    AlignedBoxCollider::new(min, max - min)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colliders::{AlignedBoxCollider, PointCollider};

    #[test]
    fn test_build_bhv_correct() {
        let mut objects = vec![
            Collider::from(PointCollider::new(Vector3::new(1.0, 1.0, 1.0))),
            Collider::from(PointCollider::new(Vector3::new(2.0, 2.0, 2.0))),
            Collider::from(PointCollider::new(Vector3::new(3.0, 3.0, 3.0))),
            Collider::from(PointCollider::new(Vector3::new(4.0, 4.0, 4.0))),
        ];

        let tree = BvhTree::build(&mut objects[..]);

        assert!(tree.is_some());

        assert_eq!(
            tree.unwrap(),
            BvhTree::Branch(
                AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(3.0, 3.0, 3.0)),
                Some(Box::new(BvhTree::Branch(
                    AlignedBoxCollider::new(
                        Vector3::new(1.0, 1.0, 1.0),
                        Vector3::new(1.0, 1.0, 1.0)
                    ),
                    Some(Box::new(BvhTree::Leaf(Collider::from(PointCollider::new(
                        Vector3::new(1.0, 1.0, 1.0)
                    ))))),
                    Some(Box::new(BvhTree::Leaf(Collider::from(PointCollider::new(
                        Vector3::new(2.0, 2.0, 2.0)
                    ))))),
                ))),
                Some(Box::new(BvhTree::Branch(
                    AlignedBoxCollider::new(
                        Vector3::new(3.0, 3.0, 3.0),
                        Vector3::new(1.0, 1.0, 1.0)
                    ),
                    Some(Box::new(BvhTree::Leaf(Collider::from(PointCollider::new(
                        Vector3::new(3.0, 3.0, 3.0)
                    ))))),
                    Some(Box::new(BvhTree::Leaf(Collider::from(PointCollider::new(
                        Vector3::new(4.0, 4.0, 4.0)
                    ))))),
                ))),
            )
        );
    }
}
