extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=D:/vcpkg/installed/x64-windows/lib");

    println!("cargo:rustc-link-lib=freerdp2");
    println!("cargo:rustc-link-lib=freerdp-client2");
    println!("cargo:rustc-link-lib=winpr2");
    println!("cargo:rustc-link-lib=winpr-tools2");

    println!("cargo:rerun-if-changed=rdp-wrapper.h");

    let bindings = bindgen::Builder::default()
        .allowlist_function(".*Channel.*")
        .allowlist_function("Close.*")
        .allowlist_function("Create.*")
        .allowlist_function("FreeRDP.*")
        .allowlist_function("Get.*")
        .allowlist_function("PubSub.*")
        .allowlist_function("WLog.*")
        .allowlist_function("Wait.*")
        .allowlist_function("client_.*")
        .allowlist_function("cliprdr_.*")
        .allowlist_function("freerdp_.*")
        .allowlist_function("gdi_.*")
        .allowlist_function("graphics_.*")
        .allowlist_function("rdpgfx_.*")
        .allowlist_function("stream_.*")
        .allowlist_type(".*ClientContext")
        .allowlist_type("CLIP.*")
        .allowlist_type("Disp.*")
        .allowlist_type("Rdp.*")
        .allowlist_var("AUDIN_.*")
        .allowlist_var("CAT_.*")
        .allowlist_var("CB_.*")
        .allowlist_var("CF_.*")
        .allowlist_var("CLIPRDR_.*")
        .allowlist_var("CONNECTION_.*")
        .allowlist_var("ERRBASE.*")
        .allowlist_var("ERRCONNECT.*")
        .allowlist_var("ERRINFO.*")
        .allowlist_var("FREERDP.*")
        .allowlist_var("FreeRDP.*")
        .allowlist_var("OS.*")
        .allowlist_var("PIXEL_.*")
        .allowlist_var("RDP.*")
        .allowlist_var("WAIT_.*")
        .header("rdp-wrapper.h")
        .clang_arg("-ID:/vcpkg/installed/x64-windows/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
