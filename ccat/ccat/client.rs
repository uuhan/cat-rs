extern {
    fn CLogLogWithLocation(
        type_ : u16,
        format : *const u8,
        file : *const u8,
        line : i32,
        function : *const u8,
        ...
    );
    fn __swbuf(arg1 : i32, arg2 : *mut __sFILE) -> i32;
    fn addCountMetricToAggregator(name : *const u8, count : i32);
    fn addDurationMetricToAggregator(name : *const u8, timeMs : i32);
    fn catChecktPtrWithName(
        ptr : *mut ::std::os::raw::c_void, ptrName : *mut u8
    );
    fn catMessageManagerDestroy();
    fn catMessageManagerStartTrans(trans : *mut _CatTransaction);
    fn catsdsfree(s : *mut u8);
    fn catsdsfromlonglong(value : isize) -> *mut u8;
    fn catsdsnew(init : *const u8) -> *mut u8;
    fn clearCatAggregatorThread();
    fn clearCatClientConfig();
    fn clearCatMonitor();
    fn clearCatSenderThread();
    fn clearCatServerConnManager();
    fn createCatEvent(
        type_ : *const u8, name : *const u8
    ) -> *mut _CatMessage;
    fn createCatHeartBeat(
        type_ : *const u8, name : *const u8
    ) -> *mut _CatMessage;
    fn createCatMetric(
        type_ : *const u8, name : *const u8
    ) -> *mut _CatMessage;
    fn createCatTransaction(
        type_ : *const u8, name : *const u8
    ) -> *mut _CatTransaction;
    fn destroyMessageIdHelper();
    static mut g_cat_enabledFlag : i32;
    static mut g_cat_nullMsg : _CatMessage;
    static mut g_cat_nullTrans : _CatTransaction;
    static mut g_config : _CatClientInnerConfig;
    fn getContextMessageTree() -> *mut _CatMessageTree;
    fn getNextMessageId() -> *mut u8;
    fn getNextMessageIdByAppkey(domain : *const u8) -> *mut u8;
    fn gettimeofday(
        arg1 : *mut timeval, arg2 : *mut ::std::os::raw::c_void
    ) -> i32;
    fn initCatAggregatorThread();
    fn initCatClientConfig(config : *mut _CatClientConfig);
    fn initCatMonitorThread();
    fn initCatSenderThread();
    fn initCatServerConnManager() -> i32;
    fn initMessageIdHelper();
    fn initMessageManager(domain : *const u8, hostName : *const u8);
    fn isCatEnabled() -> i32;
    fn loadCatClientConfig(filename : *const u8) -> i32;
    fn signal(
        arg1 : i32, arg2 : unsafe extern fn(i32)
    ) -> unsafe extern fn(i32);
}

enum __sFILEX {
}

#[derive(Copy)]
#[repr(C)]
pub struct __sbuf {
    pub _base : *mut u8,
    pub _size : i32,
}

impl Clone for __sbuf {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct __sFILE {
    pub _p : *mut u8,
    pub _r : i32,
    pub _w : i32,
    pub _flags : i16,
    pub _file : i16,
    pub _bf : __sbuf,
    pub _lbfsize : i32,
    pub _cookie : *mut ::std::os::raw::c_void,
    pub _close : unsafe extern fn(*mut ::std::os::raw::c_void) -> i32,
    pub _read : unsafe extern fn(*mut ::std::os::raw::c_void, *mut u8, i32) -> i32,
    pub _seek : unsafe extern fn(*mut ::std::os::raw::c_void, isize, i32) -> isize,
    pub _write : unsafe extern fn(*mut ::std::os::raw::c_void, *const u8, i32) -> i32,
    pub _ub : __sbuf,
    pub _extra : *mut __sFILEX,
    pub _ur : i32,
    pub _ubuf : [u8; 3],
    pub _nbuf : [u8; 1],
    pub _lb : __sbuf,
    pub _blksize : i32,
    pub _offset : isize,
}

impl Clone for __sFILE {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn __sputc(
    mut _c : i32, mut _p : *mut __sFILE
) -> i32 {
    if {
           (*_p)._w = (*_p)._w - 1;
           (*_p)._w
       } >= 0i32 || (*_p)._w >= (*_p)._lbfsize && (_c as (u8) as (i32) != b'\n' as (i32)) {
        ({
             let _rhs = _c;
             let _lhs
                 = &mut *{
                             let _old = (*_p)._p;
                             (*_p)._p = (*_p)._p.offset(1isize);
                             _old
                         };
             *_lhs = _rhs as (u8);
             *_lhs
         }) as (i32)
    } else {
        __swbuf(_c,_p)
    }
}

static mut g_cat_init : i32 = 0i32;

#[no_mangle]
pub static mut g_multiprocessing_pid_str
    : *mut u8
    = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);

#[derive(Copy)]
#[repr(C)]
pub struct _CatClientConfig {
    pub encoderType : i32,
    pub enableHeartbeat : i32,
    pub enableSampling : i32,
    pub enableMultiprocessing : i32,
    pub enableDebugLog : i32,
}

impl Clone for _CatClientConfig {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut DEFAULT_CCAT_CONFIG
    : _CatClientConfig
    = _CatClientConfig {
          encoderType: 1i32,
          enableHeartbeat: 1i32,
          enableSampling: 1i32,
          enableMultiprocessing: 0i32,
          enableDebugLog: 0i32
      };

#[no_mangle]
pub unsafe extern fn catClientInit(mut appkey : *const u8) -> i32 {
    catClientInitWithConfig(
        appkey,
        &mut DEFAULT_CCAT_CONFIG as (*mut _CatClientConfig)
    )
}

#[derive(Copy)]
#[repr(C)]
pub struct _CatClientInnerConfig {
    pub appkey : *mut u8,
    pub selfHost : *mut u8,
    pub serverHost : *mut u8,
    pub defaultIp : *mut u8,
    pub defaultIpHex : *mut u8,
    pub serverPort : u32,
    pub serverNum : i32,
    pub serverAddresses : *mut *mut u8,
    pub messageEnableFlag : i32,
    pub messageQueueSize : i32,
    pub messageQueueBlockPrintCount : i32,
    pub maxChildSize : i32,
    pub maxContextElementSize : i32,
    pub logFlag : i32,
    pub logSaveFlag : i32,
    pub logDebugFlag : i32,
    pub logFileWithTime : i32,
    pub logFilePerDay : i32,
    pub logLevel : i32,
    pub configDir : *mut u8,
    pub dataDir : *mut u8,
    pub indexFileName : *mut u8,
    pub encoderType : i32,
    pub enableHeartbeat : i32,
    pub enableSampling : i32,
    pub enableMultiprocessing : i32,
}

impl Clone for _CatClientInnerConfig {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn catClientInitWithConfig(
    mut appkey : *const u8, mut config : *mut _CatClientConfig
) -> i32 {
    if g_cat_init != 0 {
        0i32
    } else {
        g_cat_init = 1i32;
        signal(13i32,1i32 as (unsafe extern fn(i32)));
        initCatClientConfig(config);
        (if loadCatClientConfig(
                (*b"/data/appdatas/cat/client.xml\0").as_ptr()
            ) < 0i32 {
             g_cat_init = 0i32;
             g_cat_enabledFlag = 0i32;
             CLogLogWithLocation(
                 0x8u16,
                 (*b"Failed to initialize cat: Error occurred while loading client config.\0").as_ptr(
                 ),
                 file!().as_ptr(),
                 line!() as (i32),
                 (*b"catClientInitWithConfig\0").as_ptr()
             );
             0i32
         } else {
             g_config.appkey = catsdsnew(appkey);
             initMessageManager(appkey,g_config.selfHost as (*const u8));
             initMessageIdHelper();
             (if initCatServerConnManager() == 0 {
                  g_cat_init = 0i32;
                  g_cat_enabledFlag = 0i32;
                  CLogLogWithLocation(
                      0x8u16,
                      (*b"Failed to initialize cat: Error occurred while getting router from remote server.\0").as_ptr(
                      ),
                      file!().as_ptr(),
                      line!() as (i32),
                      (*b"catClientInitWithConfig\0").as_ptr()
                  );
                  0i32
              } else {
                  initCatAggregatorThread();
                  initCatSenderThread();
                  initCatMonitorThread();
                  g_cat_enabledFlag = 1i32;
                  CLogLogWithLocation(
                      0x2u16,
                      (*b"Cat has been successfully initialized with appkey: %s\0").as_ptr(
                      ),
                      file!().as_ptr(),
                      line!() as (i32),
                      (*b"catClientInitWithConfig\0").as_ptr(),
                      appkey
                  );
                  1i32
              })
         })
    }
}

#[no_mangle]
pub unsafe extern fn catClientDestroy() -> i32 {
    g_cat_enabledFlag = 0i32;
    g_cat_init = 0i32;
    clearCatMonitor();
    catMessageManagerDestroy();
    clearCatAggregatorThread();
    clearCatSenderThread();
    clearCatServerConnManager();
    destroyMessageIdHelper();
    clearCatClientConfig();
    1i32
}

#[no_mangle]
pub unsafe extern fn catVersion() -> *const u8 {
    (*b"3.0.1\0").as_ptr()
}

#[derive(Copy)]
#[repr(C)]
pub struct _CatMessageTree {
    pub root : *mut _CatMessage,
    pub messageId : *mut u8,
    pub parentMessageId : *mut u8,
    pub rootMessageId : *mut u8,
    pub sessionToken : *mut u8,
    pub threadGroupName : *mut u8,
    pub threadId : *mut u8,
    pub threadName : *mut u8,
    pub canDiscard : i32,
}

impl Clone for _CatMessageTree {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn logError(
    mut msg : *const u8, mut errStr : *const u8
) {
    (*getContextMessageTree()).canDiscard = 0i32;
    logEvent(
        (*b"Exception\0").as_ptr(),
        msg,
        (*b"ERROR\0").as_ptr(),
        errStr
    );
}

#[derive(Copy)]
#[repr(C)]
pub struct _CatMessage {
    pub addData : unsafe extern fn(*mut _CatMessage, *const u8),
    pub addKV : unsafe extern fn(*mut _CatMessage, *const u8, *const u8),
    pub setStatus : unsafe extern fn(*mut _CatMessage, *const u8),
    pub setTimestamp : unsafe extern fn(*mut _CatMessage, usize),
    pub complete : unsafe extern fn(*mut _CatMessage),
}

impl Clone for _CatMessage {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn logEvent(
    mut type_ : *const u8,
    mut name : *const u8,
    mut status : *const u8,
    mut data : *const u8
) { if isCatEnabled() == 0 {
    } else {
        let mut event : *mut _CatMessage = newEvent(type_,name);
        catChecktPtrWithName(
            event as (*mut ::std::os::raw::c_void),
            (*b"event\0").as_ptr() as (*mut u8)
        );
        (if event == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _CatMessage) {
         } else {
             if data != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
                 ((*event).addData)(event,data);
             }
             ((*event).setStatus)(event,status);
             ((*event).complete)(event);
         })
    }
}

#[no_mangle]
pub unsafe extern fn _logMetric(
    mut name : *const u8, mut status : *const u8, mut value : *const u8
) {
    let mut metric
        : *mut _CatMessage
        = newMetric((*b"\0").as_ptr(),name);
    catChecktPtrWithName(
        metric as (*mut ::std::os::raw::c_void),
        (*b"metric\0").as_ptr() as (*mut u8)
    );
    if value != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
        ((*metric).addData)(metric,value);
    }
    ((*metric).setStatus)(metric,status);
    ((*metric).complete)(metric);
}

#[no_mangle]
pub unsafe extern fn logMetricForCount(
    mut name : *const u8, mut quantity : i32
) { if isCatEnabled() == 0 {
    } else if g_config.enableSampling != 0 {
        addCountMetricToAggregator(name,quantity);
    } else if quantity == 1i32 {
        _logMetric(name,(*b"C\0").as_ptr(),(*b"1\0").as_ptr());
    } else {
        let mut val : *mut u8 = catsdsfromlonglong(quantity as (isize));
        catChecktPtrWithName(
            val as (*mut ::std::os::raw::c_void),
            (*b"val\0").as_ptr() as (*mut u8)
        );
        _logMetric(name,(*b"C\0").as_ptr(),val as (*const u8));
        catsdsfree(val);
    }
}

#[no_mangle]
pub unsafe extern fn logMetricForDuration(
    mut name : *const u8, mut duration : usize
) { if isCatEnabled() == 0 {
    } else if g_config.enableSampling != 0 {
        addDurationMetricToAggregator(name,duration as (i32));
    } else {
        let mut val : *mut u8 = catsdsfromlonglong(duration as (isize));
        catChecktPtrWithName(
            val as (*mut ::std::os::raw::c_void),
            (*b"val\0").as_ptr() as (*mut u8)
        );
        _logMetric(name,(*b"T\0").as_ptr(),val as (*const u8));
        catsdsfree(val);
    }
}

#[no_mangle]
pub unsafe extern fn newEvent(
    mut type_ : *const u8, mut name : *const u8
) -> *mut _CatMessage {
    if isCatEnabled() == 0 {
        &mut g_cat_nullMsg as (*mut _CatMessage)
    } else {
        let mut event : *mut _CatMessage = createCatEvent(type_,name);
        catChecktPtrWithName(
            event as (*mut ::std::os::raw::c_void),
            (*b"event\0").as_ptr() as (*mut u8)
        );
        event
    }
}

#[no_mangle]
pub unsafe extern fn newMetric(
    mut type_ : *const u8, mut name : *const u8
) -> *mut _CatMessage {
    if isCatEnabled() == 0 {
        &mut g_cat_nullMsg as (*mut _CatMessage)
    } else {
        let mut metric : *mut _CatMessage = createCatMetric(type_,name);
        catChecktPtrWithName(
            metric as (*mut ::std::os::raw::c_void),
            (*b"metric\0").as_ptr() as (*mut u8)
        );
        metric
    }
}

#[no_mangle]
pub unsafe extern fn newHeartBeat(
    mut type_ : *const u8, mut name : *const u8
) -> *mut _CatMessage {
    if isCatEnabled() == 0 {
        &mut g_cat_nullMsg as (*mut _CatMessage)
    } else {
        (*getContextMessageTree()).canDiscard = 0i32;
        let mut hb : *mut _CatMessage = createCatHeartBeat(type_,name);
        catChecktPtrWithName(
            hb as (*mut ::std::os::raw::c_void),
            (*b"hb\0").as_ptr() as (*mut u8)
        );
        hb
    }
}

#[no_mangle]
pub unsafe extern fn newTransaction(
    mut type_ : *const u8, mut name : *const u8
) -> *mut _CatTransaction {
    if isCatEnabled() == 0 {
        &mut g_cat_nullTrans as (*mut _CatTransaction)
    } else {
        let mut trans
            : *mut _CatTransaction
            = createCatTransaction(type_,name);
        catChecktPtrWithName(
            trans as (*mut ::std::os::raw::c_void),
            (*b"trans\0").as_ptr() as (*mut u8)
        );
        (if trans == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _CatTransaction) {
             0i32 as (*mut ::std::os::raw::c_void) as (*mut _CatTransaction)
         } else {
             catMessageManagerStartTrans(trans);
             trans
         })
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct _CatTransaction {
    pub addData : unsafe extern fn(*mut _CatTransaction, *const u8),
    pub addKV : unsafe extern fn(*mut _CatTransaction, *const u8, *const u8),
    pub setStatus : unsafe extern fn(*mut _CatTransaction, *const u8),
    pub setTimestamp : unsafe extern fn(*mut _CatTransaction, usize),
    pub complete : unsafe extern fn(*mut _CatTransaction),
    pub addChild : unsafe extern fn(*mut _CatTransaction, *mut _CatMessage),
    pub setDurationInMillis : unsafe extern fn(*mut _CatTransaction, usize),
    pub setDurationStart : unsafe extern fn(*mut _CatTransaction, usize),
}

impl Clone for _CatTransaction {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct timeval {
    pub tv_sec : isize,
    pub tv_usec : i32,
}

impl Clone for timeval {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn GetTime64() -> usize {
    let mut buf : usize;
    let mut tv : timeval;
    gettimeofday(
        &mut tv as (*mut timeval),
        0i32 as (*mut ::std::os::raw::c_void)
    );
    buf = (tv.tv_sec * 1000isize + (tv.tv_usec / 1000i32) as (isize)) as (usize);
    buf
}

#[no_mangle]
pub unsafe extern fn newTransactionWithDuration(
    mut type_ : *const u8, mut name : *const u8, mut duration : usize
) -> *mut _CatTransaction {
    let mut trans : *mut _CatTransaction = newTransaction(type_,name);
    ((*trans).setDurationInMillis)(trans,duration);
    if duration < (60i32 * 1000i32) as (usize) {
        ((*trans).setTimestamp)(trans,GetTime64().wrapping_sub(duration));
    }
    trans
}

#[no_mangle]
pub unsafe extern fn newCompletedTransactionWithDuration(
    mut type_ : *const u8, mut name : *const u8, mut duration : usize
) {
    let mut trans
        : *mut _CatTransaction
        = newTransactionWithDuration(type_,name,duration);
    ((*trans).complete)(trans);
}

#[no_mangle]
pub unsafe extern fn createMessageId() -> *mut u8 {
    if isCatEnabled() == 0 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        getNextMessageId()
    }
}

#[no_mangle]
pub unsafe extern fn createRemoteServerMessageId(
    mut appkey : *const u8
) -> *mut u8 {
    if isCatEnabled() == 0 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        getNextMessageIdByAppkey(appkey)
    }
}

#[no_mangle]
pub unsafe extern fn getThreadLocalMessageTreeId() -> *mut u8 {
    if isCatEnabled() == 0 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        (*getContextMessageTree()).messageId
    }
}

#[no_mangle]
pub unsafe extern fn getThreadLocalMessageTreeRootId() -> *mut u8 {
    if isCatEnabled() == 0 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        (*getContextMessageTree()).rootMessageId
    }
}

#[no_mangle]
pub unsafe extern fn getThreadLocalMessageTreeParentId() -> *mut u8 {
    if isCatEnabled() == 0 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        (*getContextMessageTree()).parentMessageId
    }
}

#[no_mangle]
pub unsafe extern fn setThreadLocalMessageTreeId(
    mut messageId : *mut u8
) { if isCatEnabled() == 0 {
    } else {
        let mut pTree : *mut _CatMessageTree = getContextMessageTree();
        if (*pTree).messageId != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            catsdsfree((*pTree).messageId);
            (*pTree).messageId = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
        (*pTree).messageId = catsdsnew(messageId as (*const u8));
    }
}

#[no_mangle]
pub unsafe extern fn setThreadLocalMessageTreeRootId(
    mut messageId : *mut u8
) { if isCatEnabled() == 0 {
    } else {
        let mut pTree : *mut _CatMessageTree = getContextMessageTree();
        if (*pTree).rootMessageId != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            catsdsfree((*pTree).rootMessageId);
            (*pTree).rootMessageId = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
        (*pTree).rootMessageId = catsdsnew(messageId as (*const u8));
    }
}

#[no_mangle]
pub unsafe extern fn setThreadLocalMessageTreeParentId(
    mut messageId : *mut u8
) { if isCatEnabled() == 0 {
    } else {
        let mut pTree : *mut _CatMessageTree = getContextMessageTree();
        if (*pTree).parentMessageId != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            catsdsfree((*pTree).parentMessageId);
            (*pTree).parentMessageId = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
        (*pTree).parentMessageId = catsdsnew(messageId as (*const u8));
    }
}
