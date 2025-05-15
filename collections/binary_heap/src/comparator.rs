//! # Comparator
//! Core trait to use MinHeap
//! Ord trait이 없는 T, 혹은 기존의 Ord와는 다른 기준으로 정렬을 위해 도입한 trait
//! wrapper class를 활용하여 다양성을 주는 std의 BinaryHeap과는 달리, 비교 logic 자체를 전달하도록 구현
//!
//! 임의 타입 T에 대한 ref를 둘 받아서 Ordering을 반환함
//! inline을 위하여 trait을 monomorphization 할 필요가 있음
//!
//! 만약, T가 Ord를 갖추었다면, DefaultComparator를 제공한다.
//!
//! # DefaultComparator
//! 데이터의 기본적인 비교 방법을 활용한 comparator 구현, Ord 트레잇을 요구한다.
//!
//! ord 트레잇에 의해서 비교는 std::cmp로 이루어지며, Ordering enum을 반환함
//! variant로는 Greater, Equal, Less의 3종이 존재
//!

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
