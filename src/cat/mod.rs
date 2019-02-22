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

    pub fn init(&mut self, config: Option<CatClientConfig>) -> &Self {
        match config {
            Some(mut config) => unsafe {
                let appkey = c!(self.appkey.clone());
                info!("cat client <{}> init with config: {}", self.appkey, config);
                let rc = catClientInitWithConfig(appkey, &mut config);
                if rc != 0 {
                    info!("success!")
                } else {
                    error!("failed!")
                }
            },
            None => unsafe {
                let appkey = c!(self.appkey.clone());
                info!(
                    "cat client <{}> init with config: {}",
                    self.appkey, &self.config
                );
                let rc = catClientInitWithConfig(appkey, &mut self.config);
                if rc != 0 {
                    info!("success!")
                } else {
                    error!("failed!")
                }
            },
        }

        self
    }

    pub fn destroy(&self) {
        warn!("cat client is being destroyed!");
        let rc = unsafe { catClientDestroy() };
        if rc != 0 {
            warn!("cat is destroyed successfully!")
        } else {
            error!("cat is destroyed failed!")
        }
    }

    pub fn version(&self) -> &str {
        catVersion()
    }
}
