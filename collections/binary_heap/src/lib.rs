//!
//! # Description
//! Introduction to Algorithm의 BinaryHeap을 구현한다.
//! 기존에 prelude에 Max heap이 있으므로, min heap으로 구현한다.
//!
//! ## Sub Modules
//! ### comparator
//! 원소 사이의 비교를 정의하는 객체에 대한 trait을 정의.
//! ### heap_logic
//! heapify, build_heap등 실제 이진 힙 트리를 구현 및 유지하는 핵심 logic을 구현.
//! ### min_heap
//! 실제 사용할 MinHeap과 그 method를 정의.
//!
//! ## Difference between Rust and C++ in priority_queue
//! ### C++
//! 실제 container가 아닌, container에 api를 추가한 adaptor, 내부 구현 선택 가능
//! 기본적으로 연산자 오버로딩을 통한 max heap, 다만, comparator를 전달하여 임의 순서 가능
//! comparator는 두 원소를 비교하고 bool을 반환하는 함수객체, 반환값이 true라면 swap한다
//! 이 때 swap은 move 기반의 구현이라 비용이 절약된다.
//!
//! 핵심적인 차이는 comparator의 유무로, C++는 원소 사이의 우열을 타입 내부적으로 정의된 비교연산자(Ord)에
//! 추가적으로 comparator라는 객체를 통해서 비교 가능하게 함.
//!
//! 현재 MinHeap은 C++의 comparator 기반으로 구현함.
//!
//! ### Rust
//! Rust에서는 C++와 달리, move에 기반하기에 복사 등의 비용 걱정이 없음.
//! 또 rust는 trait의 구현이 더욱 자연스럽고 구체적임. 따라서, wrapper type에 trait을 정의해서 사용하는게 일반적임
//!

mod comparator;
mod heap_logic;
mod min_heap;

pub use crate::comparator::*;
pub use crate::min_heap::*;
