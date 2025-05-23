//! # MinHeap
//!
//! ## Fields
//! - data : 실제 heap tree가 구성되는 벡터, memory management는 Vec의 method에 일임한다
//! - comparator : 비교 방법을 정의한 객체
//!
//! ## Generic type
//! ### T
//! 실제 저장하게 될 데이터의 타입
//! ### C
//! Comparator trait을 구현한 임의 타입
//!
//! ## Sub Modules
//!
//! ### comparator
//! Comparator trait의 구현체.
//!

use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

use crate::comparator::Comparator;
use crate::heap_logic::{build_heap, get_parent, min_heapify};

pub struct MinHeap<T, C: Comparator<T>> {
    data: Vec<T>,
    comparator: C,
}

impl<T, C> MinHeap<T, C>
where
    C: Comparator<T>,
{
    /// create empty min heap
    pub fn new(comp: C) -> MinHeap<T, C> {
        MinHeap {
            data: Vec::new(),
            comparator: comp,
        }
    }

    /// create min heap with vector
    pub fn from_vec(mut source: Vec<T>, comp: C) -> MinHeap<T, C> {
        let comparator = comp;
        build_heap(&mut source, &comparator);
        MinHeap {
            data: source,
            comparator,
        }
    }

    /// push new element to the heap
    pub fn push(&mut self, elem: T) {
        let data = &mut self.data;
        data.push(elem);
        let mut cur_idx = data.len() - 1;
        let mut parent_idx = get_parent(cur_idx);
        while parent_idx < data.len()
            && cur_idx < data.len()
            && Ordering::Greater == self.comparator.compare(&data[parent_idx], &data[cur_idx])
        {
            data.swap(parent_idx, cur_idx); // pull up
            cur_idx = parent_idx;
            parent_idx = get_parent(parent_idx);
        }
    }

    /// add multiple element to the heap
    pub fn extend(&mut self, elems: &mut Vec<T>) {
        let data = &mut self.data;
        data.append(elems);
        build_heap(data, &self.comparator); // O(n)
    }

    /// extract ownership of the element at root
    /// O(log n)
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let data = &mut self.data;
        let end_idx = data.len() - 1; // already checked the emptiness, no overflow
        data.swap(0, end_idx);
        let result = data.pop();
        min_heapify(data, &self.comparator, 0); // O(log n)
        result
    }

    /// # Description
    /// get mutable reference of root of binary heap
    /// it's source will be heaped when the PeekMut drops
    pub fn peek_mut<'a>(&'a mut self) -> Option<PeekMut<'a, T, C>> {
        (!self.is_empty()).then_some(PeekMut { source: self })
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn top(&self) -> Option<&T> {
        self.data.get(0)
    }
}

impl<T, C> FromIterator<T> for MinHeap<T, C>
where
    C: Comparator<T> + Default,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        MinHeap::<T, C>::from_vec(iter.into_iter().collect(), C::default())
    }
}

///
/// # Description
/// BinaryHeap의 PeekMut의 replica, MinHeap의 top에 대한 smart pointer.
/// MinHeap의 root data에 대한 &와 &mut를 제공, MinHeap은 동결되며,
/// drop될 때, min_heapify를 수행하여 invariant를 복원하며, &mut를 반환한다.
///
/// # Field
/// - comp : 참조 대상인 MinHeap의 comparator
/// - source : 참조 대상인 MinHeap의 data
///
/// # try-error
/// 1. PeekMut의 원소로 &mut T와 &mut MinHeap을 주고, 각각 &mut self.data[0]와 self로 초기화
/// 	-> MinHeap에 대한 &mut의 중복으로 실패.
/// 2. PeekMut의 원소로 T와 &mut MinHeap을 주고, self.pop()과 self로 초기화
/// 	-> drop 시 T를 self에 push하려고 하였으나, 소유권 이동에 실패.
///
pub struct PeekMut<'a, T, C: Comparator<T>> {
    source: &'a mut MinHeap<T, C>,
}

// deref trait
impl<'a, T, C> Deref for PeekMut<'a, T, C>
where
    C: Comparator<T>,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.source.data[0]
    }
}

// deref mut trait
impl<'a, T, C> DerefMut for PeekMut<'a, T, C>
where
    C: Comparator<T>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.source.data[0]
    }
}

/// drop trait for PeekMut
/// recover invariant of it's MinHeap
impl<'a, T, C> Drop for PeekMut<'a, T, C>
where
    C: Comparator<T>,
{
    /// recover invariant of heap tree
    fn drop(&mut self) {
        min_heapify(&mut self.source.data, &self.source.comparator, 0);
    }
}
