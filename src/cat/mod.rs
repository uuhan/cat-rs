#![allow(non_snake_case)]
use crate::cat_version;
use crate::ffi::*;
use std::ffi::CString;

pub struct CatClient {
    appkey: String,
    config: CatClientConfig,
}

impl CatClient {
    pub fn new<T: ToString>(appkey: T) -> Self {
        CatClient {
            appkey: appkey.to_string(),
            config: CatClientConfig::default(),
        }
    }

    pub fn config(&mut self, config: &mut CatClientConfig) -> &Self {
        self.config = *config;
        self
    }

    pub fn init(&mut self) -> &Self {
        unsafe {
            catClientInitWithConfig(
                CString::new(self.appkey.clone()).unwrap().as_ptr() as *const u8,
                &mut self.config,
            );
            self
        }
    }

    pub fn destroy(&self) {
        warn!("cat client is being destroyed!");
        unsafe { catClientDestroy() };
    }

    pub fn version(&self) -> &str {
        cat_version()
    }
}

impl Drop for CatClient {
    fn drop(&mut self) {
        warn!("cat client destroyed!");
        self.destroy()
    }
}

pub fn logEvent<S: ToString>(type_: S, name_: S, status: S, data: S) {
    unsafe {
        crate::ffi::logEvent(
            c!(type_.to_string()),
            c!(name_.to_string()),
            c!(status.to_string()),
            c!(data.to_string()),
        )
    }
}
