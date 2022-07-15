use std::time::Duration;

use criterion::{
    criterion_group,
    BatchSize,
    Criterion
};
use rust_fm::{
    alphabet::DNAAlphabet,
    index::fm_index::FMIndex
};

use crate::util::generator::AlphabetGenerator;

const NEW_FM_INDEX_SIZE: usize = 1_000_000_000;

const MATCH_AMOUNT_OF_CHARACTERS: usize = 1_000_000;
const MATCH_PATTERN_SIZE: usize = 100;

const SAMPLE_SIZE: usize = 1_000;
const MEASUREMENT_TIME: u64 = 20;

fn bench_new(c: &mut Criterion) {
    let generator = AlphabetGenerator::<DNAAlphabet>::default();

    c.bench_function("bench_new", |b| {
        b.iter_batched(
            // Create a new string of characters
            || generator.generate_string(NEW_FM_INDEX_SIZE),
            // Create a new fm index
            |characters| FMIndex::new(characters, 1),
            BatchSize::SmallInput
        )
    });
}

fn bench_exact_match(c: &mut Criterion) {
    let generator = AlphabetGenerator::<DNAAlphabet>::default();

    let fm_index = FMIndex::new(generator.generate_string(MATCH_AMOUNT_OF_CHARACTERS), 1);

    c.bench_function("bench_exact_match", |b| {
        b.iter_batched_ref(
            // Create a new string of characters
            || generator.generate_pattern(MATCH_PATTERN_SIZE),
            // Create a new fm index
            |mut pattern| fm_index.exact_match(&mut pattern),
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
    targets = /*bench_new,*/ bench_exact_match
);
