use criterion::{criterion_group, criterion_main, Criterion};
use glamour::{Renderer, Transform, VertTrans};

fn criterion_benchmark(c: &mut Criterion) {
    let t = Transform::new();
    c.bench_function("Transform::matrix", |b| {
        b.iter(|| {
            return t.matrix();
        })
    });

    let count = 100_000;
    let mut vertices: Vec<VertTrans> = Vec::new();
    vertices.resize_with(count, std::default::Default::default);
    let mut transforms: Vec<Transform> = Vec::new();
    transforms.resize_with(count, std::default::Default::default);
    c.bench_function("ForwardRenderer::set_vert_trans", |b| {
        b.iter(|| {
            Renderer::set_vert_trans(&mut vertices, &transforms);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
