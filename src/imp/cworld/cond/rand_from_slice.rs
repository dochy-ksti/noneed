use rand::prelude::*;
use std::ops::Index;

pub(crate) fn rand_from_slice<T>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        None
    } else {
        let mut r = rand::thread_rng();
        let n = r.gen_range(0..slice.len());
        Some(slice.index(n))
    }
}
