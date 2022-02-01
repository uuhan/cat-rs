#![allow(unused)]
extern crate cat_rs as cat;

use std::alloc::System;
use std::error::Error;
use std::result::Result;

#[global_allocator]
static GLOBAL: System = System;

use cat::log_event;
use cat::CatClient;
use cat::CatTransaction;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut cat = CatClient::new("test");
    cat.init()?;

    let mut tr = CatTransaction::new("foo", "bar");

    tr.log("test", "it", "0", " ");
    tr.complete();

    Ok(())
}
