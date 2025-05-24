#![allow(unused)]

use std::alloc::{Layout, alloc, dealloc};
use std::ptr::{self, copy_nonoverlapping, read, write};

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
            copy_nonoverlapping(merge_buffer, &mut slice[0] as *mut T, merge_start_pos.min(len));
        }
        seg_size = seg_size << 1;
    }

    unsafe {
        dealloc(merge_buffer as *mut u8, layout);
    }
}

/// comp must not panic or data in slice can be lost
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
            copy_nonoverlapping(merge_buffer, &mut slice[0] as *mut T, merge_start_pos.min(len));
        }
        seg_size = seg_size << 1;
    }

    unsafe {
        dealloc(merge_buffer as *mut u8, layout);
    }
}

#[cfg(test)]
mod tests {}
