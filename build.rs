use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        .clang_args(&["-x", "c", "-std=gnu99"])
        .header("lib/cat/lib/c/include/client.h")
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out.join("ffi.rs"))
        .expect("Can not write bindings!");

    let lib = cmake::build("lib/cat/lib/c/");
    println!("cargo:rustc-link-search=native={}/lib", lib.display());
    println!("cargo:rustc-link-lib=static=catclient");
}
