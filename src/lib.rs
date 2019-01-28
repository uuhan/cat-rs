extern crate libc;
#[macro_use]
extern crate log;
use std::ffi::CStr;
use std::ffi::CString;

mod ffi;

/// re-export struct from mod ffi
pub use ffi::{
    CatClient, CatClientConfig, CatEvent, CatHeartBeat, CatMessage, CatMetric, CatTransaction,
};

pub fn cat_version() -> String {
    let r = unsafe {
        let t = ffi::catVersion();
        CStr::from_ptr(t).to_str().unwrap()
    };

    r.to_owned()
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

pub fn cat_client_init_with_config(appkey: String, config: &ffi::CatClientConfig) -> i32 {
    unsafe { ffi::catClientInitWithConfig(c!(appkey), config) }
}
