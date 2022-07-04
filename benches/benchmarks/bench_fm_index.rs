use std::str;
use std::ops::Range;
use std::time::Duration;
use criterion::{ criterion_group, Criterion, BatchSize };
use rand::distributions::{ Distribution, Uniform };

use rust_fm::fm_index::FMIndex;
use rust_fm::alphabet::{ DNAAlphabet, AlphabetChar };

const AMOUNT_OF_CHARACTERS: usize = 1_000_000;

const SAMPLE_SIZE: usize    = 100;
const MEASUREMENT_TIME: u64 = 20;

fn generate_indices(n: usize, range: Range<usize>) -> Vec<usize> {
    let mut rng = rand::thread_rng();

    let distribution = Uniform::<usize>::from(range);

    return (0 .. n).map(|_| distribution.sample(&mut rng)).collect();
}

fn generate_characters(n: usize, characters: Vec<AlphabetChar>) -> Vec<AlphabetChar> {
    return generate_indices(n, 0 .. characters.len()).iter().map(|i| characters[*i]).collect();
}

fn bench_new(c: &mut Criterion) {
    c.bench_function("bench_new",
        |b| b.iter_batched(
            // Create a new string of characters
            || generate_characters(AMOUNT_OF_CHARACTERS, vec![b'A', b'C', b'G', b'T']),
            // Create a new fm index
            |characters| FMIndex::new(characters, DNAAlphabet::default())
        , BatchSize::SmallInput)
    );
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
    targets = bench_new
);
