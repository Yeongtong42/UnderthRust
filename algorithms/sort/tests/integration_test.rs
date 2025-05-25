#![allow(unused)]

use sort::*;
use std::cmp::Reverse;

use rand::distr::StandardUniform;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const TEST_SIZE: usize = 10_000;

#[test]
fn test_insertion_sort() {
    let seed: u64 = 42;
    let rng = StdRng::seed_from_u64(seed);

    let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

    unsafe {
        insertion_sort(&mut vec);
    }

    assert!(vec.is_sorted());
}

#[test]
fn test_insertion_sort_by() {
    let seed: u64 = 42;
    let rng = StdRng::seed_from_u64(seed);

    let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

    unsafe {
        insertion_sort_by(&mut vec, |a: &i32, b: &i32| Reverse(a).cmp(&Reverse(b)));
    }

    assert!(vec.is_sorted_by(|&a, &b| { a > b }));
}

#[test]
fn test_merge_sort() {
    let seed: u64 = 42;
    let rng = StdRng::seed_from_u64(seed);

    let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

    merge_sort(&mut vec);

    assert!(vec.is_sorted());
}

#[test]
fn test_merge_sort_by() {
    let seed: u64 = 42;
    let rng = StdRng::seed_from_u64(seed);

    let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

    merge_sort_by(&mut vec, |a: &i32, b: &i32| Reverse(a).cmp(&Reverse(b)));

    assert!(vec.is_sorted_by(|&a, &b| { a > b }));
}

#[test]
fn test_binary_quick_sort() {
    let seed: u64 = 42;
    let rng = StdRng::seed_from_u64(seed);

    let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

    binary_quick_sort(&mut vec);

    assert!(vec.is_sorted());
}

#[test]
fn test_binary_quick_sort_by() {
    let seed: u64 = 42;
    let rng = StdRng::seed_from_u64(seed);

    let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

    binary_quick_sort_by(&mut vec, |a: &i32, b: &i32| Reverse(a).cmp(&Reverse(b)));

    assert!(vec.is_sorted_by(|&a, &b| { a > b }));
}

#[test]
fn test_ternary_quick_sort() {
    let seed: u64 = 42;
    let rng = StdRng::seed_from_u64(seed);

    let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

    ternary_quick_sort(&mut vec);

    assert!(vec.is_sorted());
}

#[test]
fn test_ternary_quick_sort_by() {
    let seed: u64 = 42;
    let rng = StdRng::seed_from_u64(seed);

    let mut vec: Vec<i32> = rng.sample_iter(StandardUniform).take(TEST_SIZE).collect();

    ternary_quick_sort_by(&mut vec, |a: &i32, b: &i32| Reverse(a).cmp(&Reverse(b)));

    assert!(vec.is_sorted_by(|&a, &b| { a > b }));
}
