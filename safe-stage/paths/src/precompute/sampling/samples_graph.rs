use models::position::sixaxis::SixAxis;
use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

pub fn samples_graph<T>(samples: &[SixAxis], distance: f64, speed: &SixAxis) -> HashMap<T, Vec<T>>
where
    T: From<usize> + Eq + Hash,
{
    let mut graph = HashMap::new();
    for (i, start) in samples.iter().enumerate() {
        for (j, end) in samples.iter().enumerate() {
            if i == j {
                continue;
            }

            if start.time_to(end, speed) >= distance {
                continue;
            }

            graph
                .entry(T::from(i))
                .or_insert_with(Vec::new)
                .push(T::from(j));
        }
    }

    graph.values_mut().for_each(|v| v.shrink_to_fit());
    graph.shrink_to_fit();
    graph
}

pub fn samples_graph_par<T>(
    samples: &[SixAxis],
    distance: f64,
    speed: &SixAxis,
) -> HashMap<T, Vec<T>>
where
    T: From<usize> + Eq + Hash + Send,
{
    samples
        .par_iter()
        .enumerate()
        .map(|(i, start)| {
            (
                T::from(i),
                samples
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| i != *j)
                    .filter(|(_, end)| {
                        let dist = start.time_to(end, speed);
                        dist < distance
                    })
                    .map(|(j, _)| T::from(j))
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}
