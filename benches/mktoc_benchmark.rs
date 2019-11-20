#[macro_use]
extern crate criterion;

use criterion::Criterion;

extern crate mktoc;

use mktoc::*;

fn mktoc_benchmark(c: &mut Criterion) {
    c.bench_function("mktoc: tests/files/README_01.md", |b| {
        b.iter(|| make_toc(String::from("../tests/files/README_01.md")))
    });
    c.bench_function("mktoc: tests/files/README_02.md", |b| {
        b.iter(|| make_toc(String::from("../tests/files/README_02.md")))
    });
    c.bench_function("mktoc: tests/files/README_03.md", |b| {
        b.iter(|| make_toc(String::from("../tests/files/README_03.md")))
    });
}

criterion_group!(benches, mktoc_benchmark);
criterion_main!(benches);
