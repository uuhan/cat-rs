use super::raw::CatMessage;
use super::raw::CatTransaction;
use super::sds::catsdsdup;
use libc::{gettimeofday, malloc, timeval};
use std::mem;

extern "C" {
    fn catMessageManagerEndTrans(trans: *mut CatTransaction);
    fn clearMessage(message: *mut CatMessage) -> *mut ::std::os::raw::c_void;
    fn createCATStaticQueue(maxQueueSize: usize) -> *mut _CATStaticQueue;
    fn destroyCATStaticQueue(pQueue: *mut _CATStaticQueue);
    fn free(arg1: *mut ::std::os::raw::c_void);
    static mut g_config: _CatClientInnerConfig;
    fn getCATStaticQueueByIndex(
        pQueue: *mut _CATStaticQueue,
        index: usize,
    ) -> *mut ::std::os::raw::c_void;
    fn initCatMessage(pMsg: *mut CatMessage, msgType: u8, type_: *const u8, name: *const u8);
    fn isCatTransaction(message: *mut CatMessage) -> i32;
    fn pushBackCATStaticQueue(
        pQueue: *mut _CATStaticQueue,
        pData: *mut ::std::os::raw::c_void,
    ) -> i32;
}

#[derive(Copy)]
pub enum Union1 {}

impl Clone for Union1 {
    fn clone(&self) -> Self {
        *self
    }
}

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

unsafe extern "C" fn addDataPairNull(mut message: *mut CatTransaction, mut data: *const u8) {}

// #[no_mangle]
// pub static mut g_cat_nullTrans: CatTransaction = addDataPairNull as (CatTransaction);

#[derive(Copy)]
#[repr(C)]
pub struct _CATStaticQueue {
    pub maxQueueSize: usize,
    pub head: i32,
    pub tail: i32,
    pub size: usize,
    pub valueArray: [*mut ::std::os::raw::c_void; 0],
}

impl Clone for _CATStaticQueue {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct CatMessageInner {
    pub messageType: Union1,
    pub type_: *mut u8,
    pub name: *mut u8,
    pub status: *mut u8,
    pub data: *mut u8,
    pub timestampMs: usize,
    pub isComplete: i32,
}

impl Clone for CatMessageInner {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct _CatTranscationInner {
    pub children: *mut _CATStaticQueue,
    pub durationStart: usize,
    pub durationUs: usize,
    pub message: CatMessageInner,
}

impl Clone for _CatTranscationInner {
    fn clone(&self) -> Self {
        *self
    }
}

unsafe extern "C" fn deleteCatMessage(mut message: *mut CatMessage) {
    let mut p: *mut ::std::os::raw::c_void;
    if isCatTransaction(message) != 0 {
        p = clearTransaction(message);
    } else {
        p = clearMessage(message);
    }
    free(p);
}

unsafe extern "C" fn getCATStaticQueueSize(mut pQueue: *mut _CATStaticQueue) -> usize {
    (*pQueue).size
}

#[no_mangle]
pub unsafe extern "C" fn clearTransaction(
    mut message: *mut CatMessage,
) -> *mut ::std::os::raw::c_void {
    let mut pInner: *mut _CatTranscationInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatTranscationInner>() as (isize)))
        as (*mut _CatTranscationInner);
    clearMessage(message);
    let mut i: usize = 0usize;
    'loop1: loop {
        if !(i < getCATStaticQueueSize((*pInner).children)) {
            break;
        }
        let mut pMessage: *mut CatMessage =
            getCATStaticQueueByIndex((*pInner).children, i) as (*mut CatMessage);
        deleteCatMessage(pMessage);
        i = i.wrapping_add(1usize);
    }
    destroyCATStaticQueue((*pInner).children);
    pInner as (*mut ::std::os::raw::c_void)
}

#[no_mangle]
pub unsafe extern "C" fn getCatTransactionChildren(
    mut pSrcTrans: *mut CatTransaction,
) -> *mut _CATStaticQueue {
    let mut pInner: *mut _CatTranscationInner = (pSrcTrans as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatTranscationInner>() as (isize)))
        as (*mut _CatTranscationInner);
    (*pInner).children
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

unsafe extern "C" fn GetTime64() -> usize {
    let mut buf: usize;
    let mut tv: timeval = mem::uninitialized();
    gettimeofday(
        &mut tv as (*mut timeval),
        0i32 as (*mut ::std::os::raw::c_void),
    );
    buf = (tv.tv_sec * 1000 + (tv.tv_usec / 1000) as i64) as (usize);
    buf
}

unsafe extern "C" fn setTransactionComplete(mut message: *mut CatTransaction) {
    let mut pInner: *mut _CatTranscationInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatTranscationInner>() as (isize)))
        as (*mut _CatTranscationInner);
    if (*pInner).message.isComplete == 0 {
        if (*pInner).durationUs == 0usize {
            (*pInner).durationUs = GetTime64()
                .wrapping_mul(1000usize)
                .wrapping_sub((*pInner).durationStart.wrapping_div(1000usize));
        }
        (*pInner).message.isComplete = 1i32;
        catMessageManagerEndTrans(message);
    }
}

unsafe extern "C" fn addChild(mut message: *mut CatTransaction, mut childMsg: *mut CatMessage) {
    let mut pInner: *mut _CatTranscationInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatTranscationInner>() as (isize)))
        as (*mut _CatTranscationInner);
    let mut pushRst: i32 = pushBackCATStaticQueue(
        (*pInner).children,
        childMsg as (*mut ::std::os::raw::c_void),
    );
    if -1i32 == pushRst {
        error!("Transaction Add Child Error");
    }
}

unsafe extern "C" fn setCatTransactionDurationUs(
    mut trans: *mut CatTransaction,
    mut durationUs: usize,
) {
    let mut pInner: *mut _CatTranscationInner = (trans as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatTranscationInner>() as (isize)))
        as (*mut _CatTranscationInner);
    (*pInner).durationUs = durationUs;
}

unsafe extern "C" fn setDurationInMillis(mut trans: *mut CatTransaction, mut duration: usize) {
    setCatTransactionDurationUs(trans, duration.wrapping_mul(1000usize));
}

unsafe extern "C" fn setDurationStart(mut trans: *mut CatTransaction, mut start: usize) {
    let mut tInner: *mut _CatTranscationInner = (trans as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatTranscationInner>() as (isize)))
        as (*mut _CatTranscationInner);
    (*tInner).durationStart = start.wrapping_mul(1000usize).wrapping_mul(1000usize);
}

#[no_mangle]
pub unsafe extern "C" fn createCatTransaction(
    mut type_: *const u8,
    mut name: *const u8,
) -> *mut CatTransaction {
    let mut pTransInner: *mut _CatTranscationInner = malloc(
        ::std::mem::size_of::<CatTransaction>()
            .wrapping_add(::std::mem::size_of::<_CatTranscationInner>()),
    ) as (*mut _CatTranscationInner);
    if pTransInner == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _CatTranscationInner) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut CatTransaction)
    } else {
        let mut pTrans: *mut CatTransaction = (pTransInner as (*mut u8))
            .offset(::std::mem::size_of::<_CatTranscationInner>() as (isize))
            as (*mut CatTransaction);
        initCatMessage(pTrans as (*mut CatMessage), b'T', type_, name);
        (*pTransInner).children = createCATStaticQueue(g_config.maxChildSize as (usize));
        (*pTransInner).durationStart = GetTime64().wrapping_mul(1000usize).wrapping_mul(1000usize);
        (*pTransInner).durationUs = 0usize;
        (*pTrans).complete = setTransactionComplete;
        (*pTrans).addChild = addChild;
        (*pTrans).setDurationInMillis = setDurationInMillis;
        (*pTrans).setDurationStart = setDurationStart;
        ((*pTrans).setStatus)(pTrans, (*b"0\0").as_ptr());
        pTrans
    }
}

#[no_mangle]
pub unsafe extern "C" fn getCatTransactionDurationUs(mut trans: *mut CatTransaction) -> usize {
    let mut pInner: *mut _CatTranscationInner = (trans as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatTranscationInner>() as (isize)))
        as (*mut _CatTranscationInner);
    if (*pInner).durationUs > 0usize {
        (*pInner).durationUs
    } else {
        let mut tmpDuration: usize = 0usize;
        let mut len: usize = if (*pInner).children
            == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _CATStaticQueue)
        {
            0usize
        } else {
            getCATStaticQueueSize((*pInner).children)
        };
        if len > 0usize
            && ((*pInner).children
                != 0i32 as (*mut ::std::os::raw::c_void) as (*mut _CATStaticQueue))
        {
            let mut lastChild: *mut CatMessage =
                getCATStaticQueueByIndex((*pInner).children, len.wrapping_sub(1usize))
                    as (*mut CatMessage);
            let mut lastChildInner: *mut CatMessageInner = (lastChild as (*mut u8))
                .offset(-(::std::mem::size_of::<CatMessageInner>() as (isize)))
                as (*mut CatMessageInner);
            if isCatTransaction(lastChild) != 0 {
                let mut pInner: *mut _CatTranscationInner = (lastChild as (*mut u8))
                    .offset(-(::std::mem::size_of::<_CatTranscationInner>() as (isize)))
                    as (*mut _CatTranscationInner);
                tmpDuration = (*lastChildInner)
                    .timestampMs
                    .wrapping_sub((*pInner).message.timestampMs)
                    .wrapping_mul(1000usize)
                    .wrapping_add((*pInner).durationUs);
            } else {
                tmpDuration = (*lastChildInner)
                    .timestampMs
                    .wrapping_sub((*pInner).message.timestampMs)
                    .wrapping_mul(1000usize);
            }
        }
        tmpDuration
    }
}

#[no_mangle]
pub unsafe extern "C" fn copyCatTransaction(
    mut pSrcTrans: *mut CatTransaction,
) -> *mut CatTransaction {
    let mut pSrcTransInner: *mut _CatTranscationInner = (pSrcTrans as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatTranscationInner>() as (isize)))
        as (*mut _CatTranscationInner);
    let mut clonedTrans: *mut CatTransaction = createCatTransaction(
        (*pSrcTransInner).message.type_ as (*const u8),
        (*pSrcTransInner).message.name as (*const u8),
    );
    let mut clonedTransInner: *mut _CatTranscationInner = (clonedTrans as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatTranscationInner>() as (isize)))
        as (*mut _CatTranscationInner);
    (*clonedTransInner).message.timestampMs = (*pSrcTransInner).message.timestampMs;
    (*clonedTransInner).durationUs = getCatTransactionDurationUs(pSrcTrans);
    (*clonedTransInner).message.data = catsdsdup((*pSrcTransInner).message.data);
    clonedTrans
}
