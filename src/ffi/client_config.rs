use super::raw::CatClientConfig;
use super::raw::CatClientInnerConfig;
use libc::c_void;
use libc::free;
use libc::malloc;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::path::Path;
use std::ptr::null_mut;

extern "C" {
    fn catAnetGetHost(err: *mut u8, host: *mut u8, ipbuf_len: usize) -> i32;
    fn catsdscpy(s: *mut u8, t: *const u8) -> *mut u8;
    fn catsdsfree(s: *mut u8);
    fn catsdsnew(init: *const u8) -> *mut u8;
    fn catsdsnewEmpty(preAlloclen: usize) -> *mut u8;
    static mut g_log_debug: i32;
    static mut g_log_file_perDay: i32;
    static mut g_log_file_with_time: i32;
    static mut g_log_permissionOpt: i32;
    static mut g_log_saveFlag: i32;

}

#[no_mangle]
pub static mut g_config: CatClientInnerConfig = CatClientInnerConfig {
    appkey: 0 as (*mut u8),
    selfHost: 0 as (*mut u8),
    serverHost: 0 as (*mut u8),
    defaultIp: 0 as (*mut u8),
    defaultIpHex: 0 as (*mut u8),
    serverPort: 0u32,
    serverNum: 0i32,
    serverAddresses: 0 as (*mut *mut u8),
    messageEnableFlag: 0i32,
    messageQueueSize: 0i32,
    messageQueueBlockPrintCount: 0i32,
    maxChildSize: 0i32,
    maxContextElementSize: 0i32,
    logFlag: 0i32,
    logSaveFlag: 0i32,
    logDebugFlag: 0i32,
    logFileWithTime: 0i32,
    logFilePerDay: 0i32,
    logLevel: 0i32,
    configDir: 0 as (*mut u8),
    dataDir: 0 as (*mut u8),
    indexFileName: 0 as (*mut u8),
    encoderType: 0i32,
    enableHeartbeat: 0i32,
    enableSampling: 0i32,
    enableMultiprocessing: 0i32,
};

pub unsafe fn initCatClientConfig(mut config: CatClientConfig) {
    g_config = mem::uninitialized();

    g_log_debug = config.enableDebugLog;
    g_config.appkey = (*b"cat\0").as_ptr() as (*mut u8);
    g_config.selfHost = catsdsnewEmpty(128usize);
    g_config.defaultIp = catsdsnew((*b"127.0.0.1\0").as_ptr());
    g_config.defaultIpHex = catsdsnew((*b"7f000001\0").as_ptr());
    g_config.selfHost = catsdscpy(g_config.selfHost, (*b"CUnknownHost\0").as_ptr());
    info!(
        "Current hostname: {}",
        CStr::from_ptr(g_config.selfHost as *const i8)
            .to_str()
            .unwrap()
    );

    // TODO
    g_config.serverHost = catsdsnew((*b"47.99.131.78\0").as_ptr());
    g_config.serverPort = 2040;

    g_config.serverNum = 3;
    g_config.serverAddresses =
        malloc((g_config.serverNum as (usize)).wrapping_mul(::std::mem::size_of::<*mut u8>()))
            as (*mut *mut u8);
    let mut i: i32 = 0;
    i = 0;
    'loop3: loop {
        if !(i < g_config.serverNum) {
            break;
        }
        *g_config.serverAddresses.offset(i as (isize)) = catsdsnew((*b"\0").as_ptr());
        i = i + 1;
    }
    *g_config.serverAddresses.offset(0isize) = catsdscpy(
        *g_config.serverAddresses.offset(0isize),
        (*b"127.0.0.1:2280\0").as_ptr(),
    );
    *g_config.serverAddresses.offset(1isize) = catsdscpy(
        *g_config.serverAddresses.offset(1isize),
        (*b"127.0.0.1:2280\0").as_ptr(),
    );
    *g_config.serverAddresses.offset(2isize) = catsdscpy(
        *g_config.serverAddresses.offset(2isize),
        (*b"127.0.0.1:2280\0").as_ptr(),
    );
    g_config.messageEnableFlag = 1;
    g_config.messageQueueSize = 10000;
    g_config.messageQueueBlockPrintCount = 100000;
    g_config.maxContextElementSize = 2000;
    g_config.maxChildSize = 2048;
    g_config.logFlag = 1;
    g_config.logSaveFlag = 1;
    g_config.logDebugFlag = config.enableDebugLog;
    g_config.logFilePerDay = 1;
    g_config.logFileWithTime = 0;
    g_config.logLevel = 0xff;
    g_config.configDir = catsdsnew((*b"./\0").as_ptr());
    g_config.dataDir = catsdsnew((*b"/data/appdatas/cat/\0").as_ptr());
    g_config.indexFileName = catsdsnew((*b"client.idx.h\0").as_ptr());
    g_config.encoderType = config.encoderType;
    g_config.enableHeartbeat = config.enableHeartbeat;
    g_config.enableSampling = config.enableSampling;
    g_config.enableMultiprocessing = config.enableMultiprocessing;
    if g_config.logFlag == 0 {
        g_log_permissionOpt = 0;
    } else {
        g_log_permissionOpt = g_config.logLevel;
        g_log_saveFlag = g_config.logLevel;
        g_log_file_perDay = g_config.logFilePerDay;
        g_log_file_with_time = g_config.logFileWithTime;
        g_log_debug = g_config.logDebugFlag;
    }
}

pub unsafe fn clearCatClientConfig() {
    catsdsfree(g_config.appkey);
    catsdsfree(g_config.selfHost);
    catsdsfree(g_config.defaultIp);
    catsdsfree(g_config.defaultIpHex);
    catsdsfree(g_config.serverHost);
    let mut i: i32 = 0i32;
    i = 0i32;
    'loop1: loop {
        if !(i < g_config.serverNum) {
            break;
        }
        catsdsfree(*g_config.serverAddresses.offset(i as (isize)));
        i = i + 1;
    }
    free(g_config.serverAddresses as (*mut ::std::os::raw::c_void));
    catsdsfree(g_config.configDir);
    catsdsfree(g_config.dataDir);
    catsdsfree(g_config.indexFileName);
}
