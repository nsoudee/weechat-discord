use libc::*;
use std;
use std::ffi::CString;

pub fn hook_command(cmd: &str, desc: &str, args: &str, argdesc: &str, compl: &str) {
    let cmd = unwrap1!(CString::new(cmd));
    let desc = unwrap1!(CString::new(desc));
    let args = unwrap1!(CString::new(args));
    let argdesc = unwrap1!(CString::new(argdesc));
    let compl = unwrap1!(CString::new(compl));

    unsafe extern "C" fn callback(
        pointer: *const c_void,
        data: *mut c_void,
        buffer: *mut c_void,
        argc: c_int,
        argv: *mut *mut c_char,
        argv_eol: *mut *mut c_char,
    ) -> c_int {
        ::MAIN_BUFFER.print("Hiya!");
        0
    }
    unsafe {
        let plug = (*::core::WEECHAT_PLUGIN).get_mut();
        (plug.hook_command.unwrap())(
            plug,
            cmd.as_ptr(),
            desc.as_ptr(),
            args.as_ptr(),
            argdesc.as_ptr(),
            compl.as_ptr(),
            Some(callback),
            std::ptr::null(),
            std::ptr::null_mut(),
        );
    }
}
