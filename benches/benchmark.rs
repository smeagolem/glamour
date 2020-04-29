use criterion::{criterion_group, criterion_main, Criterion};
use glamour::Transform;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Transform::matrix", |b| {
        let t = Transform::new();
        b.iter(|| {
            return t.matrix();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
