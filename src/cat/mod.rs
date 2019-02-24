use crate::ffi::*;
use std::ffi::CString;

pub struct CatClient {
    appkey: String,
    config: CatClientConfig,
}

impl CatClient {
    pub fn new(appkey: String) -> Self {
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
        catVersion()
    }
}

impl Drop for CatClient {
    fn drop(&mut self) {
        warn!("cat client destroyed!");
        self.destroy()
    }
}
