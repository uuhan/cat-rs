#![allow(unused)]
extern crate cat_rs as cat;

use cat::logEvent;
use cat::CatClient;
use cat::CatTransaction;

pub fn main() {
    let mut cat = CatClient::new("test");
    cat.init();
    let tr = CatTransaction::new("foo", "bar");
    assert!(!tr.is_null());

    unsafe {
        logEvent("foo", "bar", "0", "");
        logEvent("foo", "bar", "1", "");
        (*tr).complete();
    }
}
