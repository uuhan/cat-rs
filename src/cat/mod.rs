use crate::ffi::*;
use std::ffi::CString;

pub struct CatClient<'a> {
    appkey: &'a str,
    config: CatClientConfig,
}

impl<'a> CatClient<'a> {
    pub fn new(appkey: &'a str) -> Self {
        CatClient {
            appkey,
            config: CatClientConfig::default(),
        }
    }

    pub fn config(&mut self, config: &mut CatClientConfig) -> &Self {
        self.config = *config;
        self
    }

    pub fn init(&mut self) -> &Self {
        unsafe {
            info!(
                "cat client <{}> init with config: {}",
                self.appkey, self.config
            );
            catClientInitWithConfig(
                CString::new(self.appkey).unwrap().as_ptr() as *const u8,
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
        catVersion()
    }
}
