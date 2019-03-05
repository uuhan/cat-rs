#![allow(non_snake_case)]
use std::error;
use std::ffi::CString;
use std::fmt;
use std::result;

use crate::cat_version;
use crate::ffi::raw::*;
use crate::ffi::*;

#[derive(Debug, Clone)]
pub enum CatError {
    CatClientInitError,
}

impl error::Error for CatError {
    fn description(&self) -> &str {
        "cat client init failed!"
    }
}

impl fmt::Display for CatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CatError::CatClientInitError => write!(f, "CatClientInitError"),
        }
    }
}

type Result<T> = result::Result<T, CatError>;

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
    pub fn init(&mut self) -> Result<&mut Self> {
        unsafe {
            let rc = catClientInitWithConfig(
                CString::new(self.appkey.clone()).unwrap().as_ptr() as *const u8,
                self.config,
            );
            if rc == 0 {
                error!("{}", CatError::CatClientInitError);
                Err(CatError::CatClientInitError)
            } else {
                Ok(self)
            }
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
