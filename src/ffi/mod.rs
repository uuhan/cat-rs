#![allow(non_snake_case, non_camel_case_types, unused)]

extern crate libc;

use libc::{gettimeofday, sighandler_t, signal, timeval, SIGINT, SIGPIPE, SIG_IGN};

use std::default::Default;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt::{self, Display};
use std::mem;
use std::ptr;

mod client_config;
pub(crate) mod raw;

use client_config::initCatClientConfig;
use client_config::loadCatClientConfig;
use raw::CatClientConfig;
use raw::CatClientInnerConfig;

type cstring = *const u8;

/// cat static
static mut G_CAT_INIT: i32 = 0i32;

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
    static mut g_config: CatClientInnerConfig;
    fn getContextMessageTree() -> *mut _CatMessageTree;
    fn getNextMessageId() -> *mut u8;
    fn getNextMessageIdByAppkey(domain: *const u8) -> *mut u8;
    fn initCatAggregatorThread();
    fn initCatMonitorThread();
    fn initCatSenderThread();
    fn initCatServerConnManager() -> i32;
    fn initMessageIdHelper();
    fn initMessageManager(domain: *const u8, hostName: *const u8);
}

#[inline]
pub fn isCatEnabled() -> bool {
    unsafe { g_cat_enabledFlag != 0 }
}

pub unsafe fn createMessageId() -> *mut u8 {
    if isCatEnabled() {
        getNextMessageId()
    } else {
        ptr::null_mut()
    }
}

pub unsafe fn createRemoteServerMessageId(appkey: *const u8) -> *mut u8 {
    if isCatEnabled() {
        getNextMessageIdByAppkey(appkey)
    } else {
        ptr::null_mut()
    }
}

pub unsafe fn getThreadLocalMessageTreeId() -> *mut u8 {
    if isCatEnabled() {
        (*getContextMessageTree()).messageId
    } else {
        ptr::null_mut()
    }
}

pub unsafe fn getThreadLocalMessageTreeRootId() -> *mut u8 {
    if isCatEnabled() {
        (*getContextMessageTree()).rootMessageId
    } else {
        ptr::null_mut()
    }
}

pub unsafe fn getThreadLocalMessageTreeParentId() -> *mut u8 {
    if isCatEnabled() {
        (*getContextMessageTree()).parentMessageId
    } else {
        ptr::null_mut()
    }
}

pub unsafe fn setThreadLocalMessageTreeId(messageId: *mut u8) {
    if isCatEnabled() {
        let mut pTree: *mut _CatMessageTree = getContextMessageTree();
        if !(*pTree).messageId.is_null() {
            catsdsfree((*pTree).messageId);
            (*pTree).messageId = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
        (*pTree).messageId = catsdsnew(messageId as (*const u8));
    } else {
    }
}

pub unsafe fn setThreadLocalMessageTreeRootId(messageId: *mut u8) {
    if isCatEnabled() {
        let mut pTree: *mut _CatMessageTree = getContextMessageTree();
        if !(*pTree).rootMessageId.is_null() {
            catsdsfree((*pTree).rootMessageId);
            (*pTree).rootMessageId = ptr::null_mut();
        }
        (*pTree).rootMessageId = catsdsnew(messageId as (*const u8));
    } else {
    }
}

pub unsafe fn setThreadLocalMessageTreeParentId(messageId: *mut u8) {
    if isCatEnabled() {
        let mut pTree: *mut _CatMessageTree = getContextMessageTree();
        if !(*pTree).parentMessageId.is_null() {
            catsdsfree((*pTree).parentMessageId);
            (*pTree).parentMessageId = ptr::null_mut();
        }
        (*pTree).parentMessageId = catsdsnew(messageId as (*const u8));
    } else {
    }
}

pub unsafe fn catClientInitWithConfig(appkey: *const u8, config: *mut CatClientConfig) -> i32 {
    if G_CAT_INIT != 0 {
        0i32
    } else {
        G_CAT_INIT = 1i32;
        signal(SIGPIPE, SIG_IGN);
        initCatClientConfig(config);
        (if loadCatClientConfig("/data/appdatas/cat/client.xml") < 0i32 {
            G_CAT_INIT = 0;
            g_cat_enabledFlag = 0;
            error!("Failed to initialize cat: Error occurred while loading client config.");
            0
        } else {
            g_config.appkey = catsdsnew(appkey);
            initMessageManager(appkey, g_config.selfHost);
            initMessageIdHelper();
            if initCatServerConnManager() == 0 {
                G_CAT_INIT = 0;
                g_cat_enabledFlag = 0;
                error!("Failed to initialize cat: Error occurred while getting router from remote server.");
                0
            } else {
                initCatAggregatorThread();
                initCatSenderThread();
                initCatMonitorThread();
                g_cat_enabledFlag = 1;
                1
            }
        })
    }
}

pub unsafe fn catClientInit(appkey: *const u8) -> i32 {
    catClientInitWithConfig(appkey, &mut CatClientConfig::default())
}

pub unsafe fn catClientDestroy() -> i32 {
    if g_cat_enabledFlag != 0 {
        g_cat_enabledFlag = 0;
        G_CAT_INIT = 0;
        clearCatMonitor();
        catMessageManagerDestroy();
        clearCatAggregatorThread();
        clearCatSenderThread();
        clearCatServerConnManager();
        destroyMessageIdHelper();
        clearCatClientConfig();
        1
    } else {
        1
    }
}

pub unsafe fn newTransaction(type_: String, name: String) -> *mut CatTransaction {
    if isCatEnabled() {
        let trans: *mut _CatTransaction = createCatTransaction(c!(type_), c!(name));
        if trans.is_null() {
            ptr::null_mut()
        } else {
            catMessageManagerStartTrans(trans);
            trans
        }
    } else {
        &mut CatTransaction::default()
    }
}

pub unsafe fn GetTime64() -> usize {
    let buf: usize;
    let mut tv: timeval = mem::uninitialized();
    gettimeofday(&mut tv, ptr::null_mut());
    buf = (tv.tv_sec * 1000i64 + (tv.tv_usec / 1000i32) as (i64)) as (usize);
    buf
}

pub unsafe fn newTransactionWithDuration(
    type_: String,
    name: String,
    duration: usize,
) -> *mut CatTransaction {
    let trans: *mut _CatTransaction = newTransaction(type_, name);
    ((*trans).setDurationInMillis)(trans, duration);
    if duration < (60i32 * 1000i32) as (usize) {
        ((*trans).setTimestamp)(trans, GetTime64().wrapping_sub(duration));
    }
    trans
}

pub unsafe fn newCompletedTransactionWithDuration(type_: String, name: String, duration: usize) {
    let trans: *mut _CatTransaction = newTransactionWithDuration(type_, name, duration);
    ((*trans).complete)(trans);
}

pub unsafe fn newHeartBeat(mut type_: *const u8, mut name: *const u8) -> *mut _CatMessage {
    if isCatEnabled() {
        (*getContextMessageTree()).canDiscard = 0i32;
        let mut hb: *mut _CatMessage = createCatHeartBeat(type_, name);
        hb
    } else {
        &mut CatMessage::default()
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
    if isCatEnabled() {
        let mut event: *mut _CatMessage = createCatEvent(type_, name);
        event
    } else {
        &mut CatMessage::default()
    }
}

pub unsafe fn logEvent(
    mut type_: *const u8,
    mut name: *const u8,
    mut status: *const u8,
    mut data: *const u8,
) {
    if isCatEnabled() {
        let mut event: *mut _CatMessage = newEvent(type_, name);
        if !event.is_null() {
            if !data.is_null() {
                ((*event).addData)(event, data);
            }
            ((*event).setStatus)(event, status);
            ((*event).complete)(event);
        } else {
        }
    } else {
    }
}

pub unsafe fn newMetric(mut type_: *const u8, mut name: *const u8) -> *mut _CatMessage {
    if isCatEnabled() {
        let mut metric: *mut _CatMessage = createCatMetric(type_, name);
        metric
    } else {
        &mut CatMessage::default()
    }
}

pub unsafe fn _logMetric(mut name: *const u8, mut status: *const u8, mut value: *const u8) {
    let mut metric: *mut _CatMessage = newMetric((*b"\0").as_ptr(), name);
    if !value.is_null() {
        ((*metric).addData)(metric, value);
    }
    ((*metric).setStatus)(metric, status);
    ((*metric).complete)(metric);
}

pub unsafe fn logMetricForCount(mut name: *const u8, mut quantity: i32) {
    if !isCatEnabled() {
    } else if g_config.enableSampling != 0 {
        addCountMetricToAggregator(name, quantity);
    } else if quantity == 1i32 {
        _logMetric(name, (*b"C\0").as_ptr(), (*b"1\0").as_ptr());
    } else {
        let mut val: *mut u8 = catsdsfromlonglong(quantity as (isize));
        _logMetric(name, (*b"C\0").as_ptr(), val as (*const u8));
        catsdsfree(val);
    }
}

pub unsafe fn logMetricForDuration(mut name: *const u8, mut duration: usize) {
    if !isCatEnabled() {
    } else if g_config.enableSampling != 0 {
        addDurationMetricToAggregator(name, duration as (i32));
    } else {
        let mut val: *mut u8 = catsdsfromlonglong(duration as (isize));
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

impl Default for _CatMessage {
    fn default() -> Self {
        unsafe { g_cat_nullMsg }
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

impl Default for _CatTransaction {
    fn default() -> Self {
        unsafe { g_cat_nullTrans }
    }
}

pub type CatTransaction = _CatTransaction;

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
