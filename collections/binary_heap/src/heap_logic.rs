//! Core logic of binary heap tree
use crate::comparator::*;
use std::cmp::Ordering;

/// helper function for heap tree, get index of parent node
/// because of the original algorithm was based on 1-indexed, need to convert it to 0-indexed
/// note : get_parent(0) is usize::max, will always be bigger than data.len()
#[inline]
pub fn get_parent(i: usize) -> usize {
    ((i + 1) >> 1).wrapping_sub(1)
}

/// helper function for heap tree, get index of left child node
/// because of the original algorithm was based on 1-indexed, need to convert it to 0-indexed
#[inline]
fn get_left(i: usize) -> usize {
    ((i + 1) << 1) - 1
}

/// helper function for heap tree, get index of right child node
/// because of the original algorithm was based on 1-indexed, need to convert it to 0-indexed
#[inline]
fn get_right(i: usize) -> usize {
    ((i + 1) << 1) + 1 - 1
}

/// # Description
/// keep invariant of heap tree
/// Invariant : child nodes must be bigger than it's parent node
/// for performace reason, comp.compare()'s inlining is crucial
///
/// # Performance
/// Time complexity(worst) : O(log n)
pub fn min_heapify<T>(data: &mut [T], comp: &impl Comparator<T>, i: usize) {
    let l = get_left(i);
    let r = get_right(i);
    let mut s = i;
    if l < data.len() && Ordering::Less == comp.compare(&data[l], &data[s]) {
        s = l;
    }
    if r < data.len() && Ordering::Less == comp.compare(&data[r], &data[s]) {
        s = r;
    }
    if s != i {
        data.swap(i, s);
        min_heapify(data, comp, s); // push down
    }
}

/// # Description
/// reorder vector to make heap tree
///
/// # Performance
/// Time complexity(worst) : O(n)
pub fn build_heap<T>(data: &mut [T], comp: &impl Comparator<T>) {
    let offset = data.len() / 2;
    for i in (0..offset).rev() {
        min_heapify(data, comp, i);
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use rand::distr::StandardUniform;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    use crate::Comparator;
    use crate::DefaultComparator;
    use crate::heap_logic::*;

    fn is_min_heaped<T, C: Comparator<T>>(vec: &Vec<T>, comp: &C) -> bool {
        for i in (1..vec.len()).rev() {
            let current = &vec[i];
            let parent = &vec[super::get_parent(i)];
            if let Ordering::Greater = comp.compare(parent, current) {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_build_heap_empty() {
        let dcomp = DefaultComparator;

        let mut vec1: Vec<i32> = Vec::new();
        build_heap(&mut vec1, &dcomp);
        assert!(is_min_heaped(&vec1, &dcomp));
    }

    #[test]
    fn test_build_heap_one() {
        let dcomp = DefaultComparator;

        let mut vec1: Vec<i32> = vec![0i32; 1];
        build_heap(&mut vec1, &dcomp);
        assert!(is_min_heaped(&vec1, &dcomp));
    }

    #[test]
    fn test_build_heap_ordered() {
        let dcomp = DefaultComparator;

        let mut vec1: Vec<i32> = (0..45i32).collect();
        build_heap(&mut vec1, &dcomp);
        assert!(is_min_heaped(&vec1, &dcomp));
    }
    #[test]
    fn test_build_heap_reverse_ordered() {
        let dcomp = DefaultComparator;

        let mut vec1: Vec<i32> = (0..45i32).rev().collect();
        build_heap(&mut vec1, &dcomp);
        assert!(is_min_heaped(&vec1, &dcomp));
    }
    #[test]
    fn test_build_heap_random() {
        let dcomp = DefaultComparator;

        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec1: Vec<i32> = rng.sample_iter(StandardUniform).take(1000_000).collect();

        build_heap(&mut vec1, &dcomp);
        assert!(is_min_heaped(&vec1, &dcomp));
    }

    #[test]
    fn test_heapify_empty() {
        let dcomp = DefaultComparator;

        let mut vec0: Vec<u32> = Vec::new();
        min_heapify(&mut vec0, &dcomp, 0);
        assert!(is_min_heaped(&vec0, &dcomp));
    }
    #[test]
    fn test_heapify_one() {
        let dcomp = DefaultComparator;

        let mut vec1: Vec<u32> = vec![0];
        min_heapify(&mut vec1, &dcomp, 0);
        assert!(is_min_heaped(&vec1, &dcomp));
    }
    #[test]
    fn test_heapify_general() {
        let dcomp = DefaultComparator;

        let mut vec2: Vec<u32> = vec![4, 1, 2, 3, 6, 7, 8];
        min_heapify(&mut vec2, &dcomp, 0);
        assert!(is_min_heaped(&vec2, &dcomp));

        let mut vec3: Vec<u32> = vec![1, 2, 3, 99, 5, 6, 7];
        min_heapify(&mut vec3, &dcomp, 3);
        assert!(is_min_heaped(&vec3, &dcomp));
    }
}
