#![allow(unused)]

use std::alloc::{Layout, alloc, dealloc};
use std::ptr::{self, copy_nonoverlapping, read, write};

/// non-recursive merge sort
/// use pointer instead of ref during merging
/// this function is safe because it restore all of data at once
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

/// non-recursive merge sort by comp
/// use pointer instead of ref during merging
/// this function is safe because it restore all of data at once
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
