//! # Description
//! Implementation of tim sort.

#![allow(unused)]

use std::alloc::{Layout, alloc, dealloc};
use std::collections::VecDeque;

type Run = (usize, usize);

pub fn tim_sort<T: Ord>(slice: &mut [T]) {
    tim_sort_by(slice, T::cmp)
}

pub fn tim_sort_by<T, F>(slice: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    // calculate min run size
    let size = slice.len();
    let (min_run_size, max_run_cnt) = get_min_run_size(size);
    let mut runs: Vec<Run> = Vec::with_capacity(max_run_cnt);

    // split slice into runs
    // half open range
    let mut run_start_pos = 0;
    while run_start_pos < size {
        let run_end_pos =
            get_sorted_run_from_slice(slice, &mut compare, run_start_pos, min_run_size);

        // add new run
        runs.push((run_start_pos, run_end_pos));

        // move start pos
        run_start_pos = run_end_pos;
    }

    // no need to merge
    if runs.len() == 1 {
        return;
    }

    // buffer allocation
    // allocate additional space for merge
    let merge_buffer;
    let layout = Layout::array::<T>(size / 2 + 1).unwrap();
    unsafe {
        merge_buffer = alloc(layout) as *mut T;
    }
    if merge_buffer.is_null() {
        // allocation failed
        panic!();
    }

    // merge runs using stacks
    let mut run_stack: VecDeque<Run> = VecDeque::new();
    // init run stack
    for cur_run in runs {
        run_stack.push_back(cur_run);
        keep_run_stack_invariant(slice, &mut compare, merge_buffer, &mut run_stack);
    }

    // merge all run in the stack
    while run_stack.len() > 1 {
        // merge top two run in the stack
        let cur_run = run_stack.pop_back().unwrap();
        merge_run(
            slice,
            &mut compare,
            merge_buffer,
            run_stack.back().unwrap().clone(),
            cur_run.clone(),
        );
        run_stack.back_mut().unwrap().1 = cur_run.1;
    }

    // delete merge buffer
    unsafe {
        dealloc(merge_buffer as *mut u8, layout);
    }
}

/// min_run = min(n, 32~64)
///
fn get_min_run_size(n: usize) -> (usize, usize) {
    let mut min_run_size = n;
    while min_run_size > 64 {
        min_run_size = min_run_size >> 1;
    }
    let max_run_cnt = (n / min_run_size) + ((n % min_run_size) > 0) as usize;
    (min_run_size, max_run_cnt)
}

// sort min run and append
// return end pos of processed run
fn get_sorted_run_from_slice<T, F>(
    slice: &mut [T],
    mut compare: F,
    run_start_pos: usize,
    min_run_size: usize,
) -> usize
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    let size = slice.len();

    // one off error
    if run_start_pos == size - 1 {
        return size;
    }

    // decide either increase or not
    let is_run_increase = compare(&slice[run_start_pos], &slice[run_start_pos + 1]).is_le();

    // sort min run
    let mut run_end_pos = (run_start_pos + min_run_size).min(size);
    binary_insertion_sort_by(
        &mut slice[run_start_pos..run_end_pos],
        &mut compare,
        is_run_increase,
    );

    // append run
    if is_run_increase {
        while run_end_pos < size && compare(&slice[run_end_pos - 1], &slice[run_end_pos]).is_le() {
            run_end_pos += 1;
        }
    } else {
        while run_end_pos < size && compare(&slice[run_end_pos - 1], &slice[run_end_pos]).is_gt() {
            run_end_pos += 1;
        }
    }

    // reverse decrease run
    if !is_run_increase {
        (&mut slice[run_start_pos..run_end_pos]).reverse();
    }
    run_end_pos
}

/// insertion sort with binary search
/// use std::slice::partition_point to search insertion point
/// partition_point method use binary_search like algorithm
fn binary_insertion_sort_by<T, F>(slice: &mut [T], mut comp: F, is_inc: bool)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    let len = slice.len();
    let mut cur_pos = 1usize;
    while cur_pos < len {
        let insertion_pos = match is_inc {
            true => slice[0..cur_pos].partition_point(|x| comp(x, &slice[cur_pos]).is_le()),
            false => slice[0..cur_pos].partition_point(|x| comp(x, &slice[cur_pos]).is_gt()),
        };
        slice[insertion_pos..=cur_pos].rotate_right(1);
        cur_pos += 1;
    }
}

// update run_stack
fn keep_run_stack_invariant<T, F>(
    slice: &mut [T],
    mut compare: F,
    merge_buffer: *mut T,
    run_stack: &mut VecDeque<Run>,
) where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    while !is_run_stack_ok(run_stack) {
        let first_from_top = run_stack.pop_back().unwrap();
        let second_from_top = run_stack.pop_back().unwrap();
        let third_from_top = run_stack.pop_back().unwrap();

        if is_run_gt(&third_from_top, &first_from_top) {
            merge_run(
                slice,
                &mut compare,
                merge_buffer,
                second_from_top,
                first_from_top,
            );
            run_stack.push_back(third_from_top);
            run_stack.push_back((second_from_top.0, first_from_top.1));
        } else {
            merge_run(
                slice,
                &mut compare,
                merge_buffer,
                third_from_top,
                second_from_top,
            );
            run_stack.push_back((third_from_top.0, second_from_top.1));
            run_stack.push_back(first_from_top);
        }
    }
}

fn is_run_stack_ok(run_stack: &mut VecDeque<Run>) -> bool {
    let size = run_stack.len();
    if size < 3 {
        return true;
    }

    let first = run_stack[size - 1];
    let second = run_stack[size - 2];
    let third = run_stack[size - 3];
    let first = first.1 - first.0;
    let second = second.1 - second.0;
    let third = third.1 - third.0;
    return first < second && (first + second) < third;
}

fn is_run_gt(r1: &Run, r2: &Run) -> bool {
    (r1.1 - r1.0) > (r2.1 - r2.0)
}

//
fn merge_run<T, F>(slice: &mut [T], mut comp: F, merge_buffer: *mut T, run1: Run, run2: Run) {
    todo!();
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
    fn test_min_run_size() {
        let n = 1088;
        let (min_run_size, max_run_cnt) = get_min_run_size(n);
        assert_eq!(min_run_size, 34);
        assert_eq!(max_run_cnt, 32);
    }

    #[test]
    fn test_binary_insertion_sort_by() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(1000).collect();

        binary_insertion_sort_by(
            &mut vec,
            |a: &i32, b: &i32| Reverse(a).cmp(&Reverse(b)),
            true,
        );

        assert!(vec.is_sorted_by(|&a, &b| { a > b }));
    }

    #[test]
    fn test_get_sorted_run_inc() {
        let mut slice = [2, 3, -1, -5, 0, 4, 11, 15];
        let run_end_pos = get_sorted_run_from_slice(&mut slice, i32::cmp, 0, 5);
        assert_eq!(run_end_pos, 8);
        assert_eq!(slice, [-5, -1, 0, 2, 3, 4, 11, 15]);
    }

    #[test]
    fn test_get_sorted_run_dec() {
        let mut slice = [3, 2, -1, -5, 0, 4, 11, 15];
        let run_end_pos = get_sorted_run_from_slice(&mut slice, i32::cmp, 0, 4);
        assert_eq!(run_end_pos, 4);
        assert_eq!(slice, [-5, -1, 2, 3, 0, 4, 11, 15]);
    }

    #[test]
    fn test_tim_sort_by() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

        tim_sort_by(&mut vec, |a: &i32, b: &i32| Reverse(a).cmp(&Reverse(b)));

        assert!(vec.is_sorted_by(|&a, &b| { a > b }));
    }
}
