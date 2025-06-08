//! # Description
//! Implementation of intro-sort algorithm.
use heap_on_slice::max_heap;
use insertion_sort::insertion_sort_by;
use quick_sort::ternary_partition_by;

/// # Description
/// Sorts the given slice in-place using a intro‑sort algorithm.
///
/// # Type Parameters
/// - `T`: The element type. Must implement `Ord`.
///
/// # Parameters
/// - `slice`: The mutable slice to sort.
///
/// # Panics
/// Panics if calculating partition indices overflows (only for very large slices).
/// Panics if the implementation of Ord panics.
///
/// # Examples
/// ```
/// use intro_sort::intro_sort;
/// let mut v = vec![3, 1, 4, 1, 5];
/// intro_sort(&mut v);
/// assert_eq!(v, vec![1, 1, 3, 4, 5]);
/// ```
pub fn intro_sort<T: Ord>(slice: &mut [T]) {
    intro_sort_by(slice, T::cmp)
}

/// # Description
/// Sorts the given slice in-place using a intro‑sort algorithm
/// whith comparator.
///
/// # Type Parameters
/// - `T`: The element type.
/// - `F`: The comparator type. Must implement 'FnMut' trait.
///
/// # Parameters
/// - `slice`: The mutable slice to sort.
/// - `comp`: The callable object to compare two &T data.
///
/// # Panics
/// Panics if calculating partition indices overflows (only for very large slices).
/// Panics if the implementation of 'comp' panics.
///
/// # Examples
/// ```
/// use intro_sort::intro_sort_by;
/// let mut v = vec![3, 1, 4, 1, 5];
/// intro_sort_by(&mut v, |a : &i32, b : &i32|{ a.cmp(b) });
/// assert_eq!(v, vec![1, 1, 3, 4, 5]);
/// ```
pub fn intro_sort_by<T, F>(slice: &mut [T], comp: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    let len = slice.len();
    if len == 0 {
        return;
    }
    let max_depth = (usize::ilog2(len)) << 1;

    let mut comp = comp;
    intro_recurse_sort_by(slice, &mut comp, max_depth);
}

fn intro_recurse_sort_by<T, F>(slice: &mut [T], comp: &mut F, max_depth: u32)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    if slice.len() < 16 {
        return insertion_sort_by(slice, comp);
    } else if max_depth == 0 {
        return max_heap::heapsort_by(slice, comp);
    }

    // quick sort
    // partition
    let (pivot1, pivot2) = ternary_partition_by(slice, comp);

    // recurse
    intro_recurse_sort_by(&mut slice[0..pivot1 - 1], comp, max_depth - 1);
    intro_recurse_sort_by(&mut slice[pivot1..pivot2], comp, max_depth - 1);
    intro_recurse_sort_by(&mut slice[pivot2 + 1..], comp, max_depth - 1);
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::cmp::Reverse;

    use rand::distr::StandardUniform;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    const TEST_SIZE: usize = 10_000;

    #[test]
    fn test_ternary_quick_sort() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

        intro_sort(&mut vec);

        assert!(vec.is_sorted());
    }

    #[test]
    fn test_ternary_quick_sort_by() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

        intro_sort_by(&mut vec, |a: &i32, b: &i32| Reverse(a).cmp(&Reverse(b)));

        assert!(vec.is_sorted_by(|&a, &b| { a > b }));
    }
}
