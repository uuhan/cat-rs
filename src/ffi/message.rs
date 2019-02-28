extern "C" {
    fn catMessageManagerAdd(message: *mut _CatMessage);
    fn catsdscat(s: *mut u8, t: *const u8) -> *mut u8;
    fn catsdscpy(s: *mut u8, t: *const u8) -> *mut u8;
    fn catsdsfree(s: *mut u8);
    fn catsdsnew(init: *const u8) -> *mut u8;
    fn gettimeofday(arg1: *mut timeval, arg2: *mut ::std::os::raw::c_void) -> i32;
    fn malloc(__size: usize) -> *mut ::std::os::raw::c_void;
    fn memset(
        __b: *mut ::std::os::raw::c_void,
        __c: i32,
        __len: usize,
    ) -> *mut ::std::os::raw::c_void;
}

enum Union1 {}

unsafe extern "C" fn addDataNull(mut message: *mut _CatMessage, mut data: *const u8) {}

// #[no_mangle]
// pub static mut g_cat_nullMsg: _CatMessage = addDataNull as (_CatMessage);

#[derive(Copy)]
#[repr(C)]
pub struct _CatMessageInner {
    pub messageType: Union1,
    pub type_: *mut u8,
    pub name: *mut u8,
    pub status: *mut u8,
    pub data: *mut u8,
    pub timestampMs: usize,
    pub isComplete: i32,
}

impl Clone for _CatMessageInner {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn clearMessage(
    mut message: *mut _CatMessage,
) -> *mut ::std::os::raw::c_void {
    let mut pInner: *mut _CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatMessageInner>() as (isize)))
        as (*mut _CatMessageInner);
    if (*pInner).status != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        catsdsfree((*pInner).status);
        (*pInner).status = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    if (*pInner).data != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        catsdsfree((*pInner).data);
        (*pInner).data = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    if (*pInner).type_ != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        catsdsfree((*pInner).type_);
        (*pInner).type_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    if (*pInner).name != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        catsdsfree((*pInner).name);
        (*pInner).name = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    pInner as (*mut ::std::os::raw::c_void)
}

#[derive(Copy)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: isize,
    pub tv_usec: i32,
}

impl Clone for timeval {
    fn clone(&self) -> Self {
        *self
    }
}

unsafe extern "C" fn GetTime64() -> usize {
    let mut buf: usize;
    let mut tv: timeval;
    gettimeofday(
        &mut tv as (*mut timeval),
        0i32 as (*mut ::std::os::raw::c_void),
    );
    buf = (tv.tv_sec * 1000isize + (tv.tv_usec / 1000i32) as (isize)) as (usize);
    buf
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

unsafe extern "C" fn addData(mut message: *mut _CatMessage, mut data: *const u8) {
    let mut pInner: *mut _CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatMessageInner>() as (isize)))
        as (*mut _CatMessageInner);
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) == (*pInner).data {
        (*pInner).data = catsdsnew(data);
    } else {
        (*pInner).data = catsdscat((*pInner).data, (*b"&\0").as_ptr());
        (*pInner).data = catsdscat((*pInner).data, data);
    }
}

unsafe extern "C" fn addKV(
    mut message: *mut _CatMessage,
    mut dataKey: *const u8,
    mut dataValue: *const u8,
) {
    let mut pInner: *mut _CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatMessageInner>() as (isize)))
        as (*mut _CatMessageInner);
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) == (*pInner).data {
        (*pInner).data = catsdsnew(dataKey);
        (*pInner).data = catsdscat((*pInner).data, (*b"=\0").as_ptr());
        (*pInner).data = catsdscat((*pInner).data, dataValue);
    } else {
        (*pInner).data = catsdscat((*pInner).data, (*b"&\0").as_ptr());
        (*pInner).data = catsdscat((*pInner).data, dataKey);
        (*pInner).data = catsdscat((*pInner).data, (*b"=\0").as_ptr());
        (*pInner).data = catsdscat((*pInner).data, dataValue);
    }
}

unsafe extern "C" fn setComplete(mut message: *mut _CatMessage) {
    let mut pInner: *mut _CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatMessageInner>() as (isize)))
        as (*mut _CatMessageInner);
    (*pInner).isComplete = 1i32;
}

unsafe extern "C" fn setTimestamp(mut message: *mut _CatMessage, mut timeMs: usize) {
    let mut pInner: *mut _CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatMessageInner>() as (isize)))
        as (*mut _CatMessageInner);
    (*pInner).timestampMs = timeMs;
}

unsafe extern "C" fn setStatus(mut message: *mut _CatMessage, mut status: *const u8) {
    let mut pInner: *mut _CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatMessageInner>() as (isize)))
        as (*mut _CatMessageInner);
    if (*pInner).status == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        (*pInner).status = catsdsnew(status);
    } else {
        (*pInner).status = catsdscpy((*pInner).status, status);
    }
}

#[no_mangle]
pub unsafe extern "C" fn initCatMessage(
    mut pMsg: *mut _CatMessage,
    mut msgType: u8,
    mut type_: *const u8,
    mut name: *const u8,
) {
    let mut pInner: *mut _CatMessageInner = (pMsg as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatMessageInner>() as (isize)))
        as (*mut _CatMessageInner);
    memset(
        pInner as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<_CatMessage>()
            .wrapping_add(::std::mem::size_of::<_CatMessageInner>()),
    );
    // TODO: Union1
    // (*pInner).messageType = msgType;
    (*pInner).timestampMs = GetTime64();
    (*pInner).type_ = catsdsnew(type_);
    (*pInner).name = catsdsnew(name);
    (*pMsg).addData = addData;
    (*pMsg).addKV = addKV;
    (*pMsg).complete = setComplete;
    (*pMsg).setTimestamp = setTimestamp;
    (*pMsg).setStatus = setStatus;
}

unsafe extern "C" fn setEventComplete(mut message: *mut _CatMessage) {
    let mut pInner: *mut _CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatMessageInner>() as (isize)))
        as (*mut _CatMessageInner);
    (*pInner).isComplete = 1i32;
    catMessageManagerAdd(message);
}

#[no_mangle]
pub unsafe extern "C" fn createCatEvent(
    mut type_: *const u8,
    mut name: *const u8,
) -> *mut _CatMessage {
    let mut pEventInner: *mut _CatMessageInner = malloc(
        ::std::mem::size_of::<_CatMessage>()
            .wrapping_add(::std::mem::size_of::<_CatMessageInner>()),
    ) as (*mut _CatMessageInner);
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut _CatMessageInner) == pEventInner {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut _CatMessage)
    } else {
        let mut pEvent: *mut _CatMessage = (pEventInner as (*mut u8))
            .offset(::std::mem::size_of::<_CatMessageInner>() as (isize))
            as (*mut _CatMessage);
        initCatMessage(pEvent, b'E', type_, name);
        (*pEvent).complete = setEventComplete;
        pEvent
    }
}

unsafe extern "C" fn setMetricComplete(mut message: *mut _CatMessage) {
    let mut pInner: *mut _CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatMessageInner>() as (isize)))
        as (*mut _CatMessageInner);
    (*pInner).isComplete = 1i32;
    catMessageManagerAdd(message);
}

#[no_mangle]
pub unsafe extern "C" fn createCatMetric(
    mut type_: *const u8,
    mut name: *const u8,
) -> *mut _CatMessage {
    let mut pMetricInner: *mut _CatMessageInner = malloc(
        ::std::mem::size_of::<_CatMessage>()
            .wrapping_add(::std::mem::size_of::<_CatMessageInner>()),
    ) as (*mut _CatMessageInner);
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut _CatMessageInner) == pMetricInner {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut _CatMessage)
    } else {
        let mut pMetric: *mut _CatMessage = (pMetricInner as (*mut u8))
            .offset(::std::mem::size_of::<_CatMessageInner>() as (isize))
            as (*mut _CatMessage);
        initCatMessage(pMetric, b'M', type_, name);
        (*pMetric).complete = setMetricComplete;
        pMetric
    }
}

unsafe extern "C" fn setHeartBeatComplete(mut message: *mut _CatMessage) {
    let mut pInner: *mut _CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatMessageInner>() as (isize)))
        as (*mut _CatMessageInner);
    (*pInner).isComplete = 1i32;
    catMessageManagerAdd(message);
}

#[no_mangle]
pub unsafe extern "C" fn createCatHeartBeat(
    mut type_: *const u8,
    mut name: *const u8,
) -> *mut _CatMessage {
    let mut pHeartBeatInner: *mut _CatMessageInner = malloc(
        ::std::mem::size_of::<_CatMessage>()
            .wrapping_add(::std::mem::size_of::<_CatMessageInner>()),
    ) as (*mut _CatMessageInner);
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut _CatMessageInner) == pHeartBeatInner {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut _CatMessage)
    } else {
        let mut pHB: *mut _CatMessage = (pHeartBeatInner as (*mut u8))
            .offset(::std::mem::size_of::<_CatMessageInner>() as (isize))
            as (*mut _CatMessage);
        initCatMessage(pHB, b'H', type_, name);
        (*pHB).complete = setHeartBeatComplete;
        pHB
    }
}
