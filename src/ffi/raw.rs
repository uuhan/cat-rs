use super::newTransaction;
use std::default::Default;
use std::ffi::CString;
use std::fmt::{self, Display};

extern "C" {
    static mut g_cat_nullTrans: CatTransaction;
    static mut g_cat_nullMsg: CatMessage;
}

#[derive(Copy)]
#[repr(C)]
pub struct CatClientConfig {
    pub encoderType: i32,
    pub enableHeartbeat: i32,
    pub enableSampling: i32,
    pub enableMultiprocessing: i32,
    pub enableDebugLog: i32,
}

impl Clone for CatClientConfig {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for CatClientConfig {
    fn default() -> Self {
        CatClientConfig {
            encoderType: 1i32,
            enableHeartbeat: 0i32,
            enableSampling: 1i32,
            enableMultiprocessing: 0i32,
            enableDebugLog: 0i32,
        }
    }
}

impl Display for CatClientConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct CatClientInnerConfig {
    pub appkey: *mut u8,
    pub selfHost: *mut u8,
    pub serverHost: *mut u8,
    pub defaultIp: *mut u8,
    pub defaultIpHex: *mut u8,
    pub serverPort: u32,
    pub serverNum: i32,
    pub serverAddresses: *mut *mut u8,
    pub messageEnableFlag: i32,
    pub messageQueueSize: i32,
    pub messageQueueBlockPrintCount: i32,
    pub maxChildSize: i32,
    pub maxContextElementSize: i32,
    pub logFlag: i32,
    pub logSaveFlag: i32,
    pub logDebugFlag: i32,
    pub logFileWithTime: i32,
    pub logFilePerDay: i32,
    pub logLevel: i32,
    pub configDir: *mut u8,
    pub dataDir: *mut u8,
    pub indexFileName: *mut u8,
    pub encoderType: i32,
    pub enableHeartbeat: i32,
    pub enableSampling: i32,
    pub enableMultiprocessing: i32,
}

impl Clone for CatClientInnerConfig {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct CatMessage {
    pub addData: unsafe extern "C" fn(*mut CatMessage, *const u8),
    pub addKV: unsafe extern "C" fn(*mut CatMessage, *const u8, *const u8),
    pub setStatus: unsafe extern "C" fn(*mut CatMessage, *const u8),
    pub setTimestamp: unsafe extern "C" fn(*mut CatMessage, usize),
    pub complete: unsafe extern "C" fn(*mut CatMessage),
}

impl Clone for CatMessage {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for CatMessage {
    fn default() -> Self {
        unsafe { g_cat_nullMsg }
    }
}

/// TODO: CatMessage
impl CatMessage {
    pub fn add_data(&mut self, data: String) -> &Self {
        unsafe { (self.addData)(self, c!(data)) };
        self
    }

    pub fn add_kv(&mut self, dataKey: String, dataValue: String) -> &Self {
        unsafe { (self.addKV)(self, c!(dataKey), c!(dataValue)) };
        self
    }

    pub fn set_status(&mut self, status: String) -> &Self {
        unsafe { (self.setStatus)(self, c!(status)) };
        self
    }

    pub fn set_timestamp(&mut self, timestamp: u64) -> &Self {
        unsafe { (self.setTimestamp)(self, timestamp as usize) };
        self
    }

    pub fn complete(&mut self) {
        unsafe { (self.complete)(self) }
    }
}

pub type CatEvent = CatMessage;
pub type CatMetric = CatMessage;
pub type CatHeartBeat = CatMessage;

#[derive(Copy)]
#[repr(C)]
pub struct CatMessageTree {
    pub root: *mut CatMessage,
    pub messageId: *mut u8,
    pub parentMessageId: *mut u8,
    pub rootMessageId: *mut u8,
    pub sessionToken: *mut u8,
    pub threadGroupName: *mut u8,
    pub threadId: *mut u8,
    pub threadName: *mut u8,
    pub canDiscard: i32,
}

impl Clone for CatMessageTree {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct CatTransaction {
    pub addData: unsafe extern "C" fn(*mut CatTransaction, *const u8),
    pub addKV: unsafe extern "C" fn(*mut CatTransaction, *const u8, *const u8),
    pub setStatus: unsafe extern "C" fn(*mut CatTransaction, *const u8),
    pub setTimestamp: unsafe extern "C" fn(*mut CatTransaction, usize),
    pub complete: unsafe extern "C" fn(*mut CatTransaction),
    pub addChild: unsafe extern "C" fn(*mut CatTransaction, *mut CatMessage),
    pub setDurationInMillis: unsafe extern "C" fn(*mut CatTransaction, usize),
    pub setDurationStart: unsafe extern "C" fn(*mut CatTransaction, usize),
}

impl Clone for CatTransaction {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for CatTransaction {
    fn default() -> Self {
        unsafe { g_cat_nullTrans }
    }
}

impl CatTransaction {
    pub fn new<T: ToString>(r#type: T, name: T) -> *mut Self {
        unsafe { newTransaction(r#type.to_string(), name.to_string()) }
    }

    pub fn add_data(&mut self, data: String) -> &Self {
        unsafe { (self.addData)(self, c!(data)) };
        self
    }

    pub fn add_kv(&mut self, dataKey: String, dataValue: String) -> &Self {
        unsafe { (self.addKV)(self, c!(dataKey), c!(dataValue)) };
        self
    }

    pub fn set_status(&mut self, status: String) -> &Self {
        unsafe { (self.setStatus)(self, c!(status)) };
        self
    }

    pub fn set_timestamp(&mut self, timestamp: usize) -> &Self {
        unsafe { (self.setTimestamp)(self, timestamp) };
        self
    }

    pub fn complete(&mut self) {
        unsafe { (self.complete)(self) }
    }

    pub fn add_child(&mut self, child: &mut CatMessage) -> &Self {
        unsafe { (self.addChild)(self, child) };
        self
    }

    pub fn set_duration_in_millis(&mut self, duration: usize) -> &Self {
        unsafe { (self.setDurationInMillis)(self, duration) };
        self
    }

    pub fn set_duration_start(&mut self, durationStart: usize) -> &Self {
        unsafe { (self.setDurationStart)(self, durationStart) };
        self
    }
}
