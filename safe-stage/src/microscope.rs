use crate::concrete_parts::{ConcreteChamber, ConcreteEquipment, ConcreteRetract, ConcreteStage};
use crate::concrete_resolvers::{ConcreteRetractResolver, ConcreteStageResolver};
use crate::configuration::holder::HolderConfig;
use crate::configuration::Configuration;
use crate::ffi::opaque_ffi_for_type;
use crate::id::Id;
use crate::presentation::{collider_to_triangle_buffer_per_item, TriangleBufferVec};
use crate::types::{CLinearState, CPathResultLinearState, CPathResultSixAxis, CSixAxis};
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;
use maths::Vector2;
use models::immovable::Immovable;
use models::movable::Movable;
use models::position::linear::LinearState;
use models::position::sixaxis::SixAxis;
use models::sample::height_map::height_map_to_sample_model;
use paths::resolver::StateUpdateError as ResolverUpdateError;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

opaque_ffi_for_type!(BoxSliceEquipment, Box<[ConcreteEquipment]>);
opaque_ffi_for_type!(
    HashMapRetracts,
    HashMap<Id, (ConcreteRetract, ConcreteRetractResolver, CLinearState)>
);

#[cfg_attr(feature = "ffi", repr(u8))]
#[derive(Error, Debug)]
pub enum StateUpdateError {
    #[cfg(feature = "ffi")]
    #[error("Ok")]
    Ok = 0,
    #[error("Invalid state")]
    InvalidState = 1,
    #[error("Invalid id")]
    InvalidId = 2,
}

impl From<ResolverUpdateError> for StateUpdateError {
    fn from(value: ResolverUpdateError) -> Self {
        match value {
            ResolverUpdateError::InvalidState => StateUpdateError::InvalidState,
        }
    }
}

#[cfg(feature = "ffi")]
fn result_to_error_enum(result: Result<(), StateUpdateError>) -> StateUpdateError {
    match result {
        Ok(_) => StateUpdateError::Ok,
        Err(e) => e,
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
pub struct Microscope {
    chamber: ConcreteChamber,
    stage: ConcreteStage,
    stage_resolver: ConcreteStageResolver,
    stage_state: CSixAxis,
    equipment: BoxSliceEquipment,
    retracts: HashMapRetracts,
}

#[cfg(feature = "ffi")]
impl Microscope {
    /// Build microscope from configuration.
    #[no_mangle]
    pub extern "C" fn microscope_from_config(config: &Configuration) -> Self {
        Self::build(config)
    }

    /// Clear the sample from the active holder.
    ///
    /// # Safety
    /// Expects exclusive mutable reference to the microscope.
    #[no_mangle]
    pub extern "C" fn microscope_clear_sample(&mut self) {
        self.safe_clear_sample()
    }

    /// Update the active stage holder.
    ///
    /// # Safety
    /// Expects exclusive mutable reference to the microscope.
    #[no_mangle]
    pub extern "C" fn microscope_update_holder(&mut self, holder: &HolderConfig) {
        self.safe_update_holder(holder)
    }

    /// Remove the active stage holder.
    ///
    /// # Safety
    /// Expects exclusive mutable reference to the microscope.
    #[no_mangle]
    pub extern "C" fn microscope_remove_holder(&mut self) {
        self.safe_remove_holder()
    }

    /// Update the sample height map.
    ///
    /// # Safety
    /// - Expect exclusive mutable reference to the microscope.
    /// - The `height_map` must be a pointer to an array of `f64` values with a length of `size_x * size_y`.
    #[no_mangle]
    pub unsafe extern "C" fn microscope_update_sample_height_map(
        &mut self,
        height_map: *const f64,
        size_x: usize,
        size_y: usize,
        real_x: f64,
        real_y: f64,
    ) {
        let height_map = std::ptr::slice_from_raw_parts(height_map, size_x * size_y);
        self.safe_update_sample_height_map(&*height_map, size_x, size_y, real_x, real_y)
    }

    /// Update the stage state.
    ///
    /// # Safety
    /// Expects exclusive mutable reference to the microscope.
    #[no_mangle]
    pub extern "C" fn microscope_update_stage_state(&mut self, state: &CSixAxis) {
        self.safe_update_stage_state(state)
    }

    /// Update the retract state.
    ///
    /// # Safety
    /// Expects exclusive mutable reference to the microscope.
    #[no_mangle]
    pub extern "C" fn microscope_update_retract_state(
        &mut self,
        id: Id,
        state: &CLinearState,
    ) -> StateUpdateError {
        result_to_error_enum(self.safe_update_retract_state(id, state))
    }

    /// Update the resolvers for stage and all retracts.
    ///
    /// # Safety
    /// Expects exclusive mutable reference to the microscope.
    #[no_mangle]
    pub extern "C" fn microscope_update_resolvers(&mut self) -> StateUpdateError {
        result_to_error_enum(self.safe_update_resolvers())
    }

    /// Find a path for the stage from the latest state to the given one.
    #[no_mangle]
    pub extern "C" fn microscope_find_stage_path(&self, state: &CSixAxis) -> CPathResultSixAxis {
        self.safe_find_stage_path(state)
    }

    /// Find a path for the retract from the latest state to the given one.
    #[no_mangle]
    pub extern "C" fn microscope_find_retract_path(
        &self,
        id: Id,
        state: &CLinearState,
    ) -> CPathResultLinearState {
        self.safe_find_retract_path(id, state)
    }

    /// Present the full view of static parts.
    #[no_mangle]
    pub extern "C" fn microscope_present_static_full(&self) -> TriangleBufferVec {
        self.safe_present_static_full()
    }

    /// Present the less obstructive view of static parts.
    #[no_mangle]
    pub extern "C" fn microscope_present_static_less_obstructive(&self) -> TriangleBufferVec {
        self.safe_present_static_less_obstructive()
    }

    /// Present the non-obstructive view of static parts.
    #[no_mangle]
    pub extern "C" fn microscope_present_static_non_obstructive(&self) -> TriangleBufferVec {
        self.safe_present_static_non_obstructive()
    }

    /// Present the stage at the current state.
    #[no_mangle]
    pub extern "C" fn microscope_present_stage(&self) -> TriangleBufferVec {
        self.safe_present_stage()
    }

    /// Present the stage at the given state.
    #[no_mangle]
    pub extern "C" fn microscope_present_stage_at(&self, state: &CSixAxis) -> TriangleBufferVec {
        self.safe_present_stage_at(state)
    }

    /// Present the retract at the current state.
    #[no_mangle]
    pub extern "C" fn microscope_present_retract(&self, id: Id) -> TriangleBufferVec {
        self.safe_present_retract(id)
    }

    /// Present the retract at the given state.
    #[no_mangle]
    pub extern "C" fn microscope_present_retract_at(
        &self,
        id: Id,
        state: &CLinearState,
    ) -> TriangleBufferVec {
        self.safe_present_retract_at(id, state)
    }

    /// # Safety
    /// Takes ownership of the microscope and drops it.
    #[no_mangle]
    pub extern "C" fn microscope_drop(self) {
        //  dropped after leaving scope
    }
}

#[cfg(not(feature = "ffi"))]
impl Microscope {
    /// Build microscope from configuration.
    pub fn from_config(config: &Configuration) -> Self {
        Self::build(config)
    }

    /// Clear the sample from the active holder.
    pub fn clear_sample(&mut self) {
        self.safe_clear_sample();
    }

    /// Update the active stage holder.
    pub fn update_holder(&mut self, holder: &HolderConfig) {
        self.safe_update_holder(holder);
    }

    /// Remove the active stage holder.
    pub fn remove_holder(&mut self) {
        self.safe_remove_holder();
    }

    /// Update the sample height map.
    pub fn update_sample_height_map(
        &mut self,
        height_map: &[f64],
        size_x: usize,
        size_y: usize,
        real_x: f64,
        real_y: f64,
    ) {
        self.safe_update_sample_height_map(height_map, size_x, size_y, real_x, real_y);
    }

    /// Update the stage state.
    pub fn update_stage_state(&mut self, state: &CSixAxis) {
        self.safe_update_stage_state(state);
    }

    /// Update the retract state.
    pub fn update_retract_state(
        &mut self,
        id: Id,
        state: &CLinearState,
    ) -> Result<(), StateUpdateError> {
        self.safe_update_retract_state(id, state)
    }

    /// Update the resolvers for stage and all retracts.
    pub fn update_resolvers(&mut self) -> Result<(), StateUpdateError> {
        self.safe_update_resolvers()
    }

    /// Find a path for the stage from the latest state to the given one.
    pub fn find_stage_path(&self, state: &CSixAxis) -> CPathResultSixAxis {
        self.safe_find_stage_path(state)
    }

    /// Find a path for the retract from latest state to the given one.
    pub fn find_retract_path(&self, id: Id, state: &CLinearState) -> CPathResultLinearState {
        self.safe_find_retract_path(id, state)
    }

    /// Present the full view of static parts.
    pub fn present_static_full(&self) -> TriangleBufferVec {
        self.safe_present_static_full()
    }

    /// Present the less obstructive view of static parts.
    pub fn present_static_less_obstructive(&self) -> TriangleBufferVec {
        self.safe_present_static_less_obstructive()
    }

    /// Present the non-obstructive view of static parts.
    pub fn present_static_non_obstructive(&self) -> TriangleBufferVec {
        self.safe_present_static_non_obstructive()
    }

    /// Present the stage at the current state.
    pub fn present_stage(&self) -> TriangleBufferVec {
        self.safe_present_stage()
    }

    /// Present the stage at the given state.
    pub fn present_stage_at(&self, state: &CSixAxis) -> TriangleBufferVec {
        self.safe_present_stage_at(state)
    }

    /// Present the retract at the current state.
    pub fn present_retract(&self, id: Id) -> TriangleBufferVec {
        self.safe_present_retract(id)
    }

    /// Present the retract at the given state.
    pub fn present_retract_at(&self, id: Id, state: &CLinearState) -> TriangleBufferVec {
        self.safe_present_retract_at(id, state)
    }
}

impl Microscope {
    fn build(config: &Configuration) -> Self {
        let chamber = config.chamber().build();
        let stage = config.stage().build();
        let stage_resolver = config.stage_resolver().build();
        let stage_state = CSixAxis {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rx: 0.0,
            ry: 0.0,
            rz: 0.0,
        };
        let equipment = config
            .equipment()
            .iter()
            .map(|e| e.build())
            .collect::<Vec<ConcreteEquipment>>();
        let retracts = config
            .retracts()
            .iter()
            .map(|(id, (r, rr))| (*id, (r.build(), rr.build(), CLinearState { t: 0.0 })))
            .collect::<HashMap<Id, (ConcreteRetract, ConcreteRetractResolver, CLinearState)>>();
        Self {
            chamber,
            stage,
            stage_resolver,
            stage_state,
            equipment: BoxSliceEquipment::from_inner(equipment.into_boxed_slice()),
            retracts: HashMapRetracts::from_inner(retracts),
        }
    }

    fn is_valid_retract(&self, id: Id) -> bool {
        self.retracts.inner().get(&id).is_some()
    }

    fn safe_clear_sample(&mut self) {
        if let Some(h) = self.stage.get_mut().active_holder_mut() {
            h.swap_sample(None)
        }
    }

    fn safe_update_holder(&mut self, holder: &HolderConfig) {
        let holder = holder.build();
        self.stage.get_mut().swap_holder(Some(holder));
    }

    fn safe_remove_holder(&mut self) {
        self.stage.get_mut().swap_holder(None);
    }

    fn safe_update_sample_height_map(
        &mut self,
        height_map: &[f64],
        size_x: usize,
        size_y: usize,
        real_x: f64,
        real_y: f64,
    ) {
        let real_size = Vector2::new(real_x, real_y);
        let model = height_map_to_sample_model(height_map, size_x, size_y, &real_size, 0.0);
        let sample = if model.is_empty() {
            None
        } else {
            Some(PrimaryCollider::build(&model))
        };
        if let Some(h) = self.stage.get_mut().active_holder_mut() {
            h.swap_sample(sample)
        }
    }

    fn movable_stage(&self) -> Arc<dyn Movable<SixAxis>> {
        self.stage.get_ref().as_movable()
    }

    fn movable_retract(&self, id: Id) -> Option<Arc<dyn Movable<LinearState>>> {
        self.retracts
            .inner()
            .get(&id)
            .map(|(r, _, _)| r.get_ref().as_movable())
    }

    fn add_equipment(
        &self,
        mut group: ColliderGroup<PrimaryCollider>,
    ) -> ColliderGroup<PrimaryCollider> {
        for equipment in self.equipment.inner() {
            group.extend(equipment.get_ref().collider())
        }
        group
    }

    fn always_immovable(&self) -> ColliderGroup<PrimaryCollider> {
        let immovable = self.chamber.get_ref().full();
        self.add_equipment(immovable)
    }

    fn immovable_without_stage(&self) -> Immovable {
        let mut immovable = self.always_immovable();
        for (r, _, s) in self.retracts.inner().values() {
            immovable.extend(r.get_ref().move_to(&s.into()));
        }
        immovable
    }

    /// Stage is considered the only relevant part for retracts
    fn immovable_stage(&self) -> Immovable {
        let immovable = self
            .stage
            .get_ref()
            .move_to(&SixAxis::from(&self.stage_state));
        immovable
    }

    fn update_stage_resolver_state(&mut self, state: &CSixAxis) -> Result<(), StateUpdateError> {
        let movable = self.movable_stage();
        let immovable = self.immovable_without_stage();

        self.stage_resolver.get_mut().update_state(
            &SixAxis::from(state),
            movable.as_ref(),
            &immovable,
        )?;
        Ok(())
    }

    fn update_retract_resolver_state(
        &mut self,
        id: Id,
        state: &CLinearState,
    ) -> Result<(), StateUpdateError> {
        let movable = self.movable_retract(id).unwrap();
        let immovable = self.immovable_stage();

        self.retracts
            .inner_mut()
            .get_mut(&id)
            .unwrap()
            .1
            .get_mut()
            .update_state(&LinearState::from(state), movable.as_ref(), &immovable)?;
        Ok(())
    }

    fn safe_update_stage_state(&mut self, state: &CSixAxis) {
        self.stage_state = *state;
    }

    fn safe_update_retract_state(
        &mut self,
        id: Id,
        state: &CLinearState,
    ) -> Result<(), StateUpdateError> {
        if !self.is_valid_retract(id) {
            return Err(StateUpdateError::InvalidId);
        }

        self.retracts.inner_mut().get_mut(&id).unwrap().2 = *state;
        Ok(())
    }

    fn safe_update_resolvers(&mut self) -> Result<(), StateUpdateError> {
        self.update_stage_resolver_state(&self.stage_state.clone())?;

        let retracts_id_state = self
            .retracts
            .inner()
            .iter()
            .map(|(id, (_, _, s))| (*id, *s))
            .collect::<Vec<_>>();
        for (id, state) in retracts_id_state {
            self.update_retract_resolver_state(id, &state)?;
        }

        Ok(())
    }

    fn safe_find_stage_path(&self, state: &CSixAxis) -> CPathResultSixAxis {
        let movable = self.movable_stage();
        let immovable = self.immovable_without_stage();
        let from = self.stage_state;
        let result = self.stage_resolver.get_ref().resolve_path(
            &SixAxis::from(&from),
            &SixAxis::from(state),
            movable.as_ref(),
            &immovable,
        );
        CPathResultSixAxis::from(result)
    }

    fn safe_find_retract_path(&self, id: Id, state: &CLinearState) -> CPathResultLinearState {
        let movable = self.movable_retract(id).unwrap();
        let immovable = self.immovable_stage();
        let from = self.retracts.inner()[&id].2;
        let result = self.retracts.inner()[&id].1.get_ref().resolve_path(
            &LinearState::from(&from),
            &LinearState::from(state),
            movable.as_ref(),
            &immovable,
        );
        CPathResultLinearState::from(result)
    }

    fn safe_present_static_full(&self) -> TriangleBufferVec {
        let chamber = self.chamber.get_ref().full();
        collider_to_triangle_buffer_per_item(self.add_equipment(chamber))
    }

    fn safe_present_static_less_obstructive(&self) -> TriangleBufferVec {
        let chamber = self.chamber.get_ref().less_obstructive();
        collider_to_triangle_buffer_per_item(self.add_equipment(chamber))
    }

    fn safe_present_static_non_obstructive(&self) -> TriangleBufferVec {
        let chamber = self.chamber.get_ref().non_obstructive();
        collider_to_triangle_buffer_per_item(self.add_equipment(chamber))
    }

    fn safe_present_stage(&self) -> TriangleBufferVec {
        let stage = self
            .stage
            .get_ref()
            .move_to(&SixAxis::from(&self.stage_state));
        collider_to_triangle_buffer_per_item(stage)
    }

    fn safe_present_stage_at(&self, state: &CSixAxis) -> TriangleBufferVec {
        let stage = self.stage.get_ref().move_to(&SixAxis::from(state));
        collider_to_triangle_buffer_per_item(stage)
    }

    fn safe_present_retract(&self, id: Id) -> TriangleBufferVec {
        let (retract, _, state) = &self.retracts.inner()[&id];
        let retracted = retract.get_ref().move_to(&LinearState::from(state));
        collider_to_triangle_buffer_per_item(retracted)
    }

    fn safe_present_retract_at(&self, id: Id, state: &CLinearState) -> TriangleBufferVec {
        let (retract, _, _) = &self.retracts.inner()[&id];
        let retracted = retract.get_ref().move_to(&LinearState::from(state));
        collider_to_triangle_buffer_per_item(retracted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::chamber::ChamberConfig;
    use crate::configuration::equipment::EquipmentConfig;
    use crate::configuration::resolver_retract::ResolverRetractConfig;
    use crate::configuration::resolver_stage::ResolverStageConfig;
    use crate::configuration::retract::RetractConfig;
    use crate::configuration::stage::StageConfig;
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
    fn build_microscope_without_leak() {
        let config = Configuration::new(
            ChamberConfig::ThesisChamber,
            StageConfig::ThesisStage,
            ResolverStageConfig::StageLinearResolver { step_size: STEP },
            vec![EquipmentConfig::ThesisDetectorAlpha],
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

        #[cfg(feature = "ffi")]
        let _microscope = Microscope::microscope_from_config(&config);
        #[cfg(not(feature = "ffi"))]
        let _microscope = Microscope::from_config(&config);
    }
}
