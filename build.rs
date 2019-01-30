use cmake;

fn main() {
    let dst = cmake::build("./cat-sys/lib/c");
    println!("cargo:rustc-link-search=native={}/lib/", dst.display());
    println!("cargo:rustc-link-lib=static=catclient");
}
