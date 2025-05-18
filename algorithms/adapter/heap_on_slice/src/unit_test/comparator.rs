//! implementation of unit tests for the heap_on_slice::comparator module

use crate::comparator::{Comparator, DefaultComparator, Reverse, ReverseComparator};
use std::cmp::Ordering;

#[test]
fn default_comparator_orders() {
    let comp = DefaultComparator;
    assert_eq!(comp.cmp(&1, &2), Ordering::Less);
    assert_eq!(comp.cmp(&2, &1), Ordering::Greater);
    assert_eq!(comp.cmp(&3, &3), Ordering::Equal);
}

#[test]
fn reverse_comparator_orders() {
    let comp = ReverseComparator;
    assert_eq!(comp.cmp(&1, &2), Ordering::Greater);
    assert_eq!(comp.cmp(&2, &1), Ordering::Less);
    assert_eq!(comp.cmp(&3, &3), Ordering::Equal);
}

#[test]
fn reverse_wrapper_reverses_order() {
    let dc = DefaultComparator;
    let comp = Reverse(&dc);
    assert_eq!(comp.cmp(&1, &2), Ordering::Greater);
    assert_eq!(comp.cmp(&2, &1), Ordering::Less);
    assert_eq!(comp.cmp(&3, &3), Ordering::Equal);

    let rc = ReverseComparator;
    let comp2 = Reverse(&rc);
    assert_eq!(comp2.cmp(&1, &2), Ordering::Less);
    assert_eq!(comp2.cmp(&2, &1), Ordering::Greater);
    assert_eq!(comp2.cmp(&3, &3), Ordering::Equal);
}
