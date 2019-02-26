use super::raw::CatClientConfig;
use super::raw::CatClientInnerConfig;
use libc::free;
use libc::malloc;
use libc::memset;
use std::ffi::CStr;
use std::ffi::CString;
use std::path::Path;
use std::ptr::null_mut;

extern "C" {
    fn catAnetGetHost(err: *mut u8, host: *mut u8, ipbuf_len: usize) -> i32;
    fn catsdscpy(s: *mut u8, t: *const u8) -> *mut u8;
    fn catsdsfree(s: *mut u8);
    fn catsdsnew(init: *const u8) -> *mut u8;
    fn catsdsnewEmpty(preAlloclen: usize) -> *mut u8;
    fn ezxml_attr(xml: *mut ezxml, attr: *const u8) -> *const u8;
    fn ezxml_child(xml: *mut ezxml, name: *const u8) -> *mut ezxml;
    fn ezxml_free(xml: *mut ezxml);
    fn ezxml_parse_file(file: *const i8) -> *mut ezxml;
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

#[derive(Copy)]
#[repr(C)]
pub struct ezxml {
    pub name: *mut u8,
    pub attr: *mut *mut u8,
    pub txt: *mut u8,
    pub off: usize,
    pub next: *mut ezxml,
    pub sibling: *mut ezxml,
    pub ordered: *mut ezxml,
    pub child: *mut ezxml,
    pub parent: *mut ezxml,
    pub flags: i16,
}

impl Clone for ezxml {
    fn clone(&self) -> Self {
        *self
    }
}

pub unsafe fn parseCatClientConfig(mut f1: *mut ezxml) -> i32 {
    let mut serverIndex: i32 = 0i32;
    let mut servers: *mut ezxml;
    let mut server: *mut ezxml;
    servers = ezxml_child(f1, (*b"servers\0").as_ptr());
    'loop1: loop {
        if servers.is_null() {
            break;
        }
        server = ezxml_child(servers, (*b"server\0").as_ptr());
        'loop6: loop {
            if server.is_null() {
                break;
            }
            let mut ip: *const u8;
            ip = ezxml_attr(server, (*b"ip\0").as_ptr());
            if !(0i32 as (*mut ::std::os::raw::c_void) as (*const u8) == ip
                || *ip.offset(0isize) as (i32) == b'\0' as (i32))
            {
                if serverIndex == 0i32 {
                    let mut port: *const u8;
                    g_config.serverHost = catsdsnew(ip);
                    port = ezxml_attr(server, (*b"http-port\0").as_ptr());
                    if !port.is_null() && (*port.offset(0isize) as (i32) != b'\0' as (i32)) {
                        let port = CStr::from_ptr(port as *const i8).to_str().unwrap();
                        g_config.serverPort = port.parse().unwrap();
                    }
                } else if serverIndex >= g_config.serverNum {
                    break;
                }
                serverIndex = serverIndex + 1;
            }
            server = (*server).next;
        }
        servers = (*servers).next;
    }
    ezxml_free(f1);
    if serverIndex <= 0i32 {
        -1i32
    } else {
        0i32
    }
}

unsafe fn getCatClientConfig(filename: &str) -> *mut ezxml {
    if Path::new(filename).exists() {
        ezxml_parse_file(CString::new(filename).unwrap().as_ptr())
    } else {
        null_mut()
    }
}

pub unsafe fn loadCatClientConfig(filename: &str) -> i32 {
    let mut config: *mut ezxml = getCatClientConfig(filename);
    if config.is_null() {
        error!("File {} not exists.", filename);
        error!("client.xml is required to initialize cat client!");
        -1
    } else if parseCatClientConfig(config) < 0i32 {
        error!("Failed to parse client.xml, is it a legal xml file?");
        -1
    } else {
        0i32
    }
}

pub unsafe fn initCatClientConfig(mut config: *mut CatClientConfig) {
    memset(
        &mut g_config as (*mut CatClientInnerConfig) as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<CatClientInnerConfig>(),
    );
    g_log_debug = (*config).enableDebugLog;
    g_config.appkey = (*b"cat\0").as_ptr() as (*mut u8);
    g_config.selfHost = catsdsnewEmpty(128usize);
    g_config.defaultIp = catsdsnew((*b"127.0.0.1\0").as_ptr());
    g_config.defaultIpHex = catsdsnew((*b"7f000001\0").as_ptr());
    if catAnetGetHost(
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
        g_config.selfHost,
        128usize,
    ) == -1i32
    {
        g_config.selfHost = catsdscpy(g_config.selfHost, (*b"CUnknownHost\0").as_ptr());
    }
    info!(
        "Current hostname: {}",
        CStr::from_ptr(g_config.selfHost as *const i8)
            .to_str()
            .unwrap()
    );
    g_config.serverHost = catsdsnew((*b"127.0.0.1\0").as_ptr());
    g_config.serverPort = 8080u32;
    g_config.serverNum = 3i32;
    g_config.serverAddresses =
        malloc((g_config.serverNum as (usize)).wrapping_mul(::std::mem::size_of::<*mut u8>()))
            as (*mut *mut u8);
    let mut i: i32 = 0i32;
    i = 0i32;
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
    g_config.messageEnableFlag = 1i32;
    g_config.messageQueueSize = 10000i32;
    g_config.messageQueueBlockPrintCount = 100000i32;
    g_config.maxContextElementSize = 2000i32;
    g_config.maxChildSize = 2048i32;
    g_config.logFlag = 1i32;
    g_config.logSaveFlag = 1i32;
    g_config.logDebugFlag = (*config).enableDebugLog;
    g_config.logFilePerDay = 1i32;
    g_config.logFileWithTime = 0i32;
    g_config.logLevel = 0xffi32;
    g_config.configDir = catsdsnew((*b"./\0").as_ptr());
    g_config.dataDir = catsdsnew((*b"/data/appdatas/cat/\0").as_ptr());
    g_config.indexFileName = catsdsnew((*b"client.idx.h\0").as_ptr());
    g_config.encoderType = (*config).encoderType;
    g_config.enableHeartbeat = (*config).enableHeartbeat;
    g_config.enableSampling = (*config).enableSampling;
    g_config.enableMultiprocessing = (*config).enableMultiprocessing;
    if g_config.logFlag == 0 {
        g_log_permissionOpt = 0i32;
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
