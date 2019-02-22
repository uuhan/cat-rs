extern crate cat_rs as cat;

use cat::CatClient;
use cat::CatTransaction;

extern "C" {
    fn catClientDestroy();
}

pub fn main() {
    let mut cat = CatClient::new("test".to_owned());
    cat.init(None);
    let version = cat.version();
    let mut tr = CatTransaction::new("foo".to_owned(), "bar".to_owned());
    unsafe {
        (*tr).complete();
        catClientDestroy();
    }

    println!("{}", version);
}
