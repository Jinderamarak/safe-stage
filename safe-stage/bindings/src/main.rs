mod transform;

use std::fs;
use std::path::Path;
use std::process::Command;

const DYNAMIC_LIB_NAME: &str = "safe_stage";
const C_FILE_NAME: &str = "bindings.g.h";
const CXX_FILE_NAME: &str = "bindings.g.hpp";
const CSHARP_FILE_NAME: &str = "Bindings.g.cs";

fn main() -> Result<(), String> {
    let expanded = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("generated")
        .join("crate");
    rust_expanded_crate(&expanded)?;

    let c = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("generated")
        .join(C_FILE_NAME);
    generate_c_bindings(&expanded, &c);

    let cxx = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("generated")
        .join(CXX_FILE_NAME);
    generate_cxx_bindings(&expanded, &cxx);

    //  Transform the Rust source code to make it easier for `csbindgen`
    let transformed = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("generated")
        .join("transformed.rs");
    let expanded_src = expanded.join("src").join("lib.rs");
    let source = fs::read_to_string(expanded_src).expect("Failed to read expanded source");
    let source = transform::transform_rust_source_code(&source);
    fs::write(&transformed, &source).expect("Failed to write transformed source");

    let csharp = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("generated")
        .join(CSHARP_FILE_NAME);
    generate_csharp_bindings(&transformed, &csharp, DYNAMIC_LIB_NAME);

    const START_GREEN_BOLD: &str = "\x1b[32;49;1m";
    const START_DEFAULT_BOLD: &str = "\x1b[39;49;1m";
    const RESET: &str = "\x1b[0m";
    println!(
        "{START_GREEN_BOLD}Success!{RESET} Dont forget to build the library with `ffi` feature!",
    );
    println!("Run {START_DEFAULT_BOLD}cargo build --release --features ffi{RESET}",);

    Ok(())
}

/// Expand the Rust crate to evaluate the macros.
fn rust_expanded_crate(output: impl AsRef<Path>) -> Result<(), String> {
    let expand = Command::new("cargo")
        .arg("rustc")
        .arg("--profile=check")
        .arg("--features")
        .arg("ffi")
        .arg("--")
        .arg("-Zunpretty=expanded")
        .output()
        .expect("Failed to run `cargo rustc`");

    if !expand.status.success() {
        eprintln!(
            "Library build failed:\n{}",
            String::from_utf8_lossy(&expand.stderr)
        );
        return Err("Library build failed".to_string());
    }

    let src = output.as_ref().join("src");
    fs::create_dir_all(src.as_path()).expect("Failed to create crate directory");

    let src_file = src.join("lib.rs");
    fs::write(src_file, &expand.stdout).expect("Failed to write crate source");

    let manifest = output.as_ref().join("Cargo.toml");
    fs::write(
        manifest,
        r#"
[workspace]
[package]
name = "crate"
[lib]
[dependencies]
    "#,
    )
    .expect("Failed to write crate manifest");

    println!("Crate expanded");
    Ok(())
}

/// Generate C bindings with `cbindgen`.
fn generate_c_bindings(input: impl AsRef<Path>, output: impl AsRef<Path>) {
    cbindgen::Builder::new()
        .with_crate(input)
        .with_documentation(true)
        .with_std_types(true)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Failed to generate C bindings")
        .write_to_file(output);

    println!("C bindings generated");
}

/// Generate C++ bindings with `cbindgen`.
fn generate_cxx_bindings(input: impl AsRef<Path>, output: impl AsRef<Path>) {
    cbindgen::Builder::new()
        .with_crate(input)
        .with_documentation(true)
        .with_pragma_once(true)
        .with_std_types(true)
        .with_language(cbindgen::Language::Cxx)
        .generate()
        .expect("Failed to generate C++ bindings")
        .write_to_file(output);

    println!("C++ bindings generated");
}

/// Generate C# bindings with `csbindgen`.
fn generate_csharp_bindings(input: impl AsRef<Path>, output: impl AsRef<Path>, lib_name: &str) {
    let mut builder = csbindgen::Builder::default()
        .input_extern_file(input)
        .csharp_namespace("BindingsCs.Unsafe");

    if cfg!(target_os = "windows") {
        builder = builder.csharp_dll_name(format!("{lib_name}.dll"));
    } else if cfg!(target_os = "linux") {
        builder = builder.csharp_dll_name(format!("{lib_name}.so"));
    } else if cfg!(target_os = "macos") {
        builder = builder.csharp_dll_name(format!("{lib_name}.dylib"));
    } else {
        panic!("Unsupported target OS");
    }

    builder
        .generate_csharp_file(output)
        .expect("Failed to generate C# bindings");

    println!("C# bindings generated");
}
