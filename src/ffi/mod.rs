#![allow(non_snake_case, non_camel_case_types, unused)]

extern crate libc;

use libc::{gettimeofday, sighandler_t, signal, timeval, SIGINT, SIGPIPE, SIG_IGN};

use std::default::Default;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::prelude::*;
use std::mem;
use std::ptr;

#[macro_use]
mod mac;

mod client_config;
pub mod config;
pub(crate) mod helper;
pub(crate) mod raw;
mod sds;
pub(crate) mod transaction;

use client_config::clearCatClientConfig;
use client_config::initCatClientConfig;
use config::ClientConfig;
use helper::GetTime64;
pub use raw::CatClientConfig;
pub use raw::CatClientInnerConfig;
pub use raw::CatMessage;
pub use raw::CatMessageTree;
pub use raw::CatTransaction;
use sds::catsdsfree;
use sds::catsdsfromlonglong;
use sds::catsdsnew;
use transaction::createCatTransaction;

/// cat static
static mut G_CAT_INIT: i32 = 0i32;

#[allow(dead_code)]
extern "C" {
    /// __sync_add_and_fetch
    fn addCountMetricToAggregator(name: *const u8, count: i32);
    fn addDurationMetricToAggregator(name: *const u8, timeMs: i32);

    fn catMessageManagerDestroy();
    fn catMessageManagerStartTrans(trans: *mut CatTransaction);

    /// __sync_add_and_fetch {0}
    fn clearCatAggregatorThread();
    fn clearCatMonitor();
    fn clearCatSenderThread();
    fn clearCatServerConnManager();

    fn createCatEvent(type_: *const u8, name: *const u8) -> *mut CatMessage;
    fn createCatHeartBeat(type_: *const u8, name: *const u8) -> *mut CatMessage;
    fn createCatMetric(type_: *const u8, name: *const u8) -> *mut CatMessage;
    fn destroyMessageIdHelper();
    static mut g_cat_enabledFlag: i32;
    static mut g_config: CatClientInnerConfig;
    fn getContextMessageTree() -> *mut CatMessageTree;
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
        let mut pTree: *mut CatMessageTree = getContextMessageTree();
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
        let mut pTree: *mut CatMessageTree = getContextMessageTree();
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
        let mut pTree: *mut CatMessageTree = getContextMessageTree();
        if !(*pTree).parentMessageId.is_null() {
            catsdsfree((*pTree).parentMessageId);
            (*pTree).parentMessageId = ptr::null_mut();
        }
        (*pTree).parentMessageId = catsdsnew(messageId as (*const u8));
    } else {
    }
}

pub unsafe fn catClientInitWithConfig(appkey: *const u8, config: CatClientConfig) -> i32 {
    if G_CAT_INIT != 0 {
        0
    } else {
        let mut f = File::open("cat.client.json").unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        let conf: ClientConfig = serde_json::from_str(contents.as_str()).unwrap();

        G_CAT_INIT = 1i32;
        signal(SIGPIPE, SIG_IGN);
        initCatClientConfig(config);

        // TODO: setup host & ip
        assert!(conf.servers.len() > 0);
        let ip = conf.servers[0].ip.clone();
        let port = conf.servers[0].port;
        let httpPort = conf.servers[0].httpPort;

        let ip_ptr = {
            let ip = CString::new(ip).unwrap();
            let p = ip.as_ptr();
            mem::forget(ip);
            p as *const u8
        };

        g_config.appkey = catsdsnew(appkey);
        g_config.serverHost = catsdsnew(ip_ptr);

        initMessageManager(appkey, g_config.selfHost);
        initMessageIdHelper();
        if initCatServerConnManager() == 0 {
            G_CAT_INIT = 0;
            g_cat_enabledFlag = 0;
            error!(
                "Failed to initialize cat: Error occurred while getting router from remote server."
            );
            0
        } else {
            initCatAggregatorThread();
            initCatSenderThread();
            initCatMonitorThread();
            g_cat_enabledFlag = 1;
            1
        }
    }
}

pub unsafe fn catClientInit(appkey: *const u8) -> i32 {
    catClientInitWithConfig(appkey, CatClientConfig::default())
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
        let trans: *mut CatTransaction = createCatTransaction(c!(type_), c!(name));
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

pub unsafe fn newTransactionWithDuration(
    type_: String,
    name: String,
    duration: usize,
) -> *mut CatTransaction {
    let trans: *mut CatTransaction = newTransaction(type_, name);
    ((*trans).setDurationInMillis)(trans, duration);
    if duration < (60i32 * 1000) as (usize) {
        ((*trans).setTimestamp)(trans, GetTime64().wrapping_sub(duration));
    }
    trans
}

pub unsafe fn newCompletedTransactionWithDuration(type_: String, name: String, duration: usize) {
    let trans: *mut CatTransaction = newTransactionWithDuration(type_, name, duration);
    ((*trans).complete)(trans);
}

pub unsafe fn newHeartBeat(mut type_: *const u8, mut name: *const u8) -> *mut CatMessage {
    if isCatEnabled() {
        (*getContextMessageTree()).canDiscard = 0i32;
        let mut hb: *mut CatMessage = createCatHeartBeat(type_, name);
        hb
    } else {
        &mut CatMessage::default()
    }
}

pub unsafe fn logError(mut msg: *const u8, mut errStr: *const u8) {
    (*getContextMessageTree()).canDiscard = 0i32;
    logEvent(
        (*b"Exception\0").as_ptr(),
        msg,
        (*b"ERROR\0").as_ptr(),
        errStr,
    );
}

pub unsafe fn newEvent(mut type_: *const u8, mut name: *const u8) -> *mut CatMessage {
    if isCatEnabled() {
        let mut event: *mut CatMessage = createCatEvent(type_, name);
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
        let mut event: *mut CatMessage = newEvent(type_, name);
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

pub unsafe fn newMetric(mut type_: *const u8, mut name: *const u8) -> *mut CatMessage {
    if isCatEnabled() {
        let mut metric: *mut CatMessage = createCatMetric(type_, name);
        metric
    } else {
        &mut CatMessage::default()
    }
}

pub unsafe fn _logMetric(mut name: *const u8, mut status: *const u8, mut value: *const u8) {
    let mut metric: *mut CatMessage = newMetric((*b"\0").as_ptr(), name);
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
