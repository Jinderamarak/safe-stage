# Safe Stage - Electron microscope safe navigation

System for safe navigation of a stage and retractable devices inside a chamber of a scanning electron microscope.

This project was created as a part of a bachelor's thesis at the Faculty of Informatics, Masaryk University Brno.


## Project parts

### [`safe-stage`](safe-stage/README.md)

The safety system itself is written in Rust.

### [`bindings`](bindings/README.md)

Bindings for the `safe-stage` Rust library. Currently, safe bindings only for C#, but unsafe header files are also generated for C and C++.

### [`service-app`](bindings/README.md)

Desktop service application written in C#, WPF. Connects to `safe-stage` using the safe bindings.


## Requirements

### Running

Only Windows and .NET runtime are needed to run a compiled binary of the service application.

### Building

- installed Rust with version 1.86 or newer
  - As of 18/12/2024, the *Nightly* version of the Rust toolchain results in a slightly better performance and is therefore recommended
- installed .NET SDK 8 or newer
- installed `cargo-expand` or selected nightly chain (run `cargo install cargo-expand` to install) - required for generating bindings
  - Nightly toolchain can be used as a fallback (running without `cargo-expand`), but might be unstable
- (optionally) setup `miri` with nightly toolchain to test undefined behavior
- (optionally) setup `nextest` to run unit tests with reports

### MSRV (Minimum Supported Rust Version)

- `1.86.0` - floats in `const` context, trait upcasting coercion


## Building

1. Enter the `safe-stage` directory
2. Compile the library with ffi feature and in release mode with `cargo build --release --features ffi`
3. Generate unsafe bindings with `cargo run -p bindings`
4. Go back to main directory and enter the `service-app` directory
    - Use `dotnet run --project ServiceApp` to directly run it
    - Use `dotnet publish` to create release build

When lost, try looking at the CI/CD pipeline and how it is done there.


## Representative SEM

Exported STL files can be found here: `safe-stage/models/src/assembly/thesis/models`

Or can be viewed online in Onshape: https://cad.onshape.com/documents/694e38e8d979f14ad35f8ff9/w/1727b025a7bb93644c4c09d6/e/62cb081996d848e3b3a7ae06
