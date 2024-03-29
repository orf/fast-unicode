use fast_unicode;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use criterion_cycles_per_byte::CyclesPerByte;

use rand::prelude::*;

const RNG_SEED: [u8; 32] = [
    1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
];

const TEST_CHARS: [&str; 4] = [
    "\u{0024}",
    "\u{00A2}",
    "\u{0939}",
    "\u{10348}",
];

const TEST_DATA_SIZE: usize = 10_000;
const ITERATIONS: usize = 20;

fn gen_random_string(rng: &mut StdRng, length: usize) -> String {
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, TEST_CHARS.len());
            TEST_CHARS[idx]
        })
        .collect::<Vec<&str>>()
        .join("")
}

fn bench(c: &mut Criterion) {
    let mut rng: StdRng = SeedableRng::from_seed(RNG_SEED);

    let mut group = c.benchmark_group("is_unicode");

    for i in (30..TEST_DATA_SIZE).step_by(TEST_DATA_SIZE / ITERATIONS) {
        let string = gen_random_string(&mut rng, i);
        let bytes = string.as_bytes();
        group.throughput(Throughput::Bytes(bytes.len() as u64));
        group.bench_with_input(BenchmarkId::new("Simple", bytes.len()), &bytes, |b, i| {
            b.iter(|| fast_unicode::simple::is_unicode(*i))
        });
        group.bench_with_input(BenchmarkId::new("Stdlib", bytes.len()), &bytes, |b, i| {
            b.iter(|| fast_unicode::stdlib::is_unicode(*i))
        });
        group.bench_with_input(BenchmarkId::new("iterators", bytes.len()), &bytes, |b, i| {
            b.iter(|| fast_unicode::iterators::is_unicode(*i))
        });
        group.bench_with_input(BenchmarkId::new("match_matrix", bytes.len()), &bytes, |b, i| {
            b.iter(|| fast_unicode::match_matrix::is_unicode(*i))
        });
        group.bench_with_input(BenchmarkId::new("for_loop", bytes.len()), &bytes, |b, i| {
            b.iter(|| fast_unicode::for_loop::is_unicode(*i))
        });
    }
    group.finish();
}

fn cycles_benchmark(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("cycles");

    for test_char in TEST_CHARS.iter() {
        let test_data: String = (0..100).map(|_| test_char.to_string()).collect();
        let bytes = test_data.as_bytes();

        group.throughput(Throughput::Bytes(bytes.len() as u64));

        group.bench_with_input(BenchmarkId::new("Simple", test_char.len()), &bytes, |b, i| {
            b.iter(|| fast_unicode::simple::is_unicode(*i))
        });
        group.bench_with_input(BenchmarkId::new("Stdlib", test_char.len()), &bytes, |b, i| {
            b.iter(|| fast_unicode::stdlib::is_unicode(*i))
        });
        group.bench_with_input(BenchmarkId::new("iterators", test_char.len()), &bytes, |b, i| {
            b.iter(|| fast_unicode::iterators::is_unicode(*i))
        });
        group.bench_with_input(BenchmarkId::new("match_matrix", test_char.len()), &bytes, |b, i| {
            b.iter(|| fast_unicode::match_matrix::is_unicode(*i))
        });
        group.bench_with_input(BenchmarkId::new("for_loop", test_char.len()), &bytes, |b, i| {
            b.iter(|| fast_unicode::for_loop::is_unicode(*i))
        });
    }
    group.finish();
}

criterion_group!(
    name = cycles;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = cycles_benchmark
);
criterion_group!(benchmarks, bench);
criterion_main!(benchmarks, cycles);
