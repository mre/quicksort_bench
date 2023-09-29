use criterion::{criterion_group, criterion_main, Criterion};
use quicksort_bench::quicksort;

fn quicksort_benchmark(c: &mut Criterion) {
    // Create a very large, 1 million element vector with random values
    let arr = (0..1_000_000)
        .map(|_| rand::random::<u32>())
        .collect::<Vec<u32>>();
    println!("arr.len() = {}", arr.len());
    println!("arr[0..10] = {:?}", &arr[0..10]);

    c.bench_function("quicksort", |b| b.iter(|| quicksort(&arr)));
}

criterion_group!(benches, quicksort_benchmark);
criterion_main!(benches);
