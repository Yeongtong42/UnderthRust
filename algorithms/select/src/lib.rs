use rand::random_range;
use std::mem::swap;

pub fn select_min_max<T: Ord>(slice: &[T]) -> Option<(usize, usize)> {
    if slice.is_empty() {
        return None;
    }

    let mut result = (0, 0);
    let mut i = 1;
    if (slice.len() & 1) == 0 {
        // even
        i = 2;
        if slice[0].gt(&slice[1]) {
            result = (1, 0);
        } else {
            result = (0, 1);
        }
    };

    while i < slice.len() {
        let (mut first, mut second) = (i, i + 1);

        if slice[first].gt(&slice[second]) {
            swap(&mut first, &mut second);
        }

        if slice[first].lt(&slice[result.0]) {
            result.0 = first;
        };
        if slice[second].gt(&slice[result.1]) {
            result.1 = second;
        }

        i += 2;
    }

    Some(result)
}

/// will be moved to the quick sort crate
fn hoare_partition<T: Ord>(slice: &mut [T], pivot: usize) -> usize {
    slice.swap(0, pivot);

    let mut l = 1usize;
    let mut r = slice.len() - 1;
    while l <= r {
        if slice[l].le(&slice[0]) {
            // append left
            l += 1;
        } else {
            // append right
            slice.swap(l, r);
            r -= 1;
        }
    }
    slice.swap(r, 0);
    r
}

pub fn select_nth_elem_random<T: Ord>(slice: &mut [T], n: usize) {
    let len = slice.len();
    if len == 0 || len <= n {
        return;
    }

    // hoare's partition
    let pivot_idx = hoare_partition(slice, random_range(0..len));

    // pivot is not an answer
    if pivot_idx != n {
        // divide
        let range = if n < pivot_idx {
            0..pivot_idx
        } else {
            pivot_idx + 1..len
        };

        // conquer sub-problem
        let offset = range.start;
        select_nth_elem_random(&mut slice[range], n - offset)
    }
}

fn sort_five_in_place_with_stride<T: Ord>(slice: &mut [T], start: usize, stride: usize) {
    // sort
    for i in 0..4 {
        let mut target = i;
        for j in (i + 1)..5 {
            if slice[start + target * stride] > slice[start + j * stride] {
                target = j;
            }
        }
        slice.swap(start + i * stride, start + target * stride);
    }
}

pub fn select_nth_elem_strict<T: Ord>(slice: &mut [T], n: usize) {
    // sort left over
    let mut l = 0usize;
    let r = slice.len();
    let mut target = n;
    while ((r - l) % 5) != 0 {
        for i in (l + 1)..r {
            if slice[l] > slice[i] {
                slice.swap(l, i);
            }
        }

        // early search
        if target == 0 {
            return;
        }
        l += 1;
        target -= 1;
    }

    // median of medians
    // grouping
    let group_size = (r - l) / 5;
    for start in l..(l + group_size) {
        sort_five_in_place_with_stride(slice, start, group_size);
    }

    // get pivot using median of median
    // select median group
    select_nth_elem_strict(
        &mut slice[(l + 2 * group_size)..(l + 3 * group_size)],
        group_size / 2,
    );
    let mut median_of_median = 2 * group_size + group_size / 2; // use median of median as a pivot
    // partition slice using median of median as a pivot
    median_of_median = hoare_partition(&mut slice[l..r], median_of_median);

    // divide conquer
    if median_of_median != target {
        if target < median_of_median {
            select_nth_elem_strict(&mut slice[l..l + median_of_median], target);
        } else {
            select_nth_elem_strict(
                &mut slice[(l + median_of_median + 1)..r],
                target - median_of_median - 1,
            );
        };
    }
}

#[cfg(test)]
mod test {
    use crate::{
        hoare_partition, select_min_max, select_nth_elem_random, select_nth_elem_strict,
        sort_five_in_place_with_stride,
    };

    #[test]
    fn test_select_min_max() {
        let arr = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(select_min_max(&arr), Some((0, 6)));

        let arr1 = [4, 51, 1, 3143, 5, 7777, 3, 5, 6, 87, 9];
        assert_eq!(select_min_max(&arr1), Some((2usize, 5usize)));

        let arr2: [i32; 0] = [];
        assert_eq!(select_min_max(&arr2), None);

        let arr3 = [1];
        assert_eq!(select_min_max(&arr3), Some((0, 0)));

        let arr4 = [1, 0];
        assert_eq!(select_min_max(&arr4), Some((1, 0)));
    }

    fn is_partitioned(slice: &[i32], pivot: usize) -> bool {
        for i in 0..pivot {
            if slice[i].gt(&slice[pivot]) {
                return false;
            }
        }
        for i in pivot + 1..slice.len() {
            if slice[i].le(&slice[pivot]) {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_hoare_partition() {
        let mut arr = [51, 4, 3143, 5, 7777, 3, 5, 6, 87, 9]; // 3 4 5 5 6 9 51 87 3143 7777
        let pivot = hoare_partition(&mut arr, 5);
        println!("{:?}", arr);
        assert!(is_partitioned(&arr, pivot));

        let mut arr1 = [1, 2, 3, 4, 5, 6, 7];
        let pivot1 = hoare_partition(&mut arr1, 6);
        println!("{:?}", arr1);
        assert!(is_partitioned(&arr1, pivot1));
    }

    #[test]
    fn test_sort_five_stride() {
        let mut arr = [1, 5, 2, 4, 3, 3, 4, 2, 5, 1];
        sort_five_in_place_with_stride(&mut arr, 1, 2);
        assert_eq!(arr, [1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
    }

    #[test]
    fn test_select_nth_elem_random() {
        let mut arr2 = [1];
        select_nth_elem_random(&mut arr2, 0);
        assert_eq!(arr2[0], 1);

        let mut arr3 = [1, 0];
        select_nth_elem_random(&mut arr3, 0);
        assert_eq!(arr3[0], 0);
        assert_eq!(arr3[1], 1);

        let pivot = 6;

        let mut arr = [1, 2, 3, 4, 5, 6, 7];
        select_nth_elem_random(&mut arr, pivot);
        assert_eq!(arr[pivot], 7);

        let mut arr1 = [4, 51, 1, 3143, 5, 7777, 3, 5, 6, 87, 9]; // 1 3 4 5 5 6 9 51 87 3143 7777
        select_nth_elem_random(&mut arr1, pivot);
        println!("{:?}", arr1);
        assert_eq!(arr1[pivot], 9);
    }

    #[test]
    fn test_select_nth_elem_strict() {
        let mut arr2 = [1];
        select_nth_elem_strict(&mut arr2, 0);
        assert_eq!(arr2[0], 1);

        let mut arr3 = [1, 0];
        select_nth_elem_strict(&mut arr3, 0);
        assert_eq!(arr3[0], 0);
        assert_eq!(arr3[1], 1);

        let pivot = 6;

        let mut arr = [1, 2, 3, 4, 5, 6, 7];
        select_nth_elem_strict(&mut arr, pivot);
        println!("{:?}", arr);
        assert_eq!(arr[pivot], 7);

        let mut arr1 = [4, 51, 1, 3143, 5, 7777, 3, 5, 6, 87, 9]; // 1 3 4 5 5 6 9 51 87 3143 7777
        select_nth_elem_strict(&mut arr1, pivot);
        println!("{:?}", arr1);
        assert_eq!(arr1[pivot], 9);
    }
}
