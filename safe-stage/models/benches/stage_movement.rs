use collisions::complex::group::ColliderGroup;
use collisions::complex::BvhSphere;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maths::Vector3;
use models::assembly::thesis::ThesisStage;
use models::movable::Movable;
use models::position::sixaxis::SixAxis;

fn move_stage(stage: &ThesisStage, position: &SixAxis) -> ColliderGroup<BvhSphere> {
    stage.move_to(position)
}

fn bench_stage_transformation(c: &mut Criterion) {
    let stage = ThesisStage::default();
    let position = SixAxis {
        pos: Vector3::new(1.0, 2.0, 3.0),
        rot: Vector3::new(4.0, 5.0, 6.0),
    };

    c.bench_function("Stage Transformation", |b| {
        b.iter(|| move_stage(black_box(&stage), black_box(&position)))
    });
}

criterion_group!(collisions, bench_stage_transformation);
criterion_main!(collisions);
