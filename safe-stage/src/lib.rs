pub mod concrete_parts;
pub mod concrete_resolvers;
pub mod configuration;
mod ffi;
pub mod id;
pub mod microscope;
pub mod presentation;
pub mod types;

#[cfg(feature = "ffi")]
#[no_mangle]
pub extern "C" fn init_logger() {
    fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("safe-stage.log").unwrap())
        .apply()
        .unwrap()
}
