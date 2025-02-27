use collisions::common::Collides;
use collisions::complex::bvh_array::BvhTreeArr;
use collisions::complex::bvh_recursive::BvhRecursive;
use collisions::complex::bvh_sphere_linear::BvhSphereLinear;
use collisions::complex::bvh_sphere_recursive::BvhSphereRecursive;
use collisions::primitive::AlignedBoxCollider;
use criterion::{criterion_group, criterion_main, Criterion};
use models::assembly::ball::{ball_chamber_triangles, ball_stage_triangles};

fn collide_trees<T: Collides<T>>(tree: &T, other: &T) -> bool {
    tree.collides_with(other)
}

fn bench_tree_collision(c: &mut Criterion) {
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
    group.bench_with_input(
        "BvhRecursive<AlignedBoxCollider>",
        &recursive_aabb,
        |b, (x, y)| b.iter(|| collide_trees(x, y)),
    );
    group.bench_with_input("BvhSphereRecursive", &recursive_sphere, |b, (x, y)| {
        b.iter(|| collide_trees(x, y))
    });
    group.bench_with_input("BvhSphereLinear", &linear_sphere, |b, (x, y)| {
        b.iter(|| collide_trees(x, y))
    });
    group.bench_with_input("BvhTreeArr", &linear_arr, |b, (x, y)| {
        b.iter(|| collide_trees(x, y))
    });
    group.finish();
}

criterion_group!(collisions, bench_tree_collision);
criterion_main!(collisions);
