#![allow(unused)]
extern crate cat_rs as cat;
extern crate threadpool;

use std::alloc::System;
use std::error::Error;
use std::result::Result;
use std::thread::sleep_ms;

use threadpool::ThreadPool;

#[global_allocator]
static GLOBAL: System = System;

use cat::logEvent;
use cat::CatClient;
use cat::CatTransaction;
use cat::CatTransactionService;

pub fn main() -> Result<(), Box<Error>> {
    let mut cat = CatClient::new("test");
    cat.init()?;

    let mut trs = CatTransactionService::new().pool_size(4).init();
    let mut tr = trs.create("foo", "bar");

    tr.logEvent("test", "it", "0", " ");
    tr.complete();

    trs.pool.unwrap().join();

    Ok(())
}
