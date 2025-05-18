use crate::comparator::DefaultComparator;
use crate::max_heap::MaxHeap;

// Numeric slice tests
#[test]
fn max_heap_numeric_methods() {
    let comp = DefaultComparator;
    // initial array
    let mut arr = vec![5, 1, 8, 3, 2];
    assert!(!comp.test_heap_property(&arr));
    comp.heapify(&mut arr);
    assert!(comp.test_heap_property(&arr));
    assert_eq!(arr[0], 8);

    // break leaf to be too large
    let len = arr.len();
    arr[len - 1] = 10;
    assert!(!comp.test_heap_property(&arr));
    assert!(comp.move_upward(&mut arr, len - 1));
    assert!(comp.test_heap_property(&arr));

    // break root to be too small
    arr[0] = 0;
    assert!(!comp.test_heap_property(&arr));
    assert!(comp.move_downward(&mut arr, 0));
    assert!(comp.test_heap_property(&arr));

    // heap_pushpop: small x (should pop old max)
    let mut arr2 = vec![4, 5, 6];
    comp.heapify(&mut arr2);
    let x = comp.heap_pushpop(&mut arr2, 3);
    assert_eq!(x, 6);
    assert!(comp.test_heap_property(&arr2));

    // heap_pushpop: large x (returned without change)
    let mut arr3 = vec![4, 5, 6];
    comp.heapify(&mut arr3);
    let y = comp.heap_pushpop(&mut arr3, 7);
    assert_eq!(y, 7);
    assert!(comp.test_heap_property(&arr3));

    // heap_pop: remove max element
    let mut arr4 = vec![2, 4, 6];
    comp.heapify(&mut arr4);
    if let Some(rest) = comp.heap_pop(&mut arr4) {
        assert_eq!(rest.len(), 2);
        assert!(comp.test_heap_property(rest));
    } else {
        panic!("heap_pop returned None");
    }
}

// String and Vec<i32> tests
#[test]
fn max_heap_string_and_vec_methods() {
    let comp = DefaultComparator;
    // Strings: root should be lexicographically largest
    let mut strs = vec![
        "alpha".to_string(),
        "delta".to_string(),
        "charlie".to_string(),
    ];
    comp.heapify(&mut strs);
    assert_eq!(strs[0], "delta");
    assert!(comp.test_heap_property(&strs));

    // Vec<i32>: compare by lexicographic order
    let mut vecs = vec![vec![1], vec![3, 4], vec![2]];
    comp.heapify(&mut vecs);
    assert_eq!(vecs[0], vec![3, 4]);
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

fn slice_is_sorted<T, C: crate::Comparator<T>>(comp: &C, arr: &[T]) -> bool {
    for i in 1..arr.len() {
        if comp.cmp(&arr[i], &arr[i - 1]) == std::cmp::Ordering::Less {
            return false;
        }
    }
    true
}

#[test]
fn max_heap_custom_comparator_distance() {
    let comp = AbstractDistance { center: 3 };
    let mut arr = vec![1, 5, 3, 7, 2];
    comp.heapify(&mut arr);
    assert!(comp.test_heap_property(&arr));
    // root is farthest from center
    assert_eq!(arr[0], 7);

    // heap_sort produces ascending by distance (via reverse then pop)
    comp.heap_sort(&mut arr);
    // after sort, closest to center appears first
    assert_eq!(arr[0], 3);
    // check if the array is sorted by distance
    assert!(slice_is_sorted(&comp, &arr));
}
