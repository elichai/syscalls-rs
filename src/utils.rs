#[macro_export]
macro_rules! static_assert {
    ($test:expr) => {
        // Use the bool to access an array such that if the bool is false, the access
        // is out-of-bounds.
        #[allow(dead_code)]
        const _: () = [()][!($test as bool) as usize];
    };
}

#[macro_export]
macro_rules! result {
    ($res:path) => {
        if $res < 0 {
             // TODO: Is there a better way to do this then negating twice? maybe checking if the MSB is set? is that even better?
            // TODO: Add our own Error enum with all the errors in errno.h
            Err(::std::io::Error::from_raw_os_error(-$res as i32))
        } else {
            Ok($res as _)
        }
    };
}

#[macro_export]
macro_rules! result_none {
    ($res:path) => {
        result!($res).map(|r: usize| debug_assert_eq!(r, 0))
    };
}
