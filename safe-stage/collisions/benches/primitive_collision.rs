use collisions::common::Collides;
use collisions::primitive::{AlignedBoxCollider, SphereCollider};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maths::Vector3;

fn bench_primitive<A, B, C>(base: &A, colliding: &B, non_colliding: &C) -> bool
where
    A: Collides<B>,
    A: Collides<C>,
{
    base.collides_with(colliding) && !base.collides_with(non_colliding)
}

/// Compare the performance of primitive collisions of the following shapes:
/// - Sphere-Sphere
/// - Sphere-AABB
/// - AABB-AABB
fn bench_primitive_collisions(c: &mut Criterion) {
    let sphere_base = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 1.0);
    let sphere_colliding = SphereCollider::new(Vector3::new(1.0, 0.0, 0.0), 2.0);
    let sphere_non_colliding = SphereCollider::new(Vector3::new(3.0, 0.0, 0.0), 1.0);

    let box_base =
        AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
    let box_colliding =
        AlignedBoxCollider::new(Vector3::new(1.0, 0.0, 0.0), Vector3::new(2.0, 1.0, 1.0));
    let box_non_colliding =
        AlignedBoxCollider::new(Vector3::new(3.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

    let mut group = c.benchmark_group("Primitive Collisions");
    group.bench_function("Sphere-Sphere Collision", |b| {
        b.iter(|| {
            bench_primitive(
                black_box(&sphere_base),
                black_box(&sphere_colliding),
                black_box(&sphere_non_colliding),
            )
        })
    });
    group.bench_function("Sphere-Box Collision", |b| {
        b.iter(|| {
            bench_primitive(
                black_box(&sphere_base),
                black_box(&box_colliding),
                black_box(&box_non_colliding),
            )
        })
    });
    group.bench_function("Box-Box Collision", |b| {
        b.iter(|| {
            bench_primitive(
                black_box(&box_base),
                black_box(&box_colliding),
                black_box(&box_non_colliding),
            )
        })
    });
    group.finish();
}

criterion_group!(collisions, bench_primitive_collisions,);
criterion_main!(collisions);
