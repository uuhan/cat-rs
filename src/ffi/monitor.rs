use super::logEvent;
use super::raw::CatClientInnerConfig;
use super::raw::CatMessage;
use super::raw::CatMessageManager;
use super::raw::CatTransaction;
use super::sds::catsdsfree;
use super::sds::catsdsnew;
use libc::free;

extern "C" {
    fn catsdscatfmt(s: *mut u8, fmt: *const u8, ...) -> *mut u8;
    fn checkCatActiveConn() -> i32;
    static mut g_cat_messageManager: CatMessageManager;
    static mut g_config: CatClientInnerConfig;
    fn get_status_report() -> *mut u8;
    fn newHeartBeat(type_: *const u8, name: *const u8) -> *mut CatMessage;
    fn newTransaction(type_: *const u8, name: *const u8) -> *mut CatTransaction;
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

static mut G_CAT_MONITOR_STOP: i32 = 0i32;

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

static mut G_CAT_MONITOR_HANDLE: *mut _opaque_pthread_t = 0 as (*mut _opaque_pthread_t);

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

pub static mut G_CLIENT_INFO: Struct5 = Struct5 {
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

unsafe extern "C" fn catMonitorFun(
    mut para: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_void {
    usleep((1000i32 * 1000i32) as (u32));
    let mut reboot: *mut CatTransaction =
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
        if !(G_CAT_MONITOR_STOP == 0) {
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
            if strcmp(G_CLIENT_INFO.language as (*const u8), (*b"C\0").as_ptr()) != 0i32 {
                let mut name: *mut u8 = catsdsnew((*b"\0").as_ptr());
                name = catsdscatfmt(
                    name,
                    (*b"Cat_%s_Client_Version\0").as_ptr(),
                    G_CLIENT_INFO.language,
                );
                logEvent(
                    name as (*const u8),
                    G_CLIENT_INFO.language_version as (*const u8),
                    (*b"0\0").as_ptr(),
                    0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                );
                catsdsfree(name);
            }
            let mut t: *mut CatTransaction =
                newTransaction((*b"System\0").as_ptr(), (*b"Status\0").as_ptr());
            let mut h: *mut CatMessage = newHeartBeat(
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

pub unsafe fn initCatMonitorThread() {
    G_CAT_MONITOR_STOP = 0i32;
    pthread_create(
        &mut G_CAT_MONITOR_HANDLE as (*mut *mut _opaque_pthread_t),
        0i32 as (*mut ::std::os::raw::c_void) as (*const _opaque_pthread_attr_t),
        catMonitorFun,
        0i32 as (*mut ::std::os::raw::c_void),
    );
}

pub unsafe fn clearCatMonitor() {
    G_CAT_MONITOR_STOP = 1i32;
    pthread_join(
        G_CAT_MONITOR_HANDLE,
        0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut ::std::os::raw::c_void),
    );
}
