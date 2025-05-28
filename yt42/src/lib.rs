pub mod collections {
    pub use binary_heap;
}
pub mod algorithms {
    pub mod adapter {
        pub use heap_on_slice;
    }
    pub mod sort {
        pub mod counting_sort {
            pub use counting_sort;
            pub use radix_sort;
        }
    }
}
