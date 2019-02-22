extern crate cat_rs as cat;

use cat::CatClient;
use cat::CatTransaction;

extern "C" {
    fn catClientDestroy();
}

pub fn main() {
    let mut cat = CatClient::new("test");
    cat.init();
    let version = cat.version();
    let tr = CatTransaction::new("foo".to_owned(), "bar".to_owned());
    unsafe {
        (*tr).complete();
        catClientDestroy();
    }

    println!("{}", version);
}
