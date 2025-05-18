use crate::comparator::DefaultComparator;
use crate::heap_implementation::*;

#[test]
fn test_heap_property_true_and_false() {
    let comp = DefaultComparator;
    // empty and single-element heaps
    let empty: Vec<i32> = vec![];
    assert!(test_heap_property(&comp, &empty));
    let single = vec![1];
    assert!(test_heap_property(&comp, &single));
    // valid heap
    let heap = vec![1, 3, 2, 7, 5, 4];
    assert!(test_heap_property(&comp, &heap));
    // invalid heap: parent greater than child
    let bad = vec![2, 1];
    assert!(!test_heap_property(&comp, &bad));
}

#[test]
fn test_single_upward_and_downward_adjustments() {
    let comp = DefaultComparator;
    // Upward adjustment scenario
    let mut arr_up = vec![1, 3, 5, 7, 9];
    assert!(test_heap_property(&comp, &arr_up));
    // break heap property by making a leaf too small
    arr_up[4] = 0;
    assert!(!test_heap_property(&comp, &arr_up));
    // fix upward
    assert!(move_upward(&comp, &mut arr_up, 4));
    assert!(test_heap_property(&comp, &arr_up));

    // Downward adjustment scenario
    let mut arr_down = vec![2, 4, 6, 8, 10];
    assert!(test_heap_property(&comp, &arr_down));
    // break heap property by making root too large
    arr_down[0] = 12;
    assert!(!test_heap_property(&comp, &arr_down));
    // fix downward
    assert!(move_downward(&comp, &mut arr_down, 0));
    assert!(test_heap_property(&comp, &arr_down));
}

#[test]
fn test_heapify_builds_valid_heap() {
    let comp = DefaultComparator;
    let mut arr = vec![3, 1, 4, 2, 5];
    heapify(&comp, &mut arr);
    assert!(test_heap_property(&comp, &arr));
}

#[test]
fn test_heap_pushpop_and_heap_pop() {
    let comp = DefaultComparator;
    // pushpop: small x
    let mut arr = vec![1, 2, 3];
    let x = heap_pushpop(&comp, &mut arr, 0);
    assert_eq!(x, 0);
    assert!(test_heap_property(&comp, &arr));
    // pushpop: large x
    let mut arr2 = vec![1, 2, 3];
    let y = heap_pushpop(&comp, &mut arr2, 5);
    assert_eq!(y, 1);
    assert!(test_heap_property(&comp, &arr2));
    // heap_pop
    let mut arr3 = vec![1, 3, 2];
    heapify(&comp, &mut arr3);
    if let Some(init) = heap_pop(&comp, &mut arr3) {
        assert_eq!(init.len(), 2);
        assert!(test_heap_property(&comp, init));
    } else {
        panic!("heap_pop returned None on non-empty heap");
    }
}

#[test]
fn test_adjust_heap_up_and_down() {
    let comp = DefaultComparator;
    // upward adjustment
    let mut arr = vec![0, 4, 1, 5, 7, 3, 2, 8, 6, 9];
    assert!(test_heap_property(&comp, &arr));
    // break heap property by changing arr[7] to 3
    arr[7] = 3;
    assert!(!test_heap_property(&comp, &arr));
    assert!(adjust_heap(&comp, &mut arr, 7));
    assert!(test_heap_property(&comp, &arr));
    // downward adjustment
    let mut arr2 = vec![0, 4, 1, 5, 7, 3, 2, 8, 6, 9];
    // break heap property by changing arr[0] to 7
    arr2[0] = 7;
    assert!(!test_heap_property(&comp, &arr2));
    assert!(adjust_heap(&comp, &mut arr2, 0));
    assert!(test_heap_property(&comp, &arr2));
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
