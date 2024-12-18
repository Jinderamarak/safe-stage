use std::{cmp::Ordering, fmt::Debug};

/// # Minimum Heap State
/// A state with a weight ordered by the weight in flipped order
/// so that the smallest weight is at the top of the heap.
#[derive(Debug, Clone, PartialEq)]
pub struct MinHeapState<W, T> {
    pub weight: W,
    pub state: T,
}

impl<W, T> Eq for MinHeapState<W, T>
where
    W: PartialEq,
    T: PartialEq,
{
}

impl<W, T> PartialOrd for MinHeapState<W, T>
where
    W: PartialOrd,
    T: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.weight.partial_cmp(&self.weight)
    }
}

impl<W, T> Ord for MinHeapState<W, T>
where
    W: PartialOrd + Debug,
    T: PartialEq + Debug,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.partial_cmp(&other.weight).unwrap_or_else(|| {
            panic!(
                "Expected HeapState weights to be orderable: {:?} and {:?}",
                self.weight, other.weight
            )
        })
    }
}
