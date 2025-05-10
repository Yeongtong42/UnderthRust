
#![allow(unused_macros)]
#![allow(dead_code)]

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

	pub fn len(&self) -> usize {
		self.data.len()
	}

	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}

	// top, ref 반환
	pub fn top(&self) -> Option<&T> {
		Some(&self.data[0])
	}
}
