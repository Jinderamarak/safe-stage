use collisions::common::Collides;
use collisions::primitive::{AlignedBoxCollider, SphereCollider};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maths::Vector3;

fn bench_primitive_collisions(c: &mut Criterion) {
    let sphere_a = SphereCollider::new(Vector3::new(1.0, 2.0, 3.0), 3.0);
    let sphere_b = SphereCollider::new(Vector3::new(2.0, 2.0, 2.0), 2.0);
    let box_a = AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(5.0, 5.0, 5.0));
    let box_b = AlignedBoxCollider::new(Vector3::new(2.0, 2.0, 2.0), Vector3::new(5.0, 5.0, 5.0));

    let mut group = c.benchmark_group("Primitive Collisions");
    group.bench_function("Sphere-Sphere Collision", |b| {
        b.iter(|| black_box(&sphere_a).collides_with(black_box(&sphere_b)))
    });
    group.bench_function("Sphere-Box Collision", |b| {
        b.iter(|| black_box(&sphere_a).collides_with(black_box(&box_a)))
    });
    group.bench_function("Box-Box Collision", |b| {
        b.iter(|| black_box(&box_a).collides_with(black_box(&box_b)))
    });
    group.finish();
}

criterion_group!(collisions, bench_primitive_collisions,);
criterion_main!(collisions);
