//! # Heap on Slice
//! `slice`에 이진 힙기능을 제공하는 모듈.
//! BinaryHeap과 같은 컨테이너를 제공하는 것이 아닌, slice에 binary heap operation을 제공하는 모듈이다.
//! 따라서 이 모듈은 다음과 같은 시나리오에 적합하다.
//! * BinaryHeap과 같은 컨테이너에 데이터를 복사하지 않고, slice에 직접 이진힙 기능을 사용해 최적화 이득을 보고자 하는 경우
//! * 커스텀 기능이 있는 이진힙을 구현하고자 하는 경우
//!
//! # heap_on_slice module features
//! 이진힙은 이진트리의 일종으로, 각 노드가 자식노드보다 작거나(MinHeap) 같은 값을 가지는 트리이다.
//! 이진힙은 완전이진트리이므로, 배열로 구현할 수 있다.
//! `heap_on_slice` 모듈은 다음과 같은 기능을 제공한다.
//! * `Comparator` trait implementation을 통해 비교함수 선택가능. 이를 통해 동일한 타입에 대해서도 서로다른 비교함수를 사용하여 이진힙을 구성할 수 있다.
//! * `DefaultComparator`와 `ReverseComparator`를 제공하여, Ord trait을 구현한 타입에 대해 기본적으로 제공되는 Comparator implementation을 제공한다.
//! * Instance dependent comparator에 대한 확장성이 제공된다, 예를 들어,
//! ```
//! use heap_on_slice::{*, min_heap::MinHeap};
//! struct AbstractDistance {
//!     center: i32,
//! }
//! impl Comparator<i32> for AbstractDistance {
//!     fn cmp(&self, a: &i32, b: &i32) -> std::cmp::Ordering {
//!        Ord::cmp(&a.abs_diff(self.center), &b.abs_diff(self.center))
//!     }
//! }
//! let comp = AbstractDistance { center: 3 };
//! let mut slice = vec![1, 2, 3, 4, 5];
//! comp.heapify(&mut slice);
//! ```
//! 와 같이 인스턴스의 값에 의존하는 비교함수를 구현할 수 있으며, enum 또는 struct를 통해 다양한 비교함수를 구현할 수 있다.
//! * 비교함수에 대한 `MinHeap`과 `MaxHeap`을 모두 지원. `MinHeap`은 작은 값이 루트에 위치하고, `MaxHeap`은 큰 값이 루트에 위치한다.
//! * slice에 우선순위 큐 기능을 제공하는 메서드 제공. 이를 통해 우선순위 큐를 쉽게 구현할 수 있다.
//! * slice에 heap property를 보존할 수 있는 action 메서드 제공. 이를 통해 우선순위 큐 구현시 increase/decrease key를 쉽게 구현할 수 있다.
//! * slice에 in-place로 정렬하는 메서드 제공.
//! * 아주 적은 trait bound를 요구함. slice의 원소는 Copy, Clone, Default, Ord등을 implement하지 않아도 사용가능. Comparator object만 정의하면 즉시 사용 가능함.
//!
//! # Trait Namespace Collision
//! `MaxHeap` trait과 `MinHeap` trait은 같은 이름의 메서드를 제공하므로, 두 trait을 모두 use하는 경우 method를 구분하기 위해 명시적으로 trait을 지정해야 하는 불편함이 발생할 수 있음.
//! 예를 들어, `MinHeap` trait을 사용하고자 하는 경우,
//! ```
//! use heap_on_slice::{*, min_heap::MinHeap};
//! let comp = ReverseComparator;
//! let mut slice = vec![1, 2, 3, 4, 5];
//! comp.heapify(&mut slice);
//! ```
//! `MaxHeap` trait을 사용하고자 하는 경우,
//! ```
//! use heap_on_slice::{*, max_heap::MaxHeap};
//! let comp = DefaultComparator;
//! let mut slice = vec![1, 2, 3, 4, 5];
//! comp.heapify(&mut slice);
//! ```
//! 와 같이 둘 중 하나를 선택해야 하며, 두 trait을 모두 사용하고자 하는 경우,
//! ```
//! use heap_on_slice::{*, min_heap::MinHeap, max_heap::MaxHeap};
//! let comp = DefaultComparator;
//! let mut slice = vec![1, 2, 3, 4, 5];
//! // comp.heapify(&mut slice); // Fails to compile due to ambiguity
//! MaxHeap::heapify(&comp, &mut slice);
//! MinHeap::heapify(&comp, &mut slice);
//! ```
//! 와 같이 명시적으로 trait을 지정해야 한다.

pub mod comparator;
pub use comparator::*;

mod heap_implementation;
use heap_implementation::*;

pub mod max_heap;
pub mod min_heap;
