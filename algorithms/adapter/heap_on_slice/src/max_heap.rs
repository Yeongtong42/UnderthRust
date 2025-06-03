use super::heap_implementation as Impl;
use std::cmp::Ordering;

fn reversed_cmp<T: Ord>(a: &T, b: &T) -> Ordering {
    Ord::cmp(a, b).reverse()
}

fn reverse_compare<T, F>(mut compare: F) -> impl FnMut(&T, &T) -> Ordering
where
    F: FnMut(&T, &T) -> Ordering,
{
    move |a, b| compare(a, b).reverse()
}

fn key2reversed_compare<T, K, F>(mut key: F) -> impl FnMut(&T, &T) -> Ordering
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    move |a, b| Ord::cmp(&key(a), &key(b)).reverse()
}

pub fn is_heap<T: Ord>(arr: &[T]) -> bool {
    Impl::is_heap(arr, reversed_cmp)
}

pub fn is_heap_by<T, F>(arr: &[T], compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::is_heap(arr, reverse_compare(compare))
}

pub fn is_heap_by_key<T, K, F>(arr: &[T], key: F) -> bool
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::is_heap(arr, key2reversed_compare(key))
}

pub fn heapify<T: Ord>(arr: &mut [T]) {
    Impl::heapify(arr, reversed_cmp);
}

pub fn heapify_by<T, F>(arr: &mut [T], compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heapify(arr, reverse_compare(compare));
}

pub fn heapify_by_key<T, K, F>(arr: &mut [T], key: F)
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heapify(arr, key2reversed_compare(key));
}

pub fn heap_pushpop<T: Ord>(arr: &mut [T], x: T) -> T {
    Impl::heap_pushpop(arr, x, reversed_cmp)
}

pub fn heap_pushpop_by<T, F>(arr: &mut [T], x: T, compare: F) -> T
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heap_pushpop(arr, x, reverse_compare(compare))
}

pub fn heap_pushpop_by_key<T, K, F>(arr: &mut [T], x: T, key: F) -> T
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heap_pushpop(arr, x, key2reversed_compare(key))
}

pub fn heap_pop<T: Ord>(arr: &mut [T]) -> Option<&mut [T]> {
    Impl::heap_pop(arr, reversed_cmp)
}

pub fn heap_pop_by<T, F>(arr: &mut [T], compare: F) -> Option<&mut [T]>
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heap_pop(arr, reverse_compare(compare))
}

pub fn heap_pop_by_key<T, K, F>(arr: &mut [T], key: F) -> Option<&mut [T]>
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heap_pop(arr, key2reversed_compare(key))
}

pub fn heapsort<T: Ord>(arr: &mut [T]) {
    Impl::heap_reverse_sort(arr, reversed_cmp);
}

pub fn heapsort_by<T, F>(arr: &mut [T], compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heap_reverse_sort(arr, reverse_compare(compare));
}

pub fn heapsort_by_key<T, K, F>(arr: &mut [T], key: F)
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heap_reverse_sort(arr, key2reversed_compare(key));
}

pub fn adjust_heap<T: Ord>(arr: &mut [T], idx: usize) -> bool {
    Impl::adjust_heap(arr, idx, reversed_cmp)
}

pub fn adjust_heap_by<T, F>(arr: &mut [T], idx: usize, compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::adjust_heap(arr, idx, reverse_compare(compare))
}

pub fn adjust_heap_by_key<T, K, F>(arr: &mut [T], idx: usize, key: F) -> bool
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::adjust_heap(arr, idx, key2reversed_compare(key))
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use std::cmp::Ordering;

    // Helper function for custom comparison (reverse order for testing - making it min heap behavior)
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

        // Valid max heap
        let valid_heap = vec![8, 7, 5, 3, 4, 2, 1];
        assert!(is_heap(&valid_heap));

        // Invalid heap (parent smaller than child)
        let invalid_heap = vec![2, 3, 5];
        assert!(!is_heap(&invalid_heap));

        // Another invalid heap
        let invalid_heap2 = vec![8, 7, 6, 9];
        assert!(!is_heap(&invalid_heap2));
    }

    #[test]
    fn test_is_heap_by() {
        // Test with custom comparator (reverse order - makes it a min heap)
        let arr = vec![1, 2, 3, 4, 5];
        assert!(is_heap_by(&arr, reverse_compare));

        let invalid_with_reverse = vec![5, 3, 4];
        assert!(!is_heap_by(&invalid_with_reverse, reverse_compare));
    }

    #[test]
    fn test_is_heap_by_key() {
        let people = vec![
            Person { name: "Alice".to_string(), age: 35 },
            Person { name: "Bob".to_string(), age: 30 },
            Person { name: "Carol".to_string(), age: 28 },
            Person { name: "David".to_string(), age: 25 },
        ];
        
        // Should be a valid max heap when sorted by age
        assert!(is_heap_by_key(&people, |p| p.age));

        let invalid_people = vec![
            Person { name: "Alice".to_string(), age: 25 },
            Person { name: "Bob".to_string(), age: 30 },
        ];
        
        // Should be invalid max heap by age
        assert!(!is_heap_by_key(&invalid_people, |p| p.age));
    }

    #[test]
    fn test_heapify() {
        let mut arr = vec![5, 3, 8, 1, 9, 2, 7];
        heapify(&mut arr);
        assert!(is_heap(&arr));
        assert_eq!(arr[0], 9); // Largest element should be at root
        
        // Test with already sorted array
        let mut sorted = vec![1, 2, 3, 4, 5];
        heapify(&mut sorted);
        assert!(is_heap(&sorted));
        assert_eq!(sorted[0], 5); // Largest element should be at root

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
        // With reverse compare, smallest should be at root
        assert_eq!(arr[0], 1);
    }

    #[test]
    fn test_heapify_by_key() {
        let mut people = vec![
            Person { name: "Alice".to_string(), age: 30 },
            Person { name: "Bob".to_string(), age: 25 },
            Person { name: "Carol".to_string(), age: 35 },
            Person { name: "David".to_string(), age: 20 },
        ];
        
        heapify_by_key(&mut people, |p| p.age);
        assert!(is_heap_by_key(&people, |p| p.age));
        
        // The person with the largest age should be at the root
        assert_eq!(people[0].age, 35);
    }

    #[test]
    fn test_heap_pushpop() {
        let mut heap = vec![8, 7, 5, 3, 4];
        heapify(&mut heap);
        
        // Push a larger element - should return the larger element (since heap_pushpop returns the max)
        let result = heap_pushpop(&mut heap, 10);
        assert_eq!(result, 10);
        assert!(is_heap(&heap));
        assert_eq!(heap[0], 8); // Original max should remain at root
        
        // Push a smaller element - should return the current max
        let result = heap_pushpop(&mut heap, 1);
        assert_eq!(result, 8);
        assert!(is_heap(&heap));
        assert_eq!(heap[0], 7); // Next largest should be at root after popping 8
        
        // Test with empty heap
        let mut empty_heap: Vec<i32> = vec![];
        let result = heap_pushpop(&mut empty_heap, 5);
        assert_eq!(result, 5);
        assert!(empty_heap.is_empty());
    }

    #[test]
    fn test_heap_pushpop_by() {
        let mut arr = vec![1, 2, 3];
        heapify_by(&mut arr, reverse_compare);
        
        // With reverse compare, this is a min heap, so root should be the smallest
        // When we push 0 (smaller than root 1), it should return the smaller value (0)
        let result = heap_pushpop_by(&mut arr, 0, reverse_compare);
        assert_eq!(result, 0);
        assert!(is_heap_by(&arr, reverse_compare));
        assert_eq!(arr[0], 1); // Original smallest should remain at root
        
        // When we push 5 (larger than root 1), it should return the current min (1)
        let result = heap_pushpop_by(&mut arr, 5, reverse_compare);
        assert_eq!(result, 1);
        assert!(is_heap_by(&arr, reverse_compare));
    }

    #[test]
    fn test_heap_pushpop_by_key() {
        let mut people = vec![
            Person { name: "Alice".to_string(), age: 25 },
            Person { name: "Bob".to_string(), age: 30 },
        ];
        heapify_by_key(&mut people, |p| p.age);
        
        let new_person = Person { name: "Carol".to_string(), age: 35 };
        let result = heap_pushpop_by_key(&mut people, new_person, |p| p.age);
        
        assert_eq!(result.age, 35); // The larger age should be returned
        assert!(is_heap_by_key(&people, |p| p.age));
        assert_eq!(people[0].age, 30); // Original max should remain at root
        
        // Test pushing a smaller age
        let smaller_person = Person { name: "David".to_string(), age: 20 };
        let result = heap_pushpop_by_key(&mut people, smaller_person, |p| p.age);
        
        assert_eq!(result.age, 30); // Current max should be returned
        assert!(is_heap_by_key(&people, |p| p.age));
    }

    #[test]
    fn test_heap_pop() {
        let mut heap = vec![8, 7, 5, 3, 4, 2];
        let original_len = heap.len();
        heapify(&mut heap);
        
        // Pop should return the remaining heap without the maximum element
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
        let mut arr = vec![1, 3, 2, 5];
        heapify_by(&mut arr, reverse_compare);
        
        if let Some(remaining) = heap_pop_by(&mut arr, reverse_compare) {
            assert!(is_heap_by(remaining, reverse_compare));
        }
    }

    #[test]
    fn test_heap_pop_by_key() {
        let mut people = vec![
            Person { name: "Alice".to_string(), age: 25 },
            Person { name: "Bob".to_string(), age: 30 },
            Person { name: "Carol".to_string(), age: 35 },
        ];
        heapify_by_key(&mut people, |p| p.age);
        
        if let Some(remaining) = heap_pop_by_key(&mut people, |p| p.age) {
            assert!(is_heap_by_key(remaining, |p| p.age));
            assert_eq!(remaining.len(), 2);
        }
    }

    #[test]
    fn test_heapsort() {
        let mut arr = vec![3, 1, 4, 2, 5];
        heapsort(&mut arr);
        
        // Should be sorted in ascending order (reverse of max heap sort)
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
        
        // Test with already sorted array
        let mut sorted = vec![1, 2, 3, 4, 5];
        heapsort(&mut sorted);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
        
        // Test with empty array
        let mut empty: Vec<i32> = vec![];
        heapsort(&mut empty);
        assert!(empty.is_empty());
        
        // Test with single element
        let mut single = vec![42];
        heapsort(&mut single);
        assert_eq!(single, vec![42]);
    }

    #[test]
    fn test_heapsort_by() {
        let mut arr = vec![3, 1, 4, 2, 5];
        heapsort_by(&mut arr, reverse_compare);
        
        // With reverse comparator, should sort in descending order
        assert_eq!(arr, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_heapsort_by_key() {
        let mut people = vec![
            Person { name: "Alice".to_string(), age: 30 },
            Person { name: "Bob".to_string(), age: 25 },
            Person { name: "Carol".to_string(), age: 35 },
            Person { name: "David".to_string(), age: 20 },
        ];
        
        heapsort_by_key(&mut people, |p| p.age);
        
        // Should be sorted by age in ascending order
        let ages: Vec<u32> = people.iter().map(|p| p.age).collect();
        assert_eq!(ages, vec![20, 25, 30, 35]);
    }

    #[test]
    fn test_adjust_heap() {
        // Test upward adjustment
        let mut heap_up = vec![8, 7, 6, 5, 9]; // 9 is out of place
        assert!(!is_heap(&heap_up));
        assert!(adjust_heap(&mut heap_up, 4)); // Should move 9 upward
        assert!(is_heap(&heap_up));
        assert_eq!(heap_up[0], 9); // 9 should be at root
        
        // Test downward adjustment
        let mut heap_down = vec![1, 7, 6, 5, 4]; // 1 is out of place at root
        assert!(!is_heap(&heap_down));
        assert!(adjust_heap(&mut heap_down, 0)); // Should move 1 downward
        assert!(is_heap(&heap_down));
        
        // Test no adjustment needed
        let mut valid_heap = vec![5, 4, 3, 2, 1];
        assert!(is_heap(&valid_heap));
        assert!(!adjust_heap(&mut valid_heap, 0)); // No adjustment needed
        assert!(is_heap(&valid_heap));
    }

    #[test]
    fn test_adjust_heap_by() {
        let mut arr = vec![1, 2, 3, 4, 5]; // Create a valid min heap with reverse comparator
        heapify_by(&mut arr, reverse_compare);
        assert!(is_heap_by(&arr, reverse_compare));
        
        // Break the heap property
        arr[0] = 9; // Make root too large for min heap
        assert!(!is_heap_by(&arr, reverse_compare));
        
        // Fix it
        assert!(adjust_heap_by(&mut arr, 0, reverse_compare));
        assert!(is_heap_by(&arr, reverse_compare));
    }

    #[test]
    fn test_adjust_heap_by_key() {
        let mut people = vec![
            Person { name: "Alice".to_string(), age: 30 },
            Person { name: "Bob".to_string(), age: 25 },
            Person { name: "Carol".to_string(), age: 28 },
        ];
        assert!(is_heap_by_key(&people, |p| p.age));
        
        // Break heap property by changing age
        people[0].age = 20; // Make root too small for max heap
        assert!(!is_heap_by_key(&people, |p| p.age));
        
        // Fix it
        assert!(adjust_heap_by_key(&mut people, 0, |p| p.age));
        assert!(is_heap_by_key(&people, |p| p.age));
    }

    #[test]
    fn test_edge_cases() {
        // Test with duplicate elements
        let mut duplicates = vec![3, 3, 3, 3, 3];
        heapify(&mut duplicates);
        assert!(is_heap(&duplicates));
        
        let result = heap_pushpop(&mut duplicates, 3);
        assert_eq!(result, 3);
        assert!(is_heap(&duplicates));
        
        // Test heap operations maintain heap property
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        heapify(&mut arr);
        assert!(is_heap(&arr));
        
        // Multiple pushpop operations
        for &x in &[10, 0, 15, -1] {
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


