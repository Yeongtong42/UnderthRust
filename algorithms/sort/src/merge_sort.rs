#![allow(unused)]

use std::alloc::{Layout, alloc, dealloc};
use std::ptr::{read, write};

pub fn merge_sort<T: Ord>(slice: &mut [T]) {
    // slice size check
    let len = slice.len();
    if len <= 1 {
        // already sorted
        return;
    }

    // buffer allocation
    let mut cache = 0 as *mut T;
    let layout = Layout::array::<T>(len).unwrap();
    unsafe {
        cache = alloc(layout) as *mut T;
    }
    if cache.is_null() {
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
                    write(cache.add(i), next_val);
                }
            }
            merge_start_pos += (seg_size << 1);
        }

        // write back ordered seg from cache
        unsafe {
            for i in 0..merge_start_pos.min(len) {
                write(&mut slice[i] as *mut T, cache.add(i).read());
            }
        }
        seg_size = seg_size << 1;
    }

    unsafe {
        dealloc(cache as *mut u8, layout);
    }
}

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
    let mut cache = 0 as *mut T;
    let layout = Layout::array::<T>(len).unwrap();
    unsafe {
        cache = alloc(layout) as *mut T;
    }
    if cache.is_null() {
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
                    write(cache.add(i), next_val);
                }
            }
            merge_start_pos += (seg_size << 1);
        }

        // write back ordered seg from cache
        unsafe {
            for i in 0..merge_start_pos.min(len) {
                write(&mut slice[i] as *mut T, cache.add(i).read());
            }
        }
        seg_size = seg_size << 1;
    }

    unsafe {
        dealloc(cache as *mut u8, layout);
    }
}

#[cfg(test)]
mod tests {}
