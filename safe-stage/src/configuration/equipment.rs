use crate::concrete_parts::ConcreteEquipment;
use models::assembly::thesis::{ThesisDetectorAlpha, ThesisDetectorBeta};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum EquipmentConfig {
    ThesisDetectorAlpha,
    ThesisDetectorBeta,
    ExampleEquipmentWithConfig { position: f64, size: u32 },
}

impl EquipmentConfig {
    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn thesis_detector_alpha() -> Self {
        EquipmentConfig::ThesisDetectorAlpha
    }

    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn thesis_detector_beta() -> Self {
        EquipmentConfig::ThesisDetectorBeta
    }

    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn example_equipment_with_config(position: f64, size: u32) -> Self {
        EquipmentConfig::ExampleEquipmentWithConfig { position, size }
    }

    pub(crate) fn build(&self) -> ConcreteEquipment {
        match self {
            EquipmentConfig::ThesisDetectorAlpha => {
                ConcreteEquipment::new(ThesisDetectorAlpha::default())
            }
            EquipmentConfig::ThesisDetectorBeta => {
                ConcreteEquipment::new(ThesisDetectorBeta::default())
            }
            EquipmentConfig::ExampleEquipmentWithConfig { .. } => {
                todo!("ConcreteEquipment::new(ExampleEquipmentWithConfig::new(a, b))")
            }
        }
    }
}
