#[macro_use]
extern crate criterion;

use criterion::Criterion;

extern crate mktoc;

use mktoc::*;
use std::fs;

fn mktoc_benchmark(c: &mut Criterion) {
    for entry in fs::read_dir("tests/files/").unwrap() {
        let file = entry.unwrap();
        c.bench_function(format!("mktoc:{:?}", file.path()).as_str(), |b| {
            b.iter(|| make_toc(file.path(), 1, 6))
        });    
    }
}

criterion_group!(benches, mktoc_benchmark);
criterion_main!(benches);
