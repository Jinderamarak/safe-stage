use crate::concrete_resolvers::ConcreteRetractResolver;
use crate::types::CLinearState;
use paths::resolver::retract::linear::RetractLinearResolver;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum ResolverRetractConfig {
    RetractLinearResolver { step_size: CLinearState },
}

impl ResolverRetractConfig {
    /// Create a new **RetractLinearResolver** configuration.
    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn retract_linear_resolver(step_size: CLinearState) -> Self {
        ResolverRetractConfig::RetractLinearResolver { step_size }
    }

    pub(crate) fn build(&self) -> ConcreteRetractResolver {
        match self {
            ResolverRetractConfig::RetractLinearResolver { step_size } => {
                ConcreteRetractResolver::new(RetractLinearResolver::new(step_size.into()))
            }
        }
    }
}
