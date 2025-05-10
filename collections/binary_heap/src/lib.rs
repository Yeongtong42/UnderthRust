
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
