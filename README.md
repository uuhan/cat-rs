## cat [![Build Status](https://travis-ci.org/uuhan/cat-rs.svg?branch=master)](https://travis-ci.org/uuhan/cat-rs)

Rust [cat](https://github.com/dianping/cat#6199dd55e0d8b1d61f08f0a9ebe9281b9f6a6e2a) binding.

NB: This crate is mostly created for Nodejs's Native Addons(using neon) currently.

## Examples

#### create a new cat client

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

#### work with neon

1. cat transaction exported to nodejs.

cat.rs:

```rust
use cat_rs::{self, CatTransaction};
use neon::prelude::*;

declare_types! {
    pub class JsCatTransaction for CatTransaction {
        init(mut ctx) {
            let _type = ctx.argument::<JsString>(0)?.value();
            let _name = ctx.argument::<JsString>(1)?.value();
            let trans = CatTransaction::new(_type, _name);
            Ok(trans)
        }

        method complete(mut ctx) {
            let mut this = ctx.this();
            let guard = ctx.lock();
            {
                let mut trans  = this.borrow_mut(&guard);
                trans.complete();
            }
            Ok(ctx.undefined().upcast())
        }

        method log(mut ctx) {
            let _type = ctx.argument::<JsString>(0)?.value();
            let _name = ctx.argument::<JsString>(0)?.value();
            let _stat = ctx.argument::<JsString>(0)?.value();
            let _data = ctx.argument::<JsString>(0)?.value();

            {
                let mut this = ctx.this();
                let guard = ctx.lock();
                let mut trans = this.borrow_mut(&guard);

                trans.log(_type, _name, _stat, _data);
            }

            Ok(ctx.undefined().upcast())
        }
    }
}
```

2. register this class

lib.rs

```rust
use cat::JsCatTransaction;

register_module!(mut ctx, {
    ctx.export_class::<JsCatTransaction>("NodeCatTransaction")?;
})
```
