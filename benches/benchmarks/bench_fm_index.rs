use std::{
    ops::Range,
    str,
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
        Alphabet,
        AlphabetChar,
        AlphabetIndex,
        AlphabetPattern,
        AlphabetString,
        DNAAlphabet
    },
    index::fm_index::FMIndex
};

const AMOUNT_OF_CHARACTERS: usize = 1_000_000;
const PATTERN_SIZE: usize = 100;

const SAMPLE_SIZE: usize = 1_000;
const MEASUREMENT_TIME: u64 = 20;

pub struct AlphabetGenerator<A: Alphabet> {
    alphabet: A
}

impl<A: Alphabet> AlphabetGenerator<A> {
    pub fn generate_indices(&self, amount: usize) -> Vec<AlphabetIndex> {
        let mut rng = rand::thread_rng();

        let distribution = Uniform::<usize>::from(0 .. self.alphabet.len());

        return (0 .. amount)
            .map(|_| distribution.sample(&mut rng) as AlphabetChar)
            .collect();
    }

    pub fn generate_characters(&self, amount: usize) -> Vec<AlphabetChar> {
        return self
            .generate_indices(amount)
            .iter()
            .map(|i| self.alphabet.i2c(*i))
            .collect();
    }

    pub fn generate_string(&self, string_length: usize) -> AlphabetString<A> {
        let characters = self.generate_characters(string_length);
        let input = unsafe { str::from_utf8_unchecked(&characters) };

        return AlphabetString::<A>::from(input);
    }

    pub fn generate_pattern(&self, pattern_size: usize) -> AlphabetPattern<A> {
        let characters = self.generate_characters(pattern_size);
        let input = unsafe { str::from_utf8_unchecked(&characters) };

        return AlphabetPattern::<A>::from(input);
    }
}

impl<A: Alphabet> Default for AlphabetGenerator<A> {
    fn default() -> Self {
        Self {
            alphabet: Default::default()
        }
    }
}

fn bench_new(c: &mut Criterion) {
    let generator = AlphabetGenerator::<DNAAlphabet>::default();

    c.bench_function("bench_new", |b| {
        b.iter_batched(
            // Create a new string of characters
            || generator.generate_string(AMOUNT_OF_CHARACTERS),
            // Create a new fm index
            |characters| FMIndex::new(characters, 1),
            BatchSize::SmallInput
        )
    });
}

fn bench_exact_match(c: &mut Criterion) {
    let generator = AlphabetGenerator::<DNAAlphabet>::default();

    let fm_index = FMIndex::new(generator.generate_string(AMOUNT_OF_CHARACTERS), 1);

    c.bench_function("bench_exact_match", |b| {
        b.iter_batched_ref(
            // Create a new string of characters
            || generator.generate_pattern(PATTERN_SIZE),
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
