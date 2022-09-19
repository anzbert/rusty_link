use bindgen;
use cmake;

fn main() {
    // ---------
    // - CMAKE -
    // ---------
    let out_dir = cmake::Config::new("cmake")
        // .cxxflag("-fno-rtti")
        .build_target("lib_link_c")
        .build();

    // WINDOWS: Visual Studio output to OUT_DIR/{Debug, Release, RelWithDebInfo} etc.
    #[cfg(target_os = "windows")]
    let build_dir = out_dir
        .join("build")
        .join(cmake::Config::new("cmake").get_profile());

    // NOT WINDOWS: Other generators just output directly to OUT_DIR
    #[cfg(not(target_os = "windows"))]
    let build_dir = out_dir.join("build");

    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib=static=lib_link_c");

    // MACOS: Apparently important! Otherwise linker errors, apparently only on macOS
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");

    // -----------
    // - BINDGEN -
    // -----------

    let bindings = bindgen::builder()
        .header("link/extensions/abl_link/include/abl_link.h")
        .allowlist_function("abl_link_.*")
        .generate()
        .expect("Failed to generate bindings");

    let bindings_file_path = out_dir.join("link_rs.rs");
    bindings
        .write_to_file(bindings_file_path)
        .expect("Failed to write bindings to file");
}
