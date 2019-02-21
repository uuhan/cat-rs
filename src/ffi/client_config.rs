extern "C" {
    fn CLogLogWithLocation(
        type_: u16,
        format: *const u8,
        file: *const u8,
        line: i32,
        function: *const u8,
        ...
    );
    fn atoi(arg1: *const u8) -> i32;
    fn catAnetGetHost(err: *mut u8, host: *mut u8, ipbuf_len: usize) -> i32;
    fn catsdscpy(s: *mut u8, t: *const u8) -> *mut u8;
    fn catsdsfree(s: *mut u8);
    fn catsdsnew(init: *const u8) -> *mut u8;
    fn catsdsnewEmpty(preAlloclen: usize) -> *mut u8;
    fn ezxml_attr(xml: *mut ezxml, attr: *const u8) -> *const u8;
    fn ezxml_child(xml: *mut ezxml, name: *const u8) -> *mut ezxml;
    fn ezxml_free(xml: *mut ezxml);
    fn ezxml_parse_file(file: *const u8) -> *mut ezxml;
    fn fopen(__filename: *const u8, __mode: *const u8) -> *mut __sFILE;
    fn free(arg1: *mut ::std::os::raw::c_void);
    static mut g_log_debug: i32;
    static mut g_log_file_perDay: i32;
    static mut g_log_file_with_time: i32;
    static mut g_log_permissionOpt: i32;
    static mut g_log_saveFlag: i32;
    fn logError(msg: *const u8, errStr: *const u8);
    fn malloc(__size: usize) -> *mut ::std::os::raw::c_void;
    fn memset(
        __b: *mut ::std::os::raw::c_void,
        __c: i32,
        __len: usize,
    ) -> *mut ::std::os::raw::c_void;
}

pub enum __sFILEX {}

#[derive(Copy)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut u8,
    pub _size: i32,
}

impl Clone for __sbuf {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut u8,
    pub _r: i32,
    pub _w: i32,
    pub _flags: i16,
    pub _file: i16,
    pub _bf: __sbuf,
    pub _lbfsize: i32,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: unsafe extern "C" fn(*mut ::std::os::raw::c_void) -> i32,
    pub _read: unsafe extern "C" fn(*mut ::std::os::raw::c_void, *mut u8, i32) -> i32,
    pub _seek: unsafe extern "C" fn(*mut ::std::os::raw::c_void, isize, i32) -> isize,
    pub _write: unsafe extern "C" fn(*mut ::std::os::raw::c_void, *const u8, i32) -> i32,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: i32,
    pub _ubuf: [u8; 3],
    pub _nbuf: [u8; 1],
    pub _lb: __sbuf,
    pub _blksize: i32,
    pub _offset: isize,
}

impl Clone for __sFILE {
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

#[no_mangle]
pub static mut g_config: _CatClientInnerConfig = _CatClientInnerConfig {
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

#[no_mangle]
pub static mut g_cat_enabledFlag: i32 = 0i32;

#[no_mangle]
pub unsafe extern "C" fn catChecktPtrWithName(
    mut ptr: *mut ::std::os::raw::c_void,
    mut ptrName: *mut u8,
) {
    if ptr == 0i32 as (*mut ::std::os::raw::c_void) {
        CLogLogWithLocation(
            0x8u16,
            (*b"memory allocation failed. (oom).\0").as_ptr(),
            file!().as_ptr(),
            line!() as (i32),
            (*b"catChecktPtrWithName\0").as_ptr(),
            ptrName,
        );
        logError((*b"Error\0").as_ptr(), (*b"OutOfMemory\0").as_ptr());
    }
}

#[no_mangle]
pub unsafe extern "C" fn isCatEnabled() -> i32 {
    g_cat_enabledFlag
}

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

#[no_mangle]
pub unsafe extern "C" fn parseCatClientConfig(mut f1: *mut ezxml) -> i32 {
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
                        g_config.serverPort = atoi(port) as (u32);
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

unsafe extern "C" fn getCatClientConfig(mut filename: *const u8) -> *mut ezxml {
    let mut file: *mut __sFILE = fopen(filename, (*b"r\0").as_ptr());
    if file == 0i32 as (*mut ::std::os::raw::c_void) as (*mut __sFILE) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut ezxml)
    } else {
        ezxml_parse_file(filename)
    }
}

#[no_mangle]
pub unsafe extern "C" fn loadCatClientConfig(mut filename: *const u8) -> i32 {
    let mut config: *mut ezxml = getCatClientConfig(filename);
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut ezxml) == config {
        CLogLogWithLocation(
            0x4u16,
            (*b"File %s not exists.\0").as_ptr(),
            file!().as_ptr(),
            line!() as (i32),
            (*b"loadCatClientConfig\0").as_ptr(),
            filename,
        );
        CLogLogWithLocation(
            0x4u16,
            (*b"client.xml is required to initialize cat client!\0").as_ptr(),
            file!().as_ptr(),
            line!() as (i32),
            (*b"loadCatClientConfig\0").as_ptr(),
        );
        -1i32
    } else if parseCatClientConfig(config) < 0i32 {
        CLogLogWithLocation(
            0x8u16,
            (*b"Failed to parse client.xml, is it a legal xml file?\0").as_ptr(),
            file!().as_ptr(),
            line!() as (i32),
            (*b"loadCatClientConfig\0").as_ptr(),
        );
        -1i32
    } else {
        0i32
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

#[no_mangle]
pub unsafe extern "C" fn initCatClientConfig(mut config: *mut _CatClientConfig) {
    memset(
        &mut g_config as (*mut _CatClientInnerConfig) as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<_CatClientInnerConfig>(),
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
    CLogLogWithLocation(
        0x2u16,
        (*b"Current hostname: %s\0").as_ptr(),
        file!().as_ptr(),
        line!() as (i32),
        (*b"initCatClientConfig\0").as_ptr(),
        g_config.selfHost,
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

#[no_mangle]
pub unsafe extern "C" fn clearCatClientConfig() {
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
