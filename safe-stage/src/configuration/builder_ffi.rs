use crate::configuration::chamber::ChamberConfig;
use crate::configuration::equipment::EquipmentConfig;
use crate::configuration::resolver_retract::ResolverRetractConfig;
use crate::configuration::resolver_stage::ResolverStageConfig;
use crate::configuration::retract::RetractConfig;
use crate::configuration::stage::StageConfig;
use crate::configuration::{ConfigBuilderResult, Configuration};
use crate::id::Id;
use std::ffi::c_void;

type ChamberType = ChamberConfig;
type StageType = StageConfig;
type StageResolverType = ResolverStageConfig;
type EquipmentsType = Vec<EquipmentConfig>;
type RetractsType = Vec<(Id, (RetractConfig, ResolverRetractConfig))>;

#[repr(C)]
pub struct ConfigurationBuilder {
    chamber: *const ChamberType,
    stage: *const StageType,
    stage_resolver: *const StageResolverType,
    equipment: *mut c_void,
    retracts: *mut c_void,
}

impl Default for ConfigurationBuilder {
    fn default() -> Self {
        Self::builder_new()
    }
}

impl ConfigurationBuilder {
    unsafe fn chamber(&self) -> Option<&ChamberType> {
        if self.chamber.is_null() {
            None
        } else {
            Some(&*self.chamber)
        }
    }

    unsafe fn stage(&self) -> Option<&StageType> {
        if self.stage.is_null() {
            None
        } else {
            Some(&*self.stage)
        }
    }

    unsafe fn stage_resolver(&self) -> Option<&StageResolverType> {
        if self.stage_resolver.is_null() {
            None
        } else {
            Some(&*self.stage_resolver)
        }
    }

    unsafe fn equipment(&mut self) -> &mut EquipmentsType {
        let ptr = self.equipment as *mut EquipmentsType;
        &mut *ptr
    }

    unsafe fn retracts(&mut self) -> &mut RetractsType {
        let ptr = self.retracts as *mut RetractsType;
        &mut *ptr
    }

    /// Create a new instance of configuration builder.
    ///
    /// # Safety
    /// The caller must ensure that the returned builder is dropped after use.
    #[no_mangle]
    pub extern "C" fn builder_new() -> Self {
        let equipment: Box<EquipmentsType> = Box::default();
        let retracts: Box<RetractsType> = Box::default();
        Self {
            chamber: std::ptr::null(),
            stage: std::ptr::null(),
            stage_resolver: std::ptr::null(),
            equipment: Box::into_raw(equipment) as *mut c_void,
            retracts: Box::into_raw(retracts) as *mut c_void,
        }
    }

    /// Set the chamber configuration.
    ///
    /// # Safety
    /// Takes ownership of the chamber configuration.
    /// The returned builder must be dropped after use.
    #[no_mangle]
    pub unsafe extern "C" fn builder_with_chamber(mut self, chamber: ChamberConfig) -> Self {
        if !self.chamber.is_null() {
            let _c = Box::from_raw(self.chamber as *mut ChamberType);
        }
        self.chamber = Box::into_raw(Box::new(chamber));
        self
    }

    /// Set the stage configuration.
    ///
    /// # Safety
    /// Takes ownership of the chamber configuration.
    /// The returned builder must be dropped after use.
    #[no_mangle]
    pub unsafe extern "C" fn builder_with_stage(
        mut self,
        stage: StageConfig,
        resolver: ResolverStageConfig,
    ) -> Self {
        if !self.stage.is_null() {
            let _s = Box::from_raw(self.stage as *mut StageType);
        }
        if !self.stage_resolver.is_null() {
            let _r = Box::from_raw(self.stage_resolver as *mut StageResolverType);
        }
        self.stage = Box::into_raw(Box::new(stage));
        self.stage_resolver = Box::into_raw(Box::new(resolver));
        self
    }

    /// Add an equipment configuration.
    ///
    /// # Safety
    /// Takes ownership of the chamber configuration.
    /// The returned builder must be dropped after use.
    #[no_mangle]
    pub unsafe extern "C" fn builder_with_equipment(mut self, equipment: EquipmentConfig) -> Self {
        self.equipment().push(equipment);
        self
    }

    /// Add a retract configuration.
    ///
    /// # Safety
    /// Takes ownership of the chamber configuration.
    /// The returned builder must be dropped after use.
    #[no_mangle]
    pub unsafe extern "C" fn builder_with_retract(
        mut self,
        id: Id,
        retract: RetractConfig,
        resolver: ResolverRetractConfig,
    ) -> Self {
        self.retracts().push((id, (retract, resolver)));
        self
    }

    /// Build the configuration.
    ///
    /// # Safety
    /// - Expects the builder was modified only through the builder methods.
    /// - The contents of pointer `config` are overwritten without dropping its original value.
    /// - Takes ownership of the builder and drops it.
    #[no_mangle]
    pub unsafe extern "C" fn builder_build(
        mut self,
        config: *mut Configuration,
    ) -> ConfigBuilderResult {
        let chamber = if let Some(chamber) = self.chamber() {
            chamber.clone()
        } else {
            return ConfigBuilderResult::MissingChamber;
        };

        let (stage, resolver) =
            if let (Some(stage), Some(resolver)) = (self.stage(), self.stage_resolver()) {
                (stage.clone(), resolver.clone())
            } else {
                return ConfigBuilderResult::MissingStage;
            };

        config.write(Configuration::new(
            chamber,
            stage,
            resolver,
            self.equipment().drain(..).collect(),
            self.retracts().drain(..).collect(),
        ));
        ConfigBuilderResult::Success
    }

    /// # Safety
    /// Takes ownership of the configuration and drops it.
    #[no_mangle]
    pub extern "C" fn builder_drop(self) {
        //  dropped after leaving scope
    }
}

impl Drop for ConfigurationBuilder {
    fn drop(&mut self) {
        unsafe {
            if !self.chamber.is_null() {
                let _c = Box::from_raw(self.chamber as *mut ChamberType);
            }
            if !self.stage.is_null() {
                let _s = Box::from_raw(self.stage as *mut StageType);
            }
            if !self.stage_resolver.is_null() {
                let _r = Box::from_raw(self.stage_resolver as *mut StageResolverType);
            }
            let _e = Box::from_raw(self.equipment as *mut EquipmentsType);
            let _r = Box::from_raw(self.retracts as *mut RetractsType);
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id::Id;
    use crate::types::{CLinearState, CSixAxis};
    use std::mem::MaybeUninit;

    const STEP: CSixAxis = CSixAxis {
        x: 0.1,
        y: 0.2,
        z: 0.3,
        rx: 0.4,
        ry: 0.5,
        rz: 0.6,
    };

    #[test]
    fn empty_builder() {
        let _builder = ConfigurationBuilder::default();
    }

    #[test]
    fn build_success() {
        unsafe {
            let mut config = MaybeUninit::<Configuration>::uninit();
            let result = ConfigurationBuilder::default()
                .builder_with_chamber(ChamberConfig::ThesisChamber)
                .builder_with_stage(
                    StageConfig::ThesisStage,
                    ResolverStageConfig::StageLinearResolver { step_size: STEP },
                )
                .builder_build(&mut *config.as_mut_ptr());

            let _config = config.assume_init();
            assert!(matches!(result, ConfigBuilderResult::Success));
        }
    }

    #[test]
    fn build_missing_chamber() {
        unsafe {
            let mut config = MaybeUninit::<Configuration>::uninit();
            let result = ConfigurationBuilder::default()
                .builder_with_stage(
                    StageConfig::ThesisStage,
                    ResolverStageConfig::StageLinearResolver { step_size: STEP },
                )
                .builder_build(&mut *config.as_mut_ptr());

            assert!(matches!(result, ConfigBuilderResult::MissingChamber));
        }
    }

    #[test]
    fn build_missing_stage() {
        unsafe {
            let mut config = MaybeUninit::<Configuration>::uninit();
            let result = ConfigurationBuilder::default()
                .builder_with_chamber(ChamberConfig::ThesisChamber)
                .builder_build(&mut *config.as_mut_ptr());

            assert!(matches!(result, ConfigBuilderResult::MissingStage));
        }
    }

    #[test]
    fn dropped_before_build_without_leak() {
        unsafe {
            let builder = ConfigurationBuilder::default()
                .builder_with_chamber(ChamberConfig::ThesisChamber)
                .builder_with_stage(
                    StageConfig::ThesisStage,
                    ResolverStageConfig::StageLinearResolver { step_size: STEP },
                )
                .builder_with_equipment(EquipmentConfig::ThesisDetectorAlpha)
                .builder_with_equipment(EquipmentConfig::ThesisDetectorBeta)
                .builder_with_retract(
                    Id::id_new(10),
                    RetractConfig::ThesisRetract,
                    ResolverRetractConfig::RetractLinearResolver {
                        step_size: CLinearState { t: 0.1 },
                    },
                );

            builder.builder_drop();
        }
    }

    #[test]
    fn overwrite_chamber_without_leak() {
        unsafe {
            let _builder = ConfigurationBuilder::default()
                .builder_with_chamber(ChamberConfig::ThesisChamber)
                .builder_with_chamber(ChamberConfig::ThesisChamber);
        }
    }

    #[test]
    fn overwrite_stage_without_leak() {
        unsafe {
            let _builder = ConfigurationBuilder::default()
                .builder_with_stage(
                    StageConfig::ThesisStage,
                    ResolverStageConfig::StageLinearResolver { step_size: STEP },
                )
                .builder_with_stage(
                    StageConfig::ThesisStage,
                    ResolverStageConfig::StageLinearResolver { step_size: STEP },
                );
        }
    }

    #[test]
    fn correct_collections() {
        unsafe {
            let mut builder = ConfigurationBuilder::default()
                .builder_with_chamber(ChamberConfig::ThesisChamber)
                .builder_with_stage(
                    StageConfig::ThesisStage,
                    ResolverStageConfig::StageLinearResolver { step_size: STEP },
                )
                .builder_with_equipment(EquipmentConfig::ThesisDetectorAlpha)
                .builder_with_equipment(EquipmentConfig::ThesisDetectorBeta)
                .builder_with_retract(
                    Id::id_new(10),
                    RetractConfig::ThesisRetract,
                    ResolverRetractConfig::RetractLinearResolver {
                        step_size: CLinearState { t: 0.1 },
                    },
                );

            let expected = [
                EquipmentConfig::ThesisDetectorAlpha,
                EquipmentConfig::ThesisDetectorBeta,
            ];
            let actual = builder.equipment();
            assert!(actual.iter().eq(expected.iter()));

            let expected = [(
                Id::id_new(10),
                (
                    RetractConfig::ThesisRetract,
                    ResolverRetractConfig::RetractLinearResolver {
                        step_size: CLinearState { t: 0.1 },
                    },
                ),
            )];
            let actual = builder.retracts();
            assert!(actual.iter().eq(expected.iter()));
        }
    }
}
