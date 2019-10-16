
#[macro_export]
macro_rules! static_assert {
    ($test:expr) => {
        // Use the bool to access an array such that if the bool is false, the access
        // is out-of-bounds.
        #[allow(dead_code)]
        const _: () = [()][!($test as bool) as usize];
    };
}
