
#![allow(unused_macros)]
#![allow(dead_code)]

use std::cmp::Ordering;

pub trait Comparator<T> {
	fn compare(&self, a : &T, b : &T) -> std::cmp::Ordering;
}

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
	C : Default + Comparator<T>
{
	#[inline]
	fn get_parent(i : usize) -> usize {
		((i + 1) >> 1) - 1
	}
	#[inline]
	fn get_left(i : usize) -> usize {
		((i + 1) << 1) - 1
	}
	#[inline]
	fn get_right(i : usize) -> usize {
		((i + 1) << 1) + 1 - 1
	}

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
		let offset = data.len() / 2 - 1;
		for i in offset..=0 {
			Self::min_heapify(data, comp, i);
		}
	}

	pub fn new() -> MinHeap<T, C> {
		MinHeap {
			data : Vec::new(),
			comparator : C::default(),
		}
	}

	pub fn from_vec(vec : Vec<T>) -> MinHeap<T, C> {
		let mut vec = vec;
		let comparator = C::default();
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
			&& std::cmp::Ordering::Greater == self.comparator.compare(&data[parent_idx], &data[cur_idx])
		{
			data.swap(parent_idx, cur_idx);	// pull up
			cur_idx = parent_idx;
			parent_idx = Self::get_parent(parent_idx);
		}
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
