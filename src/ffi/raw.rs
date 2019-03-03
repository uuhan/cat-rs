use super::newTransaction;
use libc::{c_char, c_int};
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

#[derive(Copy)]
#[repr(C)]
pub struct CATStaticQueue {
    pub maxQueueSize: usize,
    pub head: i32,
    pub tail: i32,
    pub size: usize,
    pub valueArray: [*mut ::std::os::raw::c_void; 0],
}

impl Clone for CATStaticQueue {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub union MessageType {
    pub type_: c_char,
    pub flag_: c_int,
}

impl Clone for MessageType {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct CatMessageInner {
    pub messageType: MessageType,
    pub type_: *mut u8,
    pub name: *mut u8,
    pub status: *mut u8,
    pub data: *mut u8,
    pub timestampMs: usize,
    pub isComplete: i32,
}

impl Clone for CatMessageInner {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct CatTransactionInner {
    pub children: *mut CATStaticQueue,
    pub durationStart: usize,
    pub durationUs: usize,
    pub message: CatMessageInner,
}

impl Clone for CatTransactionInner {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct CatEncoder {
    pub setAppkey: unsafe extern "C" fn(*mut CatEncoder, *const u8),
    pub setHostname: unsafe extern "C" fn(*mut CatEncoder, *const u8),
    pub setIp: unsafe extern "C" fn(*mut CatEncoder, *const u8),
    pub header: unsafe extern "C" fn(*mut CatEncoder, *mut CatMessageTree),
    pub message: unsafe extern "C" fn(*mut CatEncoder, *mut CatMessage),
    pub transactionStart: unsafe extern "C" fn(*mut CatEncoder, *mut CatTransaction),
    pub transactionEnd: unsafe extern "C" fn(*mut CatEncoder, *mut CatTransaction),
    pub transaction: unsafe extern "C" fn(*mut CatEncoder, *mut CatTransaction),
    pub event: unsafe extern "C" fn(*mut CatEncoder, *mut CatMessage),
    pub metric: unsafe extern "C" fn(*mut CatEncoder, *mut CatMessage),
    pub heartbeat: unsafe extern "C" fn(*mut CatEncoder, *mut CatMessage),
    pub ip: *const u8,
    pub hostname: *const u8,
    pub appkey: *const u8,
    pub buf: *mut *mut u8,
}

impl Clone for CatEncoder {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct CatMessageManager {
    pub domain: *mut u8,
    pub hostname: *mut u8,
    pub ip: *mut u8,
    pub ipHex: *mut u8,
    pub throttleTimes: isize,
}

impl Clone for CatMessageManager {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct CatContext {
    pub tree: *mut CatMessageTree,
    pub transactionStack: *mut CATStaticQueue,
    pub elementSize: i32,
    pub lastTruncateTransDurationUs: usize,
    pub addMessage: unsafe extern "C" fn(*mut CatContext, *mut CatMessage),
    pub addTransChild: unsafe extern "C" fn(*mut CatContext, *mut CatMessage, *mut CatTransaction),
    pub adjustForTruncatedTrans: unsafe extern "C" fn(*mut CatContext, *mut CatTransaction),
    pub startTrans: unsafe extern "C" fn(*mut CatContext, *mut CatTransaction),
    pub endTrans: unsafe extern "C" fn(*mut CatContext, *mut CatTransaction),
    pub reset: unsafe extern "C" fn(*mut CatContext),
}

impl Clone for CatContext {
    fn clone(&self) -> Self {
        *self
    }
}
