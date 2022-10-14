use bindgen;
use cmake;

fn main() {
    // ---------
    // - CMAKE -
    // ---------

    // Get cmake config from 'cmake/CMakeLists.txt', build and return $OUT_DIR
    let out_dir = cmake::Config::new("cmake")
        .build_target("lib_abl_link_c")
        .build();

    // WINDOWS: Builds into $OUT_DIR/{Debug, Release, ...}
    #[cfg(target_os = "windows")]
    let build_dir = out_dir
        .join("build")
        .join(cmake::Config::new("cmake").get_profile());

    // NON-WINDOWS: Builds into $OUT_DIR/build
    #[cfg(not(target_os = "windows"))]
    let build_dir = out_dir.join("build");

    // MACOS: Link standard C++ lib
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");

    // Statically link finished cmake build into executable
    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib=static=lib_abl_link_c");

    // -----------
    // - BINDGEN -
    // -----------

    let bindings = bindgen::builder()
        .header("link/extensions/abl_link/include/abl_link.h")
        .allowlist_function("abl_link_.*")
        .generate()
        .expect("Failed to generate C bindings");

    bindings
        .write_to_file(out_dir.join("link_bindings.rs"))
        .expect("Failed to write C bindings to link_bindings.rs");
}
