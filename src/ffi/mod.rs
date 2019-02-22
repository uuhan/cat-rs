#![allow(non_snake_case, non_camel_case_types)]

extern crate libc;
mod client_config;

use client_config::catChecktPtrWithName;

use libc::{gettimeofday, sighandler_t, signal, timeval, SIGINT, SIGPIPE, SIG_IGN};

use std::default::Default;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt::{self, Display};
use std::mem;

type cstring = *const u8;

/// cat static
static mut g_cat_init: i32 = 0i32;

static mut DEFAULT_CCAT_CONFIG: _CatClientConfig = _CatClientConfig {
    encoderType: 1i32,
    enableHeartbeat: 1i32,
    enableSampling: 1i32,
    enableMultiprocessing: 0i32,
    enableDebugLog: 1i32,
};

#[macro_export]
macro_rules! c {
    ($data:ident) => {
        CString::new($data).unwrap().as_ptr() as *const u8
    };
    ($expr:expr) => {
        CString::new($expr).unwrap().as_ptr() as *const u8
    };
}

#[allow(dead_code)]
extern "C" {
    #[link_name = "DEFAULT_CCAT_CONFIG"]
    static CCAT_CONFIG: CatClientConfig;

    fn CLogLogWithLocation(
        type_: u16,
        format: *const u8,
        file: *const u8,
        line: i32,
        function: *const u8,
        ...
    );

    /// __sync_add_and_fetch
    fn addCountMetricToAggregator(name: *const u8, count: i32);
    fn addDurationMetricToAggregator(name: *const u8, timeMs: i32);

    // fn catChecktPtrWithName(ptr: *mut ::std::os::raw::c_void, ptrName: *mut u8);
    fn catMessageManagerDestroy();
    fn catMessageManagerStartTrans(trans: *mut _CatTransaction);

    /// sds
    fn catsdsfree(s: *mut u8);
    fn catsdsfromlonglong(value: isize) -> *mut u8;
    fn catsdsnew(init: *const u8) -> *mut u8;

    /// __sync_add_and_fetch {0}
    fn clearCatAggregatorThread();
    fn clearCatClientConfig();
    fn clearCatMonitor();
    fn clearCatSenderThread();
    fn clearCatServerConnManager();

    fn createCatEvent(type_: *const u8, name: *const u8) -> *mut _CatMessage;
    fn createCatHeartBeat(type_: *const u8, name: *const u8) -> *mut _CatMessage;
    fn createCatMetric(type_: *const u8, name: *const u8) -> *mut _CatMessage;
    fn createCatTransaction(type_: *const u8, name: *const u8) -> *mut _CatTransaction;
    fn destroyMessageIdHelper();
    static mut g_cat_enabledFlag: i32;
    static mut g_cat_nullMsg: _CatMessage;
    static mut g_cat_nullTrans: _CatTransaction;
    static mut g_config: _CatClientInnerConfig;
    fn getContextMessageTree() -> *mut _CatMessageTree;
    fn getNextMessageId() -> *mut u8;
    fn getNextMessageIdByAppkey(domain: *const u8) -> *mut u8;
    fn initCatAggregatorThread();
    fn initCatClientConfig(config: *mut _CatClientConfig);
    fn initCatMonitorThread();
    fn initCatSenderThread();
    fn initCatServerConnManager() -> i32;
    fn initMessageIdHelper();
    fn initMessageManager(domain: *const u8, hostName: *const u8);
    pub fn isCatEnabled() -> i32;
    fn loadCatClientConfig(filename: *const u8) -> i32;
}

pub fn catVersion() -> &'static str {
    "3.0.1"
}

pub unsafe fn createMessageId() -> *mut u8 {
    if isCatEnabled() == 0 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        getNextMessageId()
    }
}

pub unsafe fn createRemoteServerMessageId(mut appkey: *const u8) -> *mut u8 {
    if isCatEnabled() == 0 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        getNextMessageIdByAppkey(appkey)
    }
}

pub unsafe fn getThreadLocalMessageTreeId() -> *mut u8 {
    if isCatEnabled() == 0 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        (*getContextMessageTree()).messageId
    }
}

pub unsafe fn getThreadLocalMessageTreeRootId() -> *mut u8 {
    if isCatEnabled() == 0 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        (*getContextMessageTree()).rootMessageId
    }
}

pub unsafe fn getThreadLocalMessageTreeParentId() -> *mut u8 {
    if isCatEnabled() == 0 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        (*getContextMessageTree()).parentMessageId
    }
}

pub unsafe fn setThreadLocalMessageTreeId(mut messageId: *mut u8) {
    if isCatEnabled() == 0 {
    } else {
        let mut pTree: *mut _CatMessageTree = getContextMessageTree();
        if (*pTree).messageId != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            catsdsfree((*pTree).messageId);
            (*pTree).messageId = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
        (*pTree).messageId = catsdsnew(messageId as (*const u8));
    }
}

pub unsafe fn setThreadLocalMessageTreeRootId(mut messageId: *mut u8) {
    if isCatEnabled() == 0 {
    } else {
        let mut pTree: *mut _CatMessageTree = getContextMessageTree();
        if (*pTree).rootMessageId != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            catsdsfree((*pTree).rootMessageId);
            (*pTree).rootMessageId = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
        (*pTree).rootMessageId = catsdsnew(messageId as (*const u8));
    }
}

#[no_mangle]
pub unsafe extern "C" fn setThreadLocalMessageTreeParentId(mut messageId: *mut u8) {
    if isCatEnabled() == 0 {
    } else {
        let mut pTree: *mut _CatMessageTree = getContextMessageTree();
        if (*pTree).parentMessageId != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            catsdsfree((*pTree).parentMessageId);
            (*pTree).parentMessageId = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
        (*pTree).parentMessageId = catsdsnew(messageId as (*const u8));
    }
}

#[no_mangle]
pub unsafe fn catClientInitWithConfig(
    mut appkey: *const u8,
    mut config: *mut _CatClientConfig,
) -> i32 {
    if g_cat_init != 0 {
        0i32
    } else {
        g_cat_init = 1i32;
        signal(SIGPIPE, SIG_IGN);
        initCatClientConfig(config);
        (if loadCatClientConfig((*b"/data/appdatas/cat/client.xml\0").as_ptr()) < 0i32 {
            g_cat_init = 0i32;
            g_cat_enabledFlag = 0i32;
            CLogLogWithLocation(
                0x8u16,
                (*b"Failed to initialize cat: Error occurred while loading client config.\0")
                    .as_ptr(),
                file!().as_ptr(),
                line!() as (i32),
                (*b"catClientInitWithConfig\0").as_ptr(),
            );
            0i32
        } else {
            g_config.appkey = catsdsnew(appkey);
            initMessageManager(appkey, g_config.selfHost as (*const u8));
            initMessageIdHelper();
            (if initCatServerConnManager() == 0 {
                g_cat_init = 0i32;
                g_cat_enabledFlag = 0i32;
                CLogLogWithLocation(
                    0x8u16,
                    (*b"Failed to initialize cat: Error occurred while getting router from remote server.\0").as_ptr(
                    ),
                    file!().as_ptr(),
                    line!() as (i32),
                    (*b"catClientInitWithConfig\0").as_ptr()
                );
                0i32
            } else {
                initCatAggregatorThread();
                initCatSenderThread();
                initCatMonitorThread();
                g_cat_enabledFlag = 1i32;
                CLogLogWithLocation(
                    0x2u16,
                    (*b"Cat has been successfully initialized with appkey: %s\0").as_ptr(),
                    file!().as_ptr(),
                    line!() as (i32),
                    (*b"catClientInitWithConfig\0").as_ptr(),
                    appkey,
                );
                1i32
            })
        })
    }
}

pub unsafe fn catClientInit(mut appkey: *const u8) -> i32 {
    catClientInitWithConfig(appkey, &mut DEFAULT_CCAT_CONFIG as (*mut _CatClientConfig))
}

pub unsafe fn catClientDestroy() -> i32 {
    g_cat_enabledFlag = 0i32;
    g_cat_init = 0i32;
    clearCatMonitor();
    catMessageManagerDestroy();
    clearCatAggregatorThread();
    clearCatSenderThread();
    clearCatServerConnManager();
    destroyMessageIdHelper();
    clearCatClientConfig();
    1i32
}

pub unsafe fn newTransaction(mut type_: *const u8, mut name: *const u8) -> *mut CatTransaction {
    if isCatEnabled() == 0 {
        &mut g_cat_nullTrans as (*mut _CatTransaction)
    } else {
        let mut trans: *mut _CatTransaction = createCatTransaction(type_, name);
        catChecktPtrWithName(
            trans as (*mut ::std::os::raw::c_void),
            (*b"trans\0").as_ptr() as (*mut u8),
        );
        (if trans == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _CatTransaction) {
            0i32 as (*mut ::std::os::raw::c_void) as (*mut _CatTransaction)
        } else {
            catMessageManagerStartTrans(trans);
            trans
        })
    }
}

pub unsafe fn GetTime64() -> usize {
    let mut buf: usize;
    let mut tv: timeval = mem::uninitialized();
    gettimeofday(&mut tv, 0i32 as (*mut ::std::os::raw::c_void));
    buf = (tv.tv_sec * 1000i64 + (tv.tv_usec / 1000i32) as (i64)) as (usize);
    buf
}

pub unsafe fn newTransactionWithDuration(
    mut type_: *const u8,
    mut name: *const u8,
    mut duration: usize,
) -> *mut CatTransaction {
    let mut trans: *mut _CatTransaction = newTransaction(type_, name);
    ((*trans).setDurationInMillis)(trans, duration);
    if duration < (60i32 * 1000i32) as (usize) {
        ((*trans).setTimestamp)(trans, GetTime64().wrapping_sub(duration));
    }
    trans
}

pub unsafe fn newCompletedTransactionWithDuration(
    mut type_: *const u8,
    mut name: *const u8,
    mut duration: usize,
) {
    let mut trans: *mut _CatTransaction = newTransactionWithDuration(type_, name, duration);
    ((*trans).complete)(trans);
}

pub unsafe fn newHeartBeat(mut type_: *const u8, mut name: *const u8) -> *mut _CatMessage {
    if isCatEnabled() == 0 {
        &mut g_cat_nullMsg as (*mut _CatMessage)
    } else {
        (*getContextMessageTree()).canDiscard = 0i32;
        let mut hb: *mut _CatMessage = createCatHeartBeat(type_, name);
        catChecktPtrWithName(
            hb as (*mut ::std::os::raw::c_void),
            (*b"hb\0").as_ptr() as (*mut u8),
        );
        hb
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct _CatMessageTree {
    pub root: *mut _CatMessage,
    pub messageId: *mut u8,
    pub parentMessageId: *mut u8,
    pub rootMessageId: *mut u8,
    pub sessionToken: *mut u8,
    pub threadGroupName: *mut u8,
    pub threadId: *mut u8,
    pub threadName: *mut u8,
    pub canDiscard: i32,
}

impl Clone for _CatMessageTree {
    fn clone(&self) -> Self {
        *self
    }
}

pub type CatMessageTree = _CatMessageTree;

pub unsafe fn logError(mut msg: *const u8, mut errStr: *const u8) {
    (*getContextMessageTree()).canDiscard = 0i32;
    logEvent(
        (*b"Exception\0").as_ptr(),
        msg,
        (*b"ERROR\0").as_ptr(),
        errStr,
    );
}

pub unsafe fn newEvent(mut type_: *const u8, mut name: *const u8) -> *mut _CatMessage {
    if isCatEnabled() == 0 {
        &mut g_cat_nullMsg as (*mut _CatMessage)
    } else {
        let mut event: *mut _CatMessage = createCatEvent(type_, name);
        catChecktPtrWithName(
            event as (*mut ::std::os::raw::c_void),
            (*b"event\0").as_ptr() as (*mut u8),
        );
        event
    }
}

pub unsafe fn logEvent(
    mut type_: *const u8,
    mut name: *const u8,
    mut status: *const u8,
    mut data: *const u8,
) {
    if isCatEnabled() == 0 {
    } else {
        let mut event: *mut _CatMessage = newEvent(type_, name);
        catChecktPtrWithName(
            event as (*mut ::std::os::raw::c_void),
            (*b"event\0").as_ptr() as (*mut u8),
        );
        (if event == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _CatMessage) {
        } else {
            if data != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
                ((*event).addData)(event, data);
            }
            ((*event).setStatus)(event, status);
            ((*event).complete)(event);
        })
    }
}

pub unsafe fn newMetric(mut type_: *const u8, mut name: *const u8) -> *mut _CatMessage {
    if isCatEnabled() == 0 {
        &mut g_cat_nullMsg as (*mut _CatMessage)
    } else {
        let mut metric: *mut _CatMessage = createCatMetric(type_, name);
        catChecktPtrWithName(
            metric as (*mut ::std::os::raw::c_void),
            (*b"metric\0").as_ptr() as (*mut u8),
        );
        metric
    }
}

pub unsafe fn _logMetric(mut name: *const u8, mut status: *const u8, mut value: *const u8) {
    let mut metric: *mut _CatMessage = newMetric((*b"\0").as_ptr(), name);
    catChecktPtrWithName(
        metric as (*mut ::std::os::raw::c_void),
        (*b"metric\0").as_ptr() as (*mut u8),
    );
    if value != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
        ((*metric).addData)(metric, value);
    }
    ((*metric).setStatus)(metric, status);
    ((*metric).complete)(metric);
}

pub unsafe fn logMetricForCount(mut name: *const u8, mut quantity: i32) {
    if isCatEnabled() == 0 {
    } else if g_config.enableSampling != 0 {
        addCountMetricToAggregator(name, quantity);
    } else if quantity == 1i32 {
        _logMetric(name, (*b"C\0").as_ptr(), (*b"1\0").as_ptr());
    } else {
        let mut val: *mut u8 = catsdsfromlonglong(quantity as (isize));
        catChecktPtrWithName(
            val as (*mut ::std::os::raw::c_void),
            (*b"val\0").as_ptr() as (*mut u8),
        );
        _logMetric(name, (*b"C\0").as_ptr(), val as (*const u8));
        catsdsfree(val);
    }
}

pub unsafe fn logMetricForDuration(mut name: *const u8, mut duration: usize) {
    if isCatEnabled() == 0 {
    } else if g_config.enableSampling != 0 {
        addDurationMetricToAggregator(name, duration as (i32));
    } else {
        let mut val: *mut u8 = catsdsfromlonglong(duration as (isize));
        catChecktPtrWithName(
            val as (*mut ::std::os::raw::c_void),
            (*b"val\0").as_ptr() as (*mut u8),
        );
        _logMetric(name, (*b"T\0").as_ptr(), val as (*const u8));
        catsdsfree(val);
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct _CatMessage {
    pub addData: unsafe extern "C" fn(*mut _CatMessage, *const u8),
    pub addKV: unsafe extern "C" fn(*mut _CatMessage, *const u8, *const u8),
    pub setStatus: unsafe extern "C" fn(*mut _CatMessage, *const u8),
    pub setTimestamp: unsafe extern "C" fn(*mut _CatMessage, usize),
    pub complete: unsafe extern "C" fn(*mut _CatMessage),
}

impl Clone for _CatMessage {
    fn clone(&self) -> Self {
        *self
    }
}

pub type CatMessage = _CatMessage;

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
pub struct _CatTransaction {
    pub addData: unsafe extern "C" fn(*mut _CatTransaction, *const u8),
    pub addKV: unsafe extern "C" fn(*mut _CatTransaction, *const u8, *const u8),
    pub setStatus: unsafe extern "C" fn(*mut _CatTransaction, *const u8),
    pub setTimestamp: unsafe extern "C" fn(*mut _CatTransaction, usize),
    pub complete: unsafe extern "C" fn(*mut _CatTransaction),
    pub addChild: unsafe extern "C" fn(*mut _CatTransaction, *mut _CatMessage),
    pub setDurationInMillis: unsafe extern "C" fn(*mut _CatTransaction, usize),
    pub setDurationStart: unsafe extern "C" fn(*mut _CatTransaction, usize),
}

impl Clone for _CatTransaction {
    fn clone(&self) -> Self {
        *self
    }
}

pub type CatTransaction = _CatTransaction;

impl CatTransaction {
    pub fn new(r#type: String, name: String) -> *mut Self {
        unsafe { newTransaction(c!(r#type), c!(name)) }
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
pub struct _CatClientConfig {
    pub encoderType: i32,
    pub enableHeartbeat: i32,
    pub enableSampling: i32,
    pub enableMultiprocessing: i32,
    pub enableDebugLog: i32,
}

impl Clone for _CatClientConfig {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct _CatClientInnerConfig {
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

impl Clone for _CatClientInnerConfig {
    fn clone(&self) -> Self {
        *self
    }
}

pub type CatClientConfig = _CatClientConfig;

impl Default for CatClientConfig {
    fn default() -> Self {
        unsafe { DEFAULT_CCAT_CONFIG }
    }
}

impl Display for CatClientConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
