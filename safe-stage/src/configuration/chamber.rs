use crate::concrete_parts::ConcreteChamber;
use models::assembly::thesis::ThesisChamber;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum ChamberConfig {
    ThesisChamber,
    ExampleChamberWithConfig { offset_x: f64, size: u32 },
}

impl ChamberConfig {
    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn thesis_chamber() -> Self {
        ChamberConfig::ThesisChamber
    }

    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn example_chamber_with_config(offset_x: f64, size: u32) -> Self {
        ChamberConfig::ExampleChamberWithConfig { offset_x, size }
    }

    pub(crate) fn build(&self) -> ConcreteChamber {
        match self {
            ChamberConfig::ThesisChamber => ConcreteChamber::new(ThesisChamber::default()),
            ChamberConfig::ExampleChamberWithConfig { .. } => {
                todo!("ConcreteChamber::new(ExampleChamberWithConfig::new(a, b))")
            }
        }
    }
}
