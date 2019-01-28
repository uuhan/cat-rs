#![allow(non_snake_case, non_camel_case_types)]

extern crate libc;
use libc::{c_char, c_int, c_ulong};
use std::default::Default;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt::{self, Display};

type cstring = *const c_char;

#[macro_export]
macro_rules! c {
    ($data:ident) => {
        CString::new($data).unwrap().as_ptr()
    };
}

#[repr(C)]
pub struct CatMessage {
    addData: extern "C" fn(*const CatMessage, cstring),
    addKV: extern "C" fn(*const CatMessage, cstring, cstring),
    setStatus: extern "C" fn(*const CatMessage, cstring),
    setTimestamp: extern "C" fn(*const CatMessage, c_ulong),
    complete: extern "C" fn(*const CatMessage),
}

impl CatMessage {
    pub fn add_data(&self, data: String) -> &Self {
        (self.addData)(self, c!(data));
        self
    }

    pub fn add_kv(&self, dataKey: String, dataValue: String) -> &Self {
        (self.addKV)(self, c!(dataKey), c!(dataValue));
        self
    }

    pub fn set_status(&self, status: String) -> &Self {
        (self.setStatus)(self, c!(status));
        self
    }

    pub fn set_timestamp(&self, timestamp: u64) -> &Self {
        (self.setTimestamp)(self, timestamp as c_ulong);
        self
    }

    pub fn complete(&self) {
        (self.complete)(self)
    }
}

pub type CatEvent = CatMessage;
pub type CatMetric = CatMessage;
pub type CatHeartBeat = CatMessage;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CatTransaction {
    addData: extern "C" fn(*const CatTransaction, cstring),
    addKV: extern "C" fn(*const CatTransaction, cstring, cstring),
    setStatus: extern "C" fn(*const CatTransaction, cstring),
    setTimestamp: extern "C" fn(*const CatTransaction, c_ulong),
    complete: extern "C" fn(*const CatTransaction),
    addChild: extern "C" fn(*const CatTransaction, *const CatMessage),
    setDurationInMillis: extern "C" fn(*const CatTransaction, c_ulong),
    setDurationStart: extern "C" fn(*const CatTransaction, c_ulong),
}

impl CatTransaction {
    pub fn new(r#type: String, name: String) -> Self {
        unsafe { *newTransaction(c!(r#type), c!(name)) }
    }

    pub fn add_data(&self, data: String) -> &Self {
        (self.addData)(self, c!(data));
        self
    }

    pub fn add_kv(&self, dataKey: String, dataValue: String) -> &Self {
        (self.addKV)(self, c!(dataKey), c!(dataValue));
        self
    }

    pub fn set_status(&self, status: String) -> &Self {
        (self.setStatus)(self, c!(status));
        self
    }

    pub fn set_timestamp(&self, timestamp: u64) -> &Self {
        (self.setTimestamp)(self, timestamp);
        self
    }

    pub fn complete(&self) {
        (self.complete)(self)
    }

    pub fn add_child(&self, child: &CatMessage) -> &Self {
        (self.addChild)(self, child);
        self
    }

    pub fn set_duration_in_millis(&self, duration: u64) -> &Self {
        (self.setDurationInMillis)(self, duration);
        self
    }

    pub fn set_duration_start(&self, durationStart: u64) -> &Self {
        (self.setDurationStart)(self, durationStart);
        self
    }
}

pub struct CatClient {
    appkey: String,
    config: CatClientConfig,
}

impl CatClient {
    pub fn new(appkey: String) -> Self {
        CatClient {
            appkey,
            config: CatClientConfig::default(),
        }
    }

    pub fn init(&self, config: Option<CatClientConfig>) -> &Self {
        match config {
            Some(config) => unsafe {
                let appkey = CString::new(self.appkey.clone()).unwrap().as_ptr();
                info!("cat client <{}> init with config: {}", self.appkey, config);
                let rc = catClientInitWithConfig(appkey, &config);
                if rc != 0 {
                    info!("success!")
                } else {
                    error!("failed!")
                }
            },
            None => unsafe {
                let appkey = CString::new(self.appkey.clone()).unwrap().as_ptr();
                info!(
                    "cat client <{}> init with config: {}",
                    self.appkey, &self.config
                );
                let rc = catClientInitWithConfig(appkey, &self.config);
                if rc != 0 {
                    info!("success!")
                } else {
                    error!("failed!")
                }
            },
        }

        self
    }

    pub fn destroy(&self) {
        warn!("cat client is being destroyed!");
        let rc = unsafe { catClientDestroy() };
        if rc != 0 {
            warn!("cat is destroyed successfully!")
        } else {
            error!("cat is destroyed failed!")
        }
    }

    pub fn version(&self) -> &str {
        let version = unsafe { CStr::from_ptr(catVersion()) };
        version.to_str().unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct CatClientConfig {
    pub encoder_type: c_int,
    pub encode_heartbeat: c_int,
    pub encode_sampling: c_int,
    pub enable_multiprocessing: c_int,
    pub enable_debug_log: c_int,
}

impl Default for CatClientConfig {
    fn default() -> Self {
        unsafe { CCAT_CONFIG }
    }
}

impl Display for CatClientConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[allow(dead_code)]
extern "C" {
    #[link_name = "DEFAULT_CCAT_CONFIG"]
    static CCAT_CONFIG: CatClientConfig;

    pub fn catClientInit(appKey: cstring) -> c_int;
    pub fn catClientInitWithConfig(appkey: cstring, config: *const CatClientConfig) -> c_int;
    pub fn catClientDestroy() -> c_int;

    pub fn catVersion() -> *mut c_char;
    pub fn isCatEnabled() -> c_int;

    pub fn newTransaction(r#type: cstring, name: cstring) -> *mut CatTransaction;
    pub fn newTransactionWithDuration(
        r#type: cstring,
        name: cstring,
        duration: c_ulong,
    ) -> *const CatTransaction;
    pub fn newCompletedTransactionWithDuration(r#type: cstring, name: cstring, duration: c_ulong);

    pub fn logEvent(r#type: cstring, name: cstring, status: cstring, data: cstring);
    pub fn logError(msg: cstring, errStr: cstring);
    pub fn newEvent(r#type: cstring, name: cstring) -> *const CatEvent;
    pub fn newHeartBeat(r#type: cstring, name: cstring) -> *const CatHeartBeat;

    pub fn logMetricForCount(name: cstring, quantity: c_int);
    pub fn logMEtricForDuration(name: cstring, duration: c_ulong);

    pub fn createMessageId() -> cstring;
    pub fn createRemoteServerMessageId(appKey: cstring) -> cstring;

    pub fn getThreadLocalMessageTreeId() -> cstring;
    pub fn getThreadLocalMessageTreeRootId() -> cstring;
    pub fn getThreadLocalMessageTreeParentId() -> cstring;

    pub fn setThreadLocalMessageTreeId(messageId: cstring);
    pub fn setThreadLocalMessageTreeRootId(messageId: cstring);
    pub fn setThreadLocalMessageTreeParentId(messageId: cstring);
}
