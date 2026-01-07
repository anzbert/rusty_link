fn main() {
    // ---------
    // - CMAKE -
    // ---------

    // Read `CMakeLists.txt` from `cmake` directory, build and return '$OUT_DIR'
    let out_dir = cmake::Config::new("cmake").build();

    // Link standard C++ lib
    // #[cfg(target_os = "macos")] // changed to if statements below to allow crosscompilation to different OS
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "macos" {
        println!("cargo:rustc-link-lib=c++");
    }
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "linux" {
        println!("cargo:rustc-link-lib=stdc++");
    }

    // Link finished build into executable from '$OUT_DIR/lib'
    println!("cargo:rustc-link-search=native={}/lib", out_dir.display());
    println!("cargo:rustc-link-lib=static=lib_abl_link");

    // -----------
    // - BINDGEN -
    // -----------

    let bindings = bindgen::builder()
        .header("link/extensions/abl_link/include/abl_link.h")
        .allowlist_function("abl_link_.*")
        .generate()
        .expect("Failed to generate C bindings.");

    bindings
        .write_to_file(out_dir.join("link_bindings.rs"))
        .expect("Failed to write bindings to `link_bindings.rs`.");
}
