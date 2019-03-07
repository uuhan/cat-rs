#![allow(non_snake_case)]
#[macro_use]
extern crate log;
extern crate libc;
extern crate num_cpus;
extern crate threadpool;

use std::error;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt;
use std::result;
use std::sync::mpsc;

use threadpool::ThreadPool;

macro_rules! c {
    ($data:ident) => {
        CString::new($data).unwrap().as_ptr()
    };
    ($expr:expr) => {
        CString::new($expr).unwrap().as_ptr()
    };
}

thread_local!(
    static POOL: ThreadPool = ThreadPool::new(num_cpus::get()*2)
);

pub(crate) mod ffi;

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
                CString::new(self.appkey.clone()).unwrap().as_ptr(),
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
}

impl CatTransaction {
    pub fn new<T: ToString>(_type: T, _name: T) -> Self {
        let (sender, receiver) = mpsc::channel::<CatMessage>();
        let _type = _type.to_string();
        let _name = _name.to_string();
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
                        let message = receiver.recv().unwrap();

                        match message {
                            // TODO: inner transaction
                            CatMessage::Transaction(name) => {
                                let tr = unsafe { newTransaction(c!(_type.clone()), c!(name)) };
                                if !tr.is_null() {
                                    if let Some(complete) = unsafe { (*tr).complete } {
                                        unsafe {
                                            complete(tr);
                                        };
                                    } else {
                                        error!("transaction's complete method is missing");
                                    }
                                }
                            }
                            CatMessage::LogEvent(type_, name, status, data) => {
                                logEvent(type_, name, status, data)
                            }
                            CatMessage::CompleteThis => {
                                break 'trans;
                            }
                        }
                    }

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
        CatTransaction { sender }
    }

    pub fn complete(&self) {
        self.sender.send(CatMessage::CompleteThis).unwrap()
    }

    pub fn log<T: ToString>(&self, type_: T, name: T, status: T, data: T) {
        self.sender
            .send(CatMessage::LogEvent(
                type_.to_string(),
                name.to_string(),
                status.to_string(),
                data.to_string(),
            ))
            .unwrap()
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
