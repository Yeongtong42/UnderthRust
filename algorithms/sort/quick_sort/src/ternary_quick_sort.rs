#![allow(unused)]

/// # Description
/// Partition slice in 3 part and return it's delimeter.
/// This function is based on Dijkstra's Dutch national flag algorithm.
/// Two pivot's are one at the front and the other at back.,
///
/// # Type Parameters
/// - `T`: The element type. Must implement `Ord`.
///
/// # Parameters
/// - `slice`: The mutable slice to partition.
///
/// # Panics
/// Panics if the implementation of Ord panics.
///
/// # Examples
/// ```
/// use quick_sort::ternary_partition;
/// let mut v = vec![3, 1, 4, 1, 5];
/// let (i, j) = ternary_partition(&mut v);
/// assert_eq!(i, 3);
/// assert_eq!(j, 4);
/// assert_eq!(v, vec![1, 1, 3, 4, 5]);
/// ```
pub fn ternary_partition<T: Ord>(slice: &mut [T]) -> (usize, usize) {
    use std::cmp::Ordering as O;
    let end = slice.len() - 1;

    if slice[0] > slice[end] {
        slice.swap(0, end);
    }

    // [0, i) : smaller, equal pivot 1
    // [i, j) : big than pivot 1 and smaller than pivot2
    // [j.. : big, equal pivot 2
    let mut i = 1usize;
    let mut j = 1usize;
    let mut k = end - 1;
    while j <= k {
        if slice[j].cmp(&slice[0]).is_le() {
            // left
            slice.swap(i, j);
            i += 1;
            j += 1;
        } else if slice[j].cmp(&slice[end]).is_ge() {
            // right
            slice.swap(j, k);
            k -= 1;
        } else {
            // mid
            j += 1;
        }
    }
    slice.swap(0, i - 1);
    slice.swap(j, end);
    (i, j)
}

/// # Description
/// Sorts the given slice in-place using a three-way partition quick‑sort algorithm.
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
/// use quick_sort::ternary_quick_sort;
/// let mut v = vec![3, 1, 4, 1, 5];
/// ternary_quick_sort(&mut v);
/// assert_eq!(v, vec![1, 1, 3, 4, 5]);
/// ```
pub fn ternary_quick_sort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }

    let (pivot1, pivot2) = ternary_partition(slice);

    ternary_quick_sort(&mut slice[0..pivot1 - 1]);
    ternary_quick_sort(&mut slice[pivot1..pivot2]);
    ternary_quick_sort(&mut slice[pivot2 + 1..]);
}

/// # Description
/// Partition slice in 3 part and return it's delimeter.
/// This function is based on Dijkstra's Dutch national flag algorithm.
/// Two pivot's are one at the front and the other at back.
/// Use comp to identify it's order.
///
/// # Type Parameters
/// - `T`: The element type.
/// - 'F': The comparator type. Must implement 'FnMut'.
///
/// # Parameters
/// - `slice`: The mutable slice to partition.
/// - `comp`: The callable object to compare two &T data.
///
/// # Panics
/// Panics if the implementation of Ord panics.
///
/// # Examples
/// ```
/// use quick_sort::ternary_partition_by;
/// let mut v = vec![3, 1, 4, 1, 5];
/// let (i, j) = ternary_partition_by(&mut v, &mut |a: &i32, b: &i32| a.cmp(b));
/// assert_eq!(i, 3);
/// assert_eq!(j, 4);
/// assert_eq!(v, vec![1, 1, 3, 4, 5]);
/// ```
pub fn ternary_partition_by<T, F>(slice: &mut [T], comp: &mut F) -> (usize, usize)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    use std::cmp::Ordering as O;
    let end = slice.len() - 1;

    if comp(&slice[0], &slice[end]).is_gt() {
        slice.swap(0, end);
    }

    // [0, i) : smaller, equal pivot 1
    // [i, j) : big than pivot 1 and smaller than pivot2
    // [j.. : big, equal pivot 2
    let mut i = 1usize;
    let mut j = 1usize;
    let mut k = end - 1;
    while j <= k {
        if comp(&slice[j], &slice[0]).is_le() {
            // left
            slice.swap(i, j);
            i += 1;
            j += 1;
        } else if comp(&slice[j], &slice[end]).is_ge() {
            // right
            slice.swap(j, k);
            k -= 1;
        } else {
            // mid
            j += 1;
        }
    }
    slice.swap(0, i - 1);
    slice.swap(j, end);
    (i, j)
}

fn ternary_quick_by<T, F>(slice: &mut [T], comp: &mut F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    if slice.len() <= 1 {
        return;
    }

    let (pivot1, pivot2) = ternary_partition_by(slice, comp);

    ternary_quick_by(&mut slice[0..pivot1 - 1], comp);
    ternary_quick_by(&mut slice[pivot1..pivot2], comp);
    ternary_quick_by(&mut slice[pivot2 + 1..], comp);
}

/// # Description
/// Sorts the given slice in-place using a three-way partition quick‑sort algorithm
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
/// use quick_sort::ternary_quick_sort_by;
/// let mut v = vec![3, 1, 4, 1, 5];
/// ternary_quick_sort_by(&mut v, |a : &i32, b : &i32|{ a.cmp(b) });
/// assert_eq!(v, vec![1, 1, 3, 4, 5]);
/// ```
pub fn ternary_quick_sort_by<T, F>(slice: &mut [T], comp: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    let mut cmp = comp;
    ternary_quick_by(slice, &mut cmp);
}

#[cfg(test)]
mod tests {

    use crate::*;
    use std::cmp::Reverse;

    use rand::distr::StandardUniform;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    const TEST_SIZE: usize = 10_000;

    use crate::ternary_quick_sort::ternary_partition;

    fn is_partitioned(slice: &[i32], pivot_pos: (usize, usize)) -> bool {
        let len = slice.len();

        for i in 0..pivot_pos.0 {
            if !(slice[i] <= slice[pivot_pos.0 - 1]) {
                return false;
            }
        }
        for i in (pivot_pos.0 + 1)..pivot_pos.1 {
            if !(slice[pivot_pos.0 - 1] < slice[i] && slice[i] < slice[pivot_pos.1]) {
                return false;
            }
        }
        for i in pivot_pos.1..len {
            if !(slice[pivot_pos.1] <= slice[i]) {
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

        let pivot_pos = ternary_partition(&mut vec);

        assert!(is_partitioned(&vec, pivot_pos));
    }

    #[test]
    fn test_ternary_quick_sort() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

        ternary_quick_sort(&mut vec);

        assert!(vec.is_sorted());
    }

    #[test]
    fn test_ternary_quick_sort_by() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

        ternary_quick_sort_by(&mut vec, |a: &i32, b: &i32| Reverse(a).cmp(&Reverse(b)));

        assert!(vec.is_sorted_by(|&a, &b| { a > b }));
    }
}
