//! [heap_on_slice] 모듈의 private implementation 모듈
//! # Note on heap property
//! 길이 $n$인 배열에서 leaf node는
//! $n$이 짝수 인 경우 $n = 2k$라고 하면
//! $(k - 1) * 2 + 1 = n - 1$이고 $(k - 1) * 2 + 2 = n$이므로 $k - 1$이 왼쪽 자식만을 가지는 마지막 노드이다.
//! $n$이 홀수인 경우 $n = 2k + 1$이라고 하면
//! $(k - 1) * 2 + 1 = n - 2$이고 $(k - 1) * 2 + 2 = n - 1$이므로 $k - 1$이 양쪽 자식만을 가지는 마지막 노드이다.
//! 따라서 [0, n/2)가 parent node이고 [n/2, n) 가 leaf node이다
use super::*;

/// # Note
/// [0, n/2)가 parent node이고 [n/2, n) 가 leaf node이므로 $[0, len / 2)$에 대해 확인한다.
pub(crate) fn is_heap<T, C: Comparator<T> + ?Sized>(comp: &C, arr: &[T]) -> bool {
    let len = arr.len();
    for idx in 0..len / 2 {
        let (left, right) = (2 * idx + 1, 2 * idx + 2);
        if left < len && comp.cmp(&arr[idx], &arr[left]) == std::cmp::Ordering::Greater {
            return false;
        }
        if right < len && comp.cmp(&arr[idx], &arr[right]) == std::cmp::Ordering::Greater {
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
fn single_upward<T, C: Comparator<T> + ?Sized>(comp: &C, arr: &mut [T], idx: &mut usize) -> bool {
    use std::cmp::Ordering as O;
    let Some(parent) = idx.checked_sub(1).map(|x| x / 2) else {
        return false;
    };
    match comp.cmp(&arr[*idx], &arr[parent]) {
        O::Greater | O::Equal => {
            // No swap with parent
            false
        }
        O::Less => {
            // Swap with parent
            arr.swap(*idx, parent);
            *idx = parent;
            true
        }
    }
}

/// # Note
/// ```ignore
/// if arr.get(left).map(|x| comp.cmp(x, &arr[smallest_idx])) == Some(O::Less) {
///     smallest_idx = left;
/// }
/// ```
/// 이 코드는 left가 arr의 길이를 초과하는 경우를 처리하기 위한 코드이다. 아래 코드와 동일한 기능을 한다.
/// ```ignore
/// if left < arr.len() && comp.cmp(&arr[left], &arr[smallest_idx]) == O::Less {
///    smallest_idx = left;
/// }
/// ```
#[inline]
fn single_downward<T, C: Comparator<T> + ?Sized>(comp: &C, arr: &mut [T], idx: &mut usize) -> bool {
    let smallest_idx = {
        use std::cmp::Ordering as O;
        let (left, right) = (2 * *idx + 1, 2 * *idx + 2);
        let mut smallest_idx = *idx;
        if arr.get(left).map(|x| comp.cmp(x, &arr[smallest_idx])) == Some(O::Less) {
            smallest_idx = left;
        }
        if arr.get(right).map(|x| comp.cmp(x, &arr[smallest_idx])) == Some(O::Less) {
            smallest_idx = right;
        }
        smallest_idx
    };

    if smallest_idx == *idx {
        false
    } else {
        arr.swap(*idx, smallest_idx);
        *idx = smallest_idx;
        true
    }
}

/// # Note
/// 더이상 힙 내 swap이 일어나지 않을 때까지 single_upward를 호출한다. 원래는
/// ```ignore
/// fn move_upward<T, C: Comparator<T>>(comp: &C, arr: &mut [T], mut idx: usize) -> bool {
///     let original_idx = idx;
///     while single_upward(comp, arr, &mut idx) {}
///     original_idx != idx
/// }
/// ```
/// 형태였으나, 논리적으로 move_upward 함수의 반환값은 첫 single_upward호출의 반환과 같으므로 지금과 같이 변경됨
pub(crate) fn move_upward<T, C: Comparator<T> + ?Sized>(
    comp: &C,
    arr: &mut [T],
    mut idx: usize,
) -> bool {
    if single_upward(comp, arr, &mut idx) {
        while single_upward(comp, arr, &mut idx) {}
        true
    } else {
        false
    }
}

/// # Note
/// 더이상 힙 내 swap이 일어나지 않을 때까지 single_downward를 호출한다. 원래는
/// ```ignore
/// fn move_downward<T, C: Comparator<T>>(comp: &C, arr: &mut [T], mut idx: usize) -> bool {
///     let original_idx = idx;
///     while single_downward(comp, arr, &mut idx) {}
///     original_idx != idx
/// }
/// ```
/// 형태였으나, 논리적으로 move_downward 함수의 반환값은 첫 single_downward호출의 반환과 같으므로 지금과 같이 변경됨
pub(crate) fn move_downward<T, C: Comparator<T> + ?Sized>(
    comp: &C,
    arr: &mut [T],
    mut idx: usize,
) -> bool {
    if single_downward(comp, arr, &mut idx) {
        while single_downward(comp, arr, &mut idx) {}
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
pub(crate) fn heapify<T, C: Comparator<T> + ?Sized>(comp: &C, arr: &mut [T]) {
    for idx in (0..arr.len() / 2).rev() {
        move_downward(comp, arr, idx);
    }
}

/// # Note
/// 메서드 spec은 pop 후 push가 아닌, push 후 pop이다.
/// 따라서 arr이 비어있거나, x가 arr의 root보다 작은 경우 x를 반환한다.
/// root의 값과 같은 경우 최적화를 위해 힙을 조정하지 않고 x를 반환한다.
/// ```ignore
/// if arr.first().map(|y| comp.cmp(y, &x)) == Some(O::Less) {
///     std::mem::swap(&mut arr[0], &mut x);
///     move_downward(comp, arr, 0);
/// }
/// ```
/// 이 코드는 1. arr이 비어있지 않으면서, 2. root가 x보다 작은 경우를 처리하기 위한 코드이다.
/// 아래 코드와 동일한 기능을 한다.
/// ```ignore
/// if !arr.is_empty() && comp.cmp(&arr[0], &x) == O::Less {
///     std::mem::swap(&mut arr[0], &mut x);
///     move_downward(comp, arr, 0);
/// }
/// ```
pub(crate) fn heap_pushpop<T, C: Comparator<T> + ?Sized>(comp: &C, arr: &mut [T], mut x: T) -> T {
    use std::cmp::Ordering as O;
    if arr.first().map(|y| comp.cmp(y, &x)) == Some(O::Less) {
        std::mem::swap(&mut arr[0], &mut x);
        move_downward(comp, arr, 0);
    }
    x
}

/// # Note
/// arr의 길이가 0인 경우와 1인 경우는 둘 다 특수 한 경우이다.
/// ```ignore
/// let (init, last) = arr.len().checked_sub(1).map(|len| arr.split_at_mut(len))?;
/// ```
/// 이 코드는 arr의 길이가 0인 경우를 처리하기 위한 코드이다. 아래 코드와 동일한 기능을 한다.
/// ```ignore
/// let len = arr.len();
/// if len == 0 {
///     return None;
/// }
/// let (init, last) = arr.split_at_mut(len - 1);
/// ```
pub(crate) fn heap_pop<'arr, T, C: Comparator<T> + ?Sized>(
    comp: &C,
    arr: &'arr mut [T],
) -> Option<&'arr mut [T]> {
    let (init, last) = arr.len().checked_sub(1).map(|len| arr.split_at_mut(len))?;
    if !init.is_empty() {
        // arr.len() == 1 인 경우 init은 empty slice가 된다.
        std::mem::swap(&mut init[0], &mut last[0]);
        move_downward(comp, init, 0);
    }
    Some(init)
}

pub(crate) fn heap_reverse_sort<T, C: Comparator<T> + ?Sized>(comp: &C, mut arr: &mut [T]) {
    heapify(comp, arr);
    while let Some(shrinked) = heap_pop(comp, arr) {
        arr = shrinked;
    }
}

pub(crate) fn adjust_heap<T, C: Comparator<T> + ?Sized>(
    comp: &C,
    arr: &mut [T],
    idx: usize,
) -> bool {
    move_upward(comp, arr, idx) || move_downward(comp, arr, idx)
}

#[cfg(test)]
mod unit_test {
    use crate::comparator::DefaultComparator;
    use crate::heap_implementation::*;

    #[test]
    fn is_heap_true_and_false() {
        let comp = DefaultComparator;
        // empty and single-element heaps
        let empty: Vec<i32> = vec![];
        assert!(is_heap(&comp, &empty));
        let single = vec![1];
        assert!(is_heap(&comp, &single));
        // valid heap
        let heap = vec![1, 3, 2, 7, 5, 4];
        assert!(is_heap(&comp, &heap));
        // invalid heap: parent greater than child
        let bad = vec![2, 1];
        assert!(!is_heap(&comp, &bad));
    }

    #[test]
    fn test_single_upward_and_downward_adjustments() {
        let comp = DefaultComparator;
        // Upward adjustment scenario
        let mut arr_up = vec![1, 3, 5, 7, 9];
        assert!(is_heap(&comp, &arr_up));
        // break heap property by making a leaf too small
        arr_up[4] = 0;
        assert!(!is_heap(&comp, &arr_up));
        // fix upward
        assert!(move_upward(&comp, &mut arr_up, 4));
        assert!(is_heap(&comp, &arr_up));

        // Downward adjustment scenario
        let mut arr_down = vec![2, 4, 6, 8, 10];
        assert!(is_heap(&comp, &arr_down));
        // break heap property by making root too large
        arr_down[0] = 12;
        assert!(!is_heap(&comp, &arr_down));
        // fix downward
        assert!(move_downward(&comp, &mut arr_down, 0));
        assert!(is_heap(&comp, &arr_down));
    }

    #[test]
    fn test_heapify_builds_valid_heap() {
        let comp = DefaultComparator;
        let mut arr = vec![3, 1, 4, 2, 5];
        heapify(&comp, &mut arr);
        assert!(is_heap(&comp, &arr));
    }

    #[test]
    fn test_heap_pushpop_and_heap_pop() {
        let comp = DefaultComparator;
        // pushpop: small x
        let mut arr = vec![1, 2, 3];
        let x = heap_pushpop(&comp, &mut arr, 0);
        assert_eq!(x, 0);
        assert!(is_heap(&comp, &arr));
        // pushpop: large x
        let mut arr2 = vec![1, 2, 3];
        let y = heap_pushpop(&comp, &mut arr2, 5);
        assert_eq!(y, 1);
        assert!(is_heap(&comp, &arr2));
        // heap_pop
        let mut arr3 = vec![1, 3, 2];
        heapify(&comp, &mut arr3);
        if let Some(init) = heap_pop(&comp, &mut arr3) {
            assert_eq!(init.len(), 2);
            assert!(is_heap(&comp, init));
        } else {
            panic!("heap_pop returned None on non-empty heap");
        }
    }

    #[test]
    fn test_adjust_heap_up_and_down() {
        let comp = DefaultComparator;
        // upward adjustment
        let mut arr = vec![0, 4, 1, 5, 7, 3, 2, 8, 6, 9];
        assert!(is_heap(&comp, &arr));
        // break heap property by changing arr[7] to 3
        arr[7] = 3;
        assert!(!is_heap(&comp, &arr));
        assert!(adjust_heap(&comp, &mut arr, 7));
        assert!(is_heap(&comp, &arr));
        // downward adjustment
        let mut arr2 = vec![0, 4, 1, 5, 7, 3, 2, 8, 6, 9];
        // break heap property by changing arr[0] to 7
        arr2[0] = 7;
        assert!(!is_heap(&comp, &arr2));
        assert!(adjust_heap(&comp, &mut arr2, 0));
        assert!(is_heap(&comp, &arr2));
    }

    #[test]
    fn test_heap_reverse_sort_sorts_descending() {
        let comp = DefaultComparator;
        let mut arr = vec![3, 1, 4, 2, 5];
        heap_reverse_sort(&comp, &mut arr);
        assert_eq!(arr, vec![5, 4, 3, 2, 1]);
    }

    // Optionally, ascending order via ReverseComparator
    #[test]
    fn test_heap_reverse_sort_with_reverse_comparator_sorts_ascending() {
        let comp = crate::comparator::ReverseComparator;
        let mut arr = vec![3, 1, 4, 2, 5];
        heap_reverse_sort(&comp, &mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
