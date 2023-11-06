
use crate::{treeparameter::GeneticParameter, parameters};

use super::metamer::Metamer;

#[derive(Debug)]
pub struct PlantGenetics {
    borchert_honda_lambda: f32,
    borchert_honda_alpha: f32,
    pole_length: f32,
    aux_shoot_requirement: f32,
    term_shoot_requirement: f32,
    metamer_base_length: f32,
    bud_perception_angle: f32,
    bud_perception_radius_factor: f32,
    occupancy_radius_factor: f32,
    axillary_perturbation_angle: f32,
    optimal_growth_direction_weight: f32,
    shed_treshhold: f32,
}

impl PlantGenetics {
    pub fn new() -> Self {
        Self {
            borchert_honda_lambda: parameters::BORCHERT_HONDA_LAMBDA,
            borchert_honda_alpha: parameters::BORCHERT_HONDA_ALPHA,

            pole_length: parameters::POLE_LENGTH,
            aux_shoot_requirement: parameters::AUX_SHOOT_REQUIREMENT,
            term_shoot_requirement: parameters::TERM_SHOOT_REQUIREMENT,
            metamer_base_length: parameters::METAMER_BASE_LENGTH,

            bud_perception_angle: parameters::BUD_PERCEPTION_ANGLE,
            bud_perception_radius_factor: parameters::BUD_PERCEPTION_RADIUS_FACTOR,
            occupancy_radius_factor: parameters::OCCUPANCY_RADIUS_FACTOR,
    
            axillary_perturbation_angle: parameters::AXILLARY_PERTURBATION_ANGLE,
            optimal_growth_direction_weight: parameters::OPTIMAL_GROWTH_DIRECTION_WEIGHT,
            shed_treshhold: parameters::SHED_TRESHHOLD
        }
    }

    pub const fn terminal_shoot_requirement(&self) -> f32{
        return self.term_shoot_requirement;
    }
    pub fn aux_shoot_requirement(&self, metamer: Option<&Metamer>) -> f32{

        // if terminal bud damaged, auxillary bud has terminal shoot requirements
        if let Some(metamer) = metamer{
            if metamer.terminal_bud_damage > 0.{
                return self.terminal_shoot_requirement();
            }
        }
        return  self.aux_shoot_requirement;
    }

    pub const fn metamer_base_length(&self) -> f32 {
        self.metamer_base_length
    }

    pub fn bud_perception_angle(&self) -> f32 {
        self.bud_perception_angle
    }
    pub const fn bud_perception_radius_factor(&self) -> f32 {
        self.bud_perception_radius_factor
    }
    pub const fn occupancy_radius_factor(&self) -> f32 {
        self.occupancy_radius_factor
    }

    pub fn axillary_perturbation_angle(&self) -> f32 {
        self.axillary_perturbation_angle
    }

    pub const fn borchert_honda_alpha(&self) -> f32 {
        self.borchert_honda_alpha
    }
    pub const fn borchert_honda_lambda(&self) -> f32 {
        self.borchert_honda_lambda
    }

    pub const fn optimal_growth_direction_weight(&self) -> f32 {
        self.optimal_growth_direction_weight
    }

    pub fn shed_treshhold(&self) -> f32 {
        self.shed_treshhold
    }

    pub fn base_pole_lenght(&self) -> f32 {
        return self.pole_length;
    }

    pub fn update_param(&mut self, param: GeneticParameter) {
        match param {
            GeneticParameter::BorchertHondaLambda(value) => self.borchert_honda_lambda = value,
            GeneticParameter::BorchertHondaAlpha(value) => self.borchert_honda_alpha = value,
            GeneticParameter::PoleLength(value) => self.pole_length = value,
            GeneticParameter::AuxShootReq(value) => self.aux_shoot_requirement = value,
        }
    }

    pub fn get_param(&mut self, param: GeneticParameter) -> GeneticParameter {
        match param {
            GeneticParameter::BorchertHondaLambda(_) => GeneticParameter::BorchertHondaLambda(self.borchert_honda_lambda),
            GeneticParameter::BorchertHondaAlpha(_) => GeneticParameter::BorchertHondaAlpha(self.borchert_honda_alpha),
            GeneticParameter::PoleLength(_) => GeneticParameter::PoleLength(self.pole_length),
            GeneticParameter::AuxShootReq(_) => GeneticParameter::AuxShootReq(self.aux_shoot_requirement),
        }
    }
    
}

unsafe impl Send for PlantGenetics {}
