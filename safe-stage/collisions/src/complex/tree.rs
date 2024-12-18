use std::collections::VecDeque;
use std::fmt::Debug;

/// Regular recursive binary tree using [Box] for nodes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum RecursiveTree<K, V> {
    Branch(K, Box<RecursiveTree<K, V>>, Box<RecursiveTree<K, V>>),
    Leaf(K, V),
}

impl<K, V> RecursiveTree<K, V> {
    /// Concatenates two trees into one.
    pub fn concat(self, other: RecursiveTree<K, V>, key: K) -> RecursiveTree<K, V> {
        RecursiveTree::Branch(key, Box::new(self), Box::new(other))
    }

    /// Returns the depth of the tree.
    pub fn depth(&self) -> usize {
        match self {
            RecursiveTree::Leaf(_, _) => 0,
            RecursiveTree::Branch(_, left, right) => left.depth().max(right.depth()) + 1,
        }
    }

    #[inline]
    pub const fn key(&self) -> &K {
        match self {
            RecursiveTree::Leaf(key, _) => key,
            RecursiveTree::Branch(key, _, _) => key,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct LinearTreePtr(usize);

pub enum LinearTreeNode<'a, V> {
    Branch(LinearTreePtr, LinearTreePtr),
    Value(&'a V),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct LinearNode<K, V>(pub K, pub Option<V>);

/// Implementation of binary tree with nodes in single vector.
/// Traversing is slower than [RecursiveTree] but operations
/// can be applied by iterating over the inner vector.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct LinearTree<K, V>(Vec<Option<LinearNode<K, V>>>);

impl<K, V> LinearTree<K, V> {
    #[inline]
    /// This function expects data in specific order and can create invalid tree otherwise.
    pub(crate) fn from_raw(raw: Vec<Option<LinearNode<K, V>>>) -> Self {
        LinearTree(raw)
    }

    #[inline]
    pub fn raw(&self) -> &[Option<LinearNode<K, V>>] {
        &self.0
    }

    #[inline]
    pub fn root(&self) -> LinearTreePtr {
        LinearTreePtr(0)
    }

    #[inline]
    pub fn key(&self, LinearTreePtr(ptr): &LinearTreePtr) -> &'_ K {
        &self.0[*ptr].as_ref().expect("invalid tree pointer").0
    }

    pub fn value(&self, LinearTreePtr(ptr): &LinearTreePtr) -> LinearTreeNode<'_, V> {
        if let Some(value) = self.0[*ptr]
            .as_ref()
            .expect("invalid tree pointer")
            .1
            .as_ref()
        {
            LinearTreeNode::Value(value)
        } else {
            let left = 2 * ptr + 1;
            let right = 2 * ptr + 2;
            LinearTreeNode::Branch(LinearTreePtr(left), LinearTreePtr(right))
        }
    }

    #[inline]
    pub fn depth(&self) -> usize {
        ((self.0.len() + 1) as f64).log2().ceil() as usize - 1
    }

    pub fn concat(&self, other: &LinearTree<K, V>, key: K) -> LinearTree<K, V>
    where
        K: Clone,
        V: Clone,
    {
        let depth = self.depth().max(other.depth()) + 1;
        let size = 2_usize.pow(depth as u32 + 1) - 1;

        let mut nodes: Vec<Option<LinearNode<K, V>>> = (0..size).map(|_| None).collect();
        nodes.shrink_to_fit();

        nodes[0] = Some(LinearNode(key, None));

        let mut ptr = 1;
        let mut tree = 0;
        for d in 0..=depth {
            let width = 2_usize.pow(d as u32);

            if tree + width <= self.0.len() {
                nodes[ptr..(ptr + width)].clone_from_slice(&self.0[tree..(tree + width)]);
            }
            ptr += width;

            if tree + width <= other.0.len() {
                nodes[ptr..(ptr + width)].clone_from_slice(&other.0[tree..(tree + width)]);
            }
            ptr += width;

            tree += width;
        }

        LinearTree(nodes)
    }
}

impl<K, V> From<RecursiveTree<K, V>> for LinearTree<K, V> {
    fn from(tree: RecursiveTree<K, V>) -> Self {
        let size = 2_usize.pow(tree.depth() as u32 + 1) - 1;
        let mut nodes: Vec<Option<LinearNode<K, V>>> = (0..size).map(|_| None).collect();
        nodes.shrink_to_fit();

        let mut queue = VecDeque::new();

        queue.push_back((Box::new(tree), 0));
        while let Some((next, position)) = queue.pop_front() {
            match *next {
                RecursiveTree::Branch(key, left, right) => {
                    nodes[position] = Some(LinearNode(key, None));
                    queue.push_back((left, 2 * position + 1));
                    queue.push_back((right, 2 * position + 2));
                }
                RecursiveTree::Leaf(key, value) => {
                    nodes[position] = Some(LinearNode(key, Some(value)))
                }
            }
        }

        LinearTree(nodes)
    }
}

impl<K, V> From<&LinearTree<K, V>> for RecursiveTree<K, V>
where
    K: Clone,
    V: Clone,
{
    fn from(tree: &LinearTree<K, V>) -> Self {
        let root = tree.root();
        linear_subtree_to_recursive(tree, root)
    }
}

fn linear_subtree_to_recursive<K, V>(
    tree: &LinearTree<K, V>,
    ptr: LinearTreePtr,
) -> RecursiveTree<K, V>
where
    K: Clone,
    V: Clone,
{
    let key = tree.key(&ptr);
    match tree.value(&ptr) {
        LinearTreeNode::Value(value) => RecursiveTree::Leaf(key.clone(), value.clone()),
        LinearTreeNode::Branch(left, right) => RecursiveTree::Branch(
            key.clone(),
            Box::new(linear_subtree_to_recursive(tree, left)),
            Box::new(linear_subtree_to_recursive(tree, right)),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursive_to_linear() {
        let tree = RecursiveTree::Branch(
            1,
            Box::new(RecursiveTree::Branch(
                2,
                Box::new(RecursiveTree::Leaf(4, 44)),
                Box::new(RecursiveTree::Leaf(5, 55)),
            )),
            Box::new(RecursiveTree::Leaf(3, 33)),
        );

        let linear = LinearTree::from(tree);

        let root = linear.root();
        assert_eq!(1, *linear.key(&root));

        let LinearTreeNode::Branch(left, right) = linear.value(&root) else {
            panic!("expected branch, got value")
        };

        assert_eq!(2, *linear.key(&left));
        assert_eq!(3, *linear.key(&right));

        let LinearTreeNode::Value(leaf) = linear.value(&right) else {
            panic!("expected value, got branch");
        };
        assert_eq!(33, *leaf);

        let LinearTreeNode::Branch(left, right) = linear.value(&left) else {
            panic!("expected branch, got value");
        };

        assert_eq!(4, *linear.key(&left));
        assert_eq!(5, *linear.key(&right));

        let LinearTreeNode::Value(leaf) = linear.value(&left) else {
            panic!("expected value, got branch");
        };

        assert_eq!(44, *leaf);

        let LinearTreeNode::Value(leaf) = linear.value(&right) else {
            panic!("expected value, got branch");
        };

        assert_eq!(55, *leaf);
    }

    #[test]
    fn right_aligned_to_linear() {
        let tree = RecursiveTree::Branch(
            1,
            Box::new(RecursiveTree::Leaf(2, 22)),
            Box::new(RecursiveTree::Branch(
                3,
                Box::new(RecursiveTree::Leaf(4, 44)),
                Box::new(RecursiveTree::Leaf(5, 55)),
            )),
        );

        let linear = LinearTree::from(tree);
        assert_eq!(1, *linear.key(&linear.root()))
    }

    #[test]
    fn concat_recursive_equal_linear() {
        let recursive1 = RecursiveTree::Branch(
            1,
            Box::new(RecursiveTree::Leaf(2, 22)),
            Box::new(RecursiveTree::Leaf(3, 33)),
        );
        let linear1 = LinearTree::from(recursive1.clone());

        let recursive2 = RecursiveTree::Branch(
            4,
            Box::new(RecursiveTree::Leaf(5, 55)),
            Box::new(RecursiveTree::Leaf(6, 66)),
        );
        let linear2 = LinearTree::from(recursive2.clone());

        let recursive = recursive1.concat(recursive2, 7);
        let linear = linear1.concat(&linear2, 7);

        assert_eq!(recursive, RecursiveTree::from(&linear));
    }

    #[test]
    fn concat_unbalanced_recursive_equal_linear() {
        let recursive1 = RecursiveTree::Branch(
            1,
            Box::new(RecursiveTree::Leaf(2, 22)),
            Box::new(RecursiveTree::Leaf(3, 33)),
        );
        let linear1 = LinearTree::from(recursive1.clone());

        let recursive2 = RecursiveTree::Branch(
            4,
            Box::new(RecursiveTree::Leaf(5, 55)),
            Box::new(RecursiveTree::Branch(
                6,
                Box::new(RecursiveTree::Leaf(7, 77)),
                Box::new(RecursiveTree::Leaf(8, 88)),
            )),
        );
        let linear2 = LinearTree::from(recursive2.clone());

        let recursive = recursive1.concat(recursive2, 9);
        let linear = linear1.concat(&linear2, 9);

        assert_eq!(recursive, RecursiveTree::from(&linear));
    }
}
