use rayon::prelude::*;

pub fn quicksort<T: Ord + Clone>(array: &[T]) -> Vec<T> {
    if array.len() <= 1 {
        return array.to_vec();
    }

    let pivot = &array[0];
    let higher: Vec<T> = array[1..].iter().cloned().filter(|x| x > pivot).collect();
    let lower: Vec<T> = array[1..].iter().cloned().filter(|x| x <= pivot).collect();

    [quicksort(&lower), vec![pivot.clone()], quicksort(&higher)].concat()
}

pub fn quicksort_par<T: Ord + Clone + Sync + Send>(array: &[T]) -> Vec<T> {
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

pub fn quicksort_mut<T: Ord>(mut arr: Vec<T>) -> Vec<T> {
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
