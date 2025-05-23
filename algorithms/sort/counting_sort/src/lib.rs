//! # Counting Sort
//! 이 모듈에서는 Counting sort의 구현을 제공합니다.
//! Counting sort는 정수 키를 가진 배열을 정렬하는 효율적인 알고리즘입니다.
//!
//! # 구성
//! 이 모듈은 다음과 같은 trait을 제공합니다:
//! - `CountingSort`: 기본 카운팅 정렬을 수행하는 trait. u8, u16, usize 타입과 같이, Into<usize>를 구현하는 타입에 대한 정렬을 지원합니다.
//! - `TryCountingSort`: i8, i16, i32, i64, isize, u32, u64와 같이, TryInto<usize>를 구현하는 타입에 대한 정렬을 지원합니다.
//! - `CountingSortByKey`: 키를 기준으로 정렬하는 trait. 키 함수를 인자로 받아 정렬을 수행합니다. 키 값을 계산해 정렬하는 데 사용됩니다.
//! - `CountingSortByKeyUncached`: `CountingSortByKey`와 동일하지만, 각 요소에 대해 키를 두번씩 계산합니다. 따라서 키를 계산하는 비용이 키를 캐싱하는 벡터를 할당하는 비용보다 적을 때 사용할 수 있습니다.
//!
//! # Clone trait
//! `CountingSort`와 `TryCountingSort`는 clone이 cheap한 Numeric 타입을 상정하고 디자인 되어 있습니다.
//! `CountingSortByKey`와 `CountingSortByKeyUncached`는 Clone을 implement하지 않은 타입에 대해서도 사용될 수 있도록 swap을 사용하여 구현되었습니다.
//!
//! # 사용법
//! 각 trait은 단일 메서드를 제공합니다. 다음은 counting sort를 사용하는 예시입니다:
//! ```
//! use counting_sort::CountingSort;
//!
//! let mut arr = [4usize, 2, 2, 8, 3, 3, 1];
//! arr.counting_sort();
//!
//! assert_eq!(arr, [1, 2, 2, 3, 3, 4, 8]);
//! ```
//! try_counting_sort를 사용하여, usize로 변환 할 수 있는 numeric 타입에 대해 손쉽게 정렬할 수 있습니다. 음수나 시스템 usize보다 큰 숫자를 포함하는 경우, `Err`를 반환하며, 배열에 아무 변경사항을 발생시키지 않습니다.
//! ```
//! use counting_sort::TryCountingSort;
//!
//! let mut arr = [4, 2, 2, 8, 3, 3, 1];
//! assert!(arr.try_counting_sort().is_ok());
//! assert_eq!(arr, [1, 2, 2, 3, 3, 4, 8]); // sorted
//!
//! let mut arr2 = [4, 2, -2, 8, 3, -3, 1];
//! assert!(arr2.try_counting_sort().is_err());
//! assert_eq!(arr2, [4, 2, -2, 8, 3, -3, 1]); // No side effect
//! ```
//! `CountingSortByKey` trait을 사용하여, 키를 기준으로 정렬할 수 있습니다. 이 trait은 `Fn(&Self) -> usize` 타입의 함수를 인자로 받을 수 있습니다.
//! ```
//! use counting_sort::CountingSortByKey;
//!
//! let mut strings = vec![
//!     "counting".to_string(),
//!     "hello".to_string(),
//!     "sort____".to_string(),
//!     "a".to_string(),
//!     "world".to_string(),
//!     "is______".to_string(),
//!     "rust".to_string(),
//!     "bb".to_string(),
//!     "stable__".to_string(),
//!     "ccc".to_string(),
//! ];
//! let answer = vec![
//!     "a".to_string(),
//!     "bb".to_string(),
//!     "ccc".to_string(),
//!     "rust".to_string(),
//!     "hello".to_string(),
//!     "world".to_string(),
//!     "counting".to_string(),
//!     "sort____".to_string(),
//!     "is______".to_string(),
//!     "stable__".to_string(),
//! ];
//!
//! strings.counting_sort_by_key(|s| s.len());
//!
//! assert_eq!(strings, answer);
//! ```
//! `CountingSortByKeyUncached` trait을 사용하여, 키를 기준으로 정렬할 수 있습니다. 이 trait은 key를 계산하는 비용이 캐싱하는 비용보다 적을 때 사용할 수 있습니다.
//! ```
//! use counting_sort::CountingSortByKeyUncached;
//!
//! let mut strings = vec![
//!     "counting".to_string(),
//!     "hello".to_string(),
//!     "sort____".to_string(),
//!     "a".to_string(),
//!     "world".to_string(),
//!     "is______".to_string(),
//!     "rust".to_string(),
//!     "bb".to_string(),
//!     "stable__".to_string(),
//!     "ccc".to_string(),
//! ];
//! let answer = vec![
//!     "a".to_string(),
//!     "bb".to_string(),
//!     "ccc".to_string(),
//!     "rust".to_string(),
//!     "hello".to_string(),
//!     "world".to_string(),
//!     "counting".to_string(),
//!     "sort____".to_string(),
//!     "is______".to_string(),
//!     "stable__".to_string(),
//! ];
//!
//! strings.counting_sort_by_key_uncached(|s| s.len());
//!
//! assert_eq!(strings, answer);
//! ```

/// Into<usize>와 Clone을 implement하는 타입에 대해 autoimplement됩니다.
/// counting_sort는 Failure가 발생하지 않으며, 반환이 없습니다.
/// u8, u16, usize 와 같은 타입에 대해 사용됩니다.
pub trait CountingSort: Sized {
    fn counting_sort(self);
}

/// TryInto<usize>와 Clone을 implement하는 타입에 대해 autoimplement됩니다.
pub trait TryCountingSort: Sized {
    type TryIntoError;
    fn try_counting_sort(self) -> Result<(), Self::TryIntoError>;
}

pub trait CountingSortByKey: Sized {
    type IsArrayOf;
    fn counting_sort_by_key<F>(self, key_fn: F)
    where
        F: Fn(&Self::IsArrayOf) -> usize;
}

pub trait CountingSortByKeyUncached: Sized {
    type IsArrayOf;
    fn counting_sort_by_key_uncached<F>(self, key_fn: F)
    where
        F: Fn(&Self::IsArrayOf) -> usize;
}

impl<T> CountingSort for &mut [T]
where
    T: Into<usize> + Copy,
{
    fn counting_sort(self) {
        let mut counter = Vec::new();
        let mut max_key = 0;

        for item in self.iter() {
            let key = (*item).into();
            if counter.len() <= key {
                counter.resize((key + 1).next_power_of_two(), 0);
            }
            counter[key] += 1;
            max_key = max_key.max(key);
        }

        for i in 1..=max_key {
            counter[i] += counter[i - 1];
        }

        let mut perm = vec![0; self.len()];
        for (idx, item) in self.iter().enumerate().rev() {
            let key = (*item).into();
            perm[idx] = counter[key] - 1;
            counter[key] -= 1;
        }

        let cloned_self = self.to_vec();
        for idx in 0..self.len() {
            self[perm[idx]] = cloned_self[idx];
        }
    }
}

impl<T> TryCountingSort for &mut [T]
where
    T: TryInto<usize> + Copy,
{
    type TryIntoError = <T as TryInto<usize>>::Error;
    fn try_counting_sort(self) -> Result<(), Self::TryIntoError> {
        let mut counter = Vec::new();
        let mut max_key = 0;

        for item in self.iter() {
            let key = (*item).try_into()?;
            if counter.len() <= key {
                counter.resize((key + 1).next_power_of_two(), 0);
            }
            counter[key] += 1;
            max_key = max_key.max(key);
        }

        for i in 1..=max_key {
            counter[i] += counter[i - 1];
        }

        let mut perm = vec![0; self.len()];
        for (idx, item) in self.iter().enumerate().rev() {
            let key = (*item).try_into()?;
            perm[idx] = counter[key] - 1;
            counter[key] -= 1;
        }

        let cloned_self = self.to_vec();
        for idx in 0..self.len() {
            self[perm[idx]] = cloned_self[idx];
        }

        Ok(())
    }
}

impl<T> CountingSortByKey for &mut [T] {
    type IsArrayOf = T;
    fn counting_sort_by_key<F>(self, key_fn: F)
    where
        F: Fn(&T) -> usize,
    {
        let mut counter = Vec::new();
        let mut max_key = 0;
        let mut keys = Vec::with_capacity(self.len());
        for item in self.iter() {
            let key = key_fn(item);
            keys.push(key);
            if counter.len() <= key {
                counter.resize((key + 1).next_power_of_two(), 0);
            }
            counter[key] += 1;
            max_key = max_key.max(key);
        }
        for i in 1..=max_key {
            counter[i] += counter[i - 1];
        }

        let mut perm = vec![0; self.len()];

        for idx in (0..self.len()).rev() {
            let key = keys[idx];
            perm[idx] = counter[key] - 1;
            counter[key] -= 1;
        }

        for i in 0..self.len() {
            while perm[i] != i {
                let j = perm[i];
                self.swap(i, j);
                perm.swap(i, j);
            }
        }
    }
}

impl<T> CountingSortByKeyUncached for &mut [T] {
    type IsArrayOf = T;
    fn counting_sort_by_key_uncached<F>(self, key_fn: F)
    where
        F: Fn(&T) -> usize,
    {
        let mut counter = Vec::new();
        let mut max_key = 0;

        for item in self.iter() {
            let key = key_fn(item);
            if counter.len() <= key {
                counter.resize((key + 1).next_power_of_two(), 0);
            }
            counter[key] += 1;
            max_key = max_key.max(key);
        }

        for i in 1..=max_key {
            counter[i] += counter[i - 1];
        }

        let mut perm = vec![0; self.len()];
        for (idx, item) in self.iter().enumerate().rev() {
            let key = key_fn(item);
            perm[idx] = counter[key] - 1;
            counter[key] -= 1;
        }

        for i in 0..self.len() {
            while perm[i] != i {
                let j = perm[i];
                self.swap(i, j);
                perm.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut arr: [usize; 7] = [4, 2, 2, 8, 3, 3, 1];
        arr.counting_sort();
        assert_eq!(arr, [1, 2, 2, 3, 3, 4, 8]);
    }

    #[test]
    fn test_try_counting_sort() {
        let mut arr: [i32; 7] = [4, 2, 2, 8, 3, 3, 1];
        assert!(arr.try_counting_sort().is_ok());
        assert_eq!(arr, [1, 2, 2, 3, 3, 4, 8]);

        let mut arr2: [i32; 3] = [4, -2, -8];
        assert!(arr2.try_counting_sort().is_err());
        assert_eq!(arr2, [4, -2, -8]);
    }

    #[test]
    fn test_counting_sort_by_key() {
        let mut strings = vec![
            "counting".to_string(),
            "hello".to_string(),
            "sort____".to_string(),
            "a".to_string(),
            "world".to_string(),
            "is______".to_string(),
            "rust".to_string(),
            "bb".to_string(),
            "stable__".to_string(),
            "ccc".to_string(),
        ];
        let answer = vec![
            "a".to_string(),
            "bb".to_string(),
            "ccc".to_string(),
            "rust".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "counting".to_string(),
            "sort____".to_string(),
            "is______".to_string(),
            "stable__".to_string(),
        ];
        strings.counting_sort_by_key(|s| s.len());
        assert_eq!(strings, answer);
    }

    #[test]
    fn test_counting_sort_by_key_uncached() {
        let mut strings = vec![
            "counting".to_string(),
            "hello".to_string(),
            "sort____".to_string(),
            "a".to_string(),
            "world".to_string(),
            "is______".to_string(),
            "rust".to_string(),
            "bb".to_string(),
            "stable__".to_string(),
            "ccc".to_string(),
        ];
        let answer = vec![
            "a".to_string(),
            "bb".to_string(),
            "ccc".to_string(),
            "rust".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "counting".to_string(),
            "sort____".to_string(),
            "is______".to_string(),
            "stable__".to_string(),
        ];
        strings.counting_sort_by_key_uncached(|s| s.len());
        assert_eq!(strings, answer);
    }
}
