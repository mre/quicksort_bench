//! Binary for flamegraph

use quicksort_bench::quicksort_partition;

fn main() {
    let arr = (0..1_000_000)
        .map(|_| rand::random::<u32>())
        .collect::<Vec<u32>>();
    println!("arr.len() = {}", arr.len());
    println!("arr[0..10] = {:?}", &arr[0..10]);

    quicksort_partition(&arr);
}
