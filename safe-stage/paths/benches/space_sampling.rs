use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maths::Vector3;
use models::assembly::thesis;
use models::assembly::thesis::{ThesisChamber, ThesisStage};
use models::parts::chamber::Chamber;
use paths::eager::space::sampled_space_3d::sample_grid_space_3d_par;
use paths::eager::space::space_3d::Grid3DSpace;

fn sample_space(stage: &ThesisStage, chamber: &ThesisChamber) -> Grid3DSpace {
    let (min, max) = thesis::LIMITS;
    let step = Vector3::new(20e-3, 20e-3, 20e-3);
    sample_grid_space_3d_par(
        &min.pos,
        &max.pos,
        &chamber.full(),
        stage,
        &step,
        &Vector3::ZERO,
    )
}

fn bench_space_sampling(c: &mut Criterion) {
    let stage = ThesisStage::default();
    let chamber = ThesisChamber::default();

    c.bench_function("Space Sampling", |b| {
        b.iter(|| sample_space(black_box(&stage), black_box(&chamber)))
    });
}

criterion_group!(collisions, bench_space_sampling);
criterion_main!(collisions);
