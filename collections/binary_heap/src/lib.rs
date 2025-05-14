//!
//! # Description
//! Introduction to Algorithm의 BinaryHeap을 구현한다.
//! 기존에 prelude에 Max heap이 있으므로, min heap으로 구현한다.
//!
//! # Implementation
//! ## Related Traits
//! ### Ord
//! 데이터의 비교 방법을 정의한 trait
//! 비교는 std::cmp로 이루어지며, Ordering enum을 반환함
//! variant로는 Greater, Equal, Less의 3종이 존재
//!
//! Ord가 구현되어 있다면, 이를 활용하는 DefaultComparator를 사용가능
//!
//! ### comparator
//! Ord trait이 없는 T, 혹은 기존의 Ord와는 다른 기준으로 정렬을 위해 도입한 trait
//! 임의 타입 T에 대한 ref를 둘 받아서 Ordering을 반환함
//! inline을 위하여 trait을 monomorphization 할 필요가 있음
//!
//! 만약, T가 Ord를 갖추었다면, DefaultComparator를 제공한다.
//!
//! ## Generic type
//! ### T
//! 실제 저장하게 될 데이터의 타입
//! ### C
//! Comparator trait을 구현한 임의 타입
//!
//! ## Field
//! ### data - Vec<T>
//! 실제 데이터를 소유하는 collection
//! memory management는 Vec의 method에 일임한다
//!
//! ### comparator
//! Comparator trait의 구현체.
//!
//! ## priority_queue in C++
//! 실제 container가 아닌, container에 api를 추가한 adaptor, 내부 구현 선택 가능
//! 기본적으로 연산자 오버로딩을 통한 max heap, 다만, comparator를 전달하여 임의 순서 가능
//! comparator는 두 원소를 비교하고 bool을 반환하는 함수객체, 반환값이 true라면 swap한다
//!
#![allow(unused_macros)]
#![allow(dead_code)]

use std::cmp::Ordering;

/// user decidable compare, inlining needed
pub trait Comparator<T> {
	fn compare(&self, a : &T, b : &T) -> std::cmp::Ordering;
}

/// Default comparator for type T with Ord trait.
#[derive(Default)]
pub struct DefaultComparator;
impl<T:Ord> Comparator<T> for DefaultComparator {
	#[inline]
	fn compare(&self, a: &T, b:&T) -> std::cmp::Ordering {
		a.cmp(b)
	}
}

pub struct MinHeap<T, C: Comparator<T>> {
	data: Vec<T>,
	comparator: C,
}
/// TODO : change it to macro function

/// helper function for heap tree, get index of parent node
/// note : get_parent(0) is usize::max, will always be bigger than data.len()
#[inline]
fn get_parent(i : usize) -> usize {
	((i + 1) >> 1).wrapping_sub(1)
}

/// helper function for heap tree, get index of left child node
#[inline]
fn get_left(i : usize) -> usize {
	((i + 1) << 1) - 1
}

/// helper function for heap tree, get index of right child node
#[inline]
fn get_right(i : usize) -> usize {
	((i + 1) << 1) + 1 - 1
}

/// keep order of heap tree
/// child nodes are bigger than it's parent node
/// for performace reason, comp.compare()'s inlining is crucial
/// O(log n)
fn min_heapify<T>(data : &mut Vec<T>, comp : &impl Comparator<T>, i : usize) {
	let l = get_left(i);
	let r = get_right(i);
	let mut s = i;
	if l < data.len() && Ordering::Less == comp.compare(&data[l],&data[s])   {
		s = l;
	}
	if r < data.len() && Ordering::Less == comp.compare(&data[r],&data[s])   {
		s = r;
	}
	if s != i {
		data.swap(i, s);
		min_heapify(data, comp, s);	// push down
	}
}

/// reorder vector to make heap tree
/// O(n)
fn build_heap<T>(data : &mut Vec<T>, comp : &impl Comparator<T>) {
	let offset = data.len() / 2;
	for i in (0..offset).rev() {
		min_heapify(data, comp, i);
	}
}

impl<T, C> MinHeap<T, C>
where
	C : Comparator<T>
{
	/// create empty min heap
	pub fn new(comp : C) -> MinHeap<T, C> {
		MinHeap {
			data : Vec::new(),
			comparator : comp,
		}
	}

	/// create min heap with vector
	pub fn from_vec(mut vec : Vec<T>, comp : C) -> MinHeap<T, C> {
		let comparator = comp;
		build_heap(&mut vec, &comparator);
		MinHeap {
			data : vec,
			comparator,
		}
	}

	/// push new element to the heap
	/// O(log n)
	pub fn push(&mut self, elem : T) {
		let data = &mut self.data;
		data.push(elem);
		let mut cur_idx = data.len() - 1;
		let mut parent_idx = get_parent(cur_idx);
		while parent_idx < data.len()
			&& cur_idx < data.len()
			&& Ordering::Greater == self.comparator.compare(&data[parent_idx], &data[cur_idx])
		{
			data.swap(parent_idx, cur_idx);	// pull up
			cur_idx = parent_idx;
			parent_idx = get_parent(parent_idx);
		}
	}

	/// add multiple element to the heap
	/// O(n)
	pub fn extend(&mut self, elems : &mut Vec<T>) {
		let data = &mut self.data;
		data.append(elems);
		build_heap(data,&self.comparator);
	}

	/// extract ownership of the element at root
	/// O(log n)
	pub fn pop(&mut self) -> Option<T> {
		let data = &mut self.data;
		let end_idx = data.len() - 1;
		if data.len() > 1 as usize {
			data.swap(0, end_idx);
		}
		let result = data.pop();
		min_heapify(data, &self.comparator, 0);
		result
	}

	pub fn len(&self) -> usize {
		self.data.len()
	}

	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}

	pub fn top(&self) -> Option<&T> {
		if self.data.is_empty() {
			return None;
		}
		Some(&self.data[0])
	}

	// from iter trait
}

use std::ops::{Deref, DerefMut};

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
	comp : &'a C,
	source : &'a mut Vec<T>,
}

impl<T, C> MinHeap<T, C>
where
	C : Comparator<T>
{
	/// get mutable reference of root of binary heap
	/// it's source will be heaped when the PeekMut drops
	pub fn peek_mut<'a>(&'a mut self) -> Option<PeekMut<'a, T, C>> {
		if true != self.is_empty() {
			return Some(PeekMut {
				comp : &self.comparator,
				source : &mut self.data,
			});
		}
		None
	}
}

// deref trait
impl<'a, T, C> Deref for PeekMut<'a, T, C>
where
	C : Comparator<T>
{
	type Target = T;
    fn deref(&self) -> &Self::Target {
		&self.source[0]
	}
}

// deref mut trait
impl<'a, T, C> DerefMut for PeekMut<'a, T, C>
where
	C : Comparator<T>
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.source[0]
	}

}

// drop trait
impl<'a, T, C> Drop for PeekMut<'a, T, C>
where
	C : Comparator<T>
{
	/// recover invariant of heap tree
	fn drop(&mut self) {
		min_heapify(&mut self.source, self.comp, 0);
	}
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
	use rand::distr::StandardUniform;
use rand::{Rng, SeedableRng};
	use rand::rngs::StdRng;

	use crate::Comparator;

    use crate::{DefaultComparator};

	fn is_min_heaped<T, C : Comparator<T>>(vec : &Vec<T>, comp : &C) -> bool {
		for i in (1..vec.len()).rev() {
			let current = &vec[i];
			let parent = &vec[super::get_parent(i)];
			if let Ordering::Greater = comp.compare(parent, current) {
				return false;
			}
		}
		true
	}

	#[test]
	fn test_build_heap_empty() {
		let dcomp = DefaultComparator;

		let mut vec1 : Vec<i32> = Vec::new();
		crate::build_heap(&mut vec1, &dcomp);
		assert!(is_min_heaped(&vec1, &dcomp));
	}

	#[test]
	fn test_build_heap_one() {
		let dcomp = DefaultComparator;

		let mut vec1 : Vec<i32> = vec![0i32;1];
		crate::build_heap(&mut vec1, &dcomp);
		assert!(is_min_heaped(&vec1, &dcomp));
	}

	#[test]
	fn test_build_heap_ordered() {
		let dcomp = DefaultComparator;

		let mut vec1 : Vec<i32> = (0..45i32).collect();
		crate::build_heap(&mut vec1, &dcomp);
		assert!(is_min_heaped(&vec1, &dcomp));
	}
	#[test]
	fn test_build_heap_reverse_ordered() {
		let dcomp = DefaultComparator;

		let mut vec1 : Vec<i32> = (0..45i32).rev().collect();
		crate::build_heap(&mut vec1, &dcomp);
		assert!(is_min_heaped(&vec1, &dcomp));
	}
	#[test]
	fn test_build_heap_random() {
		let dcomp = DefaultComparator;

		let seed: u64 = 42;
		let rng = StdRng::seed_from_u64(seed);

		let mut vec1: Vec<i32> = rng
			.sample_iter(StandardUniform)
			.take(1000_000)
			.collect();

		crate::build_heap(&mut vec1, &dcomp);
		assert!(is_min_heaped(&vec1, &dcomp));
	}

	#[test]
	fn test_heapifiy_empty() {
		let dcomp = DefaultComparator;

		let mut vec0 : Vec<u32> = Vec::new();
		crate::min_heapify(&mut vec0, &dcomp, 0);
		assert!(is_min_heaped(&vec0, &dcomp));
	}
	#[test]
	fn test_heapifiy_one() {
		let dcomp = DefaultComparator;

		let mut vec1 : Vec<u32> = vec![0];
		crate::min_heapify(&mut vec1, &dcomp, 0);
		assert!(is_min_heaped(&vec1, &dcomp));
	}
	#[test]
	fn test_heapifiy_general() {
		let dcomp = DefaultComparator;

		let mut vec2: Vec<u32> = vec![4, 1, 2, 3, 6, 7, 8];
		crate::min_heapify(&mut vec2, &dcomp, 0);
		assert!(is_min_heaped(&vec2, &dcomp));

		let mut vec3: Vec<u32> = vec![1, 2, 3, 99, 5, 6, 7];
		crate::min_heapify(&mut vec3, &dcomp, 3);
		assert!(is_min_heaped(&vec3, &dcomp));
	}
}
