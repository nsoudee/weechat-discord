extern crate libc;

#[macro_use]
pub mod macros;
#[macro_use]
pub mod ffi;
pub mod buffer;
pub mod core;
pub mod hook;

pub use buffer::MAIN_BUFFER;
