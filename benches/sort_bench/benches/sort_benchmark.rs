use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main}; // for Benchmark

// for pseudo random generation
use rand::distr::StandardUniform;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

// sorting algorithms to compare
// use counting_sort::TryCountingSort;
// use radix_sort::*;
use heap_on_slice::{DefaultComparator, max_heap::MaxHeap};
use insertion_sort::insertion_sort;
use intro_sort::intro_sort;
use merge_sort::merge_sort;
use quick_sort::{binary_quick_sort, ternary_quick_sort};
use tim_sort::tim_sort;

fn heap_sort_wrapper<T: Ord>(slice: &mut [T]) {
    MaxHeap::heap_sort(&DefaultComparator, slice);
}

fn criterion_benchmark(c: &mut Criterion) {
    let seed: u64 = 42;
    let rng = StdRng::seed_from_u64(seed);
    let vec: Vec<u64> = rng.sample_iter(StandardUniform).take(100_000).collect();
    let mut group = c.benchmark_group("Sorting");

    // range from 100, 500, 1000, 2000, 4000, 8000, 10000
    for size in [1000, 5000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        if *size < 100_000 {
            // insertion sort is too big, restrict it's range.
            group.bench_with_input(BenchmarkId::new("insertion", size), size, |b, &size| {
                b.iter_batched_ref(
                    || vec[0..size].to_vec(),
                    |mut data| insertion_sort(&mut data),
                    BatchSize::SmallInput,
                );
            });
        }
        group.bench_with_input(BenchmarkId::new("merge", size), size, |b, &size| {
            b.iter_batched_ref(
                || vec[0..size].to_vec(),
                |mut data| merge_sort(&mut data),
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("heap", size), size, |b, &size| {
            b.iter_batched_ref(
                || vec[0..size].to_vec(),
                |mut data| heap_sort_wrapper(&mut data),
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("quick", size), size, |b, &size| {
            b.iter_batched_ref(
                || vec[0..size].to_vec(),
                |mut data| binary_quick_sort(&mut data),
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("3 way quick", size), size, |b, &size| {
            b.iter_batched_ref(
                || vec[0..size].to_vec(),
                |mut data| ternary_quick_sort(&mut data),
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("intro", size), size, |b, &size| {
            b.iter_batched_ref(
                || vec[0..size].to_vec(),
                |mut data| intro_sort(&mut data),
                BatchSize::SmallInput,
            );
        });
        /*
        group.bench_with_input(BenchmarkId::new("tim", size), size, |b, &size| {
            b.iter_batched_ref(
                || vec[0..size].to_vec(),
                |mut data| tim_sort(&mut data),
                BatchSize::SmallInput,
            );
        });
        */
        group.bench_with_input(BenchmarkId::new("slice::sort", size), size, |b, &size| {
            b.iter_batched_ref(
                || vec[0..size].to_vec(),
                |mut data| (&mut data).sort(),
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(
            BenchmarkId::new("slice::sort_unstable", size),
            size,
            |b, &size| {
                b.iter_batched_ref(
                    || vec[0..size].to_vec(),
                    |mut data| (&mut data).sort_unstable(),
                    BatchSize::SmallInput,
                );
            },
        );
        /*
        group.bench_with_input(BenchmarkId::new("counting", size), size, |b, &size| {
            b.iter_batched_ref(
                || vec[0..size].to_vec(),
                |mut data| (&mut data).try_counting_sort(),
                BatchSize::SmallInput,
            );
        });
        */
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
