use super::raw::CATStaticQueue;
use super::raw::CatClientInnerConfig;
use super::raw::CatContext;
use super::raw::CatMessage;
use super::raw::CatMessageManager;
use super::raw::CatMessageTree;
use super::raw::CatTransaction;
use super::sds::catsdscpy;
use super::sds::catsdsfree;
use super::sds::catsdsnew;
use super::sds::catsdsnewEmpty;

extern "C" {
    fn catChecktPtrWithName(ptr: *mut ::std::os::raw::c_void, ptrName: *mut u8);
    fn copyCatMessageTree(pMsgTree: *mut CatMessageTree) -> *mut CatMessageTree;
    fn deleteCatMessageTree(pMsgTree: *mut CatMessageTree);
    static mut g_config: CatClientInnerConfig;
    static mut g_cat_messageManager: CatMessageManager;
    fn getCatContext() -> *mut CatContext;
    fn getLocalHostIp(ipBuf: *mut u8) -> i32;
    fn getLocalHostIpHex(ipHexBuf: *mut u8) -> i32;
    fn getNextMessageId() -> *mut u8;
    fn isCatSenderEnable() -> i32;
    fn sendRootMessage(tree: *mut CatMessageTree) -> i32;
}

pub unsafe extern "C" fn catMessageManagerAdd(mut message: *mut CatMessage) {
    let mut ctx: *mut CatContext = getCatContext();
    ((*ctx).addMessage)(ctx, message);
}

unsafe extern "C" fn isCATStaticQueueEmpty(mut pQueue: *mut CATStaticQueue) -> i32 {
    ((*pQueue).size == 0) as (i32)
}

pub unsafe extern "C" fn catMessageManagerEndTrans(mut message: *mut CatTransaction) {
    let mut ctx: *mut CatContext = getCatContext();
    ((*ctx).endTrans)(ctx, message);
    if isCATStaticQueueEmpty((*ctx).transactionStack) != 0 {
        let mut copiedTree: *mut CatMessageTree = copyCatMessageTree((*ctx).tree);
        if (*ctx).lastTruncateTransDurationUs > 0usize {
            ((*ctx).adjustForTruncatedTrans)(ctx, (*copiedTree).root as (*mut CatTransaction));
        }
        catMessageManagerFlush(copiedTree);
        ((*ctx).reset)(ctx);
    }
}

pub unsafe extern "C" fn catMessageManagerFlush(mut tree: *mut CatMessageTree) {
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
            warn!("Cat Message is throttled! Times");
        }
    }
}

pub unsafe fn initMessageManager(mut domain: *const u8, mut hostName: *const u8) {
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
        warn!("Cannot get self ip address, using default ip");
        g_cat_messageManager.ip =
            catsdscpy(g_cat_messageManager.ip, g_config.defaultIp as (*const u8));
    }
    g_cat_messageManager.ipHex = catsdsnewEmpty(64usize);
    catChecktPtrWithName(
        g_cat_messageManager.ipHex as (*mut ::std::os::raw::c_void),
        (*b"g_cat_messageManager.ipHex\0").as_ptr() as (*mut u8),
    );
    getLocalHostIpHex(g_cat_messageManager.ipHex);
    if *g_cat_messageManager.ipHex.offset(0isize) as (i32) == b'\0' as (i32) {
        warn!("Cannot get self ip address, using default ip hex");
        g_cat_messageManager.ipHex = catsdscpy(
            g_cat_messageManager.ipHex,
            g_config.defaultIpHex as (*const u8),
        );
    }
}

pub unsafe fn catMessageManagerDestroy() {
    catsdsfree(g_cat_messageManager.domain);
    g_cat_messageManager.domain = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    catsdsfree(g_cat_messageManager.hostname);
    g_cat_messageManager.hostname = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    catsdsfree(g_cat_messageManager.ip);
    g_cat_messageManager.ip = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    catsdsfree(g_cat_messageManager.ipHex);
    g_cat_messageManager.ipHex = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
}

pub unsafe extern "C" fn catMessageManagerStartTrans(mut trans: *mut CatTransaction) {
    let mut ctx: *mut CatContext = getCatContext();
    ((*ctx).startTrans)(ctx, trans);
}
