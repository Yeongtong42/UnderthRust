use super::*;
use test_data::{TEST_I32_SLICE_OF_SLICES, TEST_I32_SLICES, TEST_TUPLE_I32_SLICES};

#[test]
fn test_namespace() {
    use max_heap::MaxHeap;
    use min_heap::MinHeap;
    let mut slice = vec![1, 2, 3, 4, 5];
    let comp = DefaultComparator::default();
    comp.heapify(&mut slice);
    MaxHeap::heapify(&comp, &mut slice);
    MinHeap::heapify(&comp, &mut slice);
}

mod default_comparator_minheap {
    // Test default comparator for min heap
    use super::*;
    use crate::min_heap::MinHeap;

    // Test heap property
    fn test_heap_property<T, C: Comparator<T>>(comp: &C, heap: &[T]) {
        for i in 0..heap.len() {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            if left < heap.len() {
                assert!(comp.cmp(&heap[i], &heap[left]) != std::cmp::Ordering::Greater)
            }
            if right < heap.len() {
                assert!(comp.cmp(&heap[i], &heap[right]) != std::cmp::Ordering::Greater)
            }
        }
    }

    #[test]
    fn test_heapify() {
        // Test heapify
        // for each test case
        // heapify the array and check if the heap property is satisfied
        // heap property: for each node i, arr[i] <= arr[2*i+1] and arr[i] <= arr[2*i+2]
        let data = &TEST_I32_SLICES;
        let comparator = DefaultComparator::default();

        // println!("TEST 1");
        for (_tc, &arr) in data.iter().enumerate() {
            // println!("TEST CASE {}", _tc);
            let mut heap = arr.to_vec();
            comparator.heapify(&mut heap);
            test_heap_property(&comparator, &heap);
        }

        // println!("TEST 2");
        let data = &TEST_TUPLE_I32_SLICES;
        let comparator = DefaultComparator::default();
        for (_tc, &arr) in data.iter().enumerate() {
            // println!("TEST CASE {}", _tc);
            let mut heap = arr.to_vec();
            comparator.heapify(&mut heap);
            test_heap_property(&comparator, &heap);
        }

        // println!("TEST 3");
        let data = &TEST_I32_SLICE_OF_SLICES;
        let comparator = DefaultComparator::default();
        for (_tc, &arr) in data.iter().enumerate() {
            // println!("TEST CASE {}", _tc);
            let mut heap: Vec<Vec<i32>> = arr.iter().map(|x| x.to_vec()).collect();
            comparator.heapify(&mut heap);
            test_heap_property(&comparator, &heap);
        }
    }

    #[test]
    fn test_move_upward() {
        // Test move_upward
        // for each test case
        // repeat the following steps
    }

    #[test]
    fn test_move_downward() {
        // TODO: implement test for move_downward
        // todo!();
    }

    #[test]
    fn test_adjust_heap() {
        // TODO: implement test for adjust_heap
        // todo!();
    }

    #[test]
    fn test_heap_pushpop() {
        use super::min_heap::MinHeap;
        // Test heap_pushpop
        // for k in 1..=10
        // build fixed-size heap with arr[0..k]
        // and pushpop arr[k..] and check if the popped value is the minimum among heap
        let data = &TEST_I32_SLICES;
        let comparator = DefaultComparator::default();

        // println!("TEST 1");
        for (_tc, &arr) in data.iter().enumerate() {
            for k in 1..Ord::min(arr.len(), 10) {
                // println!("TEST CASE {}", _tc);
                let mut heap = arr[0..k].to_vec();
                comparator.heapify(&mut heap);
                for item in arr[k..].iter() {
                    let popped = comparator.heap_pushpop(&mut heap, *item);
                    for x in heap.iter() {
                        assert!(popped <= *x, "{:?} <= {:?}", popped, *x);
                    }
                }
            }
        }

        // println!("TEST 2");
        let data = &TEST_TUPLE_I32_SLICES;
        let comparator = DefaultComparator::default();
        for (_tc, &arr) in data.iter().enumerate() {
            for k in 1..Ord::min(arr.len(), 10) {
                // println!("TEST CASE {}", _tc);
                let mut heap = arr[0..k].to_vec();
                comparator.heapify(&mut heap);
                for item in arr[k..].iter() {
                    let popped = comparator.heap_pushpop(&mut heap, *item);
                    for x in heap.iter() {
                        assert!(popped <= *x, "{:?} <= {:?}", popped, *x);
                    }
                }
            }
        }

        // println!("TEST 3");
        let data = &TEST_I32_SLICE_OF_SLICES;
        let comparator = DefaultComparator::default();
        for (_tc, &arr) in data.iter().enumerate() {
            for k in 1..Ord::min(arr.len(), 10) {
                // println!("TEST CASE {}", _tc);
                let mut heap: Vec<Vec<i32>> = arr[0..k].iter().map(|x| x.to_vec()).collect();
                comparator.heapify(&mut heap);
                for item in arr[k..].iter() {
                    let popped = comparator.heap_pushpop(&mut heap, item.to_vec());
                    for x in heap.iter() {
                        assert!(popped <= *x, "{:?} <= {:?}", popped, *x);
                    }
                }
            }
        }
    }

    #[test]
    fn test_heap_pop() {
        // TODO: implement test for heap_pop
        // todo!();
    }

    #[test]
    fn test_heap_reverse_sort() {
        // TODO: implement test for heap_reverse_sort
        // todo!();
    }
}

mod default_comparator_maxheap {
    // TODO: implement tests for max heap
    use super::*;
    use crate::max_heap::MaxHeap;

    // Test heap property
    fn test_heap_property<T, C: Comparator<T>>(comp: &C, heap: &[T]) {
        for i in 0..heap.len() {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            if left < heap.len() {
                assert!(comp.cmp(&heap[i], &heap[left]) != std::cmp::Ordering::Less)
            }
            if right < heap.len() {
                assert!(comp.cmp(&heap[i], &heap[right]) != std::cmp::Ordering::Less)
            }
        }
    }

    #[test]
    fn test_heapify() {
        // TODO: implement test for heapify
        // todo!();
    }

    #[test]
    fn test_move_upward() {
        // TODO: implement test for move_upward
        // todo!();
    }

    #[test]
    fn test_move_downward() {
        // TODO: implement test for move_downward
        // todo!();
    }

    #[test]
    fn test_adjust_heap() {
        // TODO: implement test for adjust_heap
        // todo!();
    }

    #[test]
    fn test_heap_pushpop() {
        // TODO: implement test for heap_pushpop
        // todo!();
    }

    #[test]
    fn test_heap_pop() {
        // TODO: implement test for heap_pop
        // todo!();
    }

    #[test]
    fn test_heap_sort() {
        // TODO: implement test for heap_sort
        // todo!();
    }
}

mod custom_comparator_minheap {
    // TODO: implement tests for min heap with custom comparator
}

mod custom_comparator_maxheap {
    // TODO: implement tests for max heap with custom comparator
}
