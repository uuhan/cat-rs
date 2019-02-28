extern crate cat_rs as cat;

use cat::logEvent;
use cat::CatClient;
use cat::CatTransaction;

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::CatClient;
    use super::CatTransaction;

    #[test]
    fn test_transaction() {
        let mut cat = CatClient::new("test");
        cat.init();
        let tr = CatTransaction::new("foo", "bar");
        assert!(!tr.is_null());

        unsafe {
            super::logEvent("foo", "bar", "0", "");
            super::logEvent("foo", "bar", "1", "");
            (*tr).complete()
        }
    }
}
