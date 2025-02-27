use collisions::complex::bvh_array::BvhTreeArr;
use collisions::complex::bvh_recursive::BvhRecursive;
use collisions::complex::bvh_sphere_linear::BvhSphereLinear;
use collisions::complex::bvh_sphere_recursive::BvhSphereRecursive;
use collisions::primitive::AlignedBoxCollider;
use criterion::{criterion_group, criterion_main, Criterion};
use models::assembly::ball::ball_stage_triangles;

fn bench_tree_build(c: &mut Criterion) {
    let triangles = ball_stage_triangles();

    let mut group = c.benchmark_group("Tree Building");
    group.bench_function("BvhRecursive<AlignedBoxCollider>", |b| {
        b.iter(|| BvhRecursive::<AlignedBoxCollider>::build(&triangles))
    });
    group.bench_function("BvhSphereRecursive", |b| {
        b.iter(|| BvhSphereRecursive::build(&triangles))
    });
    group.bench_function("BvhSphereLinear", |b| {
        b.iter(|| BvhSphereLinear::build(&triangles))
    });
    group.bench_function("BvhTreeArr", |b| b.iter(|| BvhTreeArr::build(&triangles)));
    group.finish();
}

criterion_group!(collisions, bench_tree_build);
criterion_main!(collisions);
