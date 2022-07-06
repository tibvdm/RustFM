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
use rust_fm::{
    alphabet::{
        AlphabetChar,
        DNAAlphabet
    },
    fm_index::FMIndex
};

const AMOUNT_OF_CHARACTERS: usize = 1_000_000;

const SAMPLE_SIZE: usize = 100;
const MEASUREMENT_TIME: u64 = 20;

fn generate_indices(n: usize, range: Range<usize>) -> Vec<usize> {
    let mut rng = rand::thread_rng();

    let distribution = Uniform::<usize>::from(range);

    return (0 .. n).map(|_| distribution.sample(&mut rng)).collect();
}

fn generate_characters(n: usize, characters: Vec<AlphabetChar>) -> Vec<AlphabetChar> {
    return generate_indices(n, 0 .. characters.len())
        .iter()
        .map(|i| characters[*i])
        .collect();
}

fn generate_fm_index(n: usize, characters: Vec<AlphabetChar>) -> FMIndex<DNAAlphabet> {
    return FMIndex::new(generate_characters(n, characters), DNAAlphabet::default(), 1);
}

fn bench_new(c: &mut Criterion) {
    c.bench_function("bench_new", |b| {
        b.iter_batched(
            // Create a new string of characters
            || generate_characters(AMOUNT_OF_CHARACTERS, vec![b'A', b'C', b'G', b'T']),
            // Create a new fm index
            |characters| FMIndex::new(characters, DNAAlphabet::default(), 1),
            BatchSize::SmallInput
        )
    });
}

fn bench_exact_match(c: &mut Criterion) {
    let fm_index = generate_fm_index(AMOUNT_OF_CHARACTERS, vec![b'A', b'C', b'G', b'T']);

    c.bench_function("bench_exact_match", |b| {
        b.iter_batched_ref(
            // Create a new string of characters
            || generate_characters(100, vec![b'A', b'C', b'G', b'T']),
            // Create a new fm index
            |pattern| fm_index.exact_match(pattern),
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
    targets = bench_new, bench_exact_match
);
