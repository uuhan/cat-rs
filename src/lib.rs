extern crate libc;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;

#[macro_use]
pub(crate) mod ffi;
pub mod cat;

// re-export struct
pub use cat::logEvent;
pub use cat::CatClient;
pub use ffi::CatTransaction;

/// get current cat version
pub fn cat_version() -> &'static str {
    "3.0.1"
}

#[cfg(test)]
mod tests {
    use super::CatClient;
    use super::CatTransaction;

    #[test]
    fn test_transaction() {
        let mut cat = CatClient::new("test");
        cat.init();
        let tr = CatTransaction::new("foo", "bar");
        assert!(!tr.is_null());

        unsafe {
            super::logEvent("foo", "bar", "0", "");
            super::logEvent("foo", "bar", "1", "");
            (*tr).complete()
        }
    }
}
