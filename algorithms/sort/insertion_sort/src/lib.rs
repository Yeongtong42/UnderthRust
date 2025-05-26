#![allow(unused)]

use std::ptr::{read, write};

/// insertion sort
/// use pointer for insertion instead of reference
/// this function can cause panic during comparing
/// this function is unsafe because it loses owner ship of some data when panic rewinding occurs
pub unsafe fn insertion_sort<T: Ord>(slice: &mut [T]) {
    if slice.is_empty() {
        return;
    }
    let len = slice.len();
    let begin = &mut slice[0] as *mut T;
    for i in 1..len {
        unsafe {
            let right_hand = begin.add(i).read();
            let mut j = i;
            while j > 0 && slice[j - 1] > right_hand {
                write(begin.add(j), begin.add(j - 1).read());
                j -= 1;
            }
            write(begin.add(j), right_hand);
        }
    }
}

/// insertion sort by compare
/// use pointer for insertion instead of reference
/// this function can cause panic during comparing
/// this function is unsafe because it loses owner ship of some data when panic rewinding occurs
pub unsafe fn insertion_sort_by<T, F>(slice: &mut [T], comp: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    let mut cmp = comp;
    use std::cmp::Ordering as O;
    if slice.is_empty() {
        return;
    }
    let len = slice.len();
    let begin = &mut slice[0] as *mut T;
    for i in 1..len {
        unsafe {
            let right_hand = begin.add(i).read();
            let mut j = i;
            while j > 0 && (O::Greater == cmp(&slice[j - 1], &right_hand)) {
                write(begin.add(j), begin.add(j - 1).read());
                j -= 1;
            }
            write(begin.add(j), right_hand);
        }
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
    fn test_insertion_sort() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

        unsafe {
            insertion_sort(&mut vec);
        }

        assert!(vec.is_sorted());
    }

    #[test]
    fn test_insertion_sort_by() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

        unsafe {
            insertion_sort_by(&mut vec, |a: &i32, b: &i32| Reverse(a).cmp(&Reverse(b)));
        }

        assert!(vec.is_sorted_by(|&a, &b| { a > b }));
    }
}
