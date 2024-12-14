use crate::configuration::chamber::ChamberConfig;
use crate::configuration::equipment::EquipmentConfig;
use crate::configuration::resolver_retract::ResolverRetractConfig;
use crate::configuration::resolver_stage::ResolverStageConfig;
use crate::configuration::retract::RetractConfig;
use crate::configuration::stage::StageConfig;
use crate::ffi::opaque_ffi_for_type;
use crate::id::Id;
use thiserror::Error;

#[cfg(not(feature = "ffi"))]
pub mod builder;
#[cfg(feature = "ffi")]
pub mod builder_ffi; //  Keep module public for bindings generation

pub mod chamber;
pub mod equipment;
pub mod holder;
pub mod resolver_retract;
pub mod resolver_stage;
pub mod retract;
pub mod stage;

opaque_ffi_for_type!(CBoxSliceEqupment, Box<[EquipmentConfig]>);
opaque_ffi_for_type!(
    CBoxSliceRetracts,
    Box<[(Id, (RetractConfig, ResolverRetractConfig))]>
);

#[cfg_attr(feature = "ffi", repr(C))]
pub struct Configuration {
    chamber: ChamberConfig,
    stage: StageConfig,
    stage_resolver: ResolverStageConfig,
    equipment: CBoxSliceEqupment,
    retracts: CBoxSliceRetracts,
}

impl Configuration {
    pub fn new(
        chamber: ChamberConfig,
        stage: StageConfig,
        stage_resolver: ResolverStageConfig,
        equipment: Vec<EquipmentConfig>,
        retracts: Vec<(Id, (RetractConfig, ResolverRetractConfig))>,
    ) -> Self {
        let equipment = equipment.into_boxed_slice();
        let retracts = retracts.into_boxed_slice();
        Self {
            chamber,
            stage,
            stage_resolver,
            equipment: CBoxSliceEqupment::from_inner(equipment),
            retracts: CBoxSliceRetracts::from_inner(retracts),
        }
    }

    pub fn chamber(&self) -> &ChamberConfig {
        &self.chamber
    }

    pub fn stage(&self) -> &StageConfig {
        &self.stage
    }

    pub fn stage_resolver(&self) -> &ResolverStageConfig {
        &self.stage_resolver
    }

    pub fn equipment(&self) -> &[EquipmentConfig] {
        self.equipment.inner()
    }

    pub fn retracts(&self) -> &[(Id, (RetractConfig, ResolverRetractConfig))] {
        self.retracts.inner()
    }

    /// # Safety
    /// Takes ownership of the configuration and drops it.
    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn configuration_drop(self) {
        //  dropped after leaving scope
    }
}

#[derive(Debug, Error)]
#[cfg_attr(feature = "ffi", repr(u8))]
pub enum ConfigBuilderResult {
    #[cfg(feature = "ffi")]
    #[error("Success")]
    Success = 0,
    #[error("Missing configuration for chamber")]
    MissingChamber = 1,
    #[error("Missing configuration for stage")]
    MissingStage = 2,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::id::make_id;
    use crate::types::{CLinearState, CSixAxis};

    const STEP: CSixAxis = CSixAxis {
        x: 0.1,
        y: 0.2,
        z: 0.3,
        rx: 0.4,
        ry: 0.5,
        rz: 0.6,
    };

    #[test]
    fn correct_collections() {
        let config = Configuration::new(
            ChamberConfig::ThesisChamber,
            StageConfig::ThesisStage,
            ResolverStageConfig::StageLinearResolver { step_size: STEP },
            vec![
                EquipmentConfig::ThesisDetectorAlpha,
                EquipmentConfig::ThesisDetectorBeta,
            ],
            vec![(
                make_id!(11),
                (
                    RetractConfig::ThesisRetract,
                    ResolverRetractConfig::RetractLinearResolver {
                        step_size: CLinearState { t: 0.1 },
                    },
                ),
            )],
        );

        assert_eq!(config.chamber(), &ChamberConfig::ThesisChamber);
        assert_eq!(config.stage(), &StageConfig::ThesisStage);
        assert_eq!(
            config.stage_resolver(),
            &ResolverStageConfig::StageLinearResolver { step_size: STEP }
        );
        assert_eq!(
            config.equipment(),
            &[
                EquipmentConfig::ThesisDetectorAlpha,
                EquipmentConfig::ThesisDetectorBeta
            ]
        );
        assert_eq!(
            config.retracts(),
            &[(
                make_id!(11),
                (
                    RetractConfig::ThesisRetract,
                    ResolverRetractConfig::RetractLinearResolver {
                        step_size: CLinearState { t: 0.1 }
                    }
                )
            )]
        );
    }
}
