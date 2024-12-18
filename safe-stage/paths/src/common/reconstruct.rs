use std::collections::HashMap;
use std::hash::Hash;

/// Reconstructs a path from a result of an A* like algorithms.
pub fn reconstruct_path<T>(came_from: &HashMap<T, T>, current: &T) -> Vec<T>
where
    T: Copy + Eq + Hash,
{
    let mut total_path = vec![*current];
    let mut current = current;
    while let Some(next) = came_from.get(current) {
        total_path.push(*next);
        current = next;
    }
    total_path.reverse();
    total_path.shrink_to_fit();
    total_path
}
