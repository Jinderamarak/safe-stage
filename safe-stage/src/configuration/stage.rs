use crate::concrete_parts::ConcreteStage;
use models::assembly::thesis::ThesisStage;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum StageConfig {
    ThesisStage,
    ExampleStageWithConfig {
        calibration_x: f64,
        tilt_correction: f64,
    },
}

impl StageConfig {
    /// Create a new **ThesisStage** configuration.
    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn thesis_stage() -> Self {
        StageConfig::ThesisStage
    }

    /// Create a new **ExampleStageWithConfig** configuration.
    ///
    /// This is an example on how to represent a configuration with parameters.
    /// Building the stage with this configuration is not implemented.
    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn example_stage_with_config(calibration_x: f64, tilt_correction: f64) -> Self {
        StageConfig::ExampleStageWithConfig {
            calibration_x,
            tilt_correction,
        }
    }

    pub(crate) fn build(&self) -> ConcreteStage {
        match self {
            StageConfig::ThesisStage => ConcreteStage::new(ThesisStage::default()),
            StageConfig::ExampleStageWithConfig { .. } => {
                todo!("ConcreteStage::new(ExampleStageWithConfig::new(a, b))")
            }
        }
    }
}
