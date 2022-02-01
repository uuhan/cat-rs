use nodex::prelude::*;

nodex::napi_module!(init);

pub fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    exports.set("NodeCatTransaction", cat_rs::CatTransaction::class(env)?)?;
    Ok(())
}
