extern crate cc;
extern crate bindgen;

use std::env;
use std::fs;

fn main() {
    // Link Dynamsoft Barcode Reader.
    println!("cargo:rustc-link-search=./platforms/win");
    println!("cargo:rustc-link-lib=DBRx64");

    // Build C code.
    cc::Build::new()
        .include("include")
        .file("src/bridge.c")
        .compile("bridge");

    // Generates Rust FFI bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/bridge.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bridge.rs")
        .expect("Couldn't write bindings!");

    // Get the output path
    let out_dir = env::var("OUT_DIR").unwrap();
    let debug_offset = out_dir.find("debug").unwrap_or(0);
    let release_offset = out_dir.find("release").unwrap_or(0);
    let mut path = String::from("");

    if debug_offset > 0 {
        println!(">>> where is debug {}", debug_offset);
        path.push_str(&format!("{}", &out_dir[..debug_offset]));
        path.push_str("debug");
        println!("{}", path);
    }

    if release_offset > 0 {
        println!(">>> where is release {}", release_offset);
        path.push_str(&format!("{}", &out_dir[..release_offset]));
        path.push_str("release");
        println!("{}", path);
    }

    // Copy *.dll to the output path
    let _src: String = String::from("./platforms/win/DynamsoftBarcodeReaderx64.dll");
    path.push_str("/DynamsoftBarcodeReaderx64.dll");
    let _result = fs::copy(_src, &path);
}
