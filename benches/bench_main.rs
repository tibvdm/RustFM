use criterion::criterion_main;

mod benchmarks;
mod r#extern;

criterion_main! {
    benchmarks::bench_alphabet::benches,
    benchmarks::bench_bitvector::benches,
    benchmarks::bench_fm_index::benches,
    benchmarks::bench_suffix_array::benches,

    r#extern::bench_fm_index::benches
}
