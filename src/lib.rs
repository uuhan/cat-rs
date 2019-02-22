extern crate libc;
#[macro_use]
extern crate log;
use std::ffi::CString;

#[macro_use]
pub(crate) mod ffi;
pub mod cat;

/// re-export struct
pub use cat::CatClient;
pub use ffi::CatTransaction;

pub fn cat_version() -> String {
    ffi::catVersion().to_owned()
}

pub fn is_cat_enabled() -> bool {
    let rc = unsafe { ffi::isCatEnabled() };

    if rc != 0 {
        true
    } else {
        false
    }
}

pub fn cat_client_init(appkey: String) -> i32 {
    unsafe { ffi::catClientInit(c!(appkey)) }
}

pub fn cat_client_init_with_config(appkey: String, config: &mut ffi::CatClientConfig) -> i32 {
    unsafe { ffi::catClientInitWithConfig(c!(appkey), config) }
}
