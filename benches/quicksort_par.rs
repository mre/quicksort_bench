use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use quicksort_bench::quicksort_par;

fn quicksort_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("quicksort_par_group");
    group.sample_size(10); // Set the sample size to 10

    // Create a very large, 1 million element vector with random values
    let arr = (0..1_000_000)
        .map(|_| rand::random::<u32>())
        .collect::<Vec<u32>>();

    group.bench_with_input(
        BenchmarkId::new("quicksort_par", arr.len()),
        &arr,
        |b, arr| b.iter(|| quicksort_par(arr)),
    );

    group.finish();
}

criterion_group!(benches, quicksort_benchmark);
criterion_main!(benches);
