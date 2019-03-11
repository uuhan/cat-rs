use crate::ffi::{self, *};
use libc::c_char;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::sync::atomic::AtomicBool;

static mut G_CAT_INIT: AtomicBool = AtomicBool::new(false);

pub static mut g_config: CatClientInnerConfig = CatClientInnerConfig {
    appkey: 0 as (*mut c_char),
    selfHost: 0 as (*mut c_char),
    serverHost: 0 as (*mut c_char),
    defaultIp: 0 as (*mut c_char),
    defaultIpHex: 0 as (*mut c_char),
    serverPort: 0u32,
    serverNum: 0i32,
    serverAddresses: 0 as (*mut *mut c_char),
    messageEnableFlag: 0i32,
    messageQueueSize: 0i32,
    messageQueueBlockPrintCount: 0i32,
    maxChildSize: 0i32,
    maxContextElementSize: 0i32,
    logFlag: 0i32,
    logSaveFlag: 0i32,
    logDebugFlag: 0i32,
    logFileWithTime: 0i32,
    logFilePerDay: 0i32,
    logLevel: 0i32,
    configDir: 0 as (*mut c_char),
    dataDir: 0 as (*mut c_char),
    indexFileName: 0 as (*mut c_char),
    encoderType: 0i32,
    enableHeartbeat: 0i32,
    enableSampling: 0i32,
    enableMultiprocessing: 0i32,
};

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    pub ip: String,
    pub port: u16,
    #[serde(rename(deserialize = "http-port"))]
    pub http_port: u16,
}

pub unsafe fn catClientWithConfig(appkey: String, config: ffi::CatClientConfig) -> bool {
    if *G_CAT_INIT.get_mut() {
        return true;
    }

    let mut f = File::open("cat.client.json").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let conf: ClientConfig = serde_json::from_str(contents.as_str()).unwrap();

    libc::signal(libc::SIGPIPE, libc::SIG_IGN);
    initCatClientConfig(config);

    false
}

pub unsafe fn initCatClientConfig(mut config: ffi::CatClientConfig) {
    unimplemented!()
}
