extern crate libc;
#[macro_use]
extern crate log;
use std::ffi::CString;

#[macro_use]
pub(crate) mod ffi;
pub mod cat;

/// re-export struct
pub use cat::CatClient;
pub use ffi::logEvent;
pub use ffi::CatTransaction;

pub fn cat_version() -> String {
    ffi::catVersion().to_owned()
}

pub fn cat_client_init(appkey: String) -> i32 {
    unsafe { ffi::catClientInit(c!(appkey)) }
}

pub fn cat_client_init_with_config(appkey: String, config: &mut ffi::CatClientConfig) -> i32 {
    unsafe { ffi::catClientInitWithConfig(c!(appkey), config) }
}

#[cfg(test)]
mod tests {
    use super::CatClient;
    use super::CatTransaction;
    use std::ptr;

    #[test]
    fn test_transaction() {
        let mut cat = CatClient::new("test");
        cat.init();
        let tr = CatTransaction::new("foo", "bar");
        assert!(!tr.is_null());

        unsafe {
            super::logEvent(
                b"foo\0".as_ptr() as *const u8,
                b"bar\0".as_ptr() as *const u8,
                b"0\0".as_ptr() as *const u8,
                ptr::null(),
            );
            super::logEvent(
                b"foo\0".as_ptr() as *const u8,
                b"bar\0".as_ptr() as *const u8,
                b"1\0".as_ptr() as *const u8,
                ptr::null(),
            );

            (*tr).complete()
        }
    }
}
