use std::env;
use std::path::PathBuf;

fn main() {
    // 读取环境变量HW_DCMI_PATH作为库搜索路径
    let hw_dcmi_path = env::var("HW_DCMI_PATH").unwrap_or_else(|_| "/usr/local/dcmi".to_string());
    let interface_path = format!("{}/dcmi_interface_api.h", hw_dcmi_path);
    println!("cargo:rustc-link-search=native={}", hw_dcmi_path);

    // Tell cargo to tell rustc to link the dcmi shared library.
    println!("cargo:rustc-link-lib=dylib=dcmi");

    // The bindgen::Builder is the main entry point to bindgen,
    // and lets you build up options for the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header(interface_path)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // 指定输出文件的路径为 src/hw_dcmi_sys.rs
    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/hw_dcmi_sys.rs");

    // Write the bindings to the specified file.
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
