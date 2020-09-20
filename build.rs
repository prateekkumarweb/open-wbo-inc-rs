use std::env;
use std::path::PathBuf;

fn main() {
    // println!("cargo:rustc-link-search=native={}", "Open-WBO-Inc");
    // println!("cargo:rustc-link-lib=static=_release");
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    println!("cargo:rustc-link-lib=static=open-wbo-inc");

    println!("cargo:rustc-link-search=Open-WBO-Inc");

    println!("cargo:rustc-link-lib=stdc++");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .detect_include_paths(true)
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-IOpen-WBO-Inc")
        .clang_arg("-IOpen-WBO-Inc/solvers/glucose4.1")
        .clang_args(vec![
            "-DNSPACE=Glucose",
            "-DSOLVERNAME=Glucose4.1",
            "-DVERSION=core",
        ])
        .opaque_type("std::.*")
        .whitelist_type("openwbo::.*")
        // -DNSPACE=$(NSPACE) -DSOLVERNAME=$(SOLVERNAME) -DVERSION=$(VERSION)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
