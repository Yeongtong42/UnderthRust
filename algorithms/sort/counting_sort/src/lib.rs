//! # Counting Sort
//! 이 모듈에서는 Counting sort의 구현을 제공합니다.
//! Counting sort는 정수 키를 가진 배열을 정렬하는 효율적인 알고리즘입니다.
//!
//! # 구성
//! 이 모듈은 다음과 같은 trait을 제공합니다:
//! - `CountingSort`: 기본 카운팅 정렬을 수행하는 trait. u8, u16, usize 타입과 같이, Into<usize>를 구현하는 타입에 대한 정렬을 지원합니다.
//! - `TryCountingSort`: i8, i16, i32, i64, isize, u32, u64와 같이, TryInto<usize>를 구현하는 타입에 대한 정렬을 지원합니다.
//! - `CountingSortByKey`: 키를 기준으로 정렬하는 trait. 키 함수를 인자로 받아 정렬을 수행합니다.
//! - `CountingSortByKeyCached`: `CountingSortByKey`와 동일하지만, 각 요소에 대해 키를 한번씩 계산합니다. 키 값을 계산해 캐싱한 후 정렬하는 데 사용됩니다.
//!
//! 네 trait은 모두 `&mut [T]`에 autoimplement하는 것을 목적으로 정의되었으며, 사용자가 타 타입에 별도로 implement하는 것을 상정하지 않습니다.
//!
//! # Clone trait
//! `CountingSort`와 `TryCountingSort`는 clone이 cheap한 Numeric 타입을 상정하고 디자인 되어 있습니다.
//! `CountingSortByKey`와 `CountingSortByKeyCached`는 Clone을 implement하지 않은 타입에 대해서도 사용될 수 있도록 swap을 사용하여 구현되었습니다.
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
//! `CountingSortByKey` trait을 사용하여, 키를 기준으로 정렬할 수 있습니다. 이 trait은 예시와 같이 `Fn(&Self) -> usize` 타입의 함수를 인자로 받을 수 있습니다.
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
//! `CountingSortByKeyCached` trait을 사용하여, 키를 기준으로 정렬할 수 있습니다. 이 예제에서는 문자열에서 모음의 수를 키로 사용하며, 이는 단순 길이 계산보다 비용이 더 많이 드는 키 계산을 시뮬레이션합니다.
//! ```
//! use counting_sort::CountingSortByKeyCached;
//!
//! #[derive(Debug, Clone, PartialEq, Eq)] // assert_eq를 위해 Eq 추가
//! struct Word {
//!     text: String,
//! }
//! impl Word {
//!     fn new(text: &str) -> Self { Word { text: text.to_string() } }
//!     // 단순 길이 계산보다 더 복잡한 키 계산을 시뮬레이션합니다.
//!     fn vowel_count(&self) -> usize {
//!         self.text.to_lowercase().chars().filter(|&c| "aeiou".contains(c)).count()
//!     }
//! }
//!
//! let mut items = vec![
//!     Word::new("counting"), // 모음: o, u, i => 3
//!     Word::new("hello"),    // 모음: e, o => 2
//!     Word::new("sort"),     // 모음: o => 1
//!     Word::new("a"),        // 모음: a => 1
//!     Word::new("world"),    // 모음: o => 1
//!     Word::new("is"),       // 모음: i => 1
//!     Word::new("rust"),     // 모음: u => 1
//!     Word::new("bb"),       // 모음: 0
//!     Word::new("stable"),   // 모음: a, e => 2
//!     Word::new("ccc"),      // 모음: 0
//! ];
//! let answer = vec![
//!     Word::new("bb"),
//!     Word::new("ccc"),
//!     Word::new("sort"),
//!     Word::new("a"),
//!     Word::new("world"),
//!     Word::new("is"),
//!     Word::new("rust"),
//!     Word::new("hello"),
//!     Word::new("stable"),
//!     Word::new("counting"),
//! ];
//!
//! items.counting_sort_by_key_cached(|item| item.vowel_count());
//!
//! assert_eq!(items, answer);
//! ```
//!
//! # Panics
//!
//! 각 정렬 함수는 다음과 같은 상황에서 패닉을 발생시킬 수 있습니다:
//!
//! - `counting_sort`:
//!   - 아이템을 `usize`로 변환한 키 값이 `usize::MAX`인 경우 (이로 인해 `counter` 배열 크기 계산 시 오버플로우 발생).
//!   - 특정 키 값의 개수 또는 누적 개수가 `usize::MAX`를 초과하여 `counter` 값 계산 중 오버플로우가 발생하는 경우.
//!
//! - `try_counting_sort`:
//!   - 아이템을 `usize`로 성공적으로 변환했으나 그 키 값이 `usize::MAX`인 경우.
//!   - 특정 키 값의 개수 또는 누적 개수가 `usize::MAX`를 초과하는 경우.
//!   - `TryIntoError`는 패닉 대신 `Err`로 반환됩니다.
//!
//! - `counting_sort_by_key` 및 `counting_sort_by_key_cached`:
//!   - `key_fn`이 반환하는 키 값이 `usize::MAX`인 경우.
//!   - 특정 키 값의 개수 또는 누적 개수가 `usize::MAX`를 초과하는 경우.
//!
//! 내부적으로 `counter` 배열의 크기를 계산하거나 카운트를 누적할 때 `usize` 오버플로우가 발생하면 `checked_add(...).unwrap()` 호출로 인해 패닉이 발생합니다.

/// Into<usize>와 Clone을 implement하는 Sized 타입 T에 대해 &mut [T]에 autoimplement됩니다.
/// counting_sort는 Failure가 발생하지 않으며, 반환이 없습니다.
/// u8, u16, usize 와 같은 타입에 대해 사용됩니다.
pub trait CountingSort {
    fn counting_sort(self);
}

/// TryInto<usize>와 Clone을 implement하는 Sized 타입 T에 대해 &mut [T]에 autoimplement됩니다.
/// try_counting_sort는 Failure가 발생할 수 있으며, 반환은 성공시 Ok(())를 반환하고, 실패시 Err를 반환합니다.
/// i8, i16, i32, i64, isize, u32, u64와 같은 타입에 대해 사용됩니다.
/// 정수형의 형변환의 일반적인 Error는 `std::num::TryFromIntError`임을 참고하십시오.
///
/// # Panics
/// - 아이템을 `usize`로 성공적으로 변환했으나 그 키 값이 `usize::MAX`인 경우.
/// - 특정 키 값의 개수 또는 누적 개수가 `usize::MAX`를 초과하는 경우.
///
/// # Errors
/// - 아이템을 `usize`로 변환하는 데 실패하면 `TryIntoError`를 반환합니다.
pub trait TryCountingSort {
    type TryIntoError;
    fn try_counting_sort(self) -> Result<(), Self::TryIntoError>;
}

/// CountingSortByKey trait은 Sized 타입 T에 대해서 &mut [T]에 autoimplement됩니다.
/// 이 trait은 key_fn을 인자로 받아, key_fn을 통해 계산된 키를 기준으로 정렬합니다.
/// key_fn은 Fn(&T) -> usize 타입의 함수를 인자로 받습니다.
/// 이 trait은 각 요소에 대해 키를 두번씩 계산합니다.
/// 따라서 키를 계산하는 비용이 키를 캐싱하는 비용보다 적을 때 사용할 수 있습니다.
///
/// # Panics
/// - `key_fn`이 반환하는 키 값이 `usize::MAX`인 경우.
/// - 특정 키 값의 개수 또는 누적 개수가 `usize::MAX`를 초과하는 경우.
pub trait CountingSortByKey<T> {
    fn counting_sort_by_key<F>(self, key_fn: F)
    where
        F: FnMut(&T) -> usize;
}

/// CountingSortByKeyCached trait은 Sized 타입 T에 대해서 &mut [T]에 autoimplement됩니다.
/// 이 trait은 key_fn을 인자로 받아, key_fn을 통해 계산된 키를 기준으로 정렬합니다.
/// key_fn은 Fn(&T) -> usize 타입의 함수를 인자로 받습니다.
/// 이 trait은 다른 3개의 trait과 달리, key를 한번 계산한 후, 그 값을 캐싱하여 사용합니다.
/// 따라서 key를 계산하는 비용이 캐싱하는 비용보다 클 때 사용할 수 있습니다.
///
/// # Panics
/// - `key_fn`이 반환하는 키 값이 `usize::MAX`인 경우.
/// - 특정 키 값의 개수 또는 누적 개수가 `usize::MAX`를 초과하는 경우.
pub trait CountingSortByKeyCached<T> {
    fn counting_sort_by_key_cached<F>(self, key_fn: F)
    where
        F: FnMut(&T) -> usize;
}

/// 에러 타입을 사용하지 않는 연산의 오류 채널을 위한 빈 열거형입니다.
/// Result<T, Never>는 T와 동일하며, 이는
#[derive(Debug, Clone, Copy)]
enum Never {} // 이 타입의 값은 생성될 수 없습니다.

/// 주어진 아이템 반복자로부터 키를 추출하여 카운터 배열을 생성하고,
/// 각 키의 누적 등장 횟수를 계산하여 반환합니다.
///
/// # Parameters
/// - `it`: `Result<usize, E>` 타입을 반환하는 아이템 반복자입니다. 각 `usize` 값은 정렬할 요소의 키입니다.
///
/// # Returns
/// - 성공 시: 각 인덱스가 키를 나타내고, 해당 인덱스의 값이 해당 키까지의 누적 등장 횟수인 `Vec<usize>`를 반환합니다.
/// - 실패 시: 반복자에서 발생한 첫 번째 에러 `E`를 반환합니다.
///
/// # Panics
/// - 반복자에서 추출된 키 값이 `usize::MAX`인 경우.
/// - 특정 키의 등장 횟수가 `usize::MAX`를 초과하는 경우.
/// - 누적 등장 횟수 계산 중 `usize` 오버플로우가 발생하는 경우.
fn get_accumulated_counter<E, I>(it: I) -> Result<Vec<usize>, E>
where
    I: Iterator<Item = Result<usize, E>>,
{
    let mut counter: Vec<usize> = Vec::new();
    let mut max_key = 0;

    for item in it {
        let key = item?;
        if counter.len() <= key {
            counter.resize(key.checked_add(1).unwrap().next_power_of_two(), 0);
        }
        counter[key] = counter[key].checked_add(1).unwrap();
        max_key = max_key.max(key);
    }

    for i in 1..=max_key {
        counter[i] = counter[i].checked_add(counter[i - 1]).unwrap();
    }

    Ok(counter)
}

/// 누적 카운터 배열과 아이템 반복자로부터 안정 정렬을 위한 순열(permutation) 배열을 생성합니다.
fn accumulated_counter2permutation<E, I>(
    counter: &mut [usize],
    it: I,
    len: usize,
) -> Result<Vec<usize>, E>
where
    I: DoubleEndedIterator<Item = Result<usize, E>> + ExactSizeIterator,
{
    let mut perm = vec![0; len];
    for (idx, item) in it.enumerate().rev() {
        let key = item?;
        counter[key] -= 1;
        perm[idx] = counter[key];
    }
    Ok(perm)
}

/// 주어진 순열에 따라 슬라이스의 요소들을 제자리에서 재배치합니다 (swap 사용).
/// 이 함수는 `T`가 `Copy` 트레잇을 구현하지 않은 경우에 사용됩니다.
fn apply_permutation<T>(src: &mut [T], perm: &mut [usize]) {
    for i in 0..src.len() {
        while perm[i] != i {
            let j = perm[i];
            src.swap(i, j);
            perm.swap(i, j);
        }
    }
}

/// 주어진 순열에 따라 슬라이스의 요소들을 재배치합니다 (복사 사용).
/// 이 함수는 `T`가 `Copy` 트레잇을 구현한 경우에 사용됩니다.
/// 원본 슬라이스의 복사본을 만들어 정렬된 위치에 요소들을 배치합니다.
fn apply_permutation_copy<T>(src: &mut [T], perm: &[usize])
where
    T: Copy,
{
    let cloned_src = src.to_vec();
    for idx in 0..src.len() {
        src[perm[idx]] = cloned_src[idx];
    }
}

impl<T> CountingSort for &mut [T]
where
    T: Into<usize> + Copy,
{
    fn counting_sort(self) {
        if self.len() <= 1 {
            return;
        }

        let mut counter = {
            let it = self
                .iter()
                .cloned()
                .map(Into::<usize>::into)
                .map(Result::<usize, Never>::Ok);
            get_accumulated_counter(it).unwrap()
        };

        let perm = {
            let it = self
                .iter()
                .cloned()
                .map(Into::<usize>::into)
                .map(Result::<usize, Never>::Ok);
            accumulated_counter2permutation(&mut counter, it, self.len()).unwrap()
        };

        apply_permutation_copy(self, &perm);
    }
}

impl<T> TryCountingSort for &mut [T]
where
    T: TryInto<usize> + Copy,
{
    type TryIntoError = <T as TryInto<usize>>::Error;
    fn try_counting_sort(self) -> Result<(), Self::TryIntoError> {
        if self.len() <= 1 {
            return Ok(());
        }
        let mut counter = {
            let it = self.iter().cloned().map(|item| item.try_into());
            get_accumulated_counter(it)?
        };

        let perm = {
            let it = self.iter().cloned().map(|item| item.try_into());
            accumulated_counter2permutation(&mut counter, it, self.len())?
        };

        apply_permutation_copy(self, &perm);
        Ok(())
    }
}

impl<T> CountingSortByKey<T> for &mut [T] {
    fn counting_sort_by_key<F>(self, mut key_fn: F)
    where
        F: FnMut(&T) -> usize,
    {
        if self.len() <= 1 {
            return;
        }
        let mut counter: Vec<usize> = {
            let maybe_counter: Result<Vec<usize>, Never> =
                get_accumulated_counter(self.iter().map(|item| Ok(key_fn(item))));
            maybe_counter.unwrap()
        };

        let mut perm = {
            let it = self.iter().map(key_fn).map(Result::<usize, Never>::Ok);
            accumulated_counter2permutation(&mut counter, it, self.len()).unwrap()
        };

        apply_permutation(self, &mut perm);
    }
}

impl<T> CountingSortByKeyCached<T> for &mut [T] {
    fn counting_sort_by_key_cached<F>(self, key_fn: F)
    where
        F: FnMut(&T) -> usize,
    {
        if self.len() <= 1 {
            return;
        }

        let keys: Vec<usize> = self.iter().map(key_fn).collect();
        let mut counter = {
            let it = keys.iter().cloned().map(Result::<usize, Never>::Ok);
            get_accumulated_counter(it).unwrap()
        };

        let mut perm = {
            let it = keys.iter().cloned().map(Result::<usize, Never>::Ok);
            accumulated_counter2permutation(&mut counter, it, self.len()).unwrap()
        };

        apply_permutation(self, &mut perm);
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

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Word {
        text: String,
    }
    impl Word {
        fn new(text: &str) -> Self {
            Word {
                text: text.to_string(),
            }
        }
        fn vowel_count(&self) -> usize {
            self.text
                .to_lowercase()
                .chars()
                .filter(|&c| "aeiou".contains(c))
                .count()
        }
    }

    #[test]
    fn test_counting_sort_by_key_cached() {
        let mut items = vec![
            Word::new("counting"),
            Word::new("hello"),
            Word::new("sort"),
            Word::new("a"),
            Word::new("world"),
            Word::new("is"),
            Word::new("rust"),
            Word::new("bb"),
            Word::new("stable"),
            Word::new("ccc"),
        ];
        let answer = vec![
            Word::new("bb"),
            Word::new("ccc"),
            Word::new("sort"),
            Word::new("a"),
            Word::new("world"),
            Word::new("is"),
            Word::new("rust"),
            Word::new("hello"),
            Word::new("stable"),
            Word::new("counting"),
        ];
        items.counting_sort_by_key_cached(|item| item.vowel_count());
        assert_eq!(items, answer);
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

    // 빈 배열 테스트
    #[test]
    fn test_empty_array_counting_sort() {
        let mut arr: [usize; 0] = [];
        arr.counting_sort();
        assert_eq!(arr, <[usize; 0]>::default());
    }

    #[test]
    fn test_empty_array_try_counting_sort() {
        let mut arr: [i32; 0] = [];
        assert!(arr.try_counting_sort().is_ok());
        assert_eq!(arr, <[i32; 0]>::default());
    }

    #[test]
    fn test_empty_array_counting_sort_by_key() {
        let mut arr: [String; 0] = [];
        arr.counting_sort_by_key(|s| s.len());
        assert_eq!(arr, <[String; 0]>::default()); // 빈 벡터는 그대로 유지
    }

    #[test]
    fn test_empty_array_counting_sort_by_key_cached() {
        let mut arr: [String; 0] = [];
        arr.counting_sort_by_key_cached(|s| s.len());
        assert_eq!(arr, <[String; 0]>::default()); // 빈 벡터는 그대로 유지
    }

    // 모든 요소가 동일한 경우 테스트 (안정성 간접 확인)
    #[test]
    fn test_all_elements_same_key_counting_sort() {
        let mut arr = [1usize, 1, 1, 1];
        arr.counting_sort();
        assert_eq!(arr, [1, 1, 1, 1]);
    }

    #[test]
    fn test_all_elements_same_key_try_counting_sort() {
        let mut arr = [5i32, 5, 5];
        assert!(arr.try_counting_sort().is_ok());
        assert_eq!(arr, [5, 5, 5]);
    }

    #[test]
    fn test_all_elements_same_key_counting_sort_by_key() {
        // 안정성: 모든 키가 같을 때 원래 순서가 유지되어야 함
        let mut arr = vec![
            Word::new("apple"),
            Word::new("apricot"),
            Word::new("banana"),
        ];
        let expected = arr.clone(); // 원래 순서
        arr.counting_sort_by_key(|_w| 0); // 모든 키를 0으로 설정
        assert_eq!(
            arr, expected,
            "Order should be preserved for same keys (stability)"
        );
    }

    #[test]
    fn test_all_elements_same_key_counting_sort_by_key_cached() {
        let mut arr = vec![Word::new("zebra"), Word::new("yak"), Word::new("xylophone")];
        let expected = arr.clone();
        arr.counting_sort_by_key_cached(|_w| 10); // 모든 키를 10으로 설정
        assert_eq!(
            arr, expected,
            "Order should be preserved for same keys (stability)"
        );
    }

    // 이미 정렬된 배열 테스트
    #[test]
    fn test_already_sorted_counting_sort() {
        let mut arr = [1usize, 2, 3, 4];
        arr.counting_sort();
        assert_eq!(arr, [1, 2, 3, 4]);
    }

    #[test]
    fn test_already_sorted_try_counting_sort() {
        let mut arr = [10i32, 20, 30];
        assert!(arr.try_counting_sort().is_ok());
        assert_eq!(arr, [10, 20, 30]);
    }

    #[test]
    fn test_already_sorted_counting_sort_by_key() {
        let mut arr = vec!["a".to_string(), "bb".to_string(), "ccc".to_string()];
        let expected = arr.clone();
        arr.counting_sort_by_key(|s| s.len());
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_already_sorted_counting_sort_by_key_cached() {
        let mut arr = vec![Word::new("a"), Word::new("b"), Word::new("c")];
        let expected = arr.clone();
        arr.counting_sort_by_key_cached(|w| w.text.chars().next().unwrap() as usize); // 첫 글자 ASCII 값
        assert_eq!(arr, expected);
    }

    // 역순으로 정렬된 배열 테스트
    #[test]
    fn test_reverse_sorted_counting_sort() {
        let mut arr = [4usize, 3, 2, 1];
        arr.counting_sort();
        assert_eq!(arr, [1, 2, 3, 4]);
    }

    #[test]
    fn test_reverse_sorted_try_counting_sort() {
        let mut arr = [30i32, 20, 10];
        assert!(arr.try_counting_sort().is_ok());
        assert_eq!(arr, [10, 20, 30]);
    }

    #[test]
    fn test_reverse_sorted_counting_sort_by_key() {
        let mut arr = vec!["ccc".to_string(), "bb".to_string(), "a".to_string()];
        arr.counting_sort_by_key(|s| s.len());
        assert_eq!(
            arr,
            vec!["a".to_string(), "bb".to_string(), "ccc".to_string()]
        );
    }

    #[test]
    fn test_reverse_sorted_counting_sort_by_key_cached() {
        let mut arr = vec![Word::new("c"), Word::new("b"), Word::new("a")];
        arr.counting_sort_by_key_cached(|w| w.text.chars().next().unwrap() as usize);
        assert_eq!(arr, vec![Word::new("a"), Word::new("b"), Word::new("c")]);
    }

    // usize::MAX 키 값 패닉 테스트
    #[test]
    #[should_panic]
    fn test_counting_sort_with_usize_max_panics() {
        let mut arr = [0usize, usize::MAX];
        arr.counting_sort();
    }

    #[test]
    #[should_panic]
    fn test_try_counting_sort_with_usize_max_panics() {
        // T가 usize일 때, try_into()는 성공하고 그 후 key == usize::MAX 체크에서 패닉해야 함
        let mut arr_usize: [usize; 2] = [0, usize::MAX];
        // unwrap_or_else로 try_into 자체의 에러가 아님을 명확히 함
        arr_usize
            .try_counting_sort()
            .unwrap_or_else(|e| panic!("try_into failed unexpectedly for usize input: {:?}", e));
    }

    #[test]
    #[should_panic]
    fn test_counting_sort_by_key_with_usize_max_key_panics() {
        let mut arr = [0, 1]; // i32 타입 사용
        arr.counting_sort_by_key(|x: &i32| if *x == 1 { usize::MAX } else { 0 });
    }

    #[test]
    #[should_panic]
    fn test_counting_sort_by_key_cached_with_usize_max_key_panics() {
        let mut arr = [0, 1]; // i32 타입 사용
        arr.counting_sort_by_key_cached(|x: &i32| if *x == 1 { usize::MAX } else { 0 });
    }

    // TryInto<usize> 변환 실패 테스트 (TryFromIntError)
    #[test]
    fn test_try_counting_sort_conversion_failure_u64_if_usize_is_smaller() {
        // 이 테스트는 usize가 u64보다 작을 때 (예: 32비트 대상) 의미가 있음
        if usize::BITS < 64 {
            let large_val_beyond_usize_max = (usize::MAX as u64) + 1;
            let mut arr: [u64; 2] = [0, large_val_beyond_usize_max];
            assert!(
                arr.try_counting_sort().is_err(),
                "Should fail to convert large_val_beyond_usize_max to usize"
            );
            assert_eq!(
                arr,
                [0, large_val_beyond_usize_max],
                "Array should be unchanged on conversion error"
            );
        } else {
            // usize가 64비트인 경우, u64::MAX를 usize로 변환 시도 (usize::MAX와 같다면 성공, 아니면 실패)
            // 여기서는 usize::MAX + 1 개념이 u64 범위 내에서만 의미 있으므로,
            // usize가 64비트일 때는 이 특정 테스트 케이스는 스킵하거나 다르게 접근해야 함.
            // 지금은 usize < u64 경우만 명확히 테스트.
            println!(
                "Skipping test_try_counting_sort_conversion_failure_u64_if_usize_is_smaller as usize is likely 64-bit."
            );
        }
    }
    #[test]
    fn test_try_counting_sort_negative_value_fails_conversion() {
        let mut arr_i8: [i8; 2] = [5, -1];
        assert!(
            arr_i8.try_counting_sort().is_err(),
            "Should fail for negative i8 value"
        );
        assert_eq!(
            arr_i8,
            [5, -1],
            "Array should be unchanged on negative i8 error"
        );

        let mut arr_i64: [i64; 2] = [100, -100];
        assert!(
            arr_i64.try_counting_sort().is_err(),
            "Should fail for negative i64 value"
        );
        assert_eq!(
            arr_i64,
            [100, -100],
            "Array should be unchanged on negative i64 error"
        );
    }
}
