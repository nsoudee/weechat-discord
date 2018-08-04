/// The entry point to a weechat plugin.
/// The variables `plugin` and `args` provide access to the weechat plugin object
/// and process args respectively
#[macro_export]
macro_rules! weechat_plugin_init(
    ($plugin:ident, $args:ident, $init_block:block) => {
        #[allow(unreachable_code)]
        fn __custom_plugin_init($plugin: $crate::core::WeechatPlugin, $args: $crate::core::Args) -> bool {
            return $init_block
        }

        #[no_mangle]
        pub extern "C" fn weechat_plugin_init(
            plugin: *mut $crate::ffi::bindgen::t_weechat_plugin,
            argc: ::libc::c_int,
            argv: *mut *mut ::libc::c_char
        ) -> ::libc::c_int {
            return $crate::core::init(plugin, argc, argv, __custom_plugin_init);
        }
    };
    ($plugin:ident, $init_block:block) => {
        weechat_plugin_init!($plugin, __args, $init_block);
    };
    ($init_block:block) => {
        weechat_plugin_init!(__plugin, __args, $init_block);
    };
);

/// The exit point of a weechat plugin, called as the plugin is unloaded
/// The variable `plugin` points to the weechat plugin object
#[macro_export]
macro_rules! weechat_plugin_end(
    ($plugin:ident, $end_block:block) => {
        #[allow(unreachable_code)]
        fn __custom_plugin_end($plugin: $crate::core::WeechatPlugin) -> bool {
            return $end_block
        }

        #[no_mangle]
        pub extern "C" fn weechat_plugin_end(
            plugin: *mut $crate::ffi::bindgen::t_weechat_plugin
        ) -> ::libc::c_int {
            return $crate::core::end(plugin, __custom_plugin_end);
        }
    };
    ($end_block:block) => {
        weechat_plugin_end!(__plugin, $end_block);
    };
);
