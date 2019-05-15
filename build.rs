extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
   
    cc::Build::new()
        .file("duktape-src/duktape.c")
        .compile("duktape");

    let bindings = bindgen::Builder::default()
        .header("duktape-src/duktape.h")
        // .whitelist_type("DUK_(.*)")
        .whitelist_var("DUK_(.*)")
        .whitelist_function("duk_(.*)")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

}