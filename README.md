## cat [![Build Status](https://travis-ci.org/uuhan/cat-rs.svg?branch=master)](https://travis-ci.org/uuhan/cat-rs)

Rust [cat](https://github.com/dianping/cat#6199dd55e0d8b1d61f08f0a9ebe9281b9f6a6e2a) binding.

NB: This crate is mostly created for Nodejs's Native Addons(using neon) currently.

## Examples

```rust
extern crate cat_rs as cat;
use cat::logEvent;
use cat::CatClient;
use cat::CatTransaction;

let mut cat = CatClient::new("test");
cat.init()?
let mut tr = CatTransaction::new("foo", "bar")

tr.log("test", "it", "0", "");
tr.complete();

Ok(())
```
