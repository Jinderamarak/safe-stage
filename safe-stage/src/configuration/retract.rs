use crate::concrete_parts::ConcreteRetract;
use models::assembly::thesis::ThesisRetract;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum RetractConfig {
    ThesisRetract,
    ExampleRetractionWithConfig { arm_length: f64, speed: f64 },
}

impl RetractConfig {
    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn thesis_retract() -> Self {
        RetractConfig::ThesisRetract
    }

    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn example_retraction_with_config(arm_length: f64, speed: f64) -> Self {
        RetractConfig::ExampleRetractionWithConfig { arm_length, speed }
    }

    pub fn build(&self) -> ConcreteRetract {
        match self {
            RetractConfig::ThesisRetract => ConcreteRetract::new(ThesisRetract::default()),
            RetractConfig::ExampleRetractionWithConfig { .. } => {
                todo!("ConcreteRetract::new(ExampleRetractionWithConfig::new(a, b))")
            }
        }
    }
}
