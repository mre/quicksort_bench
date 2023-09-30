use rand::Rng;
use rayon::prelude::*;

/// Mutable, in-place quicksort implementation
pub fn quicksort_mut<T: PartialOrd + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quicksort_mut(&mut arr[0..pivot_index]);
    quicksort_mut(&mut arr[pivot_index + 1..]);
}

/// This is a version of the partition function that uses the first value as the pivot,
/// which is not optimal, because it is very easy to construct an array that will cause this
/// implementation to run in O(n^2) time
/// However, it is a good example of why it is important to choose a good pivot
/// and therefore is included for completeness' sake
#[allow(dead_code)]
fn partition_first_value_pivot<T: PartialOrd + Clone>(arr: &mut [T]) -> usize {
    let pivot = arr[0].clone();
    let mut i = 1;
    let mut j = 1;

    while j < arr.len() {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    arr.swap(0, i - 1);
    i - 1
}

fn partition<T: PartialOrd + Clone>(arr: &mut [T]) -> usize {
    let pivot_index = rand::thread_rng().gen_range(0..arr.len());
    arr.swap(0, pivot_index); // Move the chosen pivot to the beginning

    let pivot = arr[0].clone();
    let mut i = 1;
    let mut j = 1;

    while j < arr.len() {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    arr.swap(0, i - 1);
    i - 1
}

/// pdqsort implementation in the standard library (unstable)
/// This serves as a lower bound for the performance of the other implementations
pub fn quicksort_stdlib<T: Ord + Clone>(array: &[T]) -> Vec<T> {
    let mut sorted = array.to_vec();
    // Simply use the standard library's unstable sort
    sorted.sort_unstable();
    sorted
}

/// Immutable quicksort implementation
pub fn quicksort<T: PartialOrd + Clone>(array: &[T]) -> Vec<T> {
    if array.len() <= 1 {
        return array.to_vec();
    }

    let pivot = &array[0];
    let higher: Vec<T> = array[1..].iter().cloned().filter(|x| x > pivot).collect();
    let lower: Vec<T> = array[1..].iter().cloned().filter(|x| x <= pivot).collect();

    [quicksort(&lower), vec![pivot.clone()], quicksort(&higher)].concat()
}

/// Immutable quicksort implementation with partition, version 1
pub fn quicksort_partition_v1<T: PartialOrd + Clone>(array: &[T]) -> Vec<T> {
    if array.len() <= 1 {
        return array.to_vec();
    }

    let pivot = &array[0];
    let (higher, lower): (Vec<_>, Vec<_>) = array[1..].iter().cloned().partition(|x| x > pivot);

    [
        quicksort_partition_v1(&lower),
        vec![pivot.clone()],
        quicksort_partition_v1(&higher),
    ]
    .concat()
}

/// Immutable quicksort implementation with partition
pub fn quicksort_partition_v2<T: PartialOrd + Clone>(array: &[T]) -> Vec<T> {
    if array.len() <= 1 {
        return array.to_vec();
    }

    let pivot = &array[0];
    let (higher, lower): (Vec<_>, Vec<_>) = array[1..].iter().cloned().partition(|x| x > pivot);

    quicksort_partition_v2(&lower)
        .iter()
        .cloned()
        .chain(std::iter::once(pivot.clone()))
        .chain(quicksort_partition_v2(&higher).iter().cloned())
        .collect()
}

/// Immutable quicksort implementation with partition
pub fn quicksort_partition_v3<T: PartialOrd + Clone>(array: &[T]) -> Vec<T> {
    if array.len() <= 1 {
        return array.to_vec();
    }

    let pivot = array[0].clone();
    let (lower, higher): (Vec<_>, Vec<_>) = array[1..].iter().cloned().partition(|x| x <= &pivot);

    quicksort_partition_v3(&lower)
        .into_iter()
        .chain(std::iter::once(pivot))
        .chain(quicksort_partition_v3(&higher).into_iter())
        .collect()
}

use rand::seq::SliceRandom;

/// Immutable quicksort implementation with partition
pub fn quicksort_partition<T: PartialOrd + Clone>(array: &[T]) -> Vec<T> {
    if array.len() <= 1 {
        return array.to_vec();
    }

    // Safety: array is guaranteed to not be empty
    let pivot = array.choose(&mut rand::thread_rng()).unwrap().clone();
    let (lower, higher): (Vec<_>, Vec<_>) = array[1..].iter().cloned().partition(|x| x <= &pivot);

    quicksort_partition(&lower)
        .into_iter()
        .chain(std::iter::once(pivot))
        .chain(quicksort_partition(&higher).into_iter())
        .collect()
}

/// Uses rayon to parallelize the quicksort algorithm
pub fn quicksort_par<T: PartialOrd + Clone + Sync + Send>(array: &[T]) -> Vec<T> {
    if array.len() <= 1 {
        return array.to_vec();
    }

    let pivot = &array[0];
    let higher: Vec<T> = array[1..]
        .par_iter()
        .cloned()
        .filter(|x| x > pivot)
        .collect();
    let lower: Vec<T> = array[1..]
        .par_iter()
        .cloned()
        .filter(|x| x <= pivot)
        .collect();

    [
        quicksort_par(&lower),
        vec![pivot.clone()],
        quicksort_par(&higher),
    ]
    .concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn is_sorted<T: PartialOrd + std::fmt::Debug>(arr: &[T]) -> bool {
        arr.windows(2).all(|win| win[0] <= win[1])
    }

    fn get_test_vecs() -> Vec<Vec<u64>> {
        vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![2, 1],
            vec![1, 2, 3],
            vec![2, 1, 3],
            vec![3, 1, 2],
            vec![8, 5, 2, 6, 9, 3],
            vec![2, 3, 5, 6, 8, 9],
            vec![9, 8, 6, 5, 3, 2],
            vec![8, 4, 7, 3, 6, 2, 5, 1],
            vec![8, 1, 7, 2, 6, 3, 5, 4],
            vec![8, 1, 7, 2, 6, 3, 5, 4],
            vec![16, 14, 1, 1, 7, 18, 7, 6, 8, 18, 5],
            vec![19, 18, 14, 15, 3, 9, 8, 2, 2, 20, 11],
            vec![
                2, 3, 8, 7, 23, 26, 19, 29, 23, 32, 20, 18, 11, 11, 24, 13, 17,
            ],
            vec![0, 3, 7, 6],
            vec![6, 4, 4, 5, 11, 10, 10],
            vec![15, 13, 17, 1, 1, 19, 3, 19, 0, 11],
            vec![
                19, 19, 21, 21, 22, 25, 19, 14, 23, 25, 14, 10, 8, 4, 28, 12, 2, 33,
            ],
            vec![8, 1, 0, 5, 3],
            vec![
                27, 14, 22, 10, 8, 23, 7, 32, 28, 31, 9, 19, 30, 28, 21, 20, 13,
            ],
            vec![2, 1, 4, 17, 5, 17, 8, 2, 13, 13],
        ]
    }

    #[test]
    fn test_quicksort_mut() {
        let tests = get_test_vecs();

        for mut test in tests {
            quicksort_mut(&mut test);
            assert!(is_sorted(&test));
        }
    }

    #[test]
    fn test_quicksort() {
        let tests = get_test_vecs();

        for test in tests {
            let result = quicksort(&test);
            assert!(is_sorted(&result));
        }
    }

    #[test]
    fn test_quicksort_partition() {
        let tests = get_test_vecs();

        for test in tests {
            let result = quicksort_partition(&test);
            assert!(is_sorted(&result));
        }
    }

    #[test]
    fn test_quicksort_par() {
        let tests = get_test_vecs();

        for test in tests {
            let result = quicksort_par(&test);
            assert!(is_sorted(&result));
        }
    }
}
