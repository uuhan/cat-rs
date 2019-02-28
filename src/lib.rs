extern crate libc;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;

#[macro_use]
pub(crate) mod ffi;
pub mod cat;

// re-export struct
pub use cat::logEvent;
pub use cat::CatClient;
pub use ffi::CatTransaction;

/// get current cat version
pub fn cat_version() -> &'static str {
    "3.0.1"
}

#[cfg(test)]
mod tests {}
