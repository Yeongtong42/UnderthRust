//! # Description
//! Implementation of merge-sort algorithm.

#![allow(unused)]

use std::alloc::{Layout, alloc, dealloc};
use std::ptr::{copy_nonoverlapping, write};

/// # Description
/// Sorts the given slice stable using a non-recursive merge‑sort algorithm.
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
/// But, this function allocates internal memeory, so there can be a leak.
///
/// # Examples
/// ```
/// use merge_sort::*;
/// let mut v = vec![3, 1, 4, 1, 5];
/// merge_sort(&mut v);
/// assert_eq!(v, vec![1, 1, 3, 4, 5]);
/// ```
pub fn merge_sort<T: Ord>(slice: &mut [T]) {
    // slice size check
    let len = slice.len();
    if len <= 1 {
        // already sorted
        return;
    }

    // buffer allocation
    let mut merge_buffer = 0 as *mut T;
    let layout = Layout::array::<T>(len).unwrap();
    unsafe {
        merge_buffer = alloc(layout) as *mut T;
    }
    if merge_buffer.is_null() {
        // allocation failed
        panic!();
    }

    // merge sort, non-recursive
    let mut seg_size = 1;
    while seg_size < len {
        let mut merge_start_pos = 0usize;
        // sort each seg
        loop {
            let begin = merge_start_pos;
            let mid = begin + seg_size;
            let end = std::cmp::min(mid + seg_size, len);
            if (mid >= len) {
                // already sorted
                break;
            }

            // merge two seg
            let mut l = begin;
            let mut r = mid;
            // merge left and right to the cache
            for i in begin..end {
                unsafe {
                    let next_val = match (r == end || (l != mid && slice[l] < slice[r])) {
                        true => {
                            let tmp = (&slice[l] as *const T).read();
                            l += 1;
                            tmp
                        }
                        false => {
                            let tmp = (&slice[r] as *const T).read();
                            r += 1;
                            tmp
                        }
                    };
                    write(merge_buffer.add(i), next_val);
                }
            }
            merge_start_pos += (seg_size << 1);
        }

        // write back ordered seg from cache
        unsafe {
            copy_nonoverlapping(
                merge_buffer,
                &mut slice[0] as *mut T,
                merge_start_pos.min(len),
            );
        }
        seg_size = seg_size << 1;
    }

    unsafe {
        dealloc(merge_buffer as *mut u8, layout);
    }
}

/// # Description
/// Sorts the given slice stable using a non-recursive merge‑sort algorithm with comparator.
///
/// # Type Parameters
/// - `T`: The element type.
/// - 'F': type of comparator. Must implement 'FnMut'
///
/// # Parameters
/// - `slice`: The mutable slice to sort.
/// - 'comp': The callable object to compare two data of type T.
///
/// # Panics
/// Panics if calculating partition indices overflows (only for very large slices).
/// Panics if the implementation of Ord panics.
///
/// # Safety
/// This function is safe because it restore all of data at once.
/// Despite of the panic, there are no occurence of duplicated ownership.
///
/// But, this function allocates internal memeory, so there can be a leak.
///
/// # Examples
/// ```
/// use merge_sort::*;
/// let mut v = vec![3, 1, 4, 1, 5];
/// merge_sort(&mut v);
/// assert_eq!(v, vec![1, 1, 3, 4, 5]);
/// ```
pub fn merge_sort_by<T, F>(slice: &mut [T], comp: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    use std::cmp::Ordering as O;
    let mut cmp = comp;

    // slice size check
    let len = slice.len();
    if len <= 1 {
        // already sorted
        return;
    }

    // buffer allocation
    let mut merge_buffer = 0 as *mut T;
    let layout = Layout::array::<T>(len).unwrap();
    unsafe {
        merge_buffer = alloc(layout) as *mut T;
    }
    if merge_buffer.is_null() {
        // allocation failed
        panic!();
    }

    // merge sort, non-recursive
    let mut seg_size = 1;
    while seg_size < len {
        let mut merge_start_pos = 0usize;
        // sort each seg
        loop {
            let begin = merge_start_pos;
            let mid = begin + seg_size;
            let end = std::cmp::min(mid + seg_size, len);
            if (mid >= len) {
                // already sorted
                break;
            }

            // merge two seg
            let mut l = begin;
            let mut r = mid;
            // merge left and right to the cache
            for i in begin..end {
                unsafe {
                    let next_val =
                        match (r == end || (l != mid && (O::Less == cmp(&slice[l], &slice[r])))) {
                            true => {
                                let tmp = (&slice[l] as *const T).read();
                                l += 1;
                                tmp
                            }
                            false => {
                                let tmp = (&slice[r] as *const T).read();
                                r += 1;
                                tmp
                            }
                        };
                    write(merge_buffer.add(i), next_val);
                }
            }
            merge_start_pos += (seg_size << 1);
        }

        // write back ordered seg from cache
        unsafe {
            copy_nonoverlapping(
                merge_buffer,
                &mut slice[0] as *mut T,
                merge_start_pos.min(len),
            );
        }
        seg_size = seg_size << 1;
    }

    unsafe {
        dealloc(merge_buffer as *mut u8, layout);
    }
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
    fn test_merge_sort() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

        merge_sort(&mut vec);

        assert!(vec.is_sorted());
    }

    #[test]
    fn test_merge_sort_by() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

        merge_sort_by(&mut vec, |a: &i32, b: &i32| Reverse(a).cmp(&Reverse(b)));

        assert!(vec.is_sorted_by(|&a, &b| { a > b }));
    }
}
