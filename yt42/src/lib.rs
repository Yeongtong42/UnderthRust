pub mod collections {
    pub use binary_heap;
}
pub mod algorithms {
    pub mod adapter {
        pub mod heap_on_slice {
            pub use heap_on_slice::*;
        }
    }
}
