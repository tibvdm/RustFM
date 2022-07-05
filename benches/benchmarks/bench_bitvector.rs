use std::{
    ops::Range,
    time::Duration
};

use criterion::{
    criterion_group,
    BatchSize,
    Criterion
};
use rand::distributions::{
    Distribution,
    Uniform
};
use rust_fm::bitvector::Bitvec;

const BITVECTOR_LARGE: usize = 1_000_000_000;

const SAMPLE_SIZE: usize = 100;
const MEASUREMENT_TIME: u64 = 20;

fn generate_indices(n: usize, range: Range<usize>) -> Vec<usize> {
    let mut rng = rand::thread_rng();

    let distribution = Uniform::<usize>::from(range);

    return (0 .. n).map(|_| distribution.sample(&mut rng)).collect();
}

fn generate_bitvector(n: usize) -> Bitvec {
    let mut bv = Bitvec::new(n);

    // Set some random bits in the bitvector
    generate_indices(n / 1_000, 0 .. n)
        .iter()
        .for_each(|i| bv.set(*i, true));

    return bv;
}

fn bench_get(c: &mut Criterion) {
    let bv = generate_bitvector(BITVECTOR_LARGE);

    c.bench_function("bench_get", |b| {
        b.iter_batched(
            // Create a new list of indices to retrieve
            || generate_indices(BITVECTOR_LARGE / 1_000, 0 .. BITVECTOR_LARGE),
            // Run the benchmark for those indices
            |indices| {
                for i in indices {
                    bv.get(i);
                }
            },
            BatchSize::SmallInput
        )
    });
}

fn bench_index(c: &mut Criterion) {
    let mut bv = Bitvec::new(BITVECTOR_LARGE);

    // Set some random bits in the bitvector
    generate_indices(BITVECTOR_LARGE / 1_000, 0 .. BITVECTOR_LARGE)
        .iter()
        .for_each(|i| bv.set(*i, true));

    c.bench_function("bench_index", |b| {
        b.iter_batched_ref(
            // Create a new random bitvector to index
            || generate_bitvector(BITVECTOR_LARGE),
            // Index the new bitvector
            |bv| bv.calculate_counts(),
            BatchSize::SmallInput
        )
    });
}

fn bench_rank(c: &mut Criterion) {
    let mut bv = generate_bitvector(BITVECTOR_LARGE);

    // Calculate the counts for our bitvector
    bv.calculate_counts();

    c.bench_function("bench_rank", |b| {
        b.iter_batched(
            // Create a new list of indices to retrieve the rank for
            || generate_indices(BITVECTOR_LARGE / 1_000, 0 .. BITVECTOR_LARGE),
            // Run the benchmark for those indices
            |indices| {
                for i in indices {
                    bv.rank(i);
                }
            },
            BatchSize::SmallInput
        )
    });
}

// TODO: https://bheisler.github.io/criterion.rs/book/user_guide/advanced_configuration.html

fn custom_criterion_config() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::from_secs(MEASUREMENT_TIME))
        .sample_size(SAMPLE_SIZE)
}

criterion_group!(
    name = benches;
    config = custom_criterion_config();
    targets = bench_get, bench_index, bench_rank
);
