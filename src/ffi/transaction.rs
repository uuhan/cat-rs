extern "C" {
    fn CLogLogWithLocation(
        type_: u16,
        format: *const u8,
        file: *const u8,
        line: i32,
        function: *const u8,
        ...
    );
    fn __swbuf(arg1: i32, arg2: *mut __sFILE) -> i32;
    fn catMessageManagerEndTrans(trans: *mut _CatTransaction);
    fn catsdsdup(s: *mut u8) -> *mut u8;
    fn clearMessage(message: *mut _CatMessage) -> *mut ::std::os::raw::c_void;
    fn createCATStaticQueue(maxQueueSize: usize) -> *mut _CATStaticQueue;
    fn destroyCATStaticQueue(pQueue: *mut _CATStaticQueue);
    fn free(arg1: *mut ::std::os::raw::c_void);
    static mut g_config: _CatClientInnerConfig;
    fn getCATStaticQueueByIndex(
        pQueue: *mut _CATStaticQueue,
        index: usize,
    ) -> *mut ::std::os::raw::c_void;
    fn gettimeofday(arg1: *mut timeval, arg2: *mut ::std::os::raw::c_void) -> i32;
    fn initCatMessage(pMsg: *mut _CatMessage, msgType: u8, type_: *const u8, name: *const u8);
    fn isCatTransaction(message: *mut _CatMessage) -> i32;
    fn malloc(__size: usize) -> *mut ::std::os::raw::c_void;
    fn pushBackCATStaticQueue(
        pQueue: *mut _CATStaticQueue,
        pData: *mut ::std::os::raw::c_void,
    ) -> i32;
}

enum Union1 {}

enum _CatMessage {}

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

unsafe extern "C" fn addDataPairNull(mut message: *mut _CatTransaction, mut data: *const u8) {}

#[no_mangle]
pub static mut g_cat_nullTrans: _CatTransaction = addDataPairNull as (_CatTransaction);

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

#[derive(Copy)]
#[repr(C)]
pub struct _CatTranscationInner {
    pub children: *mut _CATStaticQueue,
    pub durationStart: usize,
    pub durationUs: usize,
    pub message: _CatMessageInner,
}

impl Clone for _CatTranscationInner {
    fn clone(&self) -> Self {
        *self
    }
}

unsafe extern "C" fn deleteCatMessage(mut message: *mut _CatMessage) {
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
    mut message: *mut _CatMessage,
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
        let mut pMessage: *mut _CatMessage =
            getCATStaticQueueByIndex((*pInner).children, i) as (*mut _CatMessage);
        deleteCatMessage(pMessage);
        i = i.wrapping_add(1usize);
    }
    destroyCATStaticQueue((*pInner).children);
    pInner as (*mut ::std::os::raw::c_void)
}

#[no_mangle]
pub unsafe extern "C" fn getCatTransactionChildren(
    mut pSrcTrans: *mut _CatTransaction,
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

unsafe extern "C" fn setTransactionComplete(mut message: *mut _CatTransaction) {
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

unsafe extern "C" fn addChild(mut message: *mut _CatTransaction, mut childMsg: *mut _CatMessage) {
    let mut pInner: *mut _CatTranscationInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatTranscationInner>() as (isize)))
        as (*mut _CatTranscationInner);
    let mut pushRst: i32 = pushBackCATStaticQueue(
        (*pInner).children,
        childMsg as (*mut ::std::os::raw::c_void),
    );
    if -1i32 == pushRst {
        CLogLogWithLocation(
            0x8u16,
            (*b"Transaction Add Child Error\xEF\xBC\x81\0").as_ptr(),
            file!().as_ptr(),
            line!() as (i32),
            (*b"addChild\0").as_ptr(),
            getCATStaticQueueSize((*pInner).children),
        );
    }
}

unsafe extern "C" fn setCatTransactionDurationUs(
    mut trans: *mut _CatTransaction,
    mut durationUs: usize,
) {
    let mut pInner: *mut _CatTranscationInner = (trans as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatTranscationInner>() as (isize)))
        as (*mut _CatTranscationInner);
    (*pInner).durationUs = durationUs;
}

unsafe extern "C" fn setDurationInMillis(mut trans: *mut _CatTransaction, mut duration: usize) {
    setCatTransactionDurationUs(trans, duration.wrapping_mul(1000usize));
}

unsafe extern "C" fn setDurationStart(mut trans: *mut _CatTransaction, mut start: usize) {
    let mut tInner: *mut _CatTranscationInner = (trans as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatTranscationInner>() as (isize)))
        as (*mut _CatTranscationInner);
    (*tInner).durationStart = start.wrapping_mul(1000usize).wrapping_mul(1000usize);
}

#[no_mangle]
pub unsafe extern "C" fn createCatTransaction(
    mut type_: *const u8,
    mut name: *const u8,
) -> *mut _CatTransaction {
    let mut pTransInner: *mut _CatTranscationInner = malloc(
        ::std::mem::size_of::<_CatTransaction>()
            .wrapping_add(::std::mem::size_of::<_CatTranscationInner>()),
    ) as (*mut _CatTranscationInner);
    if pTransInner == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _CatTranscationInner) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut _CatTransaction)
    } else {
        let mut pTrans: *mut _CatTransaction = (pTransInner as (*mut u8))
            .offset(::std::mem::size_of::<_CatTranscationInner>() as (isize))
            as (*mut _CatTransaction);
        initCatMessage(pTrans as (*mut _CatMessage), b'T', type_, name);
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
pub unsafe extern "C" fn getCatTransactionDurationUs(mut trans: *mut _CatTransaction) -> usize {
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
            let mut lastChild: *mut _CatMessage =
                getCATStaticQueueByIndex((*pInner).children, len.wrapping_sub(1usize))
                    as (*mut _CatMessage);
            let mut lastChildInner: *mut _CatMessageInner = (lastChild as (*mut u8))
                .offset(-(::std::mem::size_of::<_CatMessageInner>() as (isize)))
                as (*mut _CatMessageInner);
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
    mut pSrcTrans: *mut _CatTransaction,
) -> *mut _CatTransaction {
    let mut pSrcTransInner: *mut _CatTranscationInner = (pSrcTrans as (*mut u8))
        .offset(-(::std::mem::size_of::<_CatTranscationInner>() as (isize)))
        as (*mut _CatTranscationInner);
    let mut clonedTrans: *mut _CatTransaction = createCatTransaction(
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
