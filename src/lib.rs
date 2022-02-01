#![allow(non_snake_case)]
//! ## Rust Cat Bindings
//!
//! NB: This crate is meanly mostly created for Nodejs's Native Addons(using neon) currently.
//!
//! ## Usage
//!
//! ```rust,no_run
//! extern crate cat_rs as cat;
//! use cat::{
//!     logEvent,
//!     CatClient,
//!     CatTransaction,
//! };
//!
//! let mut cat = CatClient::new("test");
//! cat.init().unwrap();
//! let mut tr = CatTransaction::new("foo", "bar");
//! tr.log("test", "it", "0", "");
//! tr.complete();
//! ```
#[macro_use]
extern crate log;
extern crate libc;
extern crate num_cpus;
extern crate threadpool;

use std::error;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt;
use std::os::raw::{c_int, c_ulonglong};
use std::result;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use threadpool::ThreadPool;

thread_local!(
    static POOL: ThreadPool = ThreadPool::new(num_cpus::get())
);

pub(crate) mod ffi;
pub(crate) mod mac;

use ffi::catClientDestroy;
use ffi::catClientInitWithConfig;
use ffi::catVersion;
use ffi::newTransaction;
use ffi::CatClientConfig;
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
    pub fn config(&mut self, config: &mut CatClientConfig) -> &mut Self {
        self.config = *config;
        self
    }

    /// initialize cat client
    pub fn init(&mut self) -> Result<&mut Self> {
        let rc = unsafe {
            catClientInitWithConfig(
                CString::new(self.appkey.clone()).unwrap().into_raw(),
                &mut self.config,
            )
        };

        if rc == 0 {
            error!("{}", CatError::CatClientInitError);
            Err(CatError::CatClientInitError)
        } else {
            Ok(self)
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

pub enum CatMessage {
    LogEvent(String, String, String, String),
    Transaction(String),
    CompleteThis,
}

pub struct CatTransaction {
    sender: mpsc::Sender<CatMessage>,
    open: Arc<Mutex<bool>>,
}

impl CatTransaction {
    pub fn new<T: ToString>(_type: T, _name: T) -> Self {
        let (sender, receiver) = mpsc::channel::<CatMessage>();
        let _type = _type.to_string();
        let _name = _name.to_string();
        let _open = Arc::new(Mutex::new(true));
        let _open_keep = _open.clone();
        POOL.with(|pool| {
            pool.execute(move || {
                debug!("create a new transaction: {} / {}", _type, _name);
                let tr = unsafe { newTransaction(c!(_type.clone()), c!(_name)) };

                if tr.is_null() {
                    error!("create transaction failed!");
                    panic!("create transaction failed!")
                } else {
                    // loop in this thread as is this root transaction
                    'trans: loop {
                        match receiver.recv() {
                            Ok(message) => {
                                match message {
                                    // TODO: inner transaction
                                    CatMessage::Transaction(_name) => {}

                                    CatMessage::LogEvent(type_, name, status, data) => {
                                        logEvent(type_, name, status, data)
                                    }

                                    CatMessage::CompleteThis => {
                                        break 'trans;
                                    }
                                }
                            }
                            Err(err) => {
                                error!("receive job failed, err: {}", err);
                                break 'trans;
                            }
                        }
                    }

                    let _open_guard = _open.clone();
                    let mut v = _open_guard.try_lock().unwrap();
                    *v = false;

                    if let Some(complete) = unsafe { (*tr).complete } {
                        debug!("complete this transaction");
                        unsafe {
                            complete(tr);
                        };
                    } else {
                        error!("transaction's complete method is missing");
                    }
                }
            });
        });
        CatTransaction {
            sender,
            open: _open_keep,
        }
    }

    pub fn complete(&mut self) {
        let _open_guard = self.open.clone();
        if *_open_guard.try_lock().unwrap() {
            self.sender
                .send(CatMessage::CompleteThis)
                .map_err(|e| {
                    error!("complete transaction error: {}", e);
                })
                .unwrap()
        } else {
            warn!("complete a closed transaction");
        }
    }

    pub fn log<T: ToString>(&mut self, type_: T, name: T, status: T, data: T) {
        let _open_guard = self.open.clone();
        if *_open_guard.try_lock().unwrap() {
            match self
                .sender
                .send(CatMessage::LogEvent(
                    type_.to_string(),
                    name.to_string(),
                    status.to_string(),
                    data.to_string(),
                ))
                .map_err(|e| {
                    error!("log event error: {}", e);
                }) {
                Ok(_) => {}
                Err(e) => error!("log error: {:?}", e),
            }
        } else {
            warn!("log event on a closed transaction");
        }
    }
}

/// log a cat event
///
/// # Arguments
///
/// * `type_` - event type
/// * `name_` - event name
/// * `status` - event status type "0" or other
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

pub fn newHeartBeat<S: ToString>(_type: S, _name: S) {
    info!(
        "start a new heart beat: {} {}",
        _type.to_string(),
        _name.to_string(),
    );
    unsafe {
        ffi::newHeartBeat(c!(_type.to_string()), c!(_name.to_string()));
    }
}

/// Metric Apis
///
/// # logMetricForCount
///
pub fn logMetricForCount<S: ToString>(name: S, quantity: i32) {
    info!("logMetricForCount: {} {}", name.to_string(), quantity);

    unsafe {
        ffi::logMetricForCount(c!(name.to_string()), quantity as c_int);
    }
}

/// Metric Apis
///
/// #logMetricForDuration
///
pub fn logMetricForDuration<S: ToString>(name: S, duration: u64) {
    info!("logMetricForDuration: {} {}", name.to_string(), duration);
    unsafe {
        ffi::logMetricForDuration(c!(name.to_string()), duration as c_ulonglong);
    }
}
