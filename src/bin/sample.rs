extern crate cat_rs as cat;

use cat::CatClient;
use cat::CatTransaction;
use std::ptr;

pub fn main() {
    let mut cat = CatClient::new("test");
    cat.init();
    let version = cat.version();
    let tr = CatTransaction::new("foo".to_owned(), "bar".to_owned());
    unsafe {
        cat::logEvent(
            b"foo\0".as_ptr() as *const u8,
            b"bar\0".as_ptr() as *const u8,
            b"0\0".as_ptr() as *const u8,
            ptr::null(),
        );
        cat::logEvent(
            b"foo\0".as_ptr() as *const u8,
            b"bar\0".as_ptr() as *const u8,
            b"1\0".as_ptr() as *const u8,
            ptr::null(),
        );
        (*tr).complete();
    }

    println!("{}", version);
}
