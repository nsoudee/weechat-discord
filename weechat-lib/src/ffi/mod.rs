#[macro_use]
pub mod macros;
pub mod bindgen;
pub mod info;

pub fn really_bad(message: String) -> ! {
    ::MAIN_BUFFER.print(&format!("weecord: Internal error - {}", message));
    panic!(message); // hopefully we hit a catch_unwind
}
