//! Items required for intializing and creating a weechat plugin
use std::ffi::{CStr, CString};
use std::ops::Index;
use std::ptr;

use libc::*;

pub static mut WEECHAT_PLUGIN: *mut WeechatPlugin = 0 as *mut WeechatPlugin;

// Used by `weechat_plugin_init`
#[doc(hidden)]
pub fn init(
    plugin_ptr: *mut ::ffi::bindgen::t_weechat_plugin,
    argc: c_int,
    argv: *mut *mut c_char,
    f: fn(plugin: WeechatPlugin, args: Args) -> bool,
) -> c_int {
    let plugin = WeechatPlugin::from_ptr(plugin_ptr);
    unsafe {
        WEECHAT_PLUGIN = Box::into_raw(Box::new(plugin));
    }
    let plugin = WeechatPlugin::from_ptr(plugin_ptr);

    let args = Args {
        argc: argc as u32,
        argv: argv,
    };
    let result = f(plugin, args);

    if result {
        ::ffi::bindgen::WEECHAT_RC_OK as i32
    } else {
        ::ffi::bindgen::WEECHAT_RC_ERROR
    }
}

// Used by `weechat_plugin_end`
#[doc(hidden)]
pub fn end(
    plugin: *mut ::ffi::bindgen::t_weechat_plugin,
    f: fn(plugin: WeechatPlugin) -> bool,
) -> c_int {
    let plugin = WeechatPlugin::from_ptr(plugin);
    let result = f(plugin);

    if result {
        ::ffi::bindgen::WEECHAT_RC_OK as i32
    } else {
        ::ffi::bindgen::WEECHAT_RC_ERROR
    }
}

/// A wrapper around a weechat_plugin pointer
#[repr(transparent)]
pub struct WeechatPlugin {
    inner: *mut ::ffi::bindgen::t_weechat_plugin,
}

impl WeechatPlugin {
    /// Create a new `WeechatPlugin` from a raw pointer.  Usually not directly used
    pub fn from_ptr(inner: *mut ::ffi::bindgen::t_weechat_plugin) -> WeechatPlugin {
        assert!(!inner.is_null());

        WeechatPlugin { inner: inner }
    }
}

impl WeechatPlugin {
    /// Retrieve the raw underling pointer
    #[inline]
    pub(crate) fn get(&self) -> &::ffi::bindgen::t_weechat_plugin {
        unsafe { &*self.inner }
    }

    #[inline]
    pub(crate) unsafe fn get_mut(&self) -> &mut ::ffi::bindgen::t_weechat_plugin {
        unsafe { &self.inner }
    }

    /// Log a message
    pub fn log(&self, msg: &str) {
        let log_printf = self.get().log_printf.unwrap();

        let fmt = CString::new("%s").unwrap();
        let msg = CString::new(msg).unwrap();

        unsafe {
            log_printf(fmt.as_ptr(), msg.as_ptr());
        }
    }

    /// Print a message to the main buffer
    pub fn print(&self, msg: &str) {
        let printf_date_tags = self.get().printf_date_tags.unwrap();

        let fmt = CString::new("%s").unwrap();
        let msg = CString::new(msg).unwrap();

        unsafe {
            printf_date_tags(ptr::null_mut(), 0, ptr::null(), fmt.as_ptr(), msg.as_ptr());
        }
    }
}

/// Command line arguments
pub struct Args {
    argc: u32,
    argv: *mut *mut c_char,
}

impl Args {
    // Number of commands
    pub fn len(&self) -> usize {
        self.argc as usize
    }
}

impl Index<usize> for Args {
    type Output = CStr;

    fn index<'a>(&'a self, index: usize) -> &'a CStr {
        assert!(index < self.len());

        unsafe {
            let ptr = self.argv.offset(index as isize);
            CStr::from_ptr(ptr as *const c_char)
        }
    }
}
