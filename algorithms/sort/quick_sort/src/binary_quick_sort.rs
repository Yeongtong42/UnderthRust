#![allow(unused)]

/// # Description
/// Sorts the given slice in-place using a basic partition quick‑sort algorithm.
///
/// # Type Parameters
/// - `T`: The element type. Must implement `Ord`.
///
/// # Parameters
/// - `slice`: The mutable slice to sort.
///
/// # Panics
/// Panics if calculating partition indices overflows (only for very large slices).
/// Panics if the implementation of Ord panics.///
///
/// # Examples
/// ```
/// use quick_sort::binary_quick_sort;
/// let mut v = vec![3, 1, 4, 1, 5];
/// binary_quick_sort(&mut v);
/// assert_eq!(v, vec![1, 1, 3, 4, 5]);
/// ```
pub fn binary_quick_sort<T: Ord>(slice: &mut [T]) {
    binary_quick_sort_by(slice, T::cmp)
}

/// partition slice with pivot at end by comp
fn partition_by<T, F>(slice: &mut [T], comp: &mut F) -> usize
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    use std::cmp::Ordering as O;
    let mut cmp = comp;
    let len = slice.len();
    let pivot = len - 1;
    let mut cur_left_pos = 0usize;

    for i in 0..len {
        if O::Greater != cmp(&slice[i], &slice[pivot]) {
            slice.swap(cur_left_pos, i);
            cur_left_pos += 1;
        }
    }
    cur_left_pos - 1
}

fn quick_sort_by_comp<T, F>(slice: &mut [T], comp: &mut F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    if slice.len() <= 1 {
        return;
    }

    // partition
    let pivot_pos = partition_by(slice, comp);

    // recurse two part
    quick_sort_by_comp(&mut slice[0..pivot_pos], comp);
    quick_sort_by_comp(&mut slice[pivot_pos + 1..], comp)
}

/// # Description
/// Sorts the given slice in-place using a basic partition quick‑sort algorithm
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
/// Panics if the implementation of 'comp' panics.///
///
/// # Examples
/// ```
/// use quick_sort::binary_quick_sort_by;
/// let mut v = vec![3, 1, 4, 1, 5];
/// binary_quick_sort_by(&mut v, |a : &i32, b : &i32|{ a.cmp(b) });
/// assert_eq!(v, vec![1, 1, 3, 4, 5]);
/// ```
pub fn binary_quick_sort_by<T, F>(slice: &mut [T], mut comp: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    quick_sort_by_comp(slice, &mut comp);
}

#[cfg(test)]
mod tests {

    use crate::*;
    use std::cmp::Reverse;

    use rand::distr::StandardUniform;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    use crate::binary_quick_sort::partition_by;

    const TEST_SIZE: usize = 10_000;

    fn is_partitioned(slice: &[i32], pivot_pos: usize) -> bool {
        let len = slice.len();

        for i in 0..pivot_pos {
            if slice[i] > slice[pivot_pos] {
                return false;
            }
        }
        for i in (pivot_pos + 1)..len {
            if slice[i] <= slice[pivot_pos] {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_partition() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(100).collect();

        let pivot_pos = partition_by(&mut vec, &mut i32::cmp);

        assert!(is_partitioned(&vec, pivot_pos));
    }

    #[test]
    fn test_binary_quick_sort() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

        binary_quick_sort(&mut vec);

        assert!(vec.is_sorted());
    }

    #[test]
    fn test_binary_quick_sort_by() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

        binary_quick_sort_by(&mut vec, |a: &i32, b: &i32| Reverse(a).cmp(&Reverse(b)));

        assert!(vec.is_sorted_by(|&a, &b| { a > b }));
    }
}
