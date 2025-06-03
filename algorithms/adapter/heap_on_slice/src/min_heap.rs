use super::heap_implementation as Impl;
use std::cmp::Ordering;

fn key2compare<T, K, F>(mut key: F) -> impl FnMut(&T, &T) -> Ordering
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    move |a: &T, b: &T| Ord::cmp(&key(a), &key(b))
}

pub fn is_heap<T: Ord>(arr: &[T]) -> bool {
    Impl::is_heap(arr, Ord::cmp)
}

pub fn is_heap_by<T, F>(arr: &[T], compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::is_heap(arr, compare)
}

pub fn is_heap_by_key<T, K, F>(arr: &[T], key: F) -> bool
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::is_heap(arr, key2compare(key))
}

pub fn heapify<T: Ord>(arr: &mut [T]) {
    Impl::heapify(arr, Ord::cmp);
}

pub fn heapify_by<T, F>(arr: &mut [T], compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heapify(arr, compare);
}

pub fn heapify_by_key<T, K, F>(arr: &mut [T], key: F)
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heapify(arr, key2compare(key));
}

pub fn heap_pushpop<T: Ord>(arr: &mut [T], x: T) -> T {
    Impl::heap_pushpop(arr, x, Ord::cmp)
}

pub fn heap_pushpop_by<T, F>(arr: &mut [T], x: T, compare: F) -> T
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heap_pushpop(arr, x, compare)
}

pub fn heap_pushpop_by_key<T, K, F>(arr: &mut [T], x: T, key: F) -> T
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heap_pushpop(arr, x, key2compare(key))
}

pub fn heap_pop<T: Ord>(arr: &mut [T]) -> Option<&mut [T]> {
    Impl::heap_pop(arr, Ord::cmp)
}

pub fn heap_pop_by<T, F>(arr: &mut [T], compare: F) -> Option<&mut [T]>
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heap_pop(arr, compare)
}

pub fn heap_pop_by_key<T, K, F>(arr: &mut [T], key: F) -> Option<&mut [T]>
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heap_pop(arr, key2compare(key))
}

pub fn heap_reverse_sort<T: Ord>(arr: &mut [T]) {
    Impl::heap_reverse_sort(arr, Ord::cmp);
}

pub fn heap_reverse_sort_by<T, F>(arr: &mut [T], compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heap_reverse_sort(arr, compare);
}

pub fn heap_reverse_sort_by_key<T, K, F>(arr: &mut [T], key: F)
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heap_reverse_sort(arr, key2compare(key));
}

pub fn adjust_heap<T: Ord>(arr: &mut [T], idx: usize) -> bool {
    Impl::adjust_heap(arr, idx, Ord::cmp)
}

pub fn adjust_heap_by<T, F>(arr: &mut [T], idx: usize, compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::adjust_heap(arr, idx, compare)
}

pub fn adjust_heap_by_key<T, K, F>(arr: &mut [T], idx: usize, key: F) -> bool
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::adjust_heap(arr, idx, key2compare(key))
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use std::cmp::Ordering;

    // Helper function for custom comparison (reverse order for testing)
    fn reverse_compare<T: Ord>(a: &T, b: &T) -> Ordering {
        b.cmp(a)
    }

    // Helper struct for testing key-based functions
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_is_heap() {
        // Empty heap
        let empty: Vec<i32> = vec![];
        assert!(is_heap(&empty));

        // Single element heap
        let single = vec![42];
        assert!(is_heap(&single));

        // Valid min heap
        let valid_heap = vec![1, 3, 2, 7, 5, 4, 8];
        assert!(is_heap(&valid_heap));

        // Invalid heap (parent greater than child)
        let invalid_heap = vec![5, 3, 2];
        assert!(!is_heap(&invalid_heap));

        // Another invalid heap
        let invalid_heap2 = vec![1, 2, 3, 0];
        assert!(!is_heap(&invalid_heap2));
    }

    #[test]
    fn test_is_heap_by() {
        // Test with custom comparator (reverse order - makes it a max heap)
        let arr = vec![5, 3, 4, 1, 2];
        assert!(is_heap_by(&arr, reverse_compare));

        let invalid_with_reverse = vec![1, 3, 2];
        assert!(!is_heap_by(&invalid_with_reverse, reverse_compare));
    }

    #[test]
    fn test_is_heap_by_key() {
        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
            Person {
                name: "Carol".to_string(),
                age: 28,
            },
            Person {
                name: "David".to_string(),
                age: 35,
            },
        ];

        // Should be a valid min heap when sorted by age
        assert!(is_heap_by_key(&people, |p| p.age));

        let invalid_people = vec![
            Person {
                name: "Alice".to_string(),
                age: 30,
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
            },
        ];

        // Should be invalid min heap by age
        assert!(!is_heap_by_key(&invalid_people, |p| p.age));
    }

    #[test]
    fn test_heapify() {
        let mut arr = vec![5, 3, 8, 1, 9, 2, 7];
        heapify(&mut arr);
        assert!(is_heap(&arr));

        // Test with already sorted array
        let mut sorted = vec![1, 2, 3, 4, 5];
        heapify(&mut sorted);
        assert!(is_heap(&sorted));

        // Test with reverse sorted array
        let mut reverse_sorted = vec![5, 4, 3, 2, 1];
        heapify(&mut reverse_sorted);
        assert!(is_heap(&reverse_sorted));

        // Test with empty array
        let mut empty: Vec<i32> = vec![];
        heapify(&mut empty);
        assert!(is_heap(&empty));

        // Test with single element
        let mut single = vec![42];
        heapify(&mut single);
        assert!(is_heap(&single));
    }

    #[test]
    fn test_heapify_by() {
        let mut arr = vec![1, 3, 2, 7, 5];
        heapify_by(&mut arr, reverse_compare);
        assert!(is_heap_by(&arr, reverse_compare));
    }

    #[test]
    fn test_heapify_by_key() {
        let mut people = vec![
            Person {
                name: "Alice".to_string(),
                age: 30,
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
            },
            Person {
                name: "Carol".to_string(),
                age: 35,
            },
            Person {
                name: "David".to_string(),
                age: 20,
            },
        ];

        heapify_by_key(&mut people, |p| p.age);
        assert!(is_heap_by_key(&people, |p| p.age));

        // The person with the smallest age should be at the root
        assert_eq!(people[0].age, 20);
    }

    #[test]
    fn test_heap_pushpop() {
        let mut heap = vec![1, 3, 2, 7, 5];
        heapify(&mut heap);

        // Push a smaller element - should return the new smallest
        let result = heap_pushpop(&mut heap, 0);
        assert_eq!(result, 0);
        assert!(is_heap(&heap));

        // Push a larger element - should return the old smallest
        let result = heap_pushpop(&mut heap, 10);
        assert_eq!(result, 1);
        assert!(is_heap(&heap));

        // Test with empty heap
        let mut empty_heap: Vec<i32> = vec![];
        let result = heap_pushpop(&mut empty_heap, 5);
        assert_eq!(result, 5);
        assert!(empty_heap.is_empty());
    }

    #[test]
    fn test_heap_pushpop_by() {
        let mut arr = vec![5, 3, 4];
        heapify_by(&mut arr, reverse_compare);

        // With reverse compare, this is a max heap, so root should be the largest
        // When we push 6 (larger than root 5), it should return the old root 5
        let result = heap_pushpop_by(&mut arr, 6, reverse_compare);
        assert_eq!(result, 6); // Since 6 > 5, the function should return 6 immediately
        assert!(is_heap_by(&arr, reverse_compare));
    }

    #[test]
    fn test_heap_pushpop_by_key() {
        let mut people = vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
        ];
        heapify_by_key(&mut people, |p| p.age);

        let new_person = Person {
            name: "Carol".to_string(),
            age: 20,
        };
        let result = heap_pushpop_by_key(&mut people, new_person, |p| p.age);

        assert_eq!(result.age, 20);
        assert!(is_heap_by_key(&people, |p| p.age));
    }

    #[test]
    fn test_heap_pop() {
        let mut heap = vec![1, 3, 2, 7, 5, 4];
        let original_len = heap.len();
        heapify(&mut heap);

        // Pop should return the remaining heap without the minimum element
        if let Some(remaining) = heap_pop(&mut heap) {
            assert_eq!(remaining.len(), original_len - 1);
            assert!(is_heap(remaining));
        } else {
            panic!("heap_pop returned None on non-empty heap");
        }

        // Test with single element
        let mut single = vec![42];
        if let Some(remaining) = heap_pop(&mut single) {
            assert_eq!(remaining.len(), 0);
        }

        // Test with empty heap
        let mut empty: Vec<i32> = vec![];
        assert!(heap_pop(&mut empty).is_none());
    }

    #[test]
    fn test_heap_pop_by() {
        let mut arr = vec![5, 3, 4, 1];
        heapify_by(&mut arr, reverse_compare);

        if let Some(remaining) = heap_pop_by(&mut arr, reverse_compare) {
            assert!(is_heap_by(remaining, reverse_compare));
        }
    }

    #[test]
    fn test_heap_pop_by_key() {
        let mut people = vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
            Person {
                name: "Carol".to_string(),
                age: 20,
            },
        ];
        heapify_by_key(&mut people, |p| p.age);

        if let Some(remaining) = heap_pop_by_key(&mut people, |p| p.age) {
            assert!(is_heap_by_key(remaining, |p| p.age));
            assert_eq!(remaining.len(), 2);
        }
    }

    #[test]
    fn test_heap_reverse_sort() {
        let mut arr = vec![3, 1, 4, 2, 5];
        heap_reverse_sort(&mut arr);

        // Should be sorted in descending order (reverse of min heap sort)
        assert_eq!(arr, vec![5, 4, 3, 2, 1]);

        // Test with already sorted array
        let mut sorted = vec![1, 2, 3, 4, 5];
        heap_reverse_sort(&mut sorted);
        assert_eq!(sorted, vec![5, 4, 3, 2, 1]);

        // Test with empty array
        let mut empty: Vec<i32> = vec![];
        heap_reverse_sort(&mut empty);
        assert!(empty.is_empty());

        // Test with single element
        let mut single = vec![42];
        heap_reverse_sort(&mut single);
        assert_eq!(single, vec![42]);
    }

    #[test]
    fn test_heap_reverse_sort_by() {
        let mut arr = vec![3, 1, 4, 2, 5];
        heap_reverse_sort_by(&mut arr, reverse_compare);

        // With reverse comparator, should sort in ascending order
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_heap_reverse_sort_by_key() {
        let mut people = vec![
            Person {
                name: "Alice".to_string(),
                age: 30,
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
            },
            Person {
                name: "Carol".to_string(),
                age: 35,
            },
            Person {
                name: "David".to_string(),
                age: 20,
            },
        ];

        heap_reverse_sort_by_key(&mut people, |p| p.age);

        // Should be sorted by age in descending order
        let ages: Vec<u32> = people.iter().map(|p| p.age).collect();
        assert_eq!(ages, vec![35, 30, 25, 20]);
    }

    #[test]
    fn test_adjust_heap() {
        // Test upward adjustment
        let mut heap_up = vec![1, 2, 3, 4, 0]; // 0 is out of place
        assert!(!is_heap(&heap_up));
        assert!(adjust_heap(&mut heap_up, 4)); // Should move 0 upward
        assert!(is_heap(&heap_up));

        // Test downward adjustment
        let mut heap_down = vec![5, 1, 2, 3, 4]; // 5 is out of place at root
        assert!(!is_heap(&heap_down));
        assert!(adjust_heap(&mut heap_down, 0)); // Should move 5 downward
        assert!(is_heap(&heap_down));

        // Test no adjustment needed
        let mut valid_heap = vec![1, 2, 3, 4, 5];
        assert!(is_heap(&valid_heap));
        assert!(!adjust_heap(&mut valid_heap, 0)); // No adjustment needed
        assert!(is_heap(&valid_heap));
    }

    #[test]
    fn test_adjust_heap_by() {
        let mut arr = vec![5, 3, 4, 1, 2]; // Create a valid max heap with reverse comparator
        heapify_by(&mut arr, reverse_compare);
        assert!(is_heap_by(&arr, reverse_compare));

        // Break the heap property
        arr[0] = 0; // Make root too small for max heap
        assert!(!is_heap_by(&arr, reverse_compare));

        // Fix it
        assert!(adjust_heap_by(&mut arr, 0, reverse_compare));
        assert!(is_heap_by(&arr, reverse_compare));
    }

    #[test]
    fn test_adjust_heap_by_key() {
        let mut people = vec![
            Person {
                name: "Alice".to_string(),
                age: 20,
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
            },
            Person {
                name: "Carol".to_string(),
                age: 23,
            },
        ];
        assert!(is_heap_by_key(&people, |p| p.age));

        // Break heap property by changing age
        people[0].age = 30; // Make root too large
        assert!(!is_heap_by_key(&people, |p| p.age));

        // Fix it
        assert!(adjust_heap_by_key(&mut people, 0, |p| p.age));
        assert!(is_heap_by_key(&people, |p| p.age));
    }

    #[test]
    fn test_edge_cases() {
        // Test with duplicate elements
        let mut duplicates = vec![2, 2, 2, 2, 2];
        heapify(&mut duplicates);
        assert!(is_heap(&duplicates));

        let result = heap_pushpop(&mut duplicates, 2);
        assert_eq!(result, 2);
        assert!(is_heap(&duplicates));

        // Test heap operations maintain heap property
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        heapify(&mut arr);
        assert!(is_heap(&arr));

        // Multiple pushpop operations
        for &x in &[0, 10, -1, 15] {
            heap_pushpop(&mut arr, x);
            assert!(is_heap(&arr));
        }

        // Multiple pop operations
        let original_len = arr.len();
        for i in 0..original_len {
            if let Some(remaining) = heap_pop(&mut arr) {
                assert!(is_heap(remaining));
                arr = remaining.to_vec();
            } else {
                assert_eq!(i, original_len - 1); // Should be the last iteration
            }
        }
    }
}
