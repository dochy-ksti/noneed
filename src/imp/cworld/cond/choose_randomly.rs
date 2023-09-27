use rand::prelude::*;
use std::ops::Index;

/// None only when the slice is empty.
pub(crate) fn rand_from_slice<T>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        None
    } else {
        let mut r = rand::thread_rng();
        let n = r.gen_range(0..slice.len());
        Some(slice.index(n))
    }
}

/// When the iterator is empty or every weight is zero, None is returned.
pub(crate) fn rand_with_weight<'a, T>(
    wts: impl Iterator<Item = (usize, &'a T)>,
    weight_sum: usize,
) -> Option<&'a T> {
    let mut r = rand::thread_rng();
    let n = r.gen_range(0..weight_sum);
    let mut sum = 0;
    let mut result = None;
    for (weight, item) in wts {
        sum += weight;
        if n < sum {
            result = Some(item);
        }
    }
    return result;
}
