use criterion::{Criterion, criterion_group, criterion_main};
use rust_template_lib::add;

fn bench_add(c: &mut Criterion) {
    c.bench_function("add", |b| b.iter(|| add(2, 2)));
}

criterion_group!(benches, bench_add);

criterion_main!(benches);
