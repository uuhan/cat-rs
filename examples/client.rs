#![allow(unused)]
extern crate cat_rs as cat;
extern crate threadpool;

use std::alloc::System;
use std::error::Error;
use std::result::Result;

use threadpool::ThreadPool;

#[global_allocator]
static GLOBAL: System = System;

use cat::logEvent;
use cat::CatClient;
use cat::CatTransaction;

pub fn main() -> Result<(), Box<Error>> {
    let mut cat = CatClient::new("test");
    cat.init()?;

    let mut tr = CatTransaction::new("foo", "bar");

    tr.log("test", "it", "0", " ");
    tr.complete();

    Ok(())
}
