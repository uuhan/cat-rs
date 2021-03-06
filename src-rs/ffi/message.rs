use super::helper::GetTime64;
use super::raw::CatMessage;
use super::raw::CatMessageInner;
use super::raw::MessageType;
use super::sds::catsdscat;
use super::sds::catsdscpy;
use super::sds::catsdsfree;
use super::sds::catsdsnew;
use libc::malloc;
use libc::memset;
use libc::timeval;
use std::ptr;

extern "C" {
    fn catMessageManagerAdd(message: *mut CatMessage);
}

unsafe extern "C" fn addDataNull(mut message: *mut CatMessage, mut data: *const u8) {}

// #[no_mangle]
// pub static mut g_cat_nullMsg: CatMessage = addDataNull as (CatMessage);

pub unsafe fn clearMessage(mut message: *mut CatMessage) -> *mut ::std::os::raw::c_void {
    let mut pInner: *mut CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<CatMessageInner>() as (isize)))
        as (*mut CatMessageInner);
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

unsafe extern "C" fn addData(mut message: *mut CatMessage, mut data: *const u8) {
    let mut pInner: *mut CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<CatMessageInner>() as (isize)))
        as (*mut CatMessageInner);
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) == (*pInner).data {
        (*pInner).data = catsdsnew(data);
    } else {
        (*pInner).data = catsdscat((*pInner).data, (*b"&\0").as_ptr());
        (*pInner).data = catsdscat((*pInner).data, data);
    }
}

unsafe extern "C" fn addKV(
    mut message: *mut CatMessage,
    mut dataKey: *const u8,
    mut dataValue: *const u8,
) {
    let mut pInner: *mut CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<CatMessageInner>() as (isize)))
        as (*mut CatMessageInner);
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

unsafe extern "C" fn setComplete(mut message: *mut CatMessage) {
    let mut pInner: *mut CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<CatMessageInner>() as (isize)))
        as (*mut CatMessageInner);
    (*pInner).isComplete = 1i32;
}

unsafe extern "C" fn setTimestamp(mut message: *mut CatMessage, mut timeMs: usize) {
    let mut pInner: *mut CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<CatMessageInner>() as (isize)))
        as (*mut CatMessageInner);
    (*pInner).timestampMs = timeMs;
}

unsafe extern "C" fn setStatus(mut message: *mut CatMessage, mut status: *const u8) {
    let mut pInner: *mut CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<CatMessageInner>() as (isize)))
        as (*mut CatMessageInner);
    if (*pInner).status == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        (*pInner).status = catsdsnew(status);
    } else {
        (*pInner).status = catsdscpy((*pInner).status, status);
    }
}

pub unsafe fn initCatMessage(
    mut pMsg: *mut CatMessage,
    mut msgType: u8,
    mut type_: *const u8,
    mut name: *const u8,
) {
    let mut pInner: *mut CatMessageInner = (pMsg as (*mut u8))
        .offset(-(::std::mem::size_of::<CatMessageInner>() as (isize)))
        as (*mut CatMessageInner);
    memset(
        pInner as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<CatMessage>().wrapping_add(::std::mem::size_of::<CatMessageInner>()),
    );
    (*pInner).messageType = MessageType {
        type_: msgType as i8,
    };
    (*pInner).timestampMs = GetTime64();
    (*pInner).type_ = catsdsnew(type_);
    (*pInner).name = catsdsnew(name);
    (*pMsg).addData = addData;
    (*pMsg).addKV = addKV;
    (*pMsg).complete = setComplete;
    (*pMsg).setTimestamp = setTimestamp;
    (*pMsg).setStatus = setStatus;
}

unsafe extern "C" fn setEventComplete(mut message: *mut CatMessage) {
    let mut pInner: *mut CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<CatMessageInner>() as (isize)))
        as (*mut CatMessageInner);
    (*pInner).isComplete = 1;
    catMessageManagerAdd(message);
}

pub unsafe fn createCatEvent(mut type_: *const u8, mut name: *const u8) -> *mut CatMessage {
    let mut pEventInner: *mut CatMessageInner = malloc(
        ::std::mem::size_of::<CatMessage>().wrapping_add(::std::mem::size_of::<CatMessageInner>()),
    ) as (*mut CatMessageInner);
    if pEventInner.is_null() {
        ptr::null_mut()
    } else {
        let mut pEvent: *mut CatMessage = (pEventInner as (*mut u8))
            .offset(::std::mem::size_of::<CatMessageInner>() as (isize))
            as (*mut CatMessage);
        initCatMessage(pEvent, b'E', type_, name);
        (*pEvent).complete = setEventComplete;
        pEvent
    }
}

unsafe extern "C" fn setMetricComplete(mut message: *mut CatMessage) {
    let mut pInner: *mut CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<CatMessageInner>() as (isize)))
        as (*mut CatMessageInner);
    (*pInner).isComplete = 1i32;
    catMessageManagerAdd(message);
}

pub unsafe fn createCatMetric(mut type_: *const u8, mut name: *const u8) -> *mut CatMessage {
    let mut pMetricInner: *mut CatMessageInner = malloc(
        ::std::mem::size_of::<CatMessage>().wrapping_add(::std::mem::size_of::<CatMessageInner>()),
    ) as (*mut CatMessageInner);
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut CatMessageInner) == pMetricInner {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut CatMessage)
    } else {
        let mut pMetric: *mut CatMessage = (pMetricInner as (*mut u8))
            .offset(::std::mem::size_of::<CatMessageInner>() as (isize))
            as (*mut CatMessage);
        initCatMessage(pMetric, b'M', type_, name);
        (*pMetric).complete = setMetricComplete;
        pMetric
    }
}

unsafe extern "C" fn setHeartBeatComplete(mut message: *mut CatMessage) {
    let mut pInner: *mut CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<CatMessageInner>() as (isize)))
        as (*mut CatMessageInner);
    (*pInner).isComplete = 1i32;
    catMessageManagerAdd(message);
}

pub unsafe fn createCatHeartBeat(mut type_: *const u8, mut name: *const u8) -> *mut CatMessage {
    let mut pHeartBeatInner: *mut CatMessageInner = malloc(
        ::std::mem::size_of::<CatMessage>().wrapping_add(::std::mem::size_of::<CatMessageInner>()),
    ) as (*mut CatMessageInner);
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut CatMessageInner) == pHeartBeatInner {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut CatMessage)
    } else {
        let mut pHB: *mut CatMessage = (pHeartBeatInner as (*mut u8))
            .offset(::std::mem::size_of::<CatMessageInner>() as (isize))
            as (*mut CatMessage);
        initCatMessage(pHB, b'H', type_, name);
        (*pHB).complete = setHeartBeatComplete;
        pHB
    }
}
