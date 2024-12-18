//! # Safe Stage
//! Safe navigation of a stage (and retractable devices) in a Scanning Electron Microscope (SEM).
//!
//! Provides `Rust` and `C` APIs. Use the `ffi` feature to switch to the `C` API.
//!
//! To start, use the [ConfigurationBuilder] to create a [Configuration] representing
//! the microscope's static configuration, then create instance of a [Microscope]
//! from the configuration. Interact with the instance of the [Microscope] to swap parts,
//! change sample, change the state, find path for stage navigation
//! or check insertion/retraction of retractable devices.

pub mod concrete_parts;
pub mod concrete_resolvers;
pub mod configuration;
mod ffi;
pub mod id;
pub mod microscope;
pub mod presentation;
pub mod types;

/// Initializes logging for the Rust library.
///
/// Currently logs with `Debug` level to a file named `safe-stage.log`.
#[cfg(feature = "ffi")]
#[no_mangle]
pub extern "C" fn init_logger() {
    fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("safe-stage.log").unwrap())
        .apply()
        .unwrap()
}
