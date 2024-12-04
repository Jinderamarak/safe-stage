use models::assembly::thesis::{ThesisHolderCircle, ThesisHolderSquare};
use models::parts::holder::Holder;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum HolderConfig {
    ThesisHolderCircle,
    ThesisHolderSquare,
    ExampleHolderWithConfig { height: f64, width: f64 },
}

impl HolderConfig {
    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn thesis_holder_circle() -> Self {
        HolderConfig::ThesisHolderCircle
    }

    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn thesis_holder_square() -> Self {
        HolderConfig::ThesisHolderSquare
    }

    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn example_holder_with_config(height: f64, width: f64) -> Self {
        HolderConfig::ExampleHolderWithConfig { height, width }
    }

    pub(crate) fn build(&self) -> Box<dyn Holder> {
        match self {
            HolderConfig::ThesisHolderCircle => Box::new(ThesisHolderCircle::default()),
            HolderConfig::ThesisHolderSquare => Box::new(ThesisHolderSquare::default()),
            HolderConfig::ExampleHolderWithConfig { .. } => {
                todo!("Box::new(ExampleHolderWithConfig::new(a, b))")
            }
        }
    }
}
