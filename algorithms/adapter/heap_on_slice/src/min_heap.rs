//! # Min Heap 연산
//!
//! `&mut [T]` slice에 대한 minimum binary heap 연산을 제공합니다.
//!
//! Min heap은 부모 노드의 값이 항상 자식 노드의 값보다 작거나 같은 complete binary tree입니다.
//! 이 구현에서는 가장 작은 원소가 root(index 0)에 위치하며, heap property를 유지합니다.
//!
//! ## 주요 함수
//!
//! ### Heap 검증
//! - [`is_heap`]: slice가 valid min heap인지 확인
//! - [`is_heap_by`]: 사용자 정의 comparator로 heap 검증
//! - [`is_heap_by_key`]: key extraction 함수로 heap 검증
//!
//! ### Heap 구성
//! - [`heapify`]: 임의의 slice를 valid min heap으로 변환
//! - [`heapify_by`], [`heapify_by_key`]: 사용자 정의 비교 기준으로 heapify
//!
//! ### Priority Queue 연산
//! - [`heap_pushpop`]: 새 원소 추가 후 최소값 제거
//! - [`heap_pop`]: 최소 원소 제거
//! - [`adjust_heap`]: 특정 위치의 heap property 복구
//!
//! ### 정렬
//! - [`heap_reverse_sort`]: in-place 내림차순 정렬 (오름차순 정렬은 [`max_heap::heapsort`](crate::max_heap::heapsort) 사용)
//!
//! ## 사용 예시
//!
//! ### 기본 사용법
//!
//! ```rust
//! use heap_on_slice::min_heap::*;
//!
//! let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
//!
//! // Slice를 min heap으로 변환
//! heapify(&mut arr);
//! assert!(is_heap(&arr));
//! assert_eq!(arr[0], 1); // 최소값이 root에 위치
//!
//! // Priority queue로 사용
//! let min_val = heap_pushpop(&mut arr, 0); // 0을 추가하고 최소값 반환
//! assert_eq!(min_val, 0);
//!
//! // 최소 원소 제거
//! if let Some(remaining) = heap_pop(&mut arr) {
//!     assert!(is_heap(remaining));
//! }
//!
//! // 정렬
//! heap_reverse_sort(&mut arr);
//! // arr는 이제 내림차순으로 정렬됨
//! ```
//!
//! ### Custom comparator 사용
//!
//! ```rust
//! use heap_on_slice::min_heap::*;
//! use std::cmp::Ordering;
//!
//! let mut arr = vec![1, 3, 2, 5, 4];
//!
//! // 역순 비교로 max heap 동작 구현
//! heapify_by(&mut arr, |a, b| b.cmp(a));
//! assert!(is_heap_by(&arr, |a, b| b.cmp(a)));
//! ```
//!
//! ### Key-based 연산
//!
//! ```rust
//! use heap_on_slice::min_heap::*;
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
//! // 나이 기준으로 min heap 구성
//! heapify_by_key(&mut people, |p| p.age);
//! assert!(is_heap_by_key(&people, |p| p.age));
//! assert_eq!(people[0].age, 25); // 가장 어린 사람이 root
//! ```

use super::heap_implementation as Impl;
use std::cmp::Ordering;

fn key2compare<T, K, F>(mut key: F) -> impl FnMut(&T, &T) -> Ordering
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    move |a: &T, b: &T| Ord::cmp(&key(a), &key(b))
}

/// slice가 valid min heap인지 확인합니다.
///
/// Min heap property를 만족하는지 검사합니다: 모든 부모 노드는 자식 노드보다 작거나 같아야 합니다.
/// 빈 slice와 단일 원소 slice는 항상 valid heap으로 간주됩니다.
///
/// # 시간 복잡도
///
/// O(n) - 모든 노드를 한 번씩 검사합니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::min_heap::is_heap;
///
/// let valid_heap = vec![1, 3, 2, 7, 5, 4];
/// assert!(is_heap(&valid_heap));
///
/// let invalid_heap = vec![5, 3, 2]; // 5 > 3이므로 invalid
/// assert!(!is_heap(&invalid_heap));
/// ```
pub fn is_heap<T: Ord>(arr: &[T]) -> bool {
    Impl::is_heap(arr, Ord::cmp)
}

/// 사용자 정의 comparator로 slice가 valid heap인지 확인합니다.
///
/// [`is_heap`]과 동일한 기능을 하지만 `Ord::cmp` 대신 `compare` 함수를 인자로 받습니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::min_heap::is_heap_by;
///
/// let arr = vec![5, 3, 4, 1, 2];
/// // 역순 비교로 max heap 검증
/// assert!(is_heap_by(&arr, |a, b| b.cmp(a)));
/// ```
pub fn is_heap_by<T, F>(arr: &[T], compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::is_heap(arr, compare)
}

/// key extraction 함수로 slice가 valid heap인지 확인합니다.
///
/// [`is_heap`]과 동일한 기능을 하지만 원소 비교 시 `key` 함수로 추출한 값을 사용합니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::min_heap::is_heap_by_key;
///
/// #[derive(Debug)]
/// struct Person { age: u32 }
///
/// let people = vec![
///     Person { age: 25 },
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
    Impl::is_heap(arr, key2compare(key))
}

/// 임의의 slice를 valid min heap으로 변환합니다.
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
/// use heap_on_slice::min_heap::{heapify, is_heap};
///
/// let mut arr = vec![3, 1, 4, 1, 5, 9, 2];
/// heapify(&mut arr);
/// assert!(is_heap(&arr));
/// assert_eq!(arr[0], 1); // 최소값이 root에 위치
/// ```
pub fn heapify<T: Ord>(arr: &mut [T]) {
    Impl::heapify(arr, Ord::cmp);
}

/// 사용자 정의 comparator로 slice를 heap으로 변환합니다.
///
/// [`heapify`]와 동일한 기능을 하지만 `Ord::cmp` 대신 `compare` 함수를 인자로 받습니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::min_heap::{heapify_by, is_heap_by};
///
/// let mut arr = vec![1, 3, 2, 5];
/// heapify_by(&mut arr, |a, b| b.cmp(a)); // 역순 비교로 max heap 생성
/// assert!(is_heap_by(&arr, |a, b| b.cmp(a)));
/// ```
pub fn heapify_by<T, F>(arr: &mut [T], compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heapify(arr, compare);
}

/// key extraction 함수로 slice를 heap으로 변환합니다.
///
/// [`heapify`]와 동일한 기능을 하지만 원소 비교 시 `key` 함수로 추출한 값을 사용합니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::min_heap::{heapify_by_key, is_heap_by_key};
///
/// #[derive(Debug)]
/// struct Person { name: String, age: u32 }
///
/// let mut people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
/// ];
/// heapify_by_key(&mut people, |p| p.age);
/// assert_eq!(people[0].age, 25); // 가장 어린 사람이 root
/// ```
pub fn heapify_by_key<T, K, F>(arr: &mut [T], key: F)
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heapify(arr, key2compare(key));
}

/// 새로운 원소를 heap에 추가하고 최소 원소를 반환합니다.
///
/// 이 함수는 heap의 크기를 변경하지 않고 push와 pop을 한 번에 수행합니다.
/// 새 원소가 현재 최소값보다 작으면 즉시 반환하고, 그렇지 않으면 기존 최소값을
/// 새 원소로 교체한 후 heap property를 복구합니다.
///
/// # 시간 복잡도
///
/// O(log n) - 최대 한 번의 sift-down 연산이 필요합니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::min_heap::{heapify, heap_pushpop};
///
/// let mut heap = vec![1, 3, 2, 7, 5];
/// heapify(&mut heap);
///
/// // 0을 추가 - 더 작으므로 즉시 반환
/// let min_val = heap_pushpop(&mut heap, 0);
/// assert_eq!(min_val, 0);
///
/// // 10을 추가 - 기존 최소값 1이 반환됨
/// let min_val = heap_pushpop(&mut heap, 10);
/// assert_eq!(min_val, 1);
/// ```
pub fn heap_pushpop<T: Ord>(arr: &mut [T], x: T) -> T {
    Impl::heap_pushpop(arr, x, Ord::cmp)
}

/// 사용자 정의 comparator로 heap pushpop 연산을 수행합니다.
///
/// [`heap_pushpop`]과 동일한 기능을 하지만 `Ord::cmp` 대신 `compare` 함수를 인자로 받습니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::min_heap::{heapify_by, heap_pushpop_by};
///
/// let mut arr = vec![5, 3, 4];
/// heapify_by(&mut arr, |a, b| b.cmp(a)); // max heap
/// let result = heap_pushpop_by(&mut arr, 6, |a, b| b.cmp(a));
/// ```
pub fn heap_pushpop_by<T, F>(arr: &mut [T], x: T, compare: F) -> T
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heap_pushpop(arr, x, compare)
}

/// key extraction 함수로 heap pushpop 연산을 수행합니다.
///
/// [`heap_pushpop`]과 동일한 기능을 하지만 원소 비교 시 `key` 함수로 추출한 값을 사용합니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::min_heap::{heapify_by_key, heap_pushpop_by_key};
///
/// #[derive(Debug, PartialEq)]
/// struct Person { name: String, age: u32 }
///
/// let mut people = vec![
///     Person { name: "Alice".to_string(), age: 25 },
/// ];
/// heapify_by_key(&mut people, |p| p.age);
///
/// let new_person = Person { name: "Bob".to_string(), age: 20 };
/// let oldest = heap_pushpop_by_key(&mut people, new_person, |p| p.age);
/// assert_eq!(oldest.age, 20);
/// ```
pub fn heap_pushpop_by_key<T, K, F>(arr: &mut [T], x: T, key: F) -> T
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heap_pushpop(arr, x, key2compare(key))
}

/// heap에서 최소 원소를 제거하고 나머지 slice를 반환합니다.
///
/// 최소 원소(root)를 제거하고 heap property를 유지하는 나머지 원소들의 slice를 반환합니다.
/// 빈 slice의 경우 None을 반환합니다.
///
/// # 시간 복잡도
///
/// O(log n) - root 제거 후 heap property 복구가 필요합니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::min_heap::{heapify, heap_pop, is_heap};
///
/// let mut arr = vec![1, 3, 2, 7, 5];
/// heapify(&mut arr);
///
/// if let Some(remaining) = heap_pop(&mut arr) {
///     assert!(is_heap(remaining));
///     assert_eq!(remaining.len(), 4); // 원소 하나 제거됨
/// }
/// ```
pub fn heap_pop<T: Ord>(arr: &mut [T]) -> Option<&mut [T]> {
    Impl::heap_pop(arr, Ord::cmp)
}

/// 사용자 정의 comparator로 heap pop 연산을 수행합니다.
///
/// [`heap_pop`]과 동일한 기능을 하지만 `Ord::cmp` 대신 `compare` 함수를 인자로 받습니다.
pub fn heap_pop_by<T, F>(arr: &mut [T], compare: F) -> Option<&mut [T]>
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heap_pop(arr, compare)
}

/// key extraction 함수로 heap pop 연산을 수행합니다.
///
/// [`heap_pop`]과 동일한 기능을 하지만 원소 비교 시 `key` 함수로 추출한 값을 사용합니다.
pub fn heap_pop_by_key<T, K, F>(arr: &mut [T], key: F) -> Option<&mut [T]>
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heap_pop(arr, key2compare(key))
}

/// slice를 in-place로 내림차순 정렬합니다.
///
/// Heapsort algorithm을 사용하여 slice를 정렬합니다. Min heap의 특성상
/// 결과는 내림차순으로 정렬됩니다.
/// 오름차순 정렬을 원하는 경우 [`max_heap::heapsort`](crate::max_heap::heapsort)를 사용하세요.
///
/// # 시간 복잡도
///
/// O(n log n) - 표준적인 heapsort의 시간 복잡도입니다.
///
/// # Examples
///
/// ```rust
/// use heap_on_slice::min_heap::heap_reverse_sort;
///
/// let mut arr = vec![3, 1, 4, 1, 5];
/// heap_reverse_sort(&mut arr);
/// assert_eq!(arr, vec![5, 4, 3, 1, 1]); // 내림차순 정렬
/// ```
pub fn heap_reverse_sort<T: Ord>(arr: &mut [T]) {
    Impl::heap_reverse_sort(arr, Ord::cmp);
}

/// 사용자 정의 comparator로 heap sort를 수행합니다.
///
/// [`heap_reverse_sort`]와 동일한 기능을 하지만 `Ord::cmp` 대신 `compare` 함수를 인자로 받습니다.
pub fn heap_reverse_sort_by<T, F>(arr: &mut [T], compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heap_reverse_sort(arr, compare);
}

/// key extraction 함수로 heap sort를 수행합니다.
///
/// [`heap_reverse_sort`]와 동일한 기능을 하지만 원소 비교 시 `key` 함수로 추출한 값을 사용합니다.
pub fn heap_reverse_sort_by_key<T, K, F>(arr: &mut [T], key: F)
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heap_reverse_sort(arr, key2compare(key));
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
/// use heap_on_slice::min_heap::{heapify, adjust_heap, is_heap};
///
/// let mut arr = vec![1, 3, 2, 7, 5];
/// heapify(&mut arr);
///
/// // Heap property를 의도적으로 깨뜨림
/// arr[0] = 10;
/// assert!(!is_heap(&arr));
///
/// // 복구
/// adjust_heap(&mut arr, 0);
/// assert!(is_heap(&arr));
/// ```
pub fn adjust_heap<T: Ord>(arr: &mut [T], idx: usize) -> bool {
    Impl::adjust_heap(arr, idx, Ord::cmp)
}

/// 사용자 정의 comparator로 heap adjustment를 수행합니다.
///
/// [`adjust_heap`]과 동일한 기능을 하지만 `Ord::cmp` 대신 `compare` 함수를 인자로 받습니다.
pub fn adjust_heap_by<T, F>(arr: &mut [T], idx: usize, compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::adjust_heap(arr, idx, compare)
}

/// key extraction 함수로 heap adjustment를 수행합니다.
///
/// [`adjust_heap`]과 동일한 기능을 하지만 원소 비교 시 `key` 함수로 추출한 값을 사용합니다.
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
