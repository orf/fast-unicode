use fast_unicode::simple::is_unicode;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use criterion_cycles_per_byte::CyclesPerByte;
use std::time::Duration;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_unicode");
    group.measurement_time(Duration::from_secs(10));
    let test_data = include_bytes!("unicode_test_set.txt");
    let one_quarter = test_data.len() / 4;

    for i in (one_quarter..=test_data.len()).step_by(one_quarter) {
        let slice = &test_data[0..i];
        group.throughput(Throughput::Bytes(slice.len() as u64));
        group.bench_function(BenchmarkId::new("simple", slice.len()), |b| {
            b.iter(|| is_unicode(slice))
        });
    }

    group.finish()
}

fn cycles_benchmark(c: &mut Criterion<CyclesPerByte>) {
    let test_data = include_bytes!("unicode_test_set.txt");
    let mut group = c.benchmark_group("cycles");
    group.measurement_time(Duration::from_secs(10));
    group.throughput(Throughput::Bytes(test_data.len() as u64));
    group.bench_function(BenchmarkId::new("simple", test_data.len()), |b| {
        b.iter(|| is_unicode(test_data))
    });
}

criterion_group!(
    name = cycles;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = cycles_benchmark
);
criterion_group!(my_bench, bench);
criterion_main!(my_bench, cycles);
