use criterion::{criterion_group, criterion_main, Criterion};
use quicksort_bench::quicksort_mut;

fn quick_sort_benchmark(c: &mut Criterion) {
    // Create a very large, 1 million element vector with random values
    let mut arr = (0..1_000_000)
        .map(|_| rand::random::<u32>())
        .collect::<Vec<u32>>();
    println!("arr.len() = {}", arr.len());
    println!("arr[0..10] = {:?}", &arr[0..10]);

    c.bench_function("quick_sort_mut", |b| b.iter(|| quicksort_mut(&mut arr)));
}

criterion_group!(benches, quick_sort_benchmark);
criterion_main!(benches);
