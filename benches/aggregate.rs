#![allow(clippy::type_complexity)]

use std::fs::File;

use criterion::{criterion_group, criterion_main, Criterion};

use one_brc::attempt_a::AttemptA;
use one_brc::Aggregator;

fn bench_aggregate(c: &mut Criterion) {
    let mut group = c.benchmark_group("Aggregate");
    group.sample_size(10);

    let path = "data/measurements.txt";
    let aggregators: Vec<(&str, fn(File))> = vec![("A", AttemptA::aggregate)];

    for (label, aggregator) in aggregators {
        group.bench_function(label, |b| {
            b.iter(|| {
                let input_file = File::open(path).unwrap();
                aggregator(input_file);
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_aggregate);
criterion_main!(benches);
