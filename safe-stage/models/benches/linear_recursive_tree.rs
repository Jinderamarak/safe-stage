use collisions::common::{Collides, Transformation};
use collisions::complex::bvh_array::BvhTreeArr;
use collisions::complex::bvh_recursive::BvhRecursive;
use collisions::complex::bvh_sphere_linear::BvhSphereLinear;
use collisions::complex::bvh_sphere_recursive::BvhSphereRecursive;
use collisions::primitive::AlignedBoxCollider;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maths::{Quaternion, Vector3};
use models::assembly::ball::{ball_chamber_triangles, ball_stage_triangles};

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

    let recursive_aabb = BvhRecursive::<AlignedBoxCollider>::build(&triangles);
    let recursive_sphere = BvhSphereRecursive::build(&triangles);
    let linear_sphere = BvhSphereLinear::build(&triangles);
    let linear_arr = BvhTreeArr::build(&triangles);

    let rotation = Quaternion::normalized(1.0, 2.0, 3.0, 4.0);
    let pivot = Vector3::new(1.0, 2.0, 3.0);
    let translation = Vector3::new(1.0, 2.0, 3.0);

    let mut group = c.benchmark_group("Tree Transformation");
    group.bench_with_input("Recursive AABB", &recursive_aabb, |b, a| {
        b.iter(|| transform_tree(a, &rotation, &pivot, &translation))
    });
    group.bench_with_input("Recursive Sphere", &recursive_sphere, |b, r| {
        b.iter(|| transform_tree(r, &rotation, &pivot, &translation))
    });
    group.bench_with_input("Linear Sphere", &linear_sphere, |b, l| {
        b.iter(|| transform_tree(l, &rotation, &pivot, &translation))
    });
    group.bench_with_input("Linear Array", &linear_arr, |b, arr| {
        b.iter(|| transform_tree(arr, &rotation, &pivot, &translation))
    });
    group.finish();
}

fn collide_tree<T: Collides<T>>(tree: &T, other: &T) -> bool {
    tree.collides_with(other)
}

fn bench_tree_collide(c: &mut Criterion) {
    let triangles_a = ball_stage_triangles();
    let triangles_b = ball_chamber_triangles();

    let recursive_aabb_a = BvhRecursive::<AlignedBoxCollider>::build(&triangles_a);
    let recursive_aabb_b = BvhRecursive::<AlignedBoxCollider>::build(&triangles_b);
    let recursive_aabb = (recursive_aabb_a, recursive_aabb_b);

    let recursive_sphere_a = BvhSphereRecursive::build(&triangles_a);
    let recursive_sphere_b = BvhSphereRecursive::build(&triangles_b);
    let recursive_sphere = (recursive_sphere_a, recursive_sphere_b);

    let linear_sphere_a = BvhSphereLinear::build(&triangles_a);
    let linear_sphere_b = BvhSphereLinear::build(&triangles_b);
    let linear_sphere = (linear_sphere_a, linear_sphere_b);

    let linear_arr_a = BvhTreeArr::build(&triangles_a);
    let linear_arr_b = BvhTreeArr::build(&triangles_b);
    let linear_arr = (linear_arr_a, linear_arr_b);

    let mut group = c.benchmark_group("Tree Collision");
    group.bench_with_input("Recursive AABB", &recursive_aabb, |b, (x, y)| {
        b.iter(|| collide_tree(x, y))
    });
    group.bench_with_input("Recursive Sphere", &recursive_sphere, |b, (x, y)| {
        b.iter(|| collide_tree(x, y))
    });
    group.bench_with_input("Linear Sphere", &linear_sphere, |b, (x, y)| {
        b.iter(|| collide_tree(x, y))
    });
    group.bench_with_input("Linear Array", &linear_arr, |b, (x, y)| {
        b.iter(|| collide_tree(x, y))
    });
    group.finish();
}

fn bench_tree_build(c: &mut Criterion) {
    let triangles = ball_stage_triangles();

    let mut group = c.benchmark_group("Tree Building");
    group.bench_function("Recursive AABB", |b| {
        b.iter(|| black_box(BvhRecursive::<AlignedBoxCollider>::build(&triangles)))
    });
    group.bench_function("Recursive Sphere", |b| {
        b.iter(|| black_box(BvhSphereRecursive::build(&triangles)))
    });
    group.bench_function("Linear Sphere", |b| {
        b.iter(|| black_box(BvhSphereLinear::build(&triangles)))
    });
    group.bench_function("Linear Array", |b| {
        b.iter(|| black_box(BvhTreeArr::build(&triangles)))
    });
    group.finish();
}

criterion_group!(
    collisions,
    bench_tree_transform,
    bench_tree_collide,
    bench_tree_build
);
criterion_main!(collisions);
