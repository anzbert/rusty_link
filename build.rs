use bindgen;
use cmake;

fn main() {
    // ---------
    // - CMAKE -
    // ---------

    // Get cmake config from 'cmake/CMakeLists.txt', build and return '$OUT_DIR'
    let out_dir = cmake::Config::new("cmake").build();

    // Needs linking of standard C++ Lib on MacOS
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");

    // Statically link finished cmake build into executable from '$OUT_DIR/lib'
    println!("cargo:rustc-link-search=native={}/lib", out_dir.display());
    println!("cargo:rustc-link-lib=static=lib_abl_link");

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
