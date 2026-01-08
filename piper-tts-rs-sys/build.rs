use std::{env, path::PathBuf};

fn main() {
    let libdir = format!(
        "{}/libpiper/install",
        env::var("CARGO_MANIFEST_DIR").expect("Unable to find manifest dir")
    );

    println!("cargo:rustc-link-search=native={}", libdir);
    println!("cargo:rustc-link-lib=piper");

    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Ilibpiper/install/include")
        .generate()
        .expect("Bindgen failed")
        .write_to_file(out_path.join("src/bindings.rs"))
        .expect("Couldn't write bindings");
}
