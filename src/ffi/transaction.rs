use super::helper::GetTime64;
use super::raw::{
    CATStaticQueue, CatClientInnerConfig, CatMessage, CatMessageInner, CatTransaction,
    CatTransactionInner,
};
use super::sds::catsdsdup;
use libc::{gettimeofday, malloc, timeval};
use std::mem;

extern "C" {
    fn catMessageManagerEndTrans(trans: *mut CatTransaction);
    fn clearMessage(message: *mut CatMessage) -> *mut ::std::os::raw::c_void;
    fn createCATStaticQueue(maxQueueSize: usize) -> *mut CATStaticQueue;
    fn destroyCATStaticQueue(pQueue: *mut CATStaticQueue);
    fn free(arg1: *mut ::std::os::raw::c_void);
    static mut g_config: CatClientInnerConfig;
    fn getCATStaticQueueByIndex(
        pQueue: *mut CATStaticQueue,
        index: usize,
    ) -> *mut ::std::os::raw::c_void;
    fn initCatMessage(pMsg: *mut CatMessage, msgType: u8, type_: *const u8, name: *const u8);
    fn isCatTransaction(message: *mut CatMessage) -> i32;
    fn pushBackCATStaticQueue(
        pQueue: *mut CATStaticQueue,
        pData: *mut ::std::os::raw::c_void,
    ) -> i32;
}

fn addDataPairNull(mut message: *mut CatTransaction, mut data: *const u8) {}
fn addKVNull(mut message: *mut CatTransaction, mut dataKey: *const u8, mut dataValue: *const u8) {}
fn setStatusNull(mut message: *mut CatTransaction, mut status: *const u8) {}
fn setTimestampNull(mut message: *mut CatTransaction, mut timestamp: u64) {}
fn setCompleteNull(mut message: *mut CatTransaction) {}
fn addChildNull(mut message: *mut CatTransaction, mut childMsg: *const CatMessage) {}
fn setDurationInMillisNull(mut trans: *mut CatTransaction, mut duration: u64) {}
fn setDurationStartNull(mut trans: *mut CatTransaction, mut durationStart: u64) {}

// pub static mut g_cat_nullTrans: CatTransaction = addDataPairNull as (CatTransaction);

unsafe extern "C" fn deleteCatMessage(mut message: *mut CatMessage) {
    let mut p: *mut ::std::os::raw::c_void;
    if isCatTransaction(message) != 0 {
        p = clearTransaction(message);
    } else {
        p = clearMessage(message);
    }
    free(p);
}

unsafe extern "C" fn getCATStaticQueueSize(mut pQueue: *mut CATStaticQueue) -> usize {
    (*pQueue).size
}

pub unsafe extern "C" fn clearTransaction(
    mut message: *mut CatMessage,
) -> *mut ::std::os::raw::c_void {
    let mut pInner: *mut CatTransactionInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<CatTransactionInner>() as (isize)))
        as (*mut CatTransactionInner);
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

pub unsafe extern "C" fn getCatTransactionChildren(
    mut pSrcTrans: *mut CatTransaction,
) -> *mut CATStaticQueue {
    let mut pInner: *mut CatTransactionInner = (pSrcTrans as (*mut u8))
        .offset(-(::std::mem::size_of::<CatTransactionInner>() as (isize)))
        as (*mut CatTransactionInner);
    (*pInner).children
}

unsafe extern "C" fn setTransactionComplete(mut message: *mut CatTransaction) {
    let mut pInner: *mut CatTransactionInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<CatTransactionInner>() as (isize)))
        as (*mut CatTransactionInner);
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
    let mut pInner: *mut CatTransactionInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<CatTransactionInner>() as (isize)))
        as (*mut CatTransactionInner);
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
    let mut pInner: *mut CatTransactionInner = (trans as (*mut u8))
        .offset(-(::std::mem::size_of::<CatTransactionInner>() as (isize)))
        as (*mut CatTransactionInner);
    (*pInner).durationUs = durationUs;
}

unsafe extern "C" fn setDurationInMillis(mut trans: *mut CatTransaction, mut duration: usize) {
    setCatTransactionDurationUs(trans, duration.wrapping_mul(1000usize));
}

unsafe extern "C" fn setDurationStart(mut trans: *mut CatTransaction, mut start: usize) {
    let mut tInner: *mut CatTransactionInner = (trans as (*mut u8))
        .offset(-(::std::mem::size_of::<CatTransactionInner>() as (isize)))
        as (*mut CatTransactionInner);
    (*tInner).durationStart = start.wrapping_mul(1000usize).wrapping_mul(1000usize);
}

pub unsafe extern "C" fn createCatTransaction(
    mut type_: *const u8,
    mut name: *const u8,
) -> *mut CatTransaction {
    let mut pTransInner: *mut CatTransactionInner = malloc(
        ::std::mem::size_of::<CatTransaction>()
            .wrapping_add(::std::mem::size_of::<CatTransactionInner>()),
    ) as (*mut CatTransactionInner);
    if pTransInner == 0i32 as (*mut ::std::os::raw::c_void) as (*mut CatTransactionInner) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut CatTransaction)
    } else {
        let mut pTrans: *mut CatTransaction = (pTransInner as (*mut u8))
            .offset(::std::mem::size_of::<CatTransactionInner>() as (isize))
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

pub unsafe extern "C" fn getCatTransactionDurationUs(mut trans: *mut CatTransaction) -> usize {
    let mut pInner: *mut CatTransactionInner = (trans as (*mut u8))
        .offset(-(::std::mem::size_of::<CatTransactionInner>() as (isize)))
        as (*mut CatTransactionInner);
    if (*pInner).durationUs > 0usize {
        (*pInner).durationUs
    } else {
        let mut tmpDuration: usize = 0usize;
        let mut len: usize = if (*pInner).children
            == 0i32 as (*mut ::std::os::raw::c_void) as (*mut CATStaticQueue)
        {
            0usize
        } else {
            getCATStaticQueueSize((*pInner).children)
        };
        if len > 0usize
            && ((*pInner).children
                != 0i32 as (*mut ::std::os::raw::c_void) as (*mut CATStaticQueue))
        {
            let mut lastChild: *mut CatMessage =
                getCATStaticQueueByIndex((*pInner).children, len.wrapping_sub(1usize))
                    as (*mut CatMessage);
            let mut lastChildInner: *mut CatMessageInner = (lastChild as (*mut u8))
                .offset(-(::std::mem::size_of::<CatMessageInner>() as (isize)))
                as (*mut CatMessageInner);
            if isCatTransaction(lastChild) != 0 {
                let mut pInner: *mut CatTransactionInner = (lastChild as (*mut u8))
                    .offset(-(::std::mem::size_of::<CatTransactionInner>() as (isize)))
                    as (*mut CatTransactionInner);
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

pub unsafe extern "C" fn copyCatTransaction(
    mut pSrcTrans: *mut CatTransaction,
) -> *mut CatTransaction {
    let mut pSrcTransInner: *mut CatTransactionInner = (pSrcTrans as (*mut u8))
        .offset(-(::std::mem::size_of::<CatTransactionInner>() as (isize)))
        as (*mut CatTransactionInner);
    let mut clonedTrans: *mut CatTransaction = createCatTransaction(
        (*pSrcTransInner).message.type_ as (*const u8),
        (*pSrcTransInner).message.name as (*const u8),
    );
    let mut clonedTransInner: *mut CatTransactionInner = (clonedTrans as (*mut u8))
        .offset(-(::std::mem::size_of::<CatTransactionInner>() as (isize)))
        as (*mut CatTransactionInner);
    (*clonedTransInner).message.timestampMs = (*pSrcTransInner).message.timestampMs;
    (*clonedTransInner).durationUs = getCatTransactionDurationUs(pSrcTrans);
    (*clonedTransInner).message.data = catsdsdup((*pSrcTransInner).message.data);
    clonedTrans
}
