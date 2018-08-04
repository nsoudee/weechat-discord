use libc::{c_char, c_int, c_void};
use std::ffi::CString;
use std::panic;
use std::ptr;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Buffer {
    ptr: *mut c_void,
}

unsafe impl Send for Buffer {}

pub const MAIN_BUFFER: Buffer = Buffer {
    ptr: ptr::null_mut(),
};

impl Buffer {
    pub fn from_ptr(ptr: *mut c_void) -> Buffer {
        Buffer { ptr }
    }

    // pub fn new(name: &str, on_input: fn(Buffer, &str)) -> Option<Buffer> {
    //     extern "C" {
    //         fn wdc_buffer_new(
    //             name: *const c_char,
    //             pointer: *const c_void,
    //             input_callback: extern "C" fn(
    //                 *const c_void,
    //                 *mut c_void,
    //                 *mut c_void,
    //                 *const c_char,
    //             ) -> c_int,
    //             close_callback: extern "C" fn(*const c_void, *mut c_void, *mut c_void) -> c_int,
    //         ) -> *mut c_void;
    //     }
    //     extern "C" fn input_cb(
    //         pointer: *const c_void,
    //         data: *mut c_void,
    //         buffer: *mut c_void,
    //         input_data: *const c_char,
    //     ) -> c_int {
    //         let _ = data;
    //         wrap_panic(|| {
    //             let buffer = Buffer { ptr: buffer };
    //             let on_input: fn(Buffer, &str) = unsafe { ::std::mem::transmute(pointer) };
    //             let input_data = unsafe { CStr::from_ptr(input_data).to_str() };
    //             let input_data = match input_data {
    //                 Ok(x) => x,
    //                 Err(_) => return,
    //             };
    //             on_input(buffer, input_data);
    //         });
    //         0
    //     }
    //     extern "C" fn close_cb(
    //         pointer: *const c_void,
    //         data: *mut c_void,
    //         buffer: *mut c_void,
    //     ) -> c_int {
    //         let _ = pointer;
    //         let _ = data;
    //         let _ = buffer;
    //         0
    //     }
    //     unsafe {
    //         let name = unwrap1!(CString::new(name));
    //         let pointer = on_input as *const c_void;
    //         let result = wdc_buffer_new(name.as_ptr(), pointer, input_cb, close_cb);
    //         if result.is_null() {
    //             None
    //         } else {
    //             Some(Buffer { ptr: result })
    //         }
    //     }
    // }
    pub fn print(&self, message: &str) {
        // extern "C" {
        //     fn wdc_print(buffer: *mut c_void, message: *const c_char);
        // }
        unsafe {
            let msg = unwrap1!(CString::new(message));
            unsafe {
                // let printf_date_tags = ::core::WEECHAT_PLUGIN::printf_date_tags.unwrap();
            }
            // printf_date_tags(0 as *mut _, 0, std::ptr::null(), msg);
            // wdc_print(self.ptr, msg.as_ptr());
        }
    }
}
