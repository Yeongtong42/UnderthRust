
use binary_heap::*;

#[test]
fn test_min_heap_from_vec() {
	let dcomp = DefaultComparator;

	let test_vec : Vec<u32> = vec![ 3, 2, 1, 5, 4];
	let mut pq = MinHeap::<u32, DefaultComparator>::from_vec(test_vec, dcomp);
	assert_eq!(*pq.top().unwrap(), 1);
	assert_eq!(pq.pop().unwrap(), 1);
	assert_eq!(*pq.top().unwrap(), 2);
	assert_eq!(pq.pop().unwrap(), 2);
	assert_eq!(*pq.top().unwrap(), 3);
	assert_eq!(pq.pop().unwrap(), 3);
	assert_eq!(*pq.top().unwrap(), 4);
	assert_eq!(pq.pop().unwrap(), 4);
	assert_eq!(*pq.top().unwrap(), 5);
	assert_eq!(pq.pop().unwrap(), 5);
	assert!(pq.is_empty());
}

#[test]
fn test_min_heap_push() {
	let dcomp = DefaultComparator;

	let mut pq = MinHeap::<u32, DefaultComparator>::new(dcomp);
	pq.push(6);
	pq.push(1);
	pq.push(16);
	pq.push(3);
	pq.push(643);
	assert_eq!(*pq.top().unwrap(), 1);
	assert_eq!(pq.pop().unwrap(), 1);
	assert_eq!(*pq.top().unwrap(), 3);
	assert_eq!(pq.pop().unwrap(), 3);
	assert_eq!(*pq.top().unwrap(), 6);
	assert_eq!(pq.pop().unwrap(), 6);
	assert_eq!(*pq.top().unwrap(), 16);
	assert_eq!(pq.pop().unwrap(), 16);
	assert_eq!(*pq.top().unwrap(), 643);
	assert_eq!(pq.pop().unwrap(), 643);
	assert!(pq.is_empty());
}

#[test]
fn test_min_heap_extend() {
	let dcomp = DefaultComparator;

	let test_vec : Vec<u32> = vec![ 3, 2, 1, 5, 4];
	let mut test_vec_extend : Vec<u32> = vec![ 13, 21, 1, 14];
	let mut pq = MinHeap::<u32, DefaultComparator>::from_vec(test_vec, dcomp);
	pq.extend(&mut test_vec_extend);
	assert_eq!(*pq.top().unwrap(),1);
	assert_eq!(pq.pop().unwrap(),1);
	assert_eq!(*pq.top().unwrap(),1);
	assert_eq!(pq.pop().unwrap(),1);
	assert_eq!(*pq.top().unwrap(),2);
	assert_eq!(pq.pop().unwrap(),2);
	assert_eq!(*pq.top().unwrap(),3);
	assert_eq!(pq.pop().unwrap(),3);
	assert_eq!(*pq.top().unwrap(), 4);
	assert_eq!(pq.pop().unwrap(),4);
	assert_eq!(*pq.top().unwrap(),5);
	assert_eq!(pq.pop().unwrap(),5);
	assert_eq!(*pq.top().unwrap(),13);
	assert_eq!(pq.pop().unwrap(),13);
	assert_eq!(*pq.top().unwrap(),14);
	assert_eq!(pq.pop().unwrap(),14);
	assert_eq!(*pq.top().unwrap(),21);
	assert_eq!(pq.pop().unwrap(),21);
	assert!(pq.is_empty());
}

#[test]
fn test_peek_mut_deref() {
	let dcomp = DefaultComparator;

	let test_vec : Vec<u32> = vec![ 3, 2, 1, 5, 4];
	let mut pq = MinHeap::<u32, DefaultComparator>::from_vec(test_vec, dcomp);

	let pm = pq.peek_mut().unwrap();
	assert_eq!(*pm, 1);
}

#[test]
fn test_peek_mut_deref_mut() {
	let dcomp = DefaultComparator;

	let test_vec : Vec<u32> = vec![ 3, 2, 1, 5, 4];
	let mut pq = MinHeap::<u32, DefaultComparator>::from_vec(test_vec, dcomp);

	let mut pm = pq.peek_mut().unwrap();
	*pm = 44;
	std::mem::drop(pm);
	assert_eq!(pq.pop().unwrap(), 2);
}

/*
fn test_peek_mut_borrow() {
	let dcomp = DefaultComparator;

	let test_vec : Vec<u32> = vec![ 3, 2, 1, 5, 4];
	let mut pq = MinHeap::<u32, DefaultComparator>::from_vec(test_vec, dcomp);

	let mut pm = pq.peek_mut().unwrap();
	pq.push(44);	// <- double borrow mutable ref of pq, borrow check error
}
*/
