use crate::comparator::DefaultComparator;
use crate::min_heap::MinHeap;

// Numeric slice tests
#[test]
fn min_heap_numeric_methods() {
    let comp = DefaultComparator;
    let mut arr = vec![5, 1, 8, 3, 2];
    assert!(!comp.test_heap_property(&arr));
    comp.heapify(&mut arr);
    assert!(comp.test_heap_property(&arr));

    // move_downward (after breaking root)
    arr[0] = 10;
    assert!(!comp.test_heap_property(&arr));
    assert!(comp.move_downward(&mut arr, 0));
    assert!(comp.test_heap_property(&arr));

    // move_upward (after breaking leaf)
    let mut arr2 = arr.clone();
    let len = arr2.len();
    arr2[len - 1] = 0;
    assert!(!comp.test_heap_property(&arr2));
    assert!(comp.move_upward(&mut arr2, len - 1));
    assert!(comp.test_heap_property(&arr2));

    // heap_pushpop and heap_pop
    let mut arr3 = vec![2, 4, 6];
    let x = comp.heap_pushpop(&mut arr3, 1);
    assert_eq!(x, 1);
    assert!(comp.test_heap_property(&arr3));
    let mut arr4 = vec![2, 4, 6];
    if let Some(rest) = comp.heap_pop(&mut arr4) {
        assert_eq!(rest.len(), 2);
        assert!(comp.test_heap_property(rest));
    } else {
        panic!("heap_pop returned None");
    }
}

// String element tests
#[test]
fn min_heap_string_and_vec_methods() {
    let comp = DefaultComparator;
    // Strings
    let mut strs = vec![
        "delta".to_string(),
        "alpha".to_string(),
        "charlie".to_string(),
    ];
    comp.heapify(&mut strs);
    assert_eq!(strs[0], "alpha");
    assert!(comp.test_heap_property(&strs));

    // Vec<i32>
    let mut vecs = vec![vec![3], vec![1, 2], vec![1]];
    comp.heapify(&mut vecs);
    assert_eq!(vecs[0], vec![1]);
    assert!(comp.test_heap_property(&vecs));
}

// Custom comparator: AbstractDistance
struct AbstractDistance {
    center: i32,
}
impl crate::comparator::Comparator<i32> for AbstractDistance {
    fn cmp(&self, a: &i32, b: &i32) -> std::cmp::Ordering {
        let da = a.abs_diff(self.center);
        let db = b.abs_diff(self.center);
        da.cmp(&db)
    }
}

fn slice_is_reverse_sorted<T, C: crate::Comparator<T>>(comp: &C, arr: &[T]) -> bool {
    for pair in arr.windows(2) {
        if comp.cmp(&pair[0], &pair[1]) == std::cmp::Ordering::Less {
            return false;
        }
    }
    true
}

#[test]
fn min_heap_custom_comparator_distance() {
    let comp = AbstractDistance { center: 3 };
    let mut arr = vec![1, 5, 3, 7, 2];
    comp.heapify(&mut arr);
    assert!(comp.test_heap_property(&arr));
    // the root should be the element closest to center (3)
    let root = arr[0];
    assert_eq!(root, 3);
    // sorting reverse-order on distance
    comp.heap_reverse_sort(&mut arr);
    // after reverse sort by distance, farthest appears first
    assert_eq!(arr[0], 7);
    // check if the array is sorted in reverse order
    assert!(slice_is_reverse_sorted(&comp, &arr));
}
