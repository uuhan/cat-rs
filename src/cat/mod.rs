#![allow(non_snake_case)]
use crate::cat_version;
use crate::ffi::raw::*;
use crate::ffi::*;
use std::ffi::CString;

/// cat client
pub struct CatClient {
    /// client initialization key
    appkey: String,
    /// client config
    config: CatClientConfig,
}

impl CatClient {
    /// create a new cat client
    ///
    /// # Arguments
    ///
    /// * `appkey` - key which impl ToString
    ///
    pub fn new<T: ToString>(appkey: T) -> Self {
        CatClient {
            appkey: appkey.to_string(),
            config: CatClientConfig::default(),
        }
    }

    /// set cat client config
    pub fn config(&mut self, config: &mut CatClientConfig) -> &Self {
        self.config = *config;
        self
    }

    /// initialize cat client
    pub fn init(&mut self) -> &Self {
        unsafe {
            catClientInitWithConfig(
                CString::new(self.appkey.clone()).unwrap().as_ptr() as *const u8,
                self.config,
            );
            self
        }
    }

    /// destroy a cat client
    pub fn destroy(&self) {
        warn!("cat client is being destroyed!");
        unsafe { catClientDestroy() };
    }

    /// get cat client version
    pub fn version(&self) -> &str {
        cat_version()
    }
}

impl Drop for CatClient {
    fn drop(&mut self) {
        warn!("cat client destroyed!");
        self.destroy()
    }
}

/// log a cat event
///
/// # Arguments
///
/// * `type_` - event type
///
/// * `name_` - event name
///
/// * `status` - event status type "0" or other
///
/// * `data` - event data
///
/// # Example
///
/// ```rust,no_run
/// // logEvent("app", "foo", "0", "");
/// ```
pub fn logEvent<S: ToString>(type_: S, name_: S, status: S, data: S) {
    unsafe {
        crate::ffi::logEvent(
            c!(type_.to_string()),
            c!(name_.to_string()),
            c!(status.to_string()),
            c!(data.to_string()),
        )
    }
}
