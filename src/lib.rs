#![allow(non_snake_case)]
#[macro_use]
extern crate log;
extern crate libc;

use std::error;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt;
use std::result;
use std::thread;

macro_rules! c {
    ($data:ident) => {
        CString::new($data).unwrap().as_ptr()
    };
    ($expr:expr) => {
        CString::new($expr).unwrap().as_ptr()
    };
}

pub(crate) mod ffi;

use ffi::catClientDestroy;
use ffi::catClientInitWithConfig;
use ffi::catVersion;
use ffi::CatClientConfig;
use ffi::_CatTransaction;
use ffi::newTransaction;
use ffi::DEFAULT_CCAT_CONFIG;

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
            config: unsafe { DEFAULT_CCAT_CONFIG },
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
                CString::new(self.appkey.clone()).unwrap().as_ptr(),
                &mut self.config,
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
        unsafe { CStr::from_ptr(catVersion()).to_str().unwrap() }
    }
}

pub struct CatTransaction(thread::JoinHandle<()>);

impl CatTransaction {
    pub fn new<T: ToString>(type_: T, name: T) -> Self {
        unsafe {
            let t = type_.to_string();
            let n = name.to_string();
            let tr_handle: thread::JoinHandle<()> = thread::Builder::new()
                .spawn(|| {
                    let tr = newTransaction(c!(t), c!(n));
                    if tr.is_null() {
                        error!("create transaction failed!");
                        panic!("create transaction failed!")
                    } else {
                        if let Some(complete) = (*tr).complete {
                            debug!("completing this transaction");
                            complete(tr);
                        } else {
                            error!("transaction's complete method is missing");
                        }
                    }
                })
                .unwrap();
            CatTransaction(tr_handle)
        }
    }

    pub fn complete(self) {
        debug!("fake complete");
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
        ffi::logEvent(
            c!(type_.to_string()),
            c!(name_.to_string()),
            c!(status.to_string()),
            c!(data.to_string()),
        )
    }
}
