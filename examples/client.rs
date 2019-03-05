#![allow(unused)]
extern crate cat_rs as cat;

use std::alloc::System;
use std::error::Error;
use std::result::Result;

#[global_allocator]
static GLOBAL: System = System;

use cat::logEvent;
use cat::CatClient;
use cat::CatTransaction;

pub fn main() -> Result<(), Box<Error>> {
    let mut cat = CatClient::new("test");
    cat.init()?;
    let mut tr = CatTransaction::new("foo", "bar");

    unsafe {
        logEvent("foo", "bar", "0", "");
        logEvent("foo", "bar", "1", "");
    }

    Ok(())
}
