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
use rust_fm::alphabet::{
    Alphabet,
    AlphabetChar,
    DNAAlphabet
};

const AMOUNT_OF_INDICES: usize = 1_000_000;

const SAMPLE_SIZE: usize = 1_000;
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

fn bench_alphabet_dna_i2c(c: &mut Criterion) {
    let alphabet = DNAAlphabet::default();

    c.bench_function("bench_alphabet_dna_i2c", |b| {
        b.iter_batched(
            // Create a new list of indices to map
            || generate_indices(AMOUNT_OF_INDICES, 0 .. alphabet.len()),
            // Run the benchmark for those indices
            |indices| {
                for i in indices {
                    alphabet.i2c(i);
                }
            },
            BatchSize::SmallInput
        )
    });
}

fn bench_alphabet_dna_c2i(c: &mut Criterion) {
    let alphabet = DNAAlphabet::default();

    c.bench_function("bench_alphabet_dna_c2i", |b| {
        b.iter_batched(
            // Create a new list of indices to map
            || generate_characters(AMOUNT_OF_INDICES, vec![b'A', b'C', b'G', b'T']),
            // Run the benchmark for those indices
            |characters| {
                for c in characters {
                    alphabet.c2i(c);
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
    targets = bench_alphabet_dna_i2c, bench_alphabet_dna_c2i
);
