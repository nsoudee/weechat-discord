#[macro_export]
macro_rules! unwrap {
    ($expr:expr) => {
        ($expr).unwrap_or_else(|| {
            $crate::ffi::really_bad(
                concat!("Expression did not unwrap: ", stringify!($expr)).into(),
            )
        })
    };
}

#[macro_export]
macro_rules! unwrap1 {
    ($expr:expr) => {
        ($expr).unwrap_or_else(|_| {
            $crate::ffi::really_bad(
                concat!("Expression did not unwrap: ", stringify!($expr)).into(),
            )
        })
    };
}
