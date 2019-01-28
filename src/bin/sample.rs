extern crate cat_rs as cat;

use cat::CatClient;
use cat::CatTransaction;

pub fn main() {
    let cat = CatClient::new("test".to_owned());
    cat.init(None);
    let version = cat.version();
    let tr = CatTransaction::new("foo".to_owned(), "bar".to_owned());
    tr.complete();

    println!("{}", version);
}
