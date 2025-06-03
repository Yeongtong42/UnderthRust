//! # Max Heap 연산
//!
//! `&mut [T]` slice에 대한 maximum binary heap 연산을 제공합니다.
//!
//! Max heap은 부모 노드의 값이 항상 자식 노드의 값보다 크거나 같은 complete binary tree입니다.
//! 이 구현에서는 가장 큰 원소가 root(index 0)에 위치하며, heap property를 유지합니다.
//!
//! ## 주요 함수
//!
//! ### Heap 검증
//! - [`is_heap`]: slice가 valid max heap인지 확인
//! - [`is_heap_by`]: 사용자 정의 comparator로 heap 검증
//! - [`is_heap_by_key`]: key extraction 함수로 heap 검증
//!
//! ### Heap 구성
//! - [`heapify`]: 임의의 slice를 valid max heap으로 변환
//! - [`heapify_by`], [`heapify_by_key`]: 사용자 정의 비교 기준으로 heapify
//!
//! ### Priority Queue 연산
//! - [`heap_pushpop`]: 새 원소 추가 후 최대값 제거
//! - [`heap_pop`]: 최대 원소 제거
//! - [`adjust_heap`]: 특정 위치의 heap property 복구
//!
//! ### 정렬
//! - [`heapsort`]: in-place 오름차순 정렬 (내림차순 정렬은 [`min_heap::heap_reverse_sort`](crate::min_heap::heap_reverse_sort) 사용)
//!
//! ## 사용 예시
//!
//! ### 기본 사용법
//!
//! ```rust
//! use heap_on_slice::max_heap::*;
//!
//! let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
//!
//! // Slice를 max heap으로 변환
//! heapify(&mut arr);
//! assert!(is_heap(&arr));
//! assert_eq!(arr[0], 9); // 최대값이 root에 위치
//!
//! // Priority queue로 사용
//! let max_val = heap_pushpop(&mut arr, 10); // 10을 추가하고 최대값 반환
//! assert_eq!(max_val, 10);
//!
//! // 최대 원소 제거
//! if let Some(remaining) = heap_pop(&mut arr) {
//!     assert!(is_heap(remaining));
//! }
//!
//! // 정렬
//! heapsort(&mut arr);
//! // arr는 이제 오름차순으로 정렬됨
//! ```
//!
//! ### Custom comparator 사용
//!
//! ```rust
//! use heap_on_slice::max_heap::*;
//! use std::cmp::Ordering;
//!
//! let mut arr = vec![5, 3, 4, 1, 2];
//!
//! // 역순 비교로 min heap 동작 구현
//! heapify_by(&mut arr, |a, b| b.cmp(a));
//! assert!(is_heap_by(&arr, |a, b| b.cmp(a)));
//! ```
//!
//! ### Key-based 연산
//!
//! ```rust
//! use heap_on_slice::max_heap::*;
//!
//! #[derive(Debug, PartialEq)]
//! struct Person { name: String, age: u32 }
//!
//! let mut people = vec![
//!     Person { name: "Alice".to_string(), age: 30 },
//!     Person { name: "Bob".to_string(), age: 25 },
//!     Person { name: "Carol".to_string(), age: 35 },
//! ];
//!
//! // 나이 기준으로 max heap 구성
//! heapify_by_key(&mut people, |p| p.age);
//! assert!(is_heap_by_key(&people, |p| p.age));
//! assert_eq!(people[0].age, 35); // 가장 나이 많은 사람이 root
//! ```

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

/// slice가 valid max heap인지 확인합니다.
///
/// Max heap property를 만족하는지 검사합니다: 모든 부모 노드는 자식 노드보다 크거나 같아야 합니다.
/// 빈 slice와 단일 원소 slice는 항상 valid heap으로 간주됩니다.
///
/// # 시간 복잡도
///
/// O(n) - 모든 노드를 한 번씩 검사합니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::max_heap::is_heap;
///
/// let valid_heap = vec![8, 7, 5, 3, 4, 2, 1];
/// assert!(is_heap(&valid_heap));
///
/// let invalid_heap = vec![2, 3, 5]; // 2 < 3이므로 invalid
/// assert!(!is_heap(&invalid_heap));
/// ```
pub fn is_heap<T: Ord>(arr: &[T]) -> bool {
    Impl::is_heap(arr, reversed_cmp)
}

/// 사용자 정의 comparator로 slice가 valid heap인지 확인합니다.
///
/// [`is_heap`]과 동일한 기능을 하지만 `Ord::cmp` 대신 `compare` 함수를 인자로 받습니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::max_heap::is_heap_by;
///
/// let arr = vec![1, 2, 3, 4, 5];
/// // 역순 비교로 min heap 검증
/// assert!(is_heap_by(&arr, |a, b| b.cmp(a)));
/// ```
pub fn is_heap_by<T, F>(arr: &[T], compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::is_heap(arr, reverse_compare(compare))
}

/// key extraction 함수로 slice가 valid heap인지 확인합니다.
///
/// [`is_heap`]과 동일한 기능을 하지만 원소 비교 시 `key` 함수로 추출한 값을 사용합니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::max_heap::is_heap_by_key;
///
/// #[derive(Debug)]
/// struct Person { age: u32 }
///
/// let people = vec![
///     Person { age: 35 },
///     Person { age: 30 },
///     Person { age: 28 },
/// ];
/// assert!(is_heap_by_key(&people, |p| p.age));
/// ```
pub fn is_heap_by_key<T, K, F>(arr: &[T], key: F) -> bool
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::is_heap(arr, key2reversed_compare(key))
}

/// 임의의 slice를 valid max heap으로 변환합니다.
///
/// Floyd's heap construction algorithm을 사용하여 bottom-up 방식으로 heap을 구성합니다.
/// 이 과정에서 원본 slice의 원소들이 재배열되어 heap property를 만족하게 됩니다.
///
/// # 시간 복잡도
///
/// O(n) - 선형 시간에 heap을 구성합니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::max_heap::{heapify, is_heap};
///
/// let mut arr = vec![3, 1, 4, 1, 5, 9, 2];
/// heapify(&mut arr);
/// assert!(is_heap(&arr));
/// assert_eq!(arr[0], 9); // 최대값이 root에 위치
/// ```
pub fn heapify<T: Ord>(arr: &mut [T]) {
    Impl::heapify(arr, reversed_cmp);
}

/// 사용자 정의 comparator로 slice를 heap으로 변환합니다.
///
/// [`heapify`]와 동일한 기능을 하지만 `Ord::cmp` 대신 `compare` 함수를 인자로 받습니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::max_heap::{heapify_by, is_heap_by};
///
/// let mut arr = vec![5, 3, 2, 1];
/// heapify_by(&mut arr, |a, b| b.cmp(a)); // 역순 비교로 min heap 생성
/// assert!(is_heap_by(&arr, |a, b| b.cmp(a)));
/// ```
pub fn heapify_by<T, F>(arr: &mut [T], compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heapify(arr, reverse_compare(compare));
}

/// key extraction 함수로 slice를 heap으로 변환합니다.
///
/// [`heapify`]와 동일한 기능을 하지만 원소 비교 시 `key` 함수로 추출한 값을 사용합니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::max_heap::{heapify_by_key, is_heap_by_key};
///
/// #[derive(Debug)]
/// struct Person { name: String, age: u32 }
///
/// let mut people = vec![
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Bob".to_string(), age: 30 },
/// ];
/// heapify_by_key(&mut people, |p| p.age);
/// assert_eq!(people[0].age, 30); // 가장 나이 많은 사람이 root
/// ```
pub fn heapify_by_key<T, K, F>(arr: &mut [T], key: F)
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heapify(arr, key2reversed_compare(key));
}

/// 새로운 원소를 heap에 추가하고 최대 원소를 반환합니다.
///
/// 이 함수는 heap의 크기를 변경하지 않고 push와 pop을 한 번에 수행합니다.
/// 새 원소가 현재 최대값보다 크면 즉시 반환하고, 그렇지 않으면 기존 최대값을
/// 새 원소로 교체한 후 heap property를 복구합니다.
///
/// # 시간 복잡도
///
/// O(log n) - 최대 한 번의 sift-down 연산이 필요합니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::max_heap::{heapify, heap_pushpop};
///
/// let mut heap = vec![8, 7, 5, 3, 4];
/// heapify(&mut heap);
///
/// // 10을 추가 - 더 크므로 즉시 반환
/// let max_val = heap_pushpop(&mut heap, 10);
/// assert_eq!(max_val, 10);
///
/// // 1을 추가 - 기존 최대값 8이 반환됨
/// let max_val = heap_pushpop(&mut heap, 1);
/// assert_eq!(max_val, 8);
/// ```
pub fn heap_pushpop<T: Ord>(arr: &mut [T], x: T) -> T {
    Impl::heap_pushpop(arr, x, reversed_cmp)
}

/// 사용자 정의 comparator로 heap pushpop 연산을 수행합니다.
///
/// [`heap_pushpop`]과 동일한 기능을 하지만 `Ord::cmp` 대신 `compare` 함수를 인자로 받습니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::max_heap::{heapify_by, heap_pushpop_by};
///
/// let mut arr = vec![1, 2, 3];
/// heapify_by(&mut arr, |a, b| b.cmp(a)); // min heap
/// let result = heap_pushpop_by(&mut arr, 0, |a, b| b.cmp(a));
/// ```
pub fn heap_pushpop_by<T, F>(arr: &mut [T], x: T, compare: F) -> T
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heap_pushpop(arr, x, reverse_compare(compare))
}

/// key extraction 함수로 heap pushpop 연산을 수행합니다.
///
/// [`heap_pushpop`]과 동일한 기능을 하지만 원소 비교 시 `key` 함수로 추출한 값을 사용합니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::max_heap::{heapify_by_key, heap_pushpop_by_key};
///
/// #[derive(Debug, PartialEq)]
/// struct Person { name: String, age: u32 }
///
/// let mut people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
/// ];
/// heapify_by_key(&mut people, |p| p.age);
///
/// let new_person = Person { name: "Bob".to_string(), age: 35 };
/// let result = heap_pushpop_by_key(&mut people, new_person, |p| p.age);
/// assert_eq!(result.age, 35);
/// ```
pub fn heap_pushpop_by_key<T, K, F>(arr: &mut [T], x: T, key: F) -> T
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heap_pushpop(arr, x, key2reversed_compare(key))
}

/// heap에서 최대 원소를 제거하고 나머지 slice를 반환합니다.
///
/// 최대 원소(root)를 제거하고 heap property를 유지하는 나머지 원소들의 slice를 반환합니다.
/// 빈 slice의 경우 None을 반환합니다.
///
/// # 시간 복잡도
///
/// O(log n) - root 제거 후 heap property 복구가 필요합니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::max_heap::{heapify, heap_pop, is_heap};
///
/// let mut arr = vec![8, 7, 5, 3, 4];
/// heapify(&mut arr);
///
/// if let Some(remaining) = heap_pop(&mut arr) {
///     assert!(is_heap(remaining));
///     assert_eq!(remaining.len(), 4); // 원소 하나 제거됨
/// }
/// ```
pub fn heap_pop<T: Ord>(arr: &mut [T]) -> Option<&mut [T]> {
    Impl::heap_pop(arr, reversed_cmp)
}

/// 사용자 정의 comparator로 heap pop 연산을 수행합니다.
///
/// [`heap_pop`]과 동일한 기능을 하지만 `Ord::cmp` 대신 `compare` 함수를 인자로 받습니다.
pub fn heap_pop_by<T, F>(arr: &mut [T], compare: F) -> Option<&mut [T]>
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heap_pop(arr, reverse_compare(compare))
}

/// key extraction 함수로 heap pop 연산을 수행합니다.
///
/// [`heap_pop`]과 동일한 기능을 하지만 원소 비교 시 `key` 함수로 추출한 값을 사용합니다.
pub fn heap_pop_by_key<T, K, F>(arr: &mut [T], key: F) -> Option<&mut [T]>
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heap_pop(arr, key2reversed_compare(key))
}

/// slice를 in-place로 오름차순 정렬합니다.
///
/// Heapsort algorithm을 사용하여 slice를 정렬합니다. Max heap의 특성상
/// 결과는 오름차순으로 정렬됩니다.
/// 내림차순 정렬을 원하는 경우 [`min_heap::heap_reverse_sort`](crate::min_heap::heap_reverse_sort)를 사용하세요.
///
/// # 시간 복잡도
///
/// O(n log n) - 표준적인 heapsort의 시간 복잡도입니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::max_heap::heapsort;
///
/// let mut arr = vec![3, 1, 4, 1, 5];
/// heapsort(&mut arr);
/// assert_eq!(arr, vec![1, 1, 3, 4, 5]); // 오름차순 정렬
/// ```
pub fn heapsort<T: Ord>(arr: &mut [T]) {
    Impl::heap_reverse_sort(arr, reversed_cmp);
}

/// 사용자 정의 comparator로 heap sort를 수행합니다.
///
/// [`heapsort`]와 동일한 기능을 하지만 `Ord::cmp` 대신 `compare` 함수를 인자로 받습니다.
pub fn heapsort_by<T, F>(arr: &mut [T], compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heap_reverse_sort(arr, reverse_compare(compare));
}

/// key extraction 함수로 heap sort를 수행합니다.
///
/// [`heapsort`]와 동일한 기능을 하지만 원소 비교 시 `key` 함수로 추출한 값을 사용합니다.
pub fn heapsort_by_key<T, K, F>(arr: &mut [T], key: F)
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heap_reverse_sort(arr, key2reversed_compare(key));
}

/// 특정 위치의 원소에 대해 heap property를 복구합니다.
///
/// 지정된 인덱스의 원소 값이 변경되어 heap property가 깨진 경우,
/// 해당 원소를 적절한 위치로 이동시켜 heap property를 복구합니다.
///
/// # 반환값
///
/// 실제로 원소 이동이 발생한 경우 `true`, 그렇지 않으면 `false`를 반환합니다.
///
/// # 시간 복잡도
///
/// O(log n) - 최대 트리의 높이만큼 이동이 필요합니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::max_heap::{heapify, adjust_heap, is_heap};
///
/// let mut arr = vec![8, 7, 5, 3, 4];
/// heapify(&mut arr);
///
/// // Heap property를 의도적으로 깨뜨림
/// arr[0] = 1;
/// assert!(!is_heap(&arr));
///
/// // 복구
/// adjust_heap(&mut arr, 0);
/// assert!(is_heap(&arr));
/// ```
pub fn adjust_heap<T: Ord>(arr: &mut [T], idx: usize) -> bool {
    Impl::adjust_heap(arr, idx, reversed_cmp)
}

/// 사용자 정의 comparator로 heap adjustment를 수행합니다.
pub fn adjust_heap_by<T, F>(arr: &mut [T], idx: usize, compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::adjust_heap(arr, idx, reverse_compare(compare))
}

/// key extraction 함수로 heap adjustment를 수행합니다.
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
            Person {
                name: "Alice".to_string(),
                age: 35,
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
                age: 25,
            },
        ];

        // Should be a valid max heap when sorted by age
        assert!(is_heap_by_key(&people, |p| p.age));

        let invalid_people = vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
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
            age: 35,
        };
        let result = heap_pushpop_by_key(&mut people, new_person, |p| p.age);

        assert_eq!(result.age, 35); // The larger age should be returned
        assert!(is_heap_by_key(&people, |p| p.age));
        assert_eq!(people[0].age, 30); // Original max should remain at root

        // Test pushing a smaller age
        let smaller_person = Person {
            name: "David".to_string(),
            age: 20,
        };
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
                age: 35,
            },
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
                age: 28,
            },
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
