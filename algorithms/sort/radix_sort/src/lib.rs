//! # Radix Sort
//! 이 모듈에서는 Radix Sort의 구현을 제공합니다.
//! Radix Sort는 정수 또는 문자열과 같은 키를 정렬하는 데 사용되는 효율적인 알고리즘입니다.
//!
//! # 원리
//! Radix Sort는 요소의 순차적 projection에 대한 Stable Counting Sort의 반복입니다.
//! 따라서 한 타입 T에 대한 Radix Sort는 T의 projection을 나열함으로서 정의됩니다.
//!
//! 본 모듈에서 projection은 `Fn(&T) -> usize` trait을 구현하는 함수로 정의됩니다.
//! 해당 Trait을 구현하는 함수는 동일한 type을 가지지 못하므로, 서로다른 타입의 순차적 projection을 위해서는 일반적인 방법을 사용할 수 없습니다.
//! 따라서, 본 모듈에서는 2가지 방법을 제공합니다.
//!
//! ## 1. Tuple Scheme
//! Tuple Scheme은 projection을 Tuple로 나열하는 방법입니다.
//! Tuple Scheme은 최대 16개의 projection을 지원합니다. 이는 소스코드상 macro를 사용하여 구현 되어있으므로 임의의 길이로 늘릴 수 있지만, 충분할 것으로 보입니다.
//! Tuple Scheme은 매크로를 통해 함수의 연속 호출과 동일한 방식으로 projection을 수행합니다.
//! 따라서 Zero-Cost Abstraction을 보장합니다.
//!
//! ## 2. Serial Scheme
//! Serial Scheme은 projection을 Slice으로 나열하는 방법입니다.
//! Serial Scheme은 projection을 `Box<dyn Fn(&T) -> usize>`로 정의합니다. 따라서 동적 dispatch를 사용하여 projection을 수행합니다.
//! 이는 1. Tuple Scheme보다 미세한 오버헤드를 감내하는 댓가로, 런타임에 결정되는 동적 길이, 또는 동적 projection을 지원합니다.
//!
//! ## 구현 방식
//! 두 방법은 RadixScheme trait을 서로다른 방법으로 implement하여 구현됩니다.
//! RadixScheme trait은 단순한 통일된 인터페이스를 위해 `sort` 메서드를 구현하는 것으로만 정의됩니다.
//!
//! Tuple Scheme은 Tuple로 나열된 projection을 순차적으로 사용하여 Counting Sort를 하도록 RadixScheme trait을 구현합니다.
//! Serial Scheme은 `SerialScheme` struct를 사용하여 slice로 초기화된 projection을 순차적으로 사용하여 Counting Sort를 하도록 RadixScheme trait을 구현합니다.
//!
//! # 사용법
//!
//! ## 1. Tuple Scheme 사용 예시 (u32 정렬)
//!
//! `fixed_scheme!` 매크로를 사용하여 Tuple Scheme을 정의하고, 이를 `radix_sort` 메소드에 전달합니다.
//! 아래 예시는 u32 슬라이스를 4비트 단위로 LSB부터 MSD 순으로 정렬합니다.
//!
//! ```rust
//! use radix_sort::{RadixSortExt};
//!
//! let mut data = vec![0x1234_5678, 0, 0xFFFF_FFFF, 42, 17, 0xDEAD_BEEF];
//! let scheme = (
//!     |x: &u32| ((*x >> 0) & 0xF) as usize,  // LSB (가장 낮은 4비트)
//!     |x: &u32| ((*x >> 4) & 0xF) as usize,
//!     |x: &u32| ((*x >> 8) & 0xF) as usize,
//!     |x: &u32| ((*x >> 12) & 0xF) as usize,
//!     |x: &u32| ((*x >> 16) & 0xF) as usize,
//!     |x: &u32| ((*x >> 20) & 0xF) as usize,
//!     |x: &u32| ((*x >> 24) & 0xF) as usize,
//!     |x: &u32| ((*x >> 28) & 0xF) as usize, // MSB (가장 높은 4비트)
//! );
//! data.as_mut_slice().radix_sort(scheme);
//! // data는 이제 정렬된 상태입니다: [0, 17, 42, 0x12345678, 0xDEADBEEF, 0xFFFFFFFF]
//! ```
//!
//! ## 2. Serial Scheme 사용 예시 (문자열 정렬)
//!
//! `SerialScheme`을 사용하여 동적으로 projection 슬라이스를 정의하고, 이를 `radix_sort` 메소드에 전달합니다.
//! 아래 예시는 문자열 슬라이스를 사전순으로 정렬합니다. 각 문자는 ASCII 값으로 변환되어 LSB (문자열의 마지막 문자)부터 정렬됩니다.
//!
//! ```rust
//! use radix_sort::{RadixSortExt, SerialScheme, Projection};
//!
//! let mut data = vec![
//!     "apple".to_string(),
//!     "banana".to_string(),
//!     "app".to_string(),
//!     "apricot".to_string(),
//!     "bananaaa".to_string(),
//!     "".to_string(),
//! ];
//!
//! let max_len = data.iter().map(|s| s.len()).max().unwrap_or(0);
//! let mut projections: Vec<Projection<String>> = Vec::new();
//!
//! // 문자열의 뒤에서부터 (LSB) 각 문자를 projection으로 사용합니다.
//! // 짧은 문자열의 경우, 해당 위치의 문자가 없으면 0 (null 문자)으로 처리합니다.
//! for i in 0..max_len {
//!     projections.push(Box::new(move |s: &String| {
//!         let char_idx = max_len - 1 - i; // LSB부터 처리하기 위해 인덱스를 역순으로 계산
//!         s.as_bytes().get(char_idx).copied().unwrap_or(0) as usize
//!     }));
//! }
//!
//! let scheme = SerialScheme::new(&mut projections);
//! data.as_mut_slice().radix_sort(scheme);
//! // data는 이제 사전순으로 정렬된 상태입니다:
//! // ["", "app", "apple", "apricot", "banana", "bananaaa"]
//! ```
//!

use counting_sort::CountingSortByKey;

/// Counting Sort를 사용하여 Radix Sort를 구현하는 trait
/// 인터페이스를 통합하기 위해 정의된 trait입니다.
pub trait RadixScheme<T> {
    fn sort(&mut self, slice: &mut [T]);
}

/// 서로다른 type의 projection tuple에 대해 RadixScheme trait을 구현하기 위한 매크로.
/// 재귀적으로 구현됨.
/// macro expansion을 하면 다음과 같은 코드가 생성됨
/// ```ignore
/// impl<T> RadixScheme<T> for () {
///     fn sort(&self, _slice: &mut [T]) {}
/// }
///
/// impl<T, F1> RadixScheme<T> for (F1,)
/// where
///     F1: FnMut(&T) -> usize + Copy,
/// {
///     fn sort(&self, slice: &mut [T]) {
///         let (head,) = *self;
///         slice.counting_sort_by_key(head);
///         ().sort(slice);
///     }
/// }
///
/// impl<T, F1, F2> RadixScheme<T> for (F1, F2)
/// where
///     F1: FnMut(&T) -> usize + Copy,
///     F2: FnMut(&T) -> usize + Copy,
/// {
///     fn sort(&self, slice: &mut [T]) {
///         let (head1, head2) = *self;
///         slice.counting_sort_by_key(head1);
///         (head2,).sort(slice);
///     }
/// }
///
/// impl<T, F1, F2, F3> RadixScheme<T> for (F1, F2, F3)
/// where
///     F1: FnMut(&T) -> usize + Copy,
///     F2: FnMut(&T) -> usize + Copy,
///     F3: FnMut(&T) -> usize + Copy,
/// {
///     fn sort(&self, slice: &mut [T]) {
///         let (head1, head2, head3) = *self;
///         slice.counting_sort_by_key(head1);
///         (head2, head3).sort(slice);
///     }
/// }
/// /* ... */
/// ```
macro_rules! impl_tuple_scheme {
    () => {
        impl<T> RadixScheme<T> for () {
            fn sort(&mut self, _slice: &mut [T]) {}
        }
    };
    ($head:ident $(,$tail:ident)*) => {
        impl<T, $head, $($tail,)*> RadixScheme<T> for ($head, $($tail,)*)
        where
            $head: FnMut(&T) -> usize + Copy,
            ($($tail,)*) : RadixScheme<T>,
            $($tail: Copy,)*
        {
            fn sort(&mut self, slice: &mut [T]) {
                #[allow(non_snake_case)] // 타입명과 변수명이 동일해 발생하는 name convention warning을 무시합니다.
                let (head, $($tail,)*) = *self;
                slice.counting_sort_by_key(head); // 튜플상 앞의 projection이 먼저 수행됩니다.
                ($($tail,)*).sort(slice); // 재귀호출
            }
        }
        impl_tuple_scheme!($($tail),*);
    };
}

// Tuple Scheme을 구현하기 위한 매크로를 호출합니다.
// 현재 16개 projection까지 지원하지만, 필요시 늘릴 수 있습니다.
impl_tuple_scheme!(
    F16, F15, F14, F13, F12, F11, F10, F9, F8, F7, F6, F5, F4, F3, F2, F1
);

/// Radix Sort를 정의하기 위해 사용되는 projection type
/// 정적 함수, closure 및 함수객체를 모두 지원하면서 동적 dispatch를 사용하기 위해 정의됨
pub type Projection<T> = Box<dyn FnMut(&T) -> usize>;

/// Serial Scheme을 구현하기 위한 struct
/// Serial Scheme은 projection을 Slice으로 나열하는 방법을 나타냄.
pub struct SerialScheme<'a, T> {
    projs: &'a mut [Projection<T>],
}

impl<'a, T> SerialScheme<'a, T> {
    pub fn new(projs: &'a mut [Projection<T>]) -> Self {
        Self { projs }
    }
}

impl<T> RadixScheme<T> for SerialScheme<'_, T> {
    fn sort(&mut self, slice: &mut [T]) {
        for proj in self.projs.iter_mut() {
            slice.counting_sort_by_key(proj);
        }
    }
}

/// RadixScheme trait을 구현한 구조체가 슬라이스에 action을 수행하기 위한 syntactic sugar trait
/// ```ignore
/// scheme.sort(slice)
/// ```
/// 를
/// ```ignore
/// slice.radix_sort(scheme)
/// ```
/// 으로 사용할 수 있도록 함
pub trait RadixSortExt<T> {
    fn radix_sort<S: RadixScheme<T>>(self, scheme: S);
}

impl<T> RadixSortExt<T> for &'_ mut [T] {
    fn radix_sort<S: RadixScheme<T>>(self, mut scheme: S) {
        scheme.sort(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // helper to compare with standard sort for u32
    fn check_sorted_u32(data: &mut [u32]) {
        let mut expected = data.to_vec();
        expected.sort();
        assert_eq!(data, expected.as_slice());
    }

    #[test]
    fn u32_radix_1bit() {
        let mut v = vec![123, 0, 456, 1, 0xFFFF_FFFF, 999_999];
        let scheme1 = {
            (
                #[allow(clippy::identity_op)]
                |x: &u32| ((*x >> 0) & 1) as usize,
                |x: &u32| ((*x >> 1) & 1) as usize,
                |x: &u32| ((*x >> 2) & 1) as usize,
                |x: &u32| ((*x >> 3) & 1) as usize,
                |x: &u32| ((*x >> 4) & 1) as usize,
                |x: &u32| ((*x >> 5) & 1) as usize,
                |x: &u32| ((*x >> 6) & 1) as usize,
                |x: &u32| ((*x >> 7) & 1) as usize,
                |x: &u32| ((*x >> 8) & 1) as usize,
                |x: &u32| ((*x >> 9) & 1) as usize,
                |x: &u32| ((*x >> 10) & 1) as usize,
                |x: &u32| ((*x >> 11) & 1) as usize,
                |x: &u32| ((*x >> 12) & 1) as usize,
                |x: &u32| ((*x >> 13) & 1) as usize,
                |x: &u32| ((*x >> 14) & 1) as usize,
                |x: &u32| ((*x >> 15) & 1) as usize,
            )
        };
        v.as_mut_slice().radix_sort(scheme1);
        let scheme2 = {
            (
                |x: &u32| ((*x >> 16) & 1) as usize,
                |x: &u32| ((*x >> 17) & 1) as usize,
                |x: &u32| ((*x >> 18) & 1) as usize,
                |x: &u32| ((*x >> 19) & 1) as usize,
                |x: &u32| ((*x >> 20) & 1) as usize,
                |x: &u32| ((*x >> 21) & 1) as usize,
                |x: &u32| ((*x >> 22) & 1) as usize,
                |x: &u32| ((*x >> 23) & 1) as usize,
                |x: &u32| ((*x >> 24) & 1) as usize,
                |x: &u32| ((*x >> 25) & 1) as usize,
                |x: &u32| ((*x >> 26) & 1) as usize,
                |x: &u32| ((*x >> 27) & 1) as usize,
                |x: &u32| ((*x >> 28) & 1) as usize,
                |x: &u32| ((*x >> 29) & 1) as usize,
                |x: &u32| ((*x >> 30) & 1) as usize,
                |x: &u32| ((*x >> 31) & 1) as usize,
            )
        };
        v.as_mut_slice().radix_sort(scheme2);
        check_sorted_u32(&mut v);
    }

    #[test]
    fn u32_radix_2bit() {
        let mut v = vec![17, 256, 0, 65535, 1, 42, 999_999];
        let scheme = (
            #[allow(clippy::identity_op)]
            |x: &u32| ((*x >> 0) & 3) as usize,
            |x: &u32| ((*x >> 2) & 3) as usize,
            |x: &u32| ((*x >> 4) & 3) as usize,
            |x: &u32| ((*x >> 6) & 3) as usize,
            |x: &u32| ((*x >> 8) & 3) as usize,
            |x: &u32| ((*x >> 10) & 3) as usize,
            |x: &u32| ((*x >> 12) & 3) as usize,
            |x: &u32| ((*x >> 14) & 3) as usize,
            |x: &u32| ((*x >> 16) & 3) as usize,
            |x: &u32| ((*x >> 18) & 3) as usize,
            |x: &u32| ((*x >> 20) & 3) as usize,
            |x: &u32| ((*x >> 22) & 3) as usize,
            |x: &u32| ((*x >> 24) & 3) as usize,
            |x: &u32| ((*x >> 26) & 3) as usize,
            |x: &u32| ((*x >> 28) & 3) as usize,
            |x: &u32| ((*x >> 30) & 3) as usize,
        );
        v.as_mut_slice().radix_sort(scheme);
        check_sorted_u32(&mut v);
    }

    #[test]
    fn u32_radix_4bit() {
        let mut v = vec![0x1234_5678, 0, 0xFFFF_FFFF, 42, 17, 0xDEAD_BEEF];
        let scheme = (
            #[allow(clippy::identity_op)]
            |x: &u32| ((*x >> 0) & 0xF) as usize,
            |x: &u32| ((*x >> 4) & 0xF) as usize,
            |x: &u32| ((*x >> 8) & 0xF) as usize,
            |x: &u32| ((*x >> 12) & 0xF) as usize,
            |x: &u32| ((*x >> 16) & 0xF) as usize,
            |x: &u32| ((*x >> 20) & 0xF) as usize,
            |x: &u32| ((*x >> 24) & 0xF) as usize,
            |x: &u32| ((*x >> 28) & 0xF) as usize,
        );
        v.as_mut_slice().radix_sort(scheme);
        check_sorted_u32(&mut v);
    }

    #[test]
    fn u32_radix_8bit() {
        let mut v = vec![0xDEAD_BEEF, 123, 0, 0xFF00_FF00, 42, 999_999];
        let scheme = (
            #[allow(clippy::identity_op)]
            |x: &u32| ((*x >> 0) & 0xFF) as usize,
            |x: &u32| ((*x >> 8) & 0xFF) as usize,
            |x: &u32| ((*x >> 16) & 0xFF) as usize,
            |x: &u32| ((*x >> 24) & 0xFF) as usize,
        );
        v.as_mut_slice().radix_sort(scheme);
        check_sorted_u32(&mut v);
    }

    #[test]
    fn i32_sign_abs_scheme() {
        let mut v = vec![-5, 3, -2, 1, -4, 0, 7, -1];
        let scheme = (
            |x: &i32| x.unsigned_abs() as usize,
            |x: &i32| if *x < 0 { 0 } else { 1 },
        );
        v.as_mut_slice().radix_sort(scheme);

        let expected = vec![-1, -2, -4, -5, 0, 1, 3, 7];
        assert_eq!(v, expected);
    }

    #[test]
    fn ascii_string_lexicographic() {
        let mut v = vec![
            "apple".to_string(),
            "banana".to_string(),
            "app".to_string(),
            "apricot".to_string(),
            "bananaaa".to_string(),
            "".to_string(),
        ];
        let max_len = v.iter().map(|s| s.len()).max().unwrap();
        let mut projections: Vec<Projection<String>> = Vec::new();
        for i in 0..max_len {
            projections.push(Box::new(move |s: &String| {
                let idx = max_len - 1 - i; // LSB: last char
                s.as_bytes().get(idx).copied().unwrap_or(0) as usize
            }));
        }
        let scheme = SerialScheme {
            projs: &mut projections[..],
        };
        v.as_mut_slice().radix_sort(scheme);
        let expected = vec![
            "".to_string(),
            "app".to_string(),
            "apple".to_string(),
            "apricot".to_string(),
            "banana".to_string(),
            "bananaaa".to_string(),
        ];
        assert_eq!(v, expected);
    }
}
