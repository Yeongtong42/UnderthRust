pub mod collections {
    pub use binary_heap;
}
pub mod algorithms {
    pub mod adapter {
        pub use heap_on_slice;
    }
    pub mod sort {
        pub use counting_sort;
        pub use insertion_sort;
        pub use intro_sort;
        pub use merge_sort;
        pub use quick_sort;
        pub use radix_sort;
    }
}
