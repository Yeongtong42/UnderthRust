//! # select
//!
//! Ord 트레잇을 구한한 타입 T의 슬라이스에 대하여 select를 수행해하는 알고리즘의 구현체를 제공합니다.
//!
//! ## 특징
//! - fully safe(no unsafe function used).
//!
//! ## 예시
//! ```rust
//! use select::*;
//!
//! let mut slice = [3, 2, 10, 5, -6, 77];
//! assert_eq!(select_min_max(&slice).unwrap(), (4, 5));
//! select_nth_elem_random(&mut slice, 3);
//! assert_eq!(slice[3], 5);
//! select_nth_elem_strict(&mut slice, 4);
//! assert_eq!(slice[4], 10);
//!
//! ```
//!
//! ## 구현체
//! - `select_min_max`: 최소/최대 원소의 인덱스의 튜플을 Option으로 감싸서 제공합니다.
//! - `select_nth_elem_random`: 랜덤으로 피벗을 고르는 quick select 입니다.
//! - `select_nth_elem_strict`: median of medians에 기반한 quick select 입니다.
//!
//! ## 참조
//! - Introduction to algorithms 4th ed을 기준으로 구현했습니다.
use rand::random_range;
use std::mem::swap;

/// 슬라이스에서 최소값과 최대값의 인덱스를 반환합니다.
///
/// 입력된 슬라이스가 비어 있다면 `None`을 반환하고,
/// 그렇지 않으면 `(min_index, max_index)` 형태로 반환합니다.
///
/// 시간 복잡도는 O(N)이며, 짝수 개의 요소에 대해 pair-wise 비교를 수행해 최적화를 시도합니다.
///
/// # 예시
///
/// ```
/// let slice = [3, 2, 10, 5, -6, 77];
/// assert_eq!(select::select_min_max(&slice), Some((4, 5)));
/// ```
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

/// 주어진 피벗 인덱스를 기준으로 슬라이스를 Hoare 파티션 방식으로 분할합니다.
///
/// 반환값은 피벗이 위치한 최종 인덱스를 의미하며,
/// 슬라이스는 `[<= pivot | > pivot]` 구조로 재배열됩니다.
///
/// 이 함수는 내부 구현 용도로 사용되며, 공개되지 않습니다.
///
/// # Panics
/// - 슬라이스 길이가 0이면 패닉을 발생시킬 수 있습니다.
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

/// 슬라이스에서 n번째로 작은 값을 n번째 인덱스로 이동시킵니다.
///
/// 이 알고리즘은 Quick Select를 기반으로 하며,
/// 피벗은 무작위로 선택됩니다. 따라서 평균적으로 O(N) 시간복잡도를 가지지만,
/// 최악의 경우 O(N²)까지 갈 수 있습니다.
/// 후술할 strict 버젼보다 대부분의 경우에 더 빠릅니다.
///
/// # 매개변수
/// - `slice`: 정렬 대상 슬라이스입니다.
/// - `n`: 정렬하여 얻고자 하는 인덱스입니다.
///
/// # 제약
/// - `n >= slice.len()`인 경우 아무 동작도 하지 않습니다.
///
/// # 예시
/// ```
/// let mut slice = [3, 2, 10, 5, -6, 77];
/// select::select_nth_elem_random(&mut slice, 3);
/// assert_eq!(slice[3], 5); // 세 번째로 작은 원소
/// ```
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

/// 슬라이스에서 stride를 이용해 5개의 원소를 제자리에서 정렬합니다.
///
/// 이 함수는 median of medians 알고리즘을 위한 보조 함수이며,
/// stride가 1이 아닌 경우에도 작동하도록 설계되어 있습니다.
///
/// # 예시
/// - `slice[start + i * stride]` 형식으로 5개의 원소가 존재해야 합니다.
///
/// # 제약
/// - 슬라이스가 요구되는 범위보다 작다면 잘못된 동작이 발생할 수 있습니다.
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

/// median of medians 알고리즘을 기반으로 n번째로 작은 값을 슬라이스의 n번째 인덱스로 이동시킵니다.
///
/// 최악의 경우에도 O(N) 시간복잡도를 보장합니다.
/// 피벗 선택을 median of medians 방식으로 수행하여, 균형 잡힌 분할을 유도합니다.
///
/// # 매개변수
/// - `slice`: 정렬 대상 슬라이스입니다.
/// - `n`: 정렬하여 얻고자 하는 인덱스입니다.
///
/// # 예시
/// ```
/// let mut slice = [3, 2, 10, 5, -6, 77];
/// select::select_nth_elem_strict(&mut slice, 4);
/// assert_eq!(slice[4], 10);
/// ```
pub fn select_nth_elem_strict<T: Ord>(slice: &mut [T], n: usize) {
    // sort left over
    let mut l = 0usize;
    let r = slice.len();
    let mut target = n;
    if r == 0 || r <= n {
        return;
    }
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

    use rand::distr::StandardUniform;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    const TEST_SIZE: usize = 10_000;

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

    #[test]
    fn test_big_slice_random() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();
        let pivot = TEST_SIZE / 2;

        select_nth_elem_random(&mut vec, pivot);

        assert!(is_partitioned(&vec, pivot));
    }

    #[test]
    fn test_big_slice_strict() {
        let seed: u64 = 42;
        let rng = StdRng::seed_from_u64(seed);

        let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();
        let pivot = TEST_SIZE / 2;

        select_nth_elem_strict(&mut vec, pivot);

        assert!(is_partitioned(&vec, pivot));
    }
}
