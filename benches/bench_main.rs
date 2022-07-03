use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    //benchmarks::bench_bitvector::benches,
    benchmarks::bench_alphabet::benches
}
