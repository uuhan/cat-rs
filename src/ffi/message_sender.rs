use super::raw::CatClientInnerConfig;
use super::raw::CatEncoder;
use super::raw::CatMessage;
use super::raw::CatMessageManager;
use super::raw::CatMessageTree;
use super::raw::CatTransaction;
use super::sds::catsdsclear;
use super::sds::catsdsfree;
use super::sds::catsdslen;
use super::sds::catsdsnewEmpty;
use super::sds::sdshdr;
use libc::__error;
use libc::usleep;
use libc::write;
use std::mem;

extern "C" {
    fn CatMPSC_bpoll(q: *mut _queue, ms: i32) -> *mut ::std::os::raw::c_void;
    fn CatMPSC_offer(q: *mut _queue, data: *mut ::std::os::raw::c_void) -> i32;
    fn CatMPSC_poll(q: *mut _queue) -> *mut ::std::os::raw::c_void;
    fn catChecktPtrWithName(ptr: *mut ::std::os::raw::c_void, ptrName: *mut u8);
    fn catEncodeMessageTree(tree: *mut CatMessageTree, buf: *mut u8);
    fn deleteCatMPSCQueue(q: *mut _queue);
    fn deleteCatMessageTree(pMsgTree: *mut CatMessageTree);
    static mut g_cat_encoder: *mut CatEncoder;
    static mut g_cat_messageManager: CatMessageManager;
    static mut g_config: CatClientInnerConfig;
    fn hitSample() -> i32;
    fn newCatBinaryEncoder() -> *mut CatEncoder;
    fn newCatMPSCQueue(name: *const u8, capacity: i32) -> *mut _queue;
    fn newCatTextEncoder() -> *mut CatEncoder;
    fn pthread_create(
        arg1: *mut *mut _opaque_pthread_t,
        arg2: *const _opaque_pthread_attr_t,
        arg3: unsafe extern "C" fn(*mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
        arg4: *mut ::std::os::raw::c_void,
    ) -> i32;
    fn pthread_join(arg1: *mut _opaque_pthread_t, arg2: *mut *mut ::std::os::raw::c_void) -> i32;
    fn recoverCatServerConn() -> i32;
    fn sendToAggregator(pMsgTree: *mut CatMessageTree);
}

#[derive(Copy)]
#[repr(C)]
pub struct _queue {
    pub name: *mut u8,
}

impl Clone for _queue {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub normal: *mut _queue,
    pub high: *mut _queue,
    pub normalFullCount: usize,
}

impl Clone for Struct1 {
    fn clone(&self) -> Self {
        *self
    }
}

static mut G_CAT_MQ: Struct1 = Struct1 {
    normal: 0i32 as (*mut ::std::os::raw::c_void) as (*mut _queue),
    high: 0i32 as (*mut ::std::os::raw::c_void) as (*mut _queue),
    normalFullCount: 0usize,
};

static mut G_CAT_SEND_STOP: i32 = 0i32;

#[no_mangle]
pub static mut g_cat_send_fd: i32 = -1i32;

#[no_mangle]
pub static mut g_cat_send_ip: [u8; 64] = [0u8; 64];

#[no_mangle]
pub static mut g_cat_send_port: u16 = 0u16;

#[no_mangle]
pub static mut g_cat_send_blockTimes: usize = 0usize;

#[no_mangle]
pub static mut g_cat_send_failedFlag: i32 = 0i32;

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

static mut G_CAT_SEND_HANDLE: *mut _opaque_pthread_t =
    0i32 as (*mut ::std::os::raw::c_void) as (*mut _opaque_pthread_t);

static mut G_CAT_MERGE_BUF: *mut u8 = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);

#[no_mangle]
pub unsafe extern "C" fn isCatSenderEnable() -> i32 {
    (g_cat_send_fd > 0i32) as (i32)
}

unsafe extern "C" fn mqOffer(mut tree: *mut CatMessageTree) -> i32 {
    if (*tree).canDiscard != 0 {
        if CatMPSC_offer(G_CAT_MQ.normal, tree as (*mut ::std::os::raw::c_void)) != 0i32 {
            if g_config.enableSampling != 0 {
                warn!("normal queue is full, message has been aggregated.");
                sendToAggregator(tree);
            } else {
                warn!("normal queue is full, message has been discarded!");
            }
            deleteCatMessageTree(tree);
            return 0i32;
        }
    } else if CatMPSC_offer(G_CAT_MQ.high, tree as (*mut ::std::os::raw::c_void)) != 0i32 {
        warn!("high queue is full, message has been discarded!");
        deleteCatMessageTree(tree);
        return 1i32;
    }
    0i32
}

#[no_mangle]
pub unsafe extern "C" fn sendRootMessage(mut tree: *mut CatMessageTree) -> i32 {
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut CatMessageTree) == tree {
        0i32
    } else if (*tree).canDiscard == 0 {
        mqOffer(tree)
    } else if g_config.enableSampling != 0 && (hitSample() != 0) {
        mqOffer(tree)
    } else {
        sendToAggregator(tree);
        deleteCatMessageTree(tree);
        0i32
    }
}

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

unsafe extern "C" fn cat_set_thread_name(mut name: *const u8) {}

unsafe extern "C" fn mqPollMany(
    mut bufArray: *mut *mut ::std::os::raw::c_void,
    mut max: i32,
) -> i32 {
    let mut current: i32 = 0i32;
    let mut data: *mut ::std::os::raw::c_void;
    'loop1: loop {
        if !(current < max) {
            break;
        }
        data = CatMPSC_poll(G_CAT_MQ.high);
        if 0i32 as (*mut ::std::os::raw::c_void) == data {
            break;
        }
        *bufArray.offset({
            let _old = current;
            current = current + 1;
            _old
        } as (isize)) = data;
    }
    'loop4: loop {
        if !(current < max) {
            break;
        }
        data = CatMPSC_poll(G_CAT_MQ.normal);
        if 0i32 as (*mut ::std::os::raw::c_void) == data {
            break;
        }
        *bufArray.offset({
            let _old = current;
            current = current + 1;
            _old
        } as (isize)) = data;
    }
    if current < max {
        data = CatMPSC_bpoll(G_CAT_MQ.normal, 5i32);
        if 0i32 as (*mut ::std::os::raw::c_void) != data {
            *bufArray.offset({
                let _old = current;
                current = current + 1;
                _old
            } as (isize)) = data;
        }
    }
    current
}

unsafe extern "C" fn sendCatMessageBufferDirectly(
    mut sendBuf: *mut u8,
    mut checkpoint: usize,
) -> i32 {
    let mut _currentBlock;
    if g_cat_send_failedFlag != 0 {
        -1i32
    } else {
        if g_cat_send_fd < 0i32 {
            recoverCatServerConn();
            if g_cat_send_fd < 0i32 {
                return -1i32;
            }
        }
        let mut sendTotalLen: usize = checkpoint;
        let mut nowSendLen: isize = 0isize;
        let mut sendLen: isize = 0isize;
        'loop4: loop {
            if !(nowSendLen as (usize) != sendTotalLen) {
                _currentBlock = 14;
                break;
            }
            sendLen = write(
                g_cat_send_fd,
                sendBuf as (*const ::std::os::raw::c_void),
                sendTotalLen.wrapping_sub(nowSendLen as (usize)),
            );
            if sendLen == -1isize {
                if !(*__error() == 35i32) {
                    _currentBlock = 8;
                    break;
                }
                g_cat_send_blockTimes = g_cat_send_blockTimes.wrapping_add(1usize);
                if g_cat_send_blockTimes.wrapping_rem(1000000usize) == 0usize {
                    warn!("Send Cat Message : ip is blocking.");
                }
                if nowSendLen == 0isize {
                    _currentBlock = 13;
                    break;
                }
                usleep((5i32 * 1000i32) as (u32));
            } else {
                nowSendLen = nowSendLen + sendLen;
                sendBuf = sendBuf.offset(sendLen);
            }
        }
        if _currentBlock == 8 {
            warn!("Send to server : failed.");
            nowSendLen = -1isize;
        } else if _currentBlock == 13 {
            warn!("Tcp buffer is full, message has been discarded");
        }
        if nowSendLen < 0isize {
            recoverCatServerConn();
            if g_cat_send_fd < 0i32 {
                warn!("Recover failed.");
            }
        }
        1i32
    }
}

unsafe extern "C" fn catsdsrotate(mut s: *mut u8, mut offset: usize) -> *mut u8 {
    let mut sh: *mut sdshdr =
        s.offset(-(::std::mem::size_of::<sdshdr>() as (isize))) as (*mut sdshdr);
    if offset > (*sh).len as (usize) {
        catsdsclear(s);
        s
    } else {
        let mut i: usize;
        let mut l: *mut u8;
        let mut r: *mut u8;
        i = offset;
        l = s;
        r = s.offset(offset as (isize));
        'loop2: loop {
            if !(i <= (*sh).len as (usize)) {
                break;
            }
            *l = *r;
            i = i.wrapping_add(1usize);
            l = l.offset(1isize);
            r = r.offset(1isize);
        }
        (*sh).len = ((*sh).len as (usize)).wrapping_sub(offset) as (u32);
        (*sh).free = ((*sh).free as (usize)).wrapping_add(offset) as (u32);
        s
    }
}

unsafe extern "C" fn catMessageSenderFun(
    mut para: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_void {
    cat_set_thread_name((*b"cat-sender\0").as_ptr());
    let mut messageArray: [*mut CatMessageTree; 16] = mem::uninitialized();
    G_CAT_MERGE_BUF = catsdsnewEmpty((60i32 * 1024i32) as (usize));
    'loop1: loop {
        if !(G_CAT_SEND_STOP == 0) {
            break;
        }
        catsdsclear(G_CAT_MERGE_BUF);
        let mut eleNum: i32 = mqPollMany(
            messageArray.as_mut_ptr() as (*mut *mut ::std::os::raw::c_void),
            16i32,
        );
        if eleNum == 0i32 {
            continue;
        }
        let mut checkpoint: usize = 0usize;
        let mut i: i32 = 0i32;
        'loop5: loop {
            if !(i < eleNum) {
                break;
            }
            catEncodeMessageTree(messageArray[i as (usize)], G_CAT_MERGE_BUF);
            deleteCatMessageTree(messageArray[i as (usize)]);
            if catsdslen(G_CAT_MERGE_BUF) >= (60i32 * 1024i32) as (usize) {
                sendCatMessageBufferDirectly(G_CAT_MERGE_BUF, checkpoint);
                G_CAT_MERGE_BUF = catsdsrotate(G_CAT_MERGE_BUF, checkpoint);
            }
            checkpoint = catsdslen(G_CAT_MERGE_BUF);
            i = i + 1;
        }
        if !(checkpoint > 0usize) {
            continue;
        }
        sendCatMessageBufferDirectly(G_CAT_MERGE_BUF, checkpoint);
    }
    0i32 as (*mut ::std::os::raw::c_void)
}

#[no_mangle]
pub unsafe extern "C" fn initCatSenderThread() {
    G_CAT_MERGE_BUF = catsdsnewEmpty((2i32 * 1024i32 * 1024i32) as (usize));
    let switch2 = g_config.encoderType;
    if switch2 == 0i32 {
        g_cat_encoder = newCatTextEncoder();
    } else if switch2 == 1i32 {
        g_cat_encoder = newCatBinaryEncoder();
    } else {
        warn!("cat encoder has not been specified!");
        return;
    }
    ((*g_cat_encoder).setAppkey)(g_cat_encoder, g_cat_messageManager.domain as (*const u8));
    ((*g_cat_encoder).setHostname)(g_cat_encoder, g_cat_messageManager.hostname as (*const u8));
    ((*g_cat_encoder).setIp)(g_cat_encoder, g_cat_messageManager.ip as (*const u8));
    G_CAT_MQ.normal = newCatMPSCQueue((*b"sender_normal\0").as_ptr(), g_config.messageQueueSize);
    catChecktPtrWithName(
        G_CAT_MQ.normal as (*mut ::std::os::raw::c_void),
        (*b"G_CAT_MQ.normal\0").as_ptr() as (*mut u8),
    );
    G_CAT_MQ.high = newCatMPSCQueue((*b"sender_high\0").as_ptr(), g_config.messageQueueSize);
    catChecktPtrWithName(
        G_CAT_MQ.high as (*mut ::std::os::raw::c_void),
        (*b"G_CAT_MQ.high\0").as_ptr() as (*mut u8),
    );
    G_CAT_SEND_STOP = 0i32;
    pthread_create(
        &mut G_CAT_SEND_HANDLE as (*mut *mut _opaque_pthread_t),
        0i32 as (*mut ::std::os::raw::c_void) as (*const _opaque_pthread_attr_t),
        catMessageSenderFun,
        0i32 as (*mut ::std::os::raw::c_void),
    );
}

unsafe extern "C" fn clearMessageQueue(mut q: *mut _queue) {
    let mut tree: *mut CatMessageTree;
    'loop1: loop {
        if !(0i32 as (*mut ::std::os::raw::c_void) as (*mut CatMessageTree) != {
            tree = CatMPSC_poll(q) as (*mut CatMessageTree);
            tree
        }) {
            break;
        }
        deleteCatMessageTree(tree);
    }
    deleteCatMPSCQueue(q);
}

#[no_mangle]
pub unsafe extern "C" fn clearCatSenderThread() {
    G_CAT_SEND_STOP = 1i32;
    pthread_join(
        G_CAT_SEND_HANDLE,
        0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut ::std::os::raw::c_void),
    );
    clearMessageQueue(G_CAT_MQ.normal);
    clearMessageQueue(G_CAT_MQ.high);
    catsdsfree(G_CAT_MERGE_BUF);
}
