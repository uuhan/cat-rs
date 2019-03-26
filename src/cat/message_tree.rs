use super::message::clearMessage;
use super::raw::CatMessage;
use super::raw::CatMessageTree;
use super::sds::catsdsdup;
use super::sds::catsdsfree;
use super::transaction::clearTransaction;
use libc::{free, malloc, memset};
extern "C" {
    fn isCatTransaction(message: *mut CatMessage) -> i32;
}

pub unsafe fn copyCatMessageTree(mut pRootMsg: *mut CatMessageTree) -> *mut CatMessageTree {
    let mut pCopyMsg: *mut CatMessageTree =
        malloc(::std::mem::size_of::<CatMessageTree>()) as (*mut CatMessageTree);
    if pCopyMsg == 0i32 as (*mut ::std::os::raw::c_void) as (*mut CatMessageTree) {
        pCopyMsg
    } else {
        if pRootMsg == 0i32 as (*mut ::std::os::raw::c_void) as (*mut CatMessageTree) {
            memset(
                pCopyMsg as (*mut ::std::os::raw::c_void),
                0i32,
                ::std::mem::size_of::<CatMessageTree>(),
            );
            (*pCopyMsg).canDiscard = 1i32;
        } else {
            (*pCopyMsg).root = (*pRootMsg).root;
            (*pCopyMsg).messageId = catsdsdup((*pRootMsg).messageId);
            (*pCopyMsg).parentMessageId = catsdsdup((*pRootMsg).parentMessageId);
            (*pCopyMsg).rootMessageId = catsdsdup((*pRootMsg).rootMessageId);
            (*pCopyMsg).sessionToken = catsdsdup((*pRootMsg).sessionToken);
            (*pCopyMsg).threadGroupName = catsdsdup((*pRootMsg).threadGroupName);
            (*pCopyMsg).threadId = catsdsdup((*pRootMsg).threadId);
            (*pCopyMsg).threadName = catsdsdup((*pRootMsg).threadName);
            (*pCopyMsg).canDiscard = (*pRootMsg).canDiscard;
        }
        pCopyMsg
    }
}

pub unsafe fn clearCatMessageTree(mut pRootMsg: *mut CatMessageTree) {
    (*pRootMsg).root = 0i32 as (*mut ::std::os::raw::c_void) as (*mut CatMessage);
    if (*pRootMsg).messageId != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        catsdsfree((*pRootMsg).messageId);
        (*pRootMsg).messageId = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    if (*pRootMsg).parentMessageId != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        catsdsfree((*pRootMsg).parentMessageId);
        (*pRootMsg).parentMessageId = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    if (*pRootMsg).rootMessageId != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        catsdsfree((*pRootMsg).rootMessageId);
        (*pRootMsg).rootMessageId = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    if (*pRootMsg).sessionToken != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        catsdsfree((*pRootMsg).sessionToken);
        (*pRootMsg).sessionToken = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    if (*pRootMsg).threadGroupName != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        catsdsfree((*pRootMsg).threadGroupName);
        (*pRootMsg).threadGroupName = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    if (*pRootMsg).threadId != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        catsdsfree((*pRootMsg).threadId);
        (*pRootMsg).threadId = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    if (*pRootMsg).threadName != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        catsdsfree((*pRootMsg).threadName);
        (*pRootMsg).threadName = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
}

unsafe fn deleteCatMessage(mut message: *mut CatMessage) {
    let mut p: *mut ::std::os::raw::c_void;
    if isCatTransaction(message) != 0 {
        p = clearTransaction(message);
    } else {
        p = clearMessage(message);
    }
    free(p);
}

pub unsafe fn deleteCatMessageTree(mut pRootMsg: *mut CatMessageTree) {
    if (*pRootMsg).root != 0i32 as (*mut ::std::os::raw::c_void) as (*mut CatMessage) {
        deleteCatMessage((*pRootMsg).root);
        (*pRootMsg).root = 0i32 as (*mut ::std::os::raw::c_void) as (*mut CatMessage);
    }
    clearCatMessageTree(pRootMsg);
    free(pRootMsg as (*mut ::std::os::raw::c_void));
}

pub unsafe fn createCatMessageTree() -> *mut CatMessageTree {
    copyCatMessageTree(0i32 as (*mut ::std::os::raw::c_void) as (*mut CatMessageTree))
}
