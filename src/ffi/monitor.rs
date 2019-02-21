extern "C" {
    static mut _DefaultRuneLocale: Struct1;
    fn __maskrune(arg1: i32, arg2: usize) -> i32;
    fn __swbuf(arg1: i32, arg2: *mut __sFILE) -> i32;
    fn __tolower(arg1: i32) -> i32;
    fn __toupper(arg1: i32) -> i32;
    fn catsdscatfmt(s: *mut u8, fmt: *const u8, ...) -> *mut u8;
    fn catsdsfree(s: *mut u8);
    fn catsdsnew(init: *const u8) -> *mut u8;
    fn checkCatActiveConn() -> i32;
    fn free(arg1: *mut ::std::os::raw::c_void);
    static mut g_cat_messageManager: _CatMessageManager;
    static mut g_config: _CatClientInnerConfig;
    fn get_status_report() -> *mut u8;
    fn logEvent(type_: *const u8, name: *const u8, status: *const u8, data: *const u8);
    fn newHeartBeat(type_: *const u8, name: *const u8) -> *mut _CatMessage;
    fn newTransaction(type_: *const u8, name: *const u8) -> *mut _CatTransaction;
    fn pthread_create(
        arg1: *mut *mut _opaque_pthread_t,
        arg2: *const _opaque_pthread_attr_t,
        arg3: unsafe extern "C" fn(*mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
        arg4: *mut ::std::os::raw::c_void,
    ) -> i32;
    fn pthread_join(arg1: *mut _opaque_pthread_t, arg2: *mut *mut ::std::os::raw::c_void) -> i32;
    fn saveMark();
    fn strcmp(__s1: *const u8, __s2: *const u8) -> i32;
    fn updateCatServerConn() -> i32;
    fn usleep(arg1: u32) -> i32;
}

enum __sFILEX {}

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

#[no_mangle]
pub unsafe extern "C" fn __sputc(mut _c: i32, mut _p: *mut __sFILE) -> i32 {
    if {
        (*_p)._w = (*_p)._w - 1;
        (*_p)._w
    } >= 0i32
        || (*_p)._w >= (*_p)._lbfsize && (_c as (u8) as (i32) != b'\n' as (i32))
    {
        ({
            let _rhs = _c;
            let _lhs = &mut *{
                let _old = (*_p)._p;
                (*_p)._p = (*_p)._p.offset(1isize);
                _old
            };
            *_lhs = _rhs as (u8);
            *_lhs
        }) as (i32)
    } else {
        __swbuf(_c, _p)
    }
}

#[no_mangle]
pub unsafe extern "C" fn isascii(mut _c: i32) -> i32 {
    (_c & !0x7fi32 == 0i32) as (i32)
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct3 {
    pub __min: i32,
    pub __max: i32,
    pub __map: i32,
    pub __types: *mut u32,
}

impl Clone for Struct3 {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct2 {
    pub __nranges: i32,
    pub __ranges: *mut Struct3,
}

impl Clone for Struct2 {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
    pub __name: [u8; 14],
    pub __mask: u32,
}

impl Clone for Struct4 {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub __magic: [u8; 8],
    pub __encoding: [u8; 32],
    pub __sgetrune: unsafe extern "C" fn(*const u8, usize, *mut *const u8) -> i32,
    pub __sputrune: unsafe extern "C" fn(i32, *mut u8, usize, *mut *mut u8) -> i32,
    pub __invalid_rune: i32,
    pub __runetype: [u32; 256],
    pub __maplower: [i32; 256],
    pub __mapupper: [i32; 256],
    pub __runetype_ext: Struct2,
    pub __maplower_ext: Struct2,
    pub __mapupper_ext: Struct2,
    pub __variable: *mut ::std::os::raw::c_void,
    pub __variable_len: i32,
    pub __ncharclasses: i32,
    pub __charclasses: *mut Struct4,
}

impl Clone for Struct1 {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn __istype(mut _c: i32, mut _f: usize) -> i32 {
    if isascii(_c) != 0 {
        !(_DefaultRuneLocale.__runetype[_c as (usize)] as (usize) & _f == 0) as (i32)
    } else {
        !(__maskrune(_c, _f) == 0) as (i32)
    }
}

#[no_mangle]
pub unsafe extern "C" fn __isctype(mut _c: i32, mut _f: usize) -> i32 {
    if _c < 0i32 || _c >= 256i32 {
        0i32
    } else {
        !(_DefaultRuneLocale.__runetype[_c as (usize)] as (usize) & _f == 0) as (i32)
    }
}

#[no_mangle]
pub unsafe extern "C" fn __wcwidth(mut _c: i32) -> i32 {
    let mut _x: u32;
    if _c == 0i32 {
        0i32
    } else {
        _x = __maskrune(_c, 0xe0000000usize | 0x40000usize) as (u32);
        (if _x as (usize) & 0xe0000000usize != 0usize {
            ((_x as (usize) & 0xe0000000usize) >> 30i32) as (i32)
        } else if _x as (usize) & 0x40000usize != 0usize {
            1i32
        } else {
            -1i32
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn isalnum(mut _c: i32) -> i32 {
    __istype(_c, (0x100isize | 0x400isize) as (usize))
}

#[no_mangle]
pub unsafe extern "C" fn isalpha(mut _c: i32) -> i32 {
    __istype(_c, 0x100usize)
}

#[no_mangle]
pub unsafe extern "C" fn isblank(mut _c: i32) -> i32 {
    __istype(_c, 0x20000usize)
}

#[no_mangle]
pub unsafe extern "C" fn iscntrl(mut _c: i32) -> i32 {
    __istype(_c, 0x200usize)
}

#[no_mangle]
pub unsafe extern "C" fn isdigit(mut _c: i32) -> i32 {
    __isctype(_c, 0x400usize)
}

#[no_mangle]
pub unsafe extern "C" fn isgraph(mut _c: i32) -> i32 {
    __istype(_c, 0x800usize)
}

#[no_mangle]
pub unsafe extern "C" fn islower(mut _c: i32) -> i32 {
    __istype(_c, 0x1000usize)
}

#[no_mangle]
pub unsafe extern "C" fn isprint(mut _c: i32) -> i32 {
    __istype(_c, 0x40000usize)
}

#[no_mangle]
pub unsafe extern "C" fn ispunct(mut _c: i32) -> i32 {
    __istype(_c, 0x2000usize)
}

#[no_mangle]
pub unsafe extern "C" fn isspace(mut _c: i32) -> i32 {
    __istype(_c, 0x4000usize)
}

#[no_mangle]
pub unsafe extern "C" fn isupper(mut _c: i32) -> i32 {
    __istype(_c, 0x8000usize)
}

#[no_mangle]
pub unsafe extern "C" fn isxdigit(mut _c: i32) -> i32 {
    __isctype(_c, 0x10000usize)
}

#[no_mangle]
pub unsafe extern "C" fn toascii(mut _c: i32) -> i32 {
    _c & 0x7fi32
}

#[no_mangle]
pub unsafe extern "C" fn tolower(mut _c: i32) -> i32 {
    __tolower(_c)
}

#[no_mangle]
pub unsafe extern "C" fn toupper(mut _c: i32) -> i32 {
    __toupper(_c)
}

#[no_mangle]
pub unsafe extern "C" fn digittoint(mut _c: i32) -> i32 {
    __maskrune(_c, 0xfusize)
}

#[no_mangle]
pub unsafe extern "C" fn ishexnumber(mut _c: i32) -> i32 {
    __istype(_c, 0x10000usize)
}

#[no_mangle]
pub unsafe extern "C" fn isideogram(mut _c: i32) -> i32 {
    __istype(_c, 0x80000usize)
}

#[no_mangle]
pub unsafe extern "C" fn isnumber(mut _c: i32) -> i32 {
    __istype(_c, 0x400usize)
}

#[no_mangle]
pub unsafe extern "C" fn isphonogram(mut _c: i32) -> i32 {
    __istype(_c, 0x200000usize)
}

#[no_mangle]
pub unsafe extern "C" fn isrune(mut _c: i32) -> i32 {
    __istype(_c, 0xfffffff0usize)
}

#[no_mangle]
pub unsafe extern "C" fn isspecial(mut _c: i32) -> i32 {
    __istype(_c, 0x100000usize)
}

static mut g_cat_monitorStop: i32 = 0i32;

#[derive(Copy)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: unsafe extern "C" fn(*mut ::std::os::raw::c_void),
    pub __arg: *mut ::std::os::raw::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}

impl Clone for __darwin_pthread_handler_rec {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: isize,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [u8; 8176],
}

impl Clone for _opaque_pthread_t {
    fn clone(&self) -> Self {
        *self
    }
}

static mut g_cat_monitorHandle: *mut _opaque_pthread_t = 0 as (*mut _opaque_pthread_t);

#[derive(Copy)]
#[repr(C)]
pub struct Struct5 {
    pub language: *mut u8,
    pub language_version: *mut u8,
}

impl Clone for Struct5 {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub static mut g_client_info: Struct5 = Struct5 {
    language: (*b"C\0").as_ptr() as (*mut u8),
    language_version: (*b"\0").as_ptr() as (*mut u8),
};

#[derive(Copy)]
#[repr(C)]
pub struct _opaque_pthread_attr_t {
    pub __sig: isize,
    pub __opaque: [u8; 56],
}

impl Clone for _opaque_pthread_attr_t {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct _CatMessageManager {
    pub domain: *mut u8,
    pub hostname: *mut u8,
    pub ip: *mut u8,
    pub ipHex: *mut u8,
    pub throttleTimes: isize,
}

impl Clone for _CatMessageManager {
    fn clone(&self) -> Self {
        *self
    }
}

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

unsafe extern "C" fn catMonitorFun(
    mut para: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_void {
    usleep((1000i32 * 1000i32) as (u32));
    let mut reboot: *mut _CatTransaction =
        newTransaction((*b"System\0").as_ptr(), (*b"Reboot\0").as_ptr());
    logEvent(
        (*b"Reboot\0").as_ptr(),
        g_cat_messageManager.ip as (*const u8),
        (*b"0\0").as_ptr(),
        0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
    );
    ((*reboot).setStatus)(reboot, (*b"0\0").as_ptr());
    ((*reboot).complete)(reboot);
    let mut runCount: usize = 1usize;
    'loop1: loop {
        if !(g_cat_monitorStop == 0) {
            break;
        }
        checkCatActiveConn();
        if runCount.wrapping_rem(10usize) == 1usize {
            saveMark();
        }
        if runCount.wrapping_rem(180usize) == 0usize {
            updateCatServerConn();
        }
        if runCount.wrapping_rem(60usize) == 1usize && (g_config.enableHeartbeat != 0) {
            logEvent(
                (*b"Cat_C_Client_Version\0").as_ptr(),
                (*b"3.0.1\0").as_ptr(),
                (*b"0\0").as_ptr(),
                0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
            );
            if strcmp(g_client_info.language as (*const u8), (*b"C\0").as_ptr()) != 0i32 {
                let mut name: *mut u8 = catsdsnew((*b"\0").as_ptr());
                name = catsdscatfmt(
                    name,
                    (*b"Cat_%s_Client_Version\0").as_ptr(),
                    g_client_info.language,
                );
                logEvent(
                    name as (*const u8),
                    g_client_info.language_version as (*const u8),
                    (*b"0\0").as_ptr(),
                    0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                );
                catsdsfree(name);
            }
            let mut t: *mut _CatTransaction =
                newTransaction((*b"System\0").as_ptr(), (*b"Status\0").as_ptr());
            let mut h: *mut _CatMessage = newHeartBeat(
                (*b"Heartbeat\0").as_ptr(),
                g_cat_messageManager.ip as (*const u8),
            );
            let mut xmlContent: *mut u8 = get_status_report();
            ((*h).addData)(h, xmlContent as (*const u8));
            free(xmlContent as (*mut ::std::os::raw::c_void));
            ((*h).complete)(h);
            ((*t).setStatus)(t, (*b"0\0").as_ptr());
            ((*t).complete)(t);
        }
        runCount = runCount.wrapping_add(1usize);
        usleep((1000i32 * 1000i32) as (u32));
    }
    0i32 as (*mut ::std::os::raw::c_void)
}

#[no_mangle]
pub unsafe extern "C" fn initCatMonitorThread() {
    g_cat_monitorStop = 0i32;
    pthread_create(
        &mut g_cat_monitorHandle as (*mut *mut _opaque_pthread_t),
        0i32 as (*mut ::std::os::raw::c_void) as (*const _opaque_pthread_attr_t),
        catMonitorFun,
        0i32 as (*mut ::std::os::raw::c_void),
    );
}

#[no_mangle]
pub unsafe extern "C" fn clearCatMonitor() {
    g_cat_monitorStop = 1i32;
    pthread_join(
        g_cat_monitorHandle,
        0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut ::std::os::raw::c_void),
    );
}
