use std::path::Path;
use std::env;

fn main() {

    node_bindgen::build::configure();

    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = Path::new(&root_dir).join("lib");

    let acbr_lib = "acbrposprinter64";

    println!("cargo:rustc-link-lib=dylib={}", acbr_lib);
    println!("cargo:rustc-link-search=native={}", lib_dir.join("acbr").display());

}