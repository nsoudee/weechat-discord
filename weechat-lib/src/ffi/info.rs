#[macro_export]
macro_rules! weechat_plugin_name(
    ($name:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static weechat_plugin_name: [u8; $name.len()] = *$name;

        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static mut weechat_plugin_api_version: [u8; ffi::bindgen::WEECHAT_PLUGIN_API_VERSION.len()] = *ffi::bindgen::WEECHAT_PLUGIN_API_VERSION;
    }
);

#[macro_export]
macro_rules! weechat_plugin_author(
    ($author:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static mut weechat_plugin_author: [u8; $author.len()] = *$author;
    }
);

#[macro_export]
macro_rules! weechat_plugin_description(
    ($description:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static mut weechat_plugin_description: [u8; $description.len()] = *$description;
    }
);

#[macro_export]
macro_rules! weechat_plugin_version(
    ($version:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static mut weechat_plugin_version: [u8; $version.len()] = *$version;
    }
);

#[macro_export]
macro_rules! weechat_plugin_license(
    ($license:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static mut weechat_plugin_license: [u8; $license.len()] = *$license;
    }
);

#[macro_export]
macro_rules! weechat_plugin_priority(
    ($priority:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static weechat_plugin_priority: ::libc::c_int = $priority;
    }
);

#[macro_export]
macro_rules! weechat_plugin_info(
    (name: $name:expr) => {
        weechat_plugin_name!($name);
    };
    (author: $author:expr) => {
        weechat_plugin_author!($author);
    };
    (description: $description:expr) => {
        weechat_plugin_description!($description);
    };
    (version: $version:expr) => {
        weechat_plugin_version!($version);
    };
    (license: $license:expr) => {
        weechat_plugin_license!($license);
    };
    (priority: $priority:expr) => {
        weechat_plugin_priority!($priority)
    };
);

#[macro_export]
macro_rules! weechat_plugin(
    ($($name:ident: $value:expr),+) => {
        $(
            weechat_plugin_info!($name: $value);
        )+
    };
    ($($name:ident: $value:expr),+,) => {
        weechat_plugin!($($name: $value),+);
    };
);
