use std::panic;

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

fn wrap_panic<R, F: FnOnce() -> R + panic::UnwindSafe>(f: F) -> Option<R> {
    let result = panic::catch_unwind(f);
    match result {
        Ok(x) => Some(x),
        Err(err) => {
            let msg = match err.downcast_ref::<String>() {
                Some(msg) => msg,
                None => "unknown error",
            };
            let result = panic::catch_unwind(|| {
                ::MAIN_BUFFER.print(&format!("weecord: Fatal error (caught) - {}", msg))
            });
            let _ = result; // eat error without logging :(
            None
        }
    }
}
