use std::env;
use cmake::Config;

fn main() {
    let dst = Config::new("libdivsufsort")
        .define("BUILD_EXAMPLES", "ON")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_DIVSUFSORT64", "OFF")
        .define("CMAKE_INSTALL_LIBDIR", env::var("OUT_DIR").unwrap())
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=divsufsort");
}
