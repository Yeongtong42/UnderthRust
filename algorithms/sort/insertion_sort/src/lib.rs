//! # Description
//! Implementation of insertion sort.

/// # Description
/// Sorts the given slice stable using a insertion‑sort algorithm.
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
/// # Safety
/// This function is safe because it restore all of data at once.
/// Despite of the panic, there are no occurence of duplicated ownership.
///
/// # Examples
/// ```
/// use insertion_sort::*;
/// let mut v = vec![3, 1, 4, 1, 5];
/// insertion_sort(&mut v);
/// assert_eq!(v, vec![1, 1, 3, 4, 5]);
/// ```
pub fn insertion_sort<T: Ord>(slice: &mut [T]) {
    insertion_sort_by(slice, T::cmp);
}

/// # Description
/// Sorts the given slice stable using a insertion‑sort algorithm with comparator.
///
/// # Type Parameters
/// - `T`: The element type.
/// - `F`: type of comparator. Must implement 'FnMut'
///
/// # Parameters
/// - `slice`: The mutable slice to sort.
/// - `comp`: The callable object to compare two data of type T.
///
/// # Panics
/// Panics if calculating partition indices overflows (only for very large slices).
/// Panics if the implementation of Ord panics.
///
/// # Safety
/// This function is safe because it restore all of data at once.
/// Despite of the panic, there are no occurence of duplicated ownership.
///
/// # Examples
/// ```
/// use insertion_sort::*;
/// let mut v = vec![3, 1, 4, 1, 5];
/// insertion_sort_by(&mut v, |a : &i32, b:&i32| {a.cmp(b)});
/// assert_eq!(v, vec![1, 1, 3, 4, 5]);
/// ```
pub fn insertion_sort_by<T, F>(slice: &mut [T], mut comp: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    use std::cmp::Ordering as O;
    if slice.is_empty() {
        return;
    }
    let len = slice.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && (O::Greater == comp(&slice[j - 1], &slice[i])) {
            j -= 1;
        }
        slice[j..=i].rotate_right(1);
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;

    use rand::distr::StandardUniform;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    use crate::*;

    const TEST_SIZE: usize = 10_000;

    #[test]
    fn test_insertion_sort_with_empty() {
        let mut vec: Vec<i32> = Vec::new();
        insertion_sort(&mut vec);
        assert!(vec.is_sorted());
    }

    #[test]
    fn test_insertion_sort_with_size1() {
        let mut vec: Vec<i32> = vec![0; 1];
        insertion_sort(&mut vec);
        assert!(vec.is_sorted());
    }

    #[test]
    fn test_insertion_sort() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

        insertion_sort(&mut vec);

        assert!(vec.is_sorted());
    }

    #[test]
    fn test_insertion_sort_by() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

        insertion_sort_by(&mut vec, |a: &i32, b: &i32| Reverse(a).cmp(&Reverse(b)));

        assert!(vec.is_sorted_by(|&a, &b| { a > b }));
    }
}
