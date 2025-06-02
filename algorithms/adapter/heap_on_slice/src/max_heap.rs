use super::heap_implementation as Impl;
use std::cmp::Ordering;

fn reversed_cmp<T: Ord>(a: &T, b: &T) -> Ordering {
    Ord::cmp(a, b).reverse()
}

fn reverse_compare<T, F>(mut compare: F) -> impl FnMut(&T, &T) -> Ordering
where
    F: FnMut(&T, &T) -> Ordering,
{
    move |a, b| compare(a, b).reverse()
}

fn key2reversed_compare<T, K, F>(mut key: F) -> impl FnMut(&T, &T) -> Ordering
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    move |a, b| Ord::cmp(&key(a), &key(b)).reverse()
}

pub fn is_heap<T: Ord>(arr: &[T]) -> bool {
    Impl::is_heap(arr, reversed_cmp)
}

pub fn is_heap_by<T, F>(arr: &[T], compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::is_heap(arr, reverse_compare(compare))
}

pub fn is_heap_by_key<T, K, F>(arr: &[T], key: F) -> bool
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::is_heap(arr, key2reversed_compare(key))
}

pub fn heapify<T: Ord>(arr: &mut [T]) {
    Impl::heapify(arr, reversed_cmp);
}

pub fn heapify_by<T, F>(arr: &mut [T], compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heapify(arr, reverse_compare(compare));
}

pub fn heapify_by_key<T, K, F>(arr: &mut [T], key: F)
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heapify(arr, key2reversed_compare(key));
}

pub fn heap_pushpop<T: Ord>(arr: &mut [T], x: T) -> T {
    Impl::heap_pushpop(arr, x, reversed_cmp)
}

pub fn heap_pushpop_by<T, F>(arr: &mut [T], x: T, compare: F) -> T
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heap_pushpop(arr, x, reverse_compare(compare))
}

pub fn heap_pushpop_by_key<T, K, F>(arr: &mut [T], x: T, key: F) -> T
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heap_pushpop(arr, x, key2reversed_compare(key))
}

pub fn heap_pop<T: Ord>(arr: &mut [T]) -> Option<&mut [T]> {
    Impl::heap_pop(arr, reversed_cmp)
}

pub fn heap_pop_by<T, F>(arr: &mut [T], compare: F) -> Option<&mut [T]>
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heap_pop(arr, reverse_compare(compare))
}

pub fn heap_pop_by_key<T, K, F>(arr: &mut [T], key: F) -> Option<&mut [T]>
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heap_pop(arr, key2reversed_compare(key))
}

pub fn heapsort<T: Ord>(arr: &mut [T]) {
    Impl::heap_reverse_sort(arr, reversed_cmp);
}

pub fn heapsort_by<T, F>(arr: &mut [T], compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::heap_reverse_sort(arr, reverse_compare(compare));
}

pub fn heapsort_by_key<T, K, F>(arr: &mut [T], key: F)
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::heap_reverse_sort(arr, key2reversed_compare(key));
}

pub fn adjust_heap<T: Ord>(arr: &mut [T], idx: usize) -> bool {
    Impl::adjust_heap(arr, idx, reversed_cmp)
}

pub fn adjust_heap_by<T, F>(arr: &mut [T], idx: usize, compare: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    Impl::adjust_heap(arr, idx, reverse_compare(compare))
}

pub fn adjust_heap_by_key<T, K, F>(arr: &mut [T], idx: usize, key: F) -> bool
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    Impl::adjust_heap(arr, idx, key2reversed_compare(key))
}
