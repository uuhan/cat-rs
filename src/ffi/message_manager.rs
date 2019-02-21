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
    fn catChecktPtrWithName(ptr: *mut ::std::os::raw::c_void, ptrName: *mut u8);
    fn catsdscpy(s: *mut u8, t: *const u8) -> *mut u8;
    fn catsdsfree(s: *mut u8);
    fn catsdsnew(init: *const u8) -> *mut u8;
    fn catsdsnewEmpty(preAlloclen: usize) -> *mut u8;
    fn copyCatMessageTree(pMsgTree: *mut _CatMessageTree) -> *mut _CatMessageTree;
    fn deleteCatMessageTree(pMsgTree: *mut _CatMessageTree);
    static mut g_config: _CatClientInnerConfig;
    fn getCatContext() -> *mut _CatContext;
    fn getLocalHostIp(ipBuf: *mut u8) -> i32;
    fn getLocalHostIpHex(ipHexBuf: *mut u8) -> i32;
    fn getNextMessageId() -> *mut u8;
    fn isCatSenderEnable() -> i32;
    fn sendRootMessage(tree: *mut _CatMessageTree) -> i32;
}

enum _CatMessage {}

enum _CatTransaction {}

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

#[no_mangle]
pub static mut g_cat_messageManager: _CatMessageManager = _CatMessageManager {
    domain: 0i32 as (*mut u8),
    hostname: 0 as (*mut u8),
    ip: 0 as (*mut u8),
    ipHex: 0 as (*mut u8),
    throttleTimes: 0isize,
};

#[derive(Copy)]
#[repr(C)]
pub struct _CatMessageTree {
    pub root: *mut _CatMessage,
    pub messageId: *mut u8,
    pub parentMessageId: *mut u8,
    pub rootMessageId: *mut u8,
    pub sessionToken: *mut u8,
    pub threadGroupName: *mut u8,
    pub threadId: *mut u8,
    pub threadName: *mut u8,
    pub canDiscard: i32,
}

impl Clone for _CatMessageTree {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct _CATStaticQueue {
    pub maxQueueSize: usize,
    pub head: i32,
    pub tail: i32,
    pub size: usize,
    // pub valueArray:
}

impl Clone for _CATStaticQueue {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct _CatContext {
    pub tree: *mut _CatMessageTree,
    pub transactionStack: *mut _CATStaticQueue,
    pub elementSize: i32,
    pub lastTruncateTransDurationUs: usize,
    pub addMessage: unsafe extern "C" fn(*mut _CatContext, *mut _CatMessage),
    pub addTransChild:
        unsafe extern "C" fn(*mut _CatContext, *mut _CatMessage, *mut _CatTransaction),
    pub adjustForTruncatedTrans: unsafe extern "C" fn(*mut _CatContext, *mut _CatTransaction),
    pub startTrans: unsafe extern "C" fn(*mut _CatContext, *mut _CatTransaction),
    pub endTrans: unsafe extern "C" fn(*mut _CatContext, *mut _CatTransaction),
    pub reset: unsafe extern "C" fn(*mut _CatContext),
}

impl Clone for _CatContext {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn catMessageManagerAdd(mut message: *mut _CatMessage) {
    let mut ctx: *mut _CatContext = getCatContext();
    ((*ctx).addMessage)(ctx, message);
}

unsafe extern "C" fn isCATStaticQueueEmpty(mut pQueue: *mut _CATStaticQueue) -> i32 {
    ((*pQueue).size == 0) as (i32)
}

#[no_mangle]
pub unsafe extern "C" fn catMessageManagerEndTrans(mut message: *mut _CatTransaction) {
    let mut ctx: *mut _CatContext = getCatContext();
    ((*ctx).endTrans)(ctx, message);
    if isCATStaticQueueEmpty((*ctx).transactionStack) != 0 {
        let mut copiedTree: *mut _CatMessageTree = copyCatMessageTree((*ctx).tree);
        if (*ctx).lastTruncateTransDurationUs > 0usize {
            ((*ctx).adjustForTruncatedTrans)(ctx, (*copiedTree).root as (*mut _CatTransaction));
        }
        catMessageManagerFlush(copiedTree);
        ((*ctx).reset)(ctx);
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
pub unsafe extern "C" fn catMessageManagerFlush(mut tree: *mut _CatMessageTree) {
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) == (*tree).messageId {
        (*tree).messageId = getNextMessageId();
    }
    if isCatSenderEnable() != 0 && (g_config.messageEnableFlag != 0) {
        sendRootMessage(tree);
    } else {
        deleteCatMessageTree(tree);
        g_cat_messageManager.throttleTimes = g_cat_messageManager.throttleTimes + 1isize;
        if g_cat_messageManager.throttleTimes == 1isize
            || g_cat_messageManager.throttleTimes % 1000000isize == 0isize
        {
            CLogLogWithLocation(
                0x4u16,
                (*b"Cat Message is throttled! Times: %d\0").as_ptr(),
                file!().as_ptr(),
                line!() as (i32),
                (*b"catMessageManagerFlush\0").as_ptr(),
                g_cat_messageManager.throttleTimes,
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn initMessageManager(mut domain: *const u8, mut hostName: *const u8) {
    g_cat_messageManager.domain = catsdsnew(domain);
    catChecktPtrWithName(
        g_cat_messageManager.domain as (*mut ::std::os::raw::c_void),
        (*b"g_cat_messageManager.domain\0").as_ptr() as (*mut u8),
    );
    g_cat_messageManager.hostname = catsdsnew(hostName);
    catChecktPtrWithName(
        g_cat_messageManager.hostname as (*mut ::std::os::raw::c_void),
        (*b"g_cat_messageManager.hostname\0").as_ptr() as (*mut u8),
    );
    g_cat_messageManager.ip = catsdsnewEmpty(64usize);
    catChecktPtrWithName(
        g_cat_messageManager.ip as (*mut ::std::os::raw::c_void),
        (*b"g_cat_messageManager.ip\0").as_ptr() as (*mut u8),
    );
    getLocalHostIp(g_cat_messageManager.ip);
    if *g_cat_messageManager.ip.offset(0isize) as (i32) == b'\0' as (i32) {
        CLogLogWithLocation(
            0x4u16,
            (*b"Cannot get self ip address, using default ip: %s\0").as_ptr(),
            file!().as_ptr(),
            line!() as (i32),
            (*b"initMessageManager\0").as_ptr(),
            g_config.defaultIp,
        );
        g_cat_messageManager.ip =
            catsdscpy(g_cat_messageManager.ip, g_config.defaultIp as (*const u8));
    }
    CLogLogWithLocation(
        0x2u16,
        (*b"Current ip: %s\0").as_ptr(),
        file!().as_ptr(),
        line!() as (i32),
        (*b"initMessageManager\0").as_ptr(),
        g_cat_messageManager.ip,
    );
    g_cat_messageManager.ipHex = catsdsnewEmpty(64usize);
    catChecktPtrWithName(
        g_cat_messageManager.ipHex as (*mut ::std::os::raw::c_void),
        (*b"g_cat_messageManager.ipHex\0").as_ptr() as (*mut u8),
    );
    getLocalHostIpHex(g_cat_messageManager.ipHex);
    if *g_cat_messageManager.ipHex.offset(0isize) as (i32) == b'\0' as (i32) {
        CLogLogWithLocation(
            0x4u16,
            (*b"Cannot get self ip address, using default ip hex: %s\0").as_ptr(),
            file!().as_ptr(),
            line!() as (i32),
            (*b"initMessageManager\0").as_ptr(),
            g_config.defaultIpHex,
        );
        g_cat_messageManager.ipHex = catsdscpy(
            g_cat_messageManager.ipHex,
            g_config.defaultIpHex as (*const u8),
        );
    }
    CLogLogWithLocation(
        0x2u16,
        (*b"Current ip hex: %s\0").as_ptr(),
        file!().as_ptr(),
        line!() as (i32),
        (*b"initMessageManager\0").as_ptr(),
        g_cat_messageManager.ipHex,
    );
}

#[no_mangle]
pub unsafe extern "C" fn catMessageManagerDestroy() {
    catsdsfree(g_cat_messageManager.domain);
    g_cat_messageManager.domain = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    catsdsfree(g_cat_messageManager.hostname);
    g_cat_messageManager.hostname = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    catsdsfree(g_cat_messageManager.ip);
    g_cat_messageManager.ip = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    catsdsfree(g_cat_messageManager.ipHex);
    g_cat_messageManager.ipHex = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
}

#[no_mangle]
pub unsafe extern "C" fn catMessageManagerStartTrans(mut trans: *mut _CatTransaction) {
    let mut ctx: *mut _CatContext = getCatContext();
    ((*ctx).startTrans)(ctx, trans);
}
