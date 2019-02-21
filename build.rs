use cmake;

fn main() {
    let dst = cmake::build("./ccat/");
    println!("cargo:rustc-link-search=native={}/lib/", dst.display());
    println!("cargo:rustc-link-lib=static=catclient");
}
