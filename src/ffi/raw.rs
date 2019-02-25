use std::default::Default;
use std::fmt::{self, Display};

#[derive(Copy)]
#[repr(C)]
pub struct _CatClientConfig {
    pub encoderType: i32,
    pub enableHeartbeat: i32,
    pub enableSampling: i32,
    pub enableMultiprocessing: i32,
    pub enableDebugLog: i32,
}

impl Clone for _CatClientConfig {
    fn clone(&self) -> Self {
        *self
    }
}

pub type CatClientConfig = _CatClientConfig;

impl Default for CatClientConfig {
    fn default() -> Self {
        _CatClientConfig {
            encoderType: 1i32,
            enableHeartbeat: 0i32,
            enableSampling: 1i32,
            enableMultiprocessing: 0i32,
            enableDebugLog: 0i32,
        }
    }
}

impl Display for CatClientConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
