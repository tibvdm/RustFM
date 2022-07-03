use std::ops::Range;
use std::time::Duration;
use criterion::{ criterion_group, Criterion, BatchSize };
use rand::distributions::{ Distribution, Uniform };

use rust_fm::alphabet::AlphabetChar;
use rust_fm::suffix_array::{ SuffixArray, SparseSuffixArray };

const AMOUNT_OF_INDICES: usize = 1_000_000;

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

fn generate_suffix_array(n: usize, characters: Vec<AlphabetChar>) -> Vec<u32> {
    let (_, suffix_array) = SuffixArray::new(&generate_characters(n, characters)).into_parts();

    return suffix_array;
}

fn bench_from_sa(c: &mut Criterion) {
    c.bench_function("bench_from_sa",
        |b| b.iter_batched_ref(
            // Create a new list of indices to map
            || generate_suffix_array(AMOUNT_OF_INDICES, vec![b'A', b'C', b'G', b'T']), 
            // Run the benchmark for those indices
            |suffix_array| {
                SparseSuffixArray::from_sa(suffix_array, 32);
            }
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
    targets = bench_from_sa
);
