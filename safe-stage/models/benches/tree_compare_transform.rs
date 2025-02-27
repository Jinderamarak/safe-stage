use collisions::common::{Rotation, Transformation, Translation};
use collisions::complex::bvh_array::BvhTreeArr;
use collisions::complex::bvh_recursive::BvhRecursive;
use collisions::complex::bvh_sphere_linear::BvhSphereLinear;
use collisions::complex::bvh_sphere_recursive::BvhSphereRecursive;
use collisions::primitive::AlignedBoxCollider;
use criterion::{criterion_group, criterion_main, Criterion};
use maths::{Quaternion, Vector3};
use models::assembly::ball::ball_stage_triangles;

fn translate_tree<T: Translation>(tree: &T, translation: &Vector3) -> T {
    tree.translate(translation)
}

fn bench_tree_translate(c: &mut Criterion) {
    let triangles = ball_stage_triangles();

    let recursive_aabb = BvhRecursive::<AlignedBoxCollider>::build(&triangles);
    let recursive_sphere = BvhSphereRecursive::build(&triangles);
    let linear_sphere = BvhSphereLinear::build(&triangles);
    let linear_arr = BvhTreeArr::build(&triangles);

    let translation = Vector3::new(1.0, 2.0, 3.0);

    let mut group = c.benchmark_group("Tree Translation");
    group.bench_with_input(
        "BvhRecursive<AlignedBoxCollider>",
        &recursive_aabb,
        |b, a| b.iter(|| translate_tree(a, &translation)),
    );
    group.bench_with_input("BvhSphereRecursive", &recursive_sphere, |b, r| {
        b.iter(|| translate_tree(r, &translation))
    });
    group.bench_with_input("BvhSphereLinear", &linear_sphere, |b, l| {
        b.iter(|| translate_tree(l, &translation))
    });
    group.bench_with_input("BvhTreeArr", &linear_arr, |b, arr| {
        b.iter(|| translate_tree(arr, &translation))
    });
    group.finish();
}

fn transform_tree<T: Transformation>(
    tree: &T,
    rotation: &Quaternion,
    pivot: &Vector3,
    translation: &Vector3,
) -> T {
    tree.transform(rotation, pivot, translation)
}

fn rotate_tree<T: Rotation>(tree: &T, rotation: &Quaternion, pivot: &Vector3) -> T {
    tree.rotate_around(rotation, pivot)
}

fn bench_tree_rotate(c: &mut Criterion) {
    let triangles = ball_stage_triangles();

    let recursive_aabb = BvhRecursive::<AlignedBoxCollider>::build(&triangles);
    let recursive_sphere = BvhSphereRecursive::build(&triangles);
    let linear_sphere = BvhSphereLinear::build(&triangles);
    let linear_arr = BvhTreeArr::build(&triangles);

    let rotation = Quaternion::normalized(1.0, 2.0, 3.0, 4.0);
    let pivot = Vector3::new(1.0, 2.0, 3.0);

    let mut group = c.benchmark_group("Tree Rotation");
    group.bench_with_input(
        "BvhRecursive<AlignedBoxCollider>",
        &recursive_aabb,
        |b, a| b.iter(|| rotate_tree(a, &rotation, &pivot)),
    );
    group.bench_with_input("BvhSphereRecursive", &recursive_sphere, |b, r| {
        b.iter(|| rotate_tree(r, &rotation, &pivot))
    });
    group.bench_with_input("BvhSphereLinear", &linear_sphere, |b, l| {
        b.iter(|| rotate_tree(l, &rotation, &pivot))
    });
    group.bench_with_input("BvhTreeArr", &linear_arr, |b, arr| {
        b.iter(|| rotate_tree(arr, &rotation, &pivot))
    });
    group.finish();
}

fn bench_tree_transform(c: &mut Criterion) {
    let triangles = ball_stage_triangles();

    let recursive_aabb = BvhRecursive::<AlignedBoxCollider>::build(&triangles);
    let recursive_sphere = BvhSphereRecursive::build(&triangles);
    let linear_sphere = BvhSphereLinear::build(&triangles);
    let linear_arr = BvhTreeArr::build(&triangles);

    let rotation = Quaternion::normalized(1.0, 2.0, 3.0, 4.0);
    let pivot = Vector3::new(1.0, 2.0, 3.0);
    let translation = Vector3::new(1.0, 2.0, 3.0);

    let mut group = c.benchmark_group("Tree Transformation");
    group.bench_with_input(
        "BvhRecursive<AlignedBoxCollider>",
        &recursive_aabb,
        |b, a| b.iter(|| transform_tree(a, &rotation, &pivot, &translation)),
    );
    group.bench_with_input("BvhSphereRecursive", &recursive_sphere, |b, r| {
        b.iter(|| transform_tree(r, &rotation, &pivot, &translation))
    });
    group.bench_with_input("BvhSphereLinear", &linear_sphere, |b, l| {
        b.iter(|| transform_tree(l, &rotation, &pivot, &translation))
    });
    group.bench_with_input("BvhTreeArr", &linear_arr, |b, arr| {
        b.iter(|| transform_tree(arr, &rotation, &pivot, &translation))
    });
    group.finish();
}

criterion_group!(
    collisions,
    bench_tree_translate,
    bench_tree_rotate,
    bench_tree_transform
);
criterion_main!(collisions);
