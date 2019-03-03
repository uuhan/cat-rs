use super::raw::CATStaticQueue;
use super::raw::CatClientInnerConfig;
use super::raw::CatContext;
use super::raw::CatMessage;
use super::raw::CatMessageInner;
use super::raw::CatMessageTree;
use super::raw::CatTransaction;
use super::sds::catsdsfree;
use super::sds::catsdsfromlonglong;
use super::sds::catsdsnew;
use libc::malloc;
use std::mem;

extern "C" {
    fn catChecktPtrWithName(ptr: *mut ::std::os::raw::c_void, ptrName: *mut u8);
    fn catItoA(val: i32, buf: *mut u8, radix: i32) -> *mut u8;
    fn catMessageManagerFlush(tree: *mut CatMessageTree);
    fn clearCATStaticQueue(pQueue: *mut CATStaticQueue);
    fn copyCatMessageTree(pMsgTree: *mut CatMessageTree) -> *mut CatMessageTree;
    fn createCATStaticQueue(maxQueueSize: usize) -> *mut CATStaticQueue;
    fn createCatEvent(type_: *const u8, name: *const u8) -> *mut CatMessage;
    fn createCatMessageTree() -> *mut CatMessageTree;
    static mut g_config: CatClientInnerConfig;
    fn getCATStaticQueueByIndex(
        pQueue: *mut CATStaticQueue,
        index: usize,
    ) -> *mut ::std::os::raw::c_void;
    fn getCatTransactionDurationUs(trans: *mut CatTransaction) -> usize;
    fn popFrontCATStaticQueue(pQueue: *mut CATStaticQueue) -> *mut ::std::os::raw::c_void;
    fn pryFrontCATStaticQueue(pQueue: *mut CATStaticQueue) -> *mut ::std::os::raw::c_void;
    fn pthread_threadid_np(arg1: *mut _opaque_pthread_t, arg2: *mut usize) -> i32;
    fn pushFrontCATStaticQueue(
        pQueue: *mut CATStaticQueue,
        pData: *mut ::std::os::raw::c_void,
    ) -> i32;
    fn truncateAndFlush(context: *mut CatContext, timestampMs: usize);
}

pub static mut G_CAT_CONTEXT: *mut CatContext =
    0i32 as (*mut ::std::os::raw::c_void) as (*mut CatContext);

unsafe fn isCATStaticQueueEmpty(mut pQueue: *mut CATStaticQueue) -> i32 {
    ((*pQueue).size == 0) as (i32)
}

pub unsafe extern "C" fn catContextAddMessage(
    mut ctx: *mut CatContext,
    mut message: *mut CatMessage,
) {
    let mut pStack: *mut CATStaticQueue = (*ctx).transactionStack;
    if isCATStaticQueueEmpty(pStack) != 0 {
        let mut pRootCopy: *mut CatMessageTree = copyCatMessageTree((*ctx).tree);
        (*pRootCopy).root = message;
        catMessageManagerFlush(pRootCopy);
    } else {
        let mut parent: *mut CatTransaction =
            pryFrontCATStaticQueue(pStack) as (*mut CatTransaction);
        ((*ctx).addTransChild)(ctx, message, parent);
    }
}

unsafe fn catTrimToHour(mut timeMs: usize) -> usize {
    timeMs.wrapping_div((3600i32 * 1000i32) as (usize))
}

unsafe fn getCatMessageTimeStamp(mut message: *mut CatMessage) -> usize {
    let mut pInner: *mut CatMessageInner = (message as (*mut u8))
        .offset(-(::std::mem::size_of::<CatMessageInner>() as (isize)))
        as (*mut CatMessageInner);
    (*pInner).timestampMs
}

pub unsafe extern "C" fn catContextAddTransChild(
    mut ctx: *mut CatContext,
    mut message: *mut CatMessage,
    mut trans: *mut CatTransaction,
) {
    let mut treePeriod: usize = catTrimToHour(getCatMessageTimeStamp((*(*ctx).tree).root));
    let mut messagePeriod: usize = catTrimToHour(
        getCatMessageTimeStamp(message).wrapping_sub((10isize * 1000isize) as (usize)),
    );
    if treePeriod < messagePeriod || (*ctx).elementSize >= g_config.maxContextElementSize {
        truncateAndFlush(ctx, getCatMessageTimeStamp(message));
    }
    ((*trans).addChild)(trans, message);
    (*ctx).elementSize = (*ctx).elementSize + 1;
}

pub unsafe extern "C" fn catContextAdjustForTruncatedTrans(
    mut ctx: *mut CatContext,
    mut root: *mut CatTransaction,
) {
    let mut next: *mut CatMessage = createCatEvent(
        (*b"TruncatedTransaction\0").as_ptr(),
        (*b"TotalDuration\0").as_ptr(),
    );
    let mut actualDurationUs: usize = (*ctx)
        .lastTruncateTransDurationUs
        .wrapping_add(getCatTransactionDurationUs(root));
    ((*next).addData)(
        next,
        catsdsfromlonglong(actualDurationUs as (isize)) as (*const u8),
    );
    ((*next).setStatus)(next, (*b"0\0").as_ptr());
    ((*root).addChild)(root, next);
    (*ctx).lastTruncateTransDurationUs = 0usize;
}

pub unsafe extern "C" fn catContextStartTrans(
    mut ctx: *mut CatContext,
    mut trans: *mut CatTransaction,
) {
    if isCATStaticQueueEmpty((*ctx).transactionStack) == 0 {
        let mut parent: *mut CatTransaction =
            pryFrontCATStaticQueue((*ctx).transactionStack) as (*mut CatTransaction);
        catContextAddTransChild(ctx, trans as (*mut CatMessage), parent);
    } else {
        let mut t: *mut CatMessageTree = (*ctx).tree;
        (*(*ctx).tree).root = trans as (*mut CatMessage);
    }
    pushFrontCATStaticQueue(
        (*ctx).transactionStack,
        trans as (*mut ::std::os::raw::c_void),
    );
}

unsafe fn getCATStaticQueueSize(mut pQueue: *mut CATStaticQueue) -> usize {
    (*pQueue).size
}

pub unsafe extern "C" fn catContextEndTrans(
    mut ctx: *mut CatContext,
    mut trans: *mut CatTransaction,
) {
    let mut stackTrans: *mut CatTransaction;
    'loop1: loop {
        if !(getCATStaticQueueSize((*ctx).transactionStack) > 0usize) {
            break;
        }
        stackTrans =
            getCATStaticQueueByIndex((*ctx).transactionStack, 0usize) as (*mut CatTransaction);
        popFrontCATStaticQueue((*ctx).transactionStack);
        if stackTrans == trans {
            break;
        }
    }
}

pub unsafe extern "C" fn catContextReset(mut ctx: *mut CatContext) {
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) != (*(*ctx).tree).messageId {
        catsdsfree((*(*ctx).tree).messageId);
        (*(*ctx).tree).messageId = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) != (*(*ctx).tree).parentMessageId {
        catsdsfree((*(*ctx).tree).parentMessageId);
        (*(*ctx).tree).parentMessageId = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) != (*(*ctx).tree).rootMessageId {
        catsdsfree((*(*ctx).tree).rootMessageId);
        (*(*ctx).tree).rootMessageId = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    (*(*ctx).tree).root = 0i32 as (*mut ::std::os::raw::c_void) as (*mut CatMessage);
    (*ctx).elementSize = 0i32;
    (*ctx).lastTruncateTransDurationUs = 0usize;
    clearCATStaticQueue((*ctx).transactionStack);
}

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

unsafe fn cat_get_current_thread_id() -> i32 {
    let mut pid: i32 = 0i32;
    let mut tid64: usize = mem::uninitialized();
    pthread_threadid_np(
        0i32 as (*mut ::std::os::raw::c_void) as (*mut _opaque_pthread_t),
        &mut tid64 as (*mut usize),
    );
    pid = tid64 as (i32);
    pid
}

pub unsafe fn newCatContext() -> *mut CatContext {
    let mut ctx: *mut CatContext = malloc(::std::mem::size_of::<CatContext>()) as (*mut CatContext);
    catChecktPtrWithName(
        ctx as (*mut ::std::os::raw::c_void),
        (*b"ctx\0").as_ptr() as (*mut u8),
    );
    if ctx == 0i32 as (*mut ::std::os::raw::c_void) as (*mut CatContext) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut CatContext)
    } else {
        (*ctx).lastTruncateTransDurationUs = 0usize;
        (*ctx).elementSize = 0i32;
        (*ctx).tree = createCatMessageTree();
        catChecktPtrWithName(
            (*ctx).tree as (*mut ::std::os::raw::c_void),
            (*b"ctx->tree\0").as_ptr() as (*mut u8),
        );
        (*ctx).transactionStack = createCATStaticQueue(g_config.maxContextElementSize as (usize));
        catChecktPtrWithName(
            (*ctx).transactionStack as (*mut ::std::os::raw::c_void),
            (*b"ctx->transactionStack\0").as_ptr() as (*mut u8),
        );
        if (*ctx).tree != 0i32 as (*mut ::std::os::raw::c_void) as (*mut CatMessageTree) {
            (*(*ctx).tree).root = 0i32 as (*mut ::std::os::raw::c_void) as (*mut CatMessage);
            let mut pid: i32 = cat_get_current_thread_id();
            let mut tmpBuf: [u8; 32] = mem::uninitialized();
            (*(*ctx).tree).threadId =
                catsdsnew(catItoA(pid, tmpBuf.as_mut_ptr(), 10i32) as (*const u8));
            (*(*ctx).tree).threadGroupName = catsdsnew((*b"cat\0").as_ptr());
            (*(*ctx).tree).threadName = catsdsnew((*b"cat\0").as_ptr());
        }
        (*ctx).addMessage = catContextAddMessage;
        (*ctx).addTransChild = catContextAddTransChild;
        (*ctx).adjustForTruncatedTrans = catContextAdjustForTruncatedTrans;
        (*ctx).startTrans = catContextStartTrans;
        (*ctx).endTrans = catContextEndTrans;
        (*ctx).reset = catContextReset;
        ctx
    }
}

pub unsafe fn getCatContext() -> *mut CatContext {
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut CatContext) == G_CAT_CONTEXT {
        G_CAT_CONTEXT = newCatContext();
    }
    G_CAT_CONTEXT
}

pub unsafe fn getContextMessageTree() -> *mut CatMessageTree {
    (*getCatContext()).tree
}
