//!
//! # Description
//! Introduction to Algorithm의 BinaryHeap을 구현한다.
//! 기존에 prelude에 Max heap이 있으므로, min heap으로 구현한다.
//!
//! # Implementation
//! ## Traits
//! ### Ord
//! heap tree 내에서 데이터의 순서를 얻기 위한 비교 방법을 표현한 trait
//! 비교는 std::cmp로 이루어지며, Ordering 이라는 enum을 반환함
//! variant로는 Greater, Equal, Less의 3종이 존재
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
//! Comparator trait을 구현한 임의의 객체
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

impl<T, C> MinHeap<T, C>
where
	C : Comparator<T>
{
	/// note : get_parent(0) is usize::max, will always be bigger than data.len()
	/// TODO : change it to macro function
	#[inline]
	fn get_parent(i : usize) -> usize {
		((i + 1) >> 1).wrapping_sub(1)
	}
	#[inline]
	fn get_left(i : usize) -> usize {
		((i + 1) << 1) - 1
	}
	#[inline]
	fn get_right(i : usize) -> usize {
		((i + 1) << 1) + 1 - 1
	}

	/// # Description
	/// keep invariant for heap tree
	/// for performace reason, comp.compare()'s inlining is crucial
	fn min_heapify(data : &mut Vec<T>, comp : &impl Comparator<T>, i : usize) {
		let l = Self::get_left(i);
		let r = Self::get_right(i);
		let mut s = i;
		if l < data.len() && Ordering::Less == comp.compare(&data[l],&data[s])   {
			s = l;
		}
		if r < data.len() && Ordering::Less == comp.compare(&data[r],&data[s])   {
			s = r;
		}
		if s != i {
			data.swap(i, s);
			Self::min_heapify(data, comp, s);	// push down
		}
	}

	fn build_heap(data : &mut Vec<T>, comp : &impl Comparator<T>) {
		let offset = data.len() / 2;
		for i in (0..offset).rev() {
			Self::min_heapify(data, comp, i);
		}
	}

	pub fn new(comp : C) -> MinHeap<T, C> {
		MinHeap {
			data : Vec::new(),
			comparator : comp,
		}
	}

	pub fn from_vec(mut vec : Vec<T>, comp : C) -> MinHeap<T, C> {
		let comparator = comp;
		Self::build_heap(&mut vec, &comparator);
		MinHeap {
			data : vec,
			comparator,
		}
	}

	pub fn push(&mut self, elem : T) {
		let data = &mut self.data;
		data.push(elem);
		let mut cur_idx = data.len() - 1;
		let mut parent_idx = Self::get_parent(cur_idx);
		while parent_idx < data.len()
			&& cur_idx < data.len()
			&& Ordering::Greater == self.comparator.compare(&data[parent_idx], &data[cur_idx])
		{
			data.swap(parent_idx, cur_idx);	// pull up
			cur_idx = parent_idx;
			parent_idx = Self::get_parent(parent_idx);
		}
	}

	pub fn pop(&mut self) -> Option<T> {
		let data = &mut self.data;
		let end_idx = data.len() - 1;
		if data.len() > 1 as usize {
			data.swap(0, end_idx);
		}
		let result = data.pop();
		Self::min_heapify(data, &self.comparator, 0);
		result
	}

	pub fn len(&self) -> usize {
		self.data.len()
	}

	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}

	pub fn top(&self) -> Option<&T> {
		Some(&self.data[0])
	}
}
