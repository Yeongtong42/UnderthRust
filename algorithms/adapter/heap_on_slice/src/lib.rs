//! # heap_on_slice
//!
//! `&mut [T]` slice에 대한 in-place binary heap 연산을 제공하는 라이브러리입니다.
//!
//! 이 crate는 별도의 heap 자료구조를 생성하지 않고, 기존 slice의 메모리 영역을
//! 그대로 활용하여 heap 연산을 수행합니다. 표준 라이브러리의 [`BinaryHeap`]과
//! 달리 메모리 할당을 하지 않으며, slice의 고정된 크기 내에서만 동작합니다.
//!
//! [`BinaryHeap`]: std::collections::BinaryHeap
//!
//! ## 모듈 구성
//!
//! - [`min_heap`]: minimum heap 연산을 제공합니다. 가장 작은 원소가 root에 위치하며, [`heap_reverse_sort`](min_heap::heap_reverse_sort)로 내림차순 정렬을 지원합니다.
//! - [`max_heap`]: maximum heap 연산을 제공합니다. 가장 큰 원소가 root에 위치하며, [`heapsort`](max_heap::heapsort)로 오름차순 정렬을 지원합니다.
//!
//! ## 사용법
//!
//! 네임스페이스 충돌을 방지하기 위해 `min_heap` 또는 `max_heap` 중 하나만 사용하는 것을 권장합니다:
//!
//! ```rust
//! use heap_on_slice::min_heap::*;
//!
//! let mut arr = vec![3, 1, 4, 1, 5];
//! heapify(&mut arr);
//! assert!(is_heap(&arr));
//! ```
//!
//! 두 모듈을 모두 사용해야 하는 경우, 명시적으로 모듈 이름을 지정하세요:
//!
//! ```rust
//! use heap_on_slice::{min_heap, max_heap};
//!
//! let mut min_data = vec![3, 1, 4, 1, 5];
//! min_heap::heapify(&mut min_data);
//! assert!(min_heap::is_heap(&min_data));
//!
//! let mut max_data = vec![3, 1, 4, 1, 5];
//! max_heap::heapify(&mut max_data);
//! assert!(max_heap::is_heap(&max_data));
//! ```
//!
//! ## 주요 특징
//!
//! - **Zero-allocation**: 추가 메모리 할당 없이 기존 slice에서 동작
//! - **In-place 연산**: 원본 데이터를 직접 수정하여 공간 효율성 극대화
//! - **Custom comparator 지원**: 사용자 정의 비교 함수 및 key extraction 함수 지원
//! - **Type-safe**: 컴파일 타임에 타입 안전성 보장

mod heap_implementation;

pub mod max_heap;
pub mod min_heap;
