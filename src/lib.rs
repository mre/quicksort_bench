use rayon::prelude::*;

/// Mutable, in-place quicksort implementation
pub fn quicksort_mut<T: PartialOrd>(mut arr: Vec<T>) -> Vec<T> {
    if arr.len() <= 1 {
        return arr;
    }

    let pivot = arr.remove(0);
    let mut left = vec![];
    let mut right = vec![];

    for item in arr {
        if item <= pivot {
            left.push(item);
        } else {
            right.push(item);
        }
    }

    let mut sorted_left = quicksort_mut(left);
    let mut sorted_right = quicksort_mut(right);

    sorted_left.push(pivot);
    sorted_left.append(&mut sorted_right);

    sorted_left
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

/// Immutable quicksort implementation with partition
pub fn quicksort_partition<T: PartialOrd + Clone>(array: &[T]) -> Vec<T> {
    if array.len() <= 1 {
        return array.to_vec();
    }

    let pivot = &array[0];
    let (higher, lower): (Vec<_>, Vec<_>) = array[1..].iter().cloned().partition(|x| x > pivot);

    [
        quicksort_partition(&lower),
        vec![pivot.clone()],
        quicksort_partition(&higher),
    ]
    .concat()
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

        for test in tests {
            let result = quicksort_mut(test);
            assert!(is_sorted(&result));
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
