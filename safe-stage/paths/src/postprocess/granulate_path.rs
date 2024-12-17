use crate::path::PathResult;
use maths::NaNExtension;
use models::position::sixaxis::SixAxis;

pub fn granulate_path(path: PathResult<SixAxis>, step: &SixAxis) -> PathResult<SixAxis> {
    path.map(|p| granulate_path_nodes(p, step))
}

fn granulate_path_nodes(path: &[SixAxis], step: &SixAxis) -> Vec<SixAxis> {
    path.windows(2)
        .flat_map(|pair| granulate_path_segment(pair[0], pair[1], step))
        .collect()
}

fn granulate_path_segment(
    start: SixAxis,
    end: SixAxis,
    step: &SixAxis,
) -> impl Iterator<Item = SixAxis> {
    let steps = start.stepping(&end, step);
    (0..=steps).map(move |i| {
        let t = (i as f64 / steps as f64).map_nan(0.0);
        start.lerp_t(&end, t)
    })
}
