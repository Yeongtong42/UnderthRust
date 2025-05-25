#![allow(unused)]

/// partition slice with pivot at end
fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let len = slice.len();
    let pivot = len - 1;
    let mut cur_left_pos = 0usize;

    for i in 0..len {
        if slice[i] <= slice[pivot] {
            slice.swap(cur_left_pos, i);
            cur_left_pos += 1;
        }
    }
    cur_left_pos - 1
}

/// sort slice with basic quick-sort algorithm
pub fn binary_quick_sort<T: Ord>(slice: &mut [T]) {
    let len = slice.len();
    if len <= 1 {
        return;
    }

    // partition
    let pivot_pos = partition(slice);

    // recurse two part
    binary_quick_sort(&mut slice[0..pivot_pos]);
    binary_quick_sort(&mut slice[pivot_pos + 1..len]);
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
    let len = slice.len();
    if len <= 1 {
        return;
    }

    // partition
    let pivot_pos = partition_by(slice, comp);

    // recurse two part
    quick_sort_by_comp(&mut slice[0..pivot_pos], comp);
    quick_sort_by_comp(&mut slice[pivot_pos + 1..len], comp)
}

/// sort slice with basic quick-sort algorithm
pub fn binary_quick_sort_by<T, F>(slice: &mut [T], comp: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    let mut cmp = comp;
    quick_sort_by_comp(slice, &mut cmp);
}

#[cfg(test)]
mod tests {

    use rand::distr::StandardUniform;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    use crate::binary_quick_sort::partition;

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

        let pivot_pos = partition(&mut vec);

        assert!(is_partitioned(&vec, pivot_pos));
    }
}
