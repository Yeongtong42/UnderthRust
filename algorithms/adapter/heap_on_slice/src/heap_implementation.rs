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

    #[test]
    fn test_single_upward() {
        // Test single upward movement when swap is needed
        let mut arr = vec![5, 2, 3, 7, 1]; // 1 at index 4 should move up
        let mut idx = 4;
        assert!(single_upward(&mut arr, &mut idx, default_compare));
        assert_eq!(idx, 1); // Should move to parent position
        assert_eq!(arr[1], 1); // 1 should be at position 1
        assert_eq!(arr[4], 2); // 2 should be at position 4

        // Test single upward when no swap is needed
        let mut arr2 = vec![1, 2, 3, 4, 5];
        let mut idx2 = 4;
        assert!(!single_upward(&mut arr2, &mut idx2, default_compare));
        assert_eq!(idx2, 4); // Index should remain the same

        // Test with root element (should return false)
        let mut arr3 = vec![1, 2, 3];
        let mut idx3 = 0;
        assert!(!single_upward(&mut arr3, &mut idx3, default_compare));
        assert_eq!(idx3, 0);

        // Test upward with equal elements
        let mut arr4 = vec![2, 2, 3, 4, 2]; // Equal elements, should not swap
        let mut idx4 = 4;
        assert!(!single_upward(&mut arr4, &mut idx4, default_compare));
        assert_eq!(idx4, 4);
    }

    #[test]
    fn test_single_downward() {
        // Test single downward movement when swap is needed
        let mut arr = vec![5, 1, 2, 7, 8]; // 5 at root should move down
        let mut idx = 0;
        assert!(single_downward(&mut arr, &mut idx, default_compare));
        assert_eq!(idx, 1); // Should move to left child position
        assert_eq!(arr[0], 1); // 1 should be at root
        assert_eq!(arr[1], 5); // 5 should be at position 1

        // Test single downward when no swap is needed
        let mut arr2 = vec![1, 2, 3, 4, 5];
        let mut idx2 = 0;
        assert!(!single_downward(&mut arr2, &mut idx2, default_compare));
        assert_eq!(idx2, 0); // Index should remain the same

        // Test with leaf node (should return false)
        let mut arr3 = vec![1, 2, 3, 4, 5];
        let mut idx3 = 4;
        assert!(!single_downward(&mut arr3, &mut idx3, default_compare));
        assert_eq!(idx3, 4);

        // Test with only left child
        let mut arr4 = vec![3, 1]; // Only left child exists
        let mut idx4 = 0;
        assert!(single_downward(&mut arr4, &mut idx4, default_compare));
        assert_eq!(idx4, 1);
        assert_eq!(arr4[0], 1);
        assert_eq!(arr4[1], 3);

        // Test choosing smaller of two children
        let mut arr5 = vec![5, 3, 2]; // Both children exist, should choose smaller (2)
        let mut idx5 = 0;
        assert!(single_downward(&mut arr5, &mut idx5, default_compare));
        assert_eq!(idx5, 2); // Should move to right child position
        assert_eq!(arr5[0], 2);
        assert_eq!(arr5[2], 5);

        // Test with equal children (should choose left)
        let mut arr6 = vec![5, 2, 2]; // Equal children, should choose left
        let mut idx6 = 0;
        assert!(single_downward(&mut arr6, &mut idx6, default_compare));
        assert_eq!(idx6, 1); // Should move to left child position
        assert_eq!(arr6[0], 2);
        assert_eq!(arr6[1], 5);
    }

    #[test]
    fn test_move_upward_comprehensive() {
        // Test complete upward movement from a valid heap
        let mut arr = vec![1, 3, 2, 7, 5, 4, 8];
        heapify(&mut arr, default_compare); // Ensure it's a valid heap first
        assert!(is_heap(&arr, default_compare));

        // Place 0 at a leaf position that needs to bubble up
        let last_idx = arr.len() - 1;
        arr[last_idx] = 0; // Place 0 at the last position
        assert!(move_upward(&mut arr, last_idx, default_compare));
        assert!(is_heap(&arr, default_compare));
        assert_eq!(arr[0], 0); // Should reach the root

        // Test no movement needed
        let mut arr2 = vec![1, 2, 3, 4, 5];
        assert!(!move_upward(&mut arr2, 4, default_compare));
        assert_eq!(arr2, vec![1, 2, 3, 4, 5]); // Should remain unchanged

        // Test partial upward movement with valid heap
        let mut arr3 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        heapify(&mut arr3, default_compare); // Make it a valid heap
        assert!(is_heap(&arr3, default_compare));

        // Break heap property at a leaf and fix it
        arr3[8] = 0; // Place 0 at index 8 (should move up)
        assert!(move_upward(&mut arr3, 8, default_compare));
        assert!(is_heap(&arr3, default_compare));

        // Test with root index
        let mut arr4 = vec![1, 2, 3, 4, 5];
        assert!(!move_upward(&mut arr4, 0, default_compare));
        assert_eq!(arr4, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_move_downward_comprehensive() {
        // Test complete downward movement with valid heap
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
        heapify(&mut arr, default_compare); // Make it a valid heap first
        assert!(is_heap(&arr, default_compare));

        // Break heap property at root and fix it
        arr[0] = 10; // 10 at root should move down
        assert!(move_downward(&mut arr, 0, default_compare));
        assert!(is_heap(&arr, default_compare));

        // Test no movement needed
        let mut arr2 = vec![1, 2, 3, 4, 5];
        assert!(!move_downward(&mut arr2, 0, default_compare));
        assert_eq!(arr2, vec![1, 2, 3, 4, 5]); // Should remain unchanged

        // Test leaf node (no children)
        let mut arr3 = vec![1, 2, 3, 4, 5];
        assert!(!move_downward(&mut arr3, 4, default_compare));
        assert_eq!(arr3, vec![1, 2, 3, 4, 5]); // Should remain unchanged

        // Test movement from internal node
        let mut arr4 = vec![1, 2, 3, 4, 5, 6, 7];
        heapify(&mut arr4, default_compare); // Make it a valid heap
        assert!(is_heap(&arr4, default_compare));

        // Break heap property at index 1 and fix it
        arr4[1] = 8; // 8 at index 1 should move down
        assert!(move_downward(&mut arr4, 1, default_compare));
        assert!(is_heap(&arr4, default_compare));
    }

    #[test]
    fn test_edge_cases_private_functions() {
        // Test single_upward with equal elements
        let mut arr_equal = vec![2, 2, 2, 2, 1];
        let mut idx = 4;
        assert!(single_upward(&mut arr_equal, &mut idx, default_compare));

        // Test single_downward with equal elements
        let mut arr_equal2 = vec![2, 1, 1, 3, 4];
        let mut idx2 = 0;
        assert!(single_downward(&mut arr_equal2, &mut idx2, default_compare));

        // Test boundary conditions
        let mut single_elem = vec![1];
        let mut idx3 = 0;
        assert!(!single_upward(&mut single_elem, &mut idx3, default_compare));
        assert!(!single_downward(
            &mut single_elem,
            &mut idx3,
            default_compare
        ));

        // Test with two elements
        let mut two_elem = vec![2, 1];
        let mut idx4 = 1;
        assert!(single_upward(&mut two_elem, &mut idx4, default_compare));
        assert_eq!(two_elem, vec![1, 2]);

        let mut two_elem2 = vec![2, 1];
        let mut idx5 = 0;
        assert!(single_downward(&mut two_elem2, &mut idx5, default_compare));
        assert_eq!(two_elem2, vec![1, 2]);

        // Test with reverse comparator
        let mut arr_rev = vec![1, 3, 2, 4, 5];
        let mut idx6 = 4;
        assert!(single_upward(&mut arr_rev, &mut idx6, reverse_compare));
        // With reverse comparator, 5 should move up since 5 > 3

        let mut arr_rev2 = vec![1, 3, 4, 2, 5];
        let mut idx7 = 0;
        assert!(single_downward(&mut arr_rev2, &mut idx7, reverse_compare));
        // With reverse comparator, we want larger elements as children
    }

    #[test]
    fn test_comprehensive_heap_operations() {
        // Test heapify with various input patterns
        let mut arr1: Vec<i32> = vec![]; // Empty
        heapify(&mut arr1, default_compare);
        assert!(is_heap(&arr1, default_compare));

        let mut arr2 = vec![1]; // Single element
        heapify(&mut arr2, default_compare);
        assert!(is_heap(&arr2, default_compare));

        let mut arr3 = vec![1, 2]; // Two elements
        heapify(&mut arr3, default_compare);
        assert!(is_heap(&arr3, default_compare));

        let mut arr4 = vec![5, 4, 3, 2, 1]; // Reverse sorted
        heapify(&mut arr4, default_compare);
        assert!(is_heap(&arr4, default_compare));

        let mut arr5 = vec![1, 1, 1, 1, 1]; // All equal
        heapify(&mut arr5, default_compare);
        assert!(is_heap(&arr5, default_compare));

        // Test heap_pushpop edge cases
        let mut empty: Vec<i32> = vec![];
        let result = heap_pushpop(&mut empty, 5, default_compare);
        assert_eq!(result, 5);
        assert!(empty.is_empty());

        let mut single = vec![3];
        let result = heap_pushpop(&mut single, 1, default_compare);
        assert_eq!(result, 1);
        assert_eq!(single, vec![3]);

        let result = heap_pushpop(&mut single, 5, default_compare);
        assert_eq!(result, 3);
        assert_eq!(single, vec![5]);
    }
}
