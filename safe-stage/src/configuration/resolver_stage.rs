use crate::concrete_resolvers::ConcreteStageResolver;
use crate::types::{CSixAxis, CVector3};
use paths::resolver::stage::down_rotate_find::DownRotateFindResolver;
use paths::resolver::stage::linear::StageLinearResolver;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
#[allow(clippy::large_enum_variant)] //  This enum is created only during configuration, it is fine
pub enum ResolverStageConfig {
    StageLinearResolver {
        step_size: CSixAxis,
    },
    DownRotateFindResolver {
        down_point: CVector3,
        down_step: CSixAxis,
        move_speed: CVector3,
        sample_min: CVector3,
        sample_max: CVector3,
        sample_step: CVector3,
        sample_epsilon: CVector3,
        los_step: CVector3,
        smoothing_step: CSixAxis,
    },
    UnitVariant(CSixAxis),
    EmptyVariant,
}

impl ResolverStageConfig {
    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn stage_linear_resolver(step_size: CSixAxis) -> Self {
        ResolverStageConfig::StageLinearResolver { step_size }
    }

    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn down_rotate_find_resolver(
        down_point: CVector3,
        down_step: CSixAxis,
        move_speed: CVector3,
        sample_min: CVector3,
        sample_max: CVector3,
        sample_step: CVector3,
        sample_epsilon: CVector3,
        los_step: CVector3,
        smoothing_step: CSixAxis,
    ) -> Self {
        ResolverStageConfig::DownRotateFindResolver {
            down_point,
            down_step,
            move_speed,
            sample_min,
            sample_max,
            sample_step,
            sample_epsilon,
            los_step,
            smoothing_step,
        }
    }

    pub(crate) fn build(&self) -> ConcreteStageResolver {
        match self {
            ResolverStageConfig::StageLinearResolver { step_size } => {
                ConcreteStageResolver::new(StageLinearResolver::new(step_size.into()))
            }
            ResolverStageConfig::DownRotateFindResolver {
                down_point,
                down_step,
                move_speed,
                sample_min,
                sample_max,
                sample_step,
                sample_epsilon,
                los_step,
                smoothing_step,
            } => ConcreteStageResolver::new(DownRotateFindResolver::new(
                down_point.into(),
                down_step.into(),
                move_speed.into(),
                sample_min.into(),
                sample_max.into(),
                sample_step.into(),
                sample_epsilon.into(),
                los_step.into(),
                smoothing_step.into(),
            )),
            _ => unimplemented!(),
        }
    }
}
