use crate::configuration::chamber::ChamberConfig;
use crate::configuration::equipment::EquipmentConfig;
use crate::configuration::resolver_retract::ResolverRetractConfig;
use crate::configuration::resolver_stage::ResolverStageConfig;
use crate::configuration::retract::RetractConfig;
use crate::configuration::stage::StageConfig;
use crate::configuration::{ConfigBuilderResult, Configuration};
use crate::id::Id;

pub struct ConfigurationBuilder {
    chamber: Option<ChamberConfig>,
    stage: Option<StageConfig>,
    stage_resolver: Option<ResolverStageConfig>,
    equipment: Vec<EquipmentConfig>,
    retracts: Vec<(Id, (RetractConfig, ResolverRetractConfig))>,
}

impl Default for ConfigurationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigurationBuilder {
    /// Create a new configuration builder.
    pub fn new() -> Self {
        Self {
            chamber: None,
            stage: None,
            stage_resolver: None,
            equipment: Vec::new(),
            retracts: Vec::new(),
        }
    }

    /// Set the chamber configuration.
    pub fn with_chamber(mut self, chamber: ChamberConfig) -> Self {
        self.chamber = Some(chamber);
        self
    }

    /// Set the stage configuration.
    pub fn with_stage(mut self, stage: StageConfig, resolver: ResolverStageConfig) -> Self {
        self.stage = Some(stage);
        self.stage_resolver = Some(resolver);
        self
    }

    /// Add an equipment configuration.
    pub fn with_equipment(mut self, equipment: EquipmentConfig) -> Self {
        self.equipment.push(equipment);
        self
    }

    /// Add a retract configuration.
    pub fn with_retract(
        mut self,
        id: Id,
        retract: RetractConfig,
        resolver: ResolverRetractConfig,
    ) -> Self {
        self.retracts.push((id, (retract, resolver)));
        self
    }

    /// Build the configuration.
    pub fn build(self) -> Result<Configuration, ConfigBuilderResult> {
        let chamber = if let Some(chamber) = self.chamber {
            chamber
        } else {
            return Err(ConfigBuilderResult::MissingChamber);
        };

        let (stage, resolver) =
            if let (Some(stage), Some(resolver)) = (self.stage, self.stage_resolver) {
                (stage, resolver)
            } else {
                return Err(ConfigBuilderResult::MissingStage);
            };

        Ok(Configuration::new(
            chamber,
            stage,
            resolver,
            self.equipment,
            self.retracts,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{ChamberConfig, ConfigBuilderResult, ConfigurationBuilder, StageConfig};
    use crate::configuration::resolver_stage::ResolverStageConfig;
    use crate::types::CSixAxis;

    const STEP: CSixAxis = CSixAxis {
        x: 0.1,
        y: 0.2,
        z: 0.3,
        rx: 0.4,
        ry: 0.5,
        rz: 0.6,
    };

    #[test]
    fn build_success() {
        let _config = ConfigurationBuilder::default()
            .with_chamber(ChamberConfig::ThesisChamber)
            .with_stage(
                StageConfig::ThesisStage,
                ResolverStageConfig::StageLinearResolver { step_size: STEP },
            )
            .build()
            .unwrap();
    }

    #[test]
    fn build_missing_chamber() {
        let config = ConfigurationBuilder::default()
            .with_stage(
                StageConfig::ThesisStage,
                ResolverStageConfig::StageLinearResolver { step_size: STEP },
            )
            .build();
        assert!(matches!(config, Err(ConfigBuilderResult::MissingChamber)));
    }

    #[test]
    fn build_missing_stage() {
        let config = ConfigurationBuilder::default()
            .with_chamber(ChamberConfig::ThesisChamber)
            .build();
        assert!(matches!(config, Err(ConfigBuilderResult::MissingStage)));
    }
}
