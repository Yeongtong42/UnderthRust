//! [heap_on_slice] 모듈의 private implementation 모듈
//! # Note on heap property
//! 길이 $n$인 배열에서 leaf node는
//! $n$이 짝수 인 경우 $n = 2k$라고 하면
//! $(k - 1) * 2 + 1 = n - 1$이고 $(k - 1) * 2 + 2 = n$이므로 $k - 1$이 왼쪽 자식만을 가지는 마지막 노드이다.
//! $n$이 홀수인 경우 $n = 2k + 1$이라고 하면
//! $(k - 1) * 2 + 1 = n - 2$이고 $(k - 1) * 2 + 2 = n - 1$이므로 $k - 1$이 양쪽 자식만을 가지는 마지막 노드이다.
//! 따라서 [0, n/2)가 parent node이고 [n/2, n) 가 leaf node이다
use std::cmp::Ordering;

/// # Note
/// [0, n/2)가 parent node이고 [n/2, n) 가 leaf node이므로 $[0, len / 2)$에 대해 확인한다.
pub fn is_heap<T, F>(arr: &[T], mut compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = arr.len();
    for idx in 0..len / 2 {
        let (left, right) = (2 * idx + 1, 2 * idx + 2);
        if left < len && compare(&arr[idx], &arr[left]).is_gt() {
            return false;
        }
        if right < len && compare(&arr[idx], &arr[right]).is_gt() {
            return false;
        }
    }
    true
}

/// # Note
/// ```ignore
/// let Some(parent) = idx.checked_sub(1).map(|x| x / 2) else {
///     return false;
/// };
/// ```
/// 이 코드는 idx가 0인 경우를 처리하기 위한 코드이다. 아래 코드와 동일한 기능을 한다.
/// ```ignore
/// if *idx == 0 {
///    return false;
/// }
/// let parent = (*idx - 1) / 2;
/// ```
#[inline]
fn single_upward<T, F>(arr: &mut [T], idx: &mut usize, mut compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    let Some(parent) = idx.checked_sub(1).map(|x| x / 2) else {
        return false;
    };
    match compare(&arr[*idx], &arr[parent]) {
        Ordering::Greater | Ordering::Equal => {
            // No swap with parent
            false
        }
        Ordering::Less => {
            // Swap with parent
            arr.swap(*idx, parent);
            *idx = parent;
            true
        }
    }
}

#[inline]
fn single_downward<T, F>(arr: &mut [T], idx: &mut usize, mut compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    let (left, right) = (2 * *idx + 1, 2 * *idx + 2);
    let mut smallest_idx = *idx;

    if left < arr.len() && compare(&arr[left], &arr[smallest_idx]).is_lt() {
        smallest_idx = left;
    }
    if right < arr.len() && compare(&arr[right], &arr[smallest_idx]).is_lt() {
        smallest_idx = right;
    }

    if smallest_idx == *idx {
        false
    } else {
        arr.swap(*idx, smallest_idx);
        *idx = smallest_idx;
        true
    }
}

pub fn move_upward<T, F>(arr: &mut [T], mut idx: usize, mut compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    if single_upward(arr, &mut idx, &mut compare) {
        while single_upward(arr, &mut idx, &mut compare) {}
        true
    } else {
        false
    }
}

pub fn move_downward<T, F>(arr: &mut [T], mut idx: usize, mut compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    if single_downward(arr, &mut idx, &mut compare) {
        while single_downward(arr, &mut idx, &mut compare) {}
        true
    } else {
        false
    }
}

/// # Note
/// 길이 $n$인 배열에서 leaf node는
/// $n$이 짝수 인 경우 $n = 2k$라고 하면
/// $(k - 1) * 2 + 1 = n - 1$이고 $(k - 1) * 2 + 2 = n$이므로 $k - 1$이 왼쪽 자식만을 가지는 마지막 노드이다.
/// $n$이 홀수인 경우 $n = 2k + 1$이라고 하면
/// $(k - 1) * 2 + 1 = n - 2$이고 $(k - 1) * 2 + 2 = n - 1$이므로 $k - 1$이 양쪽 자식만을 가지는 마지막 노드이다.
/// 따라서 [0, n/2)가 parent node이고 [n/2, n) 가 leaf node이다
/// $n = 0$인 경우에도 위 invariant가 성립한다.
pub fn heapify<T, F>(arr: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = arr.len();
    if len == 0 {
        return;
    }
    // [0, n/2) 에 대해 downward adjustment를 수행한다.
    for idx in (0..len / 2).rev() {
        let mut current_idx = idx;
        while single_downward(arr, &mut current_idx, &mut compare) {}
    }
}

/// # Note
/// 메서드 spec은 pop 후 push가 아닌, push 후 pop이다.
/// 따라서 arr이 비어있거나, x가 arr의 root보다 작은 경우 x를 반환한다.
/// root의 값과 같은 경우 최적화를 위해 힙을 조정하지 않고 x를 반환한다.
pub fn heap_pushpop<T, F>(arr: &mut [T], mut x: T, mut compare: F) -> T
where
    F: FnMut(&T, &T) -> Ordering,
{
    if !arr.is_empty() && compare(&arr[0], &x).is_lt() {
        std::mem::swap(&mut arr[0], &mut x);
        move_downward(arr, 0, compare);
    }
    x
}

/// # Note
/// arr의 길이가 0인 경우와 1인 경우는 둘 다 특수 한 경우이다.
pub fn heap_pop<T, F>(arr: &mut [T], compare: F) -> Option<&mut [T]>
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = arr.len();
    if len == 0 {
        return None;
    }
    let (init, last) = arr.split_at_mut(len - 1);
    if !init.is_empty() {
        // arr.len() == 1 인 경우 init은 empty slice가 된다.
        std::mem::swap(&mut init[0], &mut last[0]);
        move_downward(init, 0, compare);
    }
    Some(init)
}

pub fn heap_reverse_sort<T, F>(mut arr: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    heapify(arr, &mut compare);
    while let Some(init) = heap_pop(arr, &mut compare) {
        arr = init;
    }
}

pub fn adjust_heap<T, F>(arr: &mut [T], idx: usize, mut compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    move_upward(arr, idx, &mut compare) || move_downward(arr, idx, &mut compare)
}

#[cfg(test)]
mod unit_test {
    use crate::heap_implementation::*;
    use std::cmp::Ordering;

    // Define the comparison functions that will be used in tests
    fn default_compare<T: Ord>(a: &T, b: &T) -> Ordering {
        a.cmp(b)
    }

    fn reverse_compare<T: Ord>(a: &T, b: &T) -> Ordering {
        b.cmp(a)
    }

    #[test]
    fn is_heap_true_and_false() {
        // empty and single-element heaps
        let empty: Vec<i32> = vec![];
        assert!(is_heap(&empty, default_compare));
        let single = vec![1];
        assert!(is_heap(&single, default_compare));
        // valid heap
        let heap = vec![1, 3, 2, 7, 5, 4];
        assert!(is_heap(&heap, default_compare));
        // invalid heap: parent greater than child
        let bad = vec![2, 1];
        assert!(!is_heap(&bad, default_compare));
    }

    #[test]
    fn test_single_upward_and_downward_adjustments() {
        // Upward adjustment scenario
        let mut arr_up = vec![1, 3, 5, 7, 9];
        assert!(is_heap(&arr_up, default_compare));
        // break heap property by making a leaf too small
        arr_up[4] = 0;
        assert!(!is_heap(&arr_up, default_compare));
        // fix upward
        assert!(move_upward(&mut arr_up, 4, default_compare));
        assert!(is_heap(&arr_up, default_compare));

        // Downward adjustment scenario
        let mut arr_down = vec![2, 4, 6, 8, 10];
        assert!(is_heap(&arr_down, default_compare));
        // break heap property by making root too large
        arr_down[0] = 12;
        assert!(!is_heap(&arr_down, default_compare));
        // fix downward
        assert!(move_downward(&mut arr_down, 0, default_compare));
        assert!(is_heap(&arr_down, default_compare));
    }

    #[test]
    fn test_heapify_builds_valid_heap() {
        let mut arr = vec![3, 1, 4, 2, 5];
        heapify(&mut arr, default_compare);
        assert!(is_heap(&arr, default_compare));
    }

    #[test]
    fn test_heap_pushpop_and_heap_pop() {
        // pushpop: small x
        let mut arr = vec![1, 2, 3];
        heapify(&mut arr, default_compare); // Ensure it's a heap first
        let x = heap_pushpop(&mut arr, 0, default_compare);
        assert_eq!(x, 0);
        assert!(is_heap(&arr, default_compare));
        // pushpop: large x
        let mut arr2 = vec![1, 2, 3];
        heapify(&mut arr2, default_compare); // Ensure it's a heap first
        let y = heap_pushpop(&mut arr2, 5, default_compare);
        assert_eq!(y, 1);
        assert!(is_heap(&arr2, default_compare));
        // heap_pop
        let mut arr3 = vec![1, 3, 2];
        heapify(&mut arr3, default_compare);
        if let Some(init) = heap_pop(&mut arr3, default_compare) {
            assert_eq!(init.len(), 2);
            assert!(is_heap(init, default_compare));
        } else {
            panic!("heap_pop returned None on non-empty heap");
        }
    }

    #[test]
    fn test_adjust_heap_up_and_down() {
        // upward adjustment
        let mut arr_adj_up = vec![2, 3, 4, 5, 1]; // 1 is out of place (too small for its pos)
        assert!(!is_heap(&arr_adj_up, default_compare::<i32>));
        assert!(adjust_heap(&mut arr_adj_up, 4, default_compare::<i32>));
        assert!(is_heap(&arr_adj_up, default_compare::<i32>));

        // downward adjustment
        let mut arr_adj_down = vec![5, 1, 2, 3, 4]; // 5 is out of place (too large for root)
        assert!(!is_heap(&arr_adj_down, default_compare::<i32>));
        assert!(adjust_heap(&mut arr_adj_down, 0, default_compare::<i32>));
        assert!(is_heap(&arr_adj_down, default_compare::<i32>));
    }

    #[test]
    fn test_heap_reverse_sort_sorts_descending() {
        let mut arr = vec![3, 1, 4, 2, 5];
        heap_reverse_sort(&mut arr, default_compare);
        assert_eq!(arr, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_heap_reverse_sort_with_reverse_comparator_sorts_ascending() {
        let mut arr = vec![3, 1, 4, 2, 5];
        heap_reverse_sort(&mut arr, reverse_compare);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
