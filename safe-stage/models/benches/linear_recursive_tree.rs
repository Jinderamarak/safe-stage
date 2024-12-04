use collisions::common::{Collides, Transformation};
use collisions::complex::bvh_sphere_linear::BvhSphereLinear;
use collisions::complex::bvh_sphere_recursive::BvhSphereRecursive;
use criterion::{criterion_group, criterion_main, Criterion};
use maths::{Quaternion, Vector3};
use models::assembly::ball::ball_stage_triangles;

fn transform_tree<T: Transformation>(
    tree: &T,
    rotation: &Quaternion,
    pivot: &Vector3,
    translation: &Vector3,
) -> T {
    tree.transform(rotation, pivot, translation)
}

fn bench_tree_transform(c: &mut Criterion) {
    let triangles = ball_stage_triangles();

    let recursive = BvhSphereRecursive::build(&triangles);
    let linear = BvhSphereLinear::build(&triangles);

    let rotation = Quaternion::normalized(1.0, 2.0, 3.0, 4.0);
    let pivot = Vector3::new(1.0, 2.0, 3.0);
    let translation = Vector3::new(1.0, 2.0, 3.0);

    let mut group = c.benchmark_group("Tree Transformation");
    group.bench_with_input("Recursive", &recursive, |b, r| {
        b.iter(|| transform_tree(r, &rotation, &pivot, &translation))
    });
    group.bench_with_input("Linear", &linear, |b, l| {
        b.iter(|| transform_tree(l, &rotation, &pivot, &translation))
    });
    group.finish();
}

fn collide_tree<T: Collides<T>>(tree: &T, other: &T) -> bool {
    tree.collides_with(other)
}

fn bench_tree_collide(c: &mut Criterion) {
    let triangles = ball_stage_triangles();

    let recursive = BvhSphereRecursive::build(&triangles);
    let linear = BvhSphereLinear::build(&triangles);

    let mut group = c.benchmark_group("Tree Collision");
    group.bench_with_input("Recursive", &recursive, |b, r| {
        b.iter(|| collide_tree(r, r))
    });
    group.bench_with_input("Linear", &linear, |b, l| b.iter(|| collide_tree(l, l)));
    group.finish();
}

criterion_group!(collisions, bench_tree_transform, bench_tree_collide);
criterion_main!(collisions);
