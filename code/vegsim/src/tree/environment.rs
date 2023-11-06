use patutil::{Vecf3, Vecu3};

use crate::{util::BoundingVolume, treeparameter::SpaceDividingMode, parameters};

use super::{markerset::MarkerSet, shadowvoxelset::ShadowVoxelSet, PlantGenetics};

#[derive(Debug, Clone)]
pub struct Environment {
    tropism_growth_direction_weight: f32,
    markers: MarkerSet,
    shadowvoxels: ShadowVoxelSet,
    mode: SpaceDividingMode,
    bounding_volume: BoundingVolume
}

impl Environment {
    pub fn new(bounding_volume: BoundingVolume) -> Self {
        let resolution = parameters::SPACE_DIV_RESOLUTION;
        Self {
            tropism_growth_direction_weight: parameters::TROPISM_START_WEIGTH,
            markers: MarkerSet::new(bounding_volume, Vecu3::new(resolution, resolution, resolution)),
            shadowvoxels: ShadowVoxelSet::new(bounding_volume, Vecu3::new(resolution, resolution, resolution)),
            mode: parameters::SPACE_DIV_MODE,
            bounding_volume
        }
    }

    pub fn increase_tropism(&mut self) {
        self.tropism_growth_direction_weight *= parameters::TROPISM_CHANGE_RATE;
    }
    pub fn tropism_dir(&self) -> Vecf3{
        parameters::TROPISM_DIR
    }

    // return true if point is inside the environment
    pub fn is_inside(&self, point: Vecf3)-> bool{
        return self.bounding_volume.includes(point);
    }

    pub fn calc_light_gathered(
        &self,
        bud_pos: Vecf3,
        genetics: &PlantGenetics,
        bud_id: u32,
        _length: f32,
        dir: Vecf3,
    ) -> f32 {
        let theta = genetics.bud_perception_angle();
        let r = genetics.bud_perception_radius_factor();

        match self.mode {
            SpaceDividingMode::Markers => {
                let markers = self
                    .markers
                    .total_markers_for_id_in_cone(bud_id, bud_pos, dir, theta, r);

                if markers >= 1 {
                    return 1.;
                }
                return 0.;
            }
            SpaceDividingMode::ShadowVoxels => {
                return self.shadowvoxels.get_light_exposure(bud_pos);
            }
            SpaceDividingMode::None => return 0.,
        }
    }

    pub fn optimal_growth_direction(
        &self,
        bud_pos: Vecf3,
        genetics: &PlantGenetics,
        bud_id: u32,
        _length: f32,
        dir: Vecf3,
    ) -> Option<Vecf3> {
        let theta = genetics.bud_perception_angle();
        let r = genetics.bud_perception_radius_factor();

        match self.mode {
            SpaceDividingMode::Markers => {
                return self
                    .markers
                    .markers_dir_for_id_in_cone(bud_id, bud_pos, dir, theta, r);
            }
            SpaceDividingMode::ShadowVoxels => {
                return self
                    .shadowvoxels
                    .optimal_growth_direction(bud_pos, dir, theta, r);
            }
            SpaceDividingMode::None => return None,
        }
    }

    pub const fn tropism_growth_direction_weight(&self) -> f32 {
        return self.tropism_growth_direction_weight;
    }

    pub fn reset_space(&mut self) {
        self.markers.reset();
        self.shadowvoxels.clear();
    }

    pub fn markers_mut(&mut self) -> &mut MarkerSet {
        &mut self.markers
    }

    pub fn shadowvoxels_mut(&mut self) -> &mut ShadowVoxelSet {
        &mut self.shadowvoxels
    }

    pub fn shadowvoxels(&self) -> &ShadowVoxelSet {
        &self.shadowvoxels
    }

    pub fn set_mode(&mut self, mode: SpaceDividingMode) {
        self.mode = mode;
    }

    pub fn _bounding_volume(&self) -> BoundingVolume {
        self.bounding_volume
    }

    pub fn mode(&self) -> SpaceDividingMode {
        self.mode
    }
}
