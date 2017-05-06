extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings=bindgen::Builder::default()
        .no_unstable_rust()
        .header("src/ffi/ps2000a/ps2000aApi.h")
        .generate().expect("Error generating bindings");
    
    let out_path=PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("ps2000a.rs")).expect("No bindings written");
}