use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

use patutil::{Color, Vecf3};

use crate::{util::{BoundingVolume, random::Random}, parameters};

use super::{
    branchdata::BranchData, environment::Environment, markerset::MarkerSet,
    resourcedistibutor::ResourceDistibutor, shadowvoxelset::ShadowVoxelSet,
    support_pole::SupportPole, PlantGenetics,
};

static ID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(2);
fn get_id() -> u32 {
    ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

#[derive(Debug, Clone)]
pub struct Metamer {
    branch_data: BranchData,
    pub genetics: Arc<Mutex<PlantGenetics>>,
    pub last_light_generated: f32,
    pub support_pole: Option<SupportPole>,
    pub aux_support_pole: Option<SupportPole>,

    // terminal variables
    terminal_metamer: Option<Box<Metamer>>,
    pub last_terminal_light_generated: f32,
    pub last_terminal_resources: f32,
    pub terminal_bud_data: BranchData,
    pub terminal_bud_damage: f32,

    // auxillary variables
    auxillary_metamer: Option<Box<Metamer>>,
    auxillary_direction: Vecf3,
    pub last_aux_light_generated: f32,
    pub last_aux_resources: f32,
    pub aux_bud_data: BranchData,
    pub auxillary_bud_damage: f32,
}

impl Metamer {
    pub fn new(
        start_point: Vecf3,
        end_point: Vecf3,
        genetics: Arc<Mutex<PlantGenetics>>,
        metamer_id: u32,
        support_pole: Option<SupportPole>,
    ) -> Self {
        let dir = (end_point - start_point).norm();
        let auxillary_direction =
            Self::random_perturbation(dir, genetics.lock().unwrap().axillary_perturbation_angle());

        Self {
            branch_data: BranchData::new(
                start_point,
                end_point,
                0.,
                0.,
                Color::new(151, 111, 51, 255),
                metamer_id,
            ),
            genetics,
            terminal_metamer: None,
            auxillary_metamer: None,
            auxillary_direction,
            last_aux_light_generated: 0.,
            last_terminal_light_generated: 0.,
            last_light_generated: 0.,
            last_aux_resources: 0.,
            last_terminal_resources: 0.,
            terminal_bud_data: BranchData::new(
                end_point,
                end_point + (end_point - start_point).norm() * 0.05,
                0.0001,
                0.0001,
                Color::new(255, 0, 0, 255),
                get_id(),
            ),
            aux_bud_data: BranchData::new(
                end_point,
                end_point + auxillary_direction * 0.05,
                0.0001,
                0.0001,
                Color::new(255, 0, 0, 255),
                get_id(),
            ),
            terminal_bud_damage: 0.,
            auxillary_bud_damage: 0.,
            support_pole,
            aux_support_pole: None
        }
    }

    pub fn collect_branchdata(&mut self) -> Vec<&mut BranchData> {
        let mut result = vec![];
        result.push(&mut self.branch_data);

        if let Some(metamer) = &mut self.terminal_metamer {
            result.append(&mut metamer.collect_branchdata());
        } else {
            result.push(&mut self.terminal_bud_data);
        }
        if let Some(metamer) = &mut self.auxillary_metamer {
            result.append(&mut metamer.collect_branchdata());
        } else {
            result.push(&mut self.aux_bud_data);
            if let Some(pole) = &mut self.aux_support_pole {
                if pole.visible(){
                    result.push(pole.model_mut());
                }
            }
        }

        if let Some(pole) = &mut self.support_pole {
            if pole.visible(){
                result.push(pole.model_mut());
            }
        }

        return result;
    }

    pub fn remove_markers_on_buds(&self, markers: &mut MarkerSet) {
        markers.remove_markers_in_sphere(self.end_point(), self.genetics.lock().unwrap().occupancy_radius_factor());
        if let Some(metamer) = &self.terminal_metamer {
            metamer.remove_markers_on_buds(markers);
        }
        if let Some(metamer) = &self.auxillary_metamer {
            metamer.remove_markers_on_buds(markers);
        }
    }

    pub fn place_markers(&self, markers: &mut MarkerSet) -> u32 {
        let mut total = 0;

        // set markers for buds
        let theta = self.genetics.lock().unwrap().bud_perception_angle();
        let r = self.genetics.lock().unwrap().bud_perception_radius_factor();

        if let Some(metamer) = &self.terminal_metamer {
            total += metamer.place_markers(markers);
        } else {
            total += markers.set_markers_in_cone(
                self.terminal_bud_data.id(),
                self.end_point(),
                self.direction(),
                theta,
                r,
            );
        }

        if let Some(metamer) = &self.auxillary_metamer {
            total += metamer.place_markers(markers);
        } else {
            total += markers.set_markers_in_cone(
                self.aux_bud_data.id(),
                self.end_point(),
                self.auxillary_direction,
                theta,
                r,
            );
        }

        total
    }

    pub fn place_shadows(&self, shadowvoxels: &mut ShadowVoxelSet) {
        shadowvoxels.add_shadow(self.end_point());
        if let Some(metamer) = &self.terminal_metamer {
            metamer.place_shadows(shadowvoxels);
        }
        if let Some(metamer) = &self.auxillary_metamer {
            metamer.place_shadows(shadowvoxels);
        }
    }

    /**
     * Stores and return total light calculated
     */
    pub fn calc_light_gathered(&mut self, environment: &Environment) -> f32 {
        // calc terminal metamer or bud light generation
        if let Some(metamer) = &mut self.terminal_metamer {
            self.last_terminal_light_generated = metamer.calc_light_gathered(environment);
        } else {
            self.last_terminal_light_generated = environment.calc_light_gathered(
                self.end_point(),
                &self.genetics.lock().unwrap(),
                self.terminal_bud_data.id(),
                self.length(),
                self.direction(),
            );
        }

        // calc auxillary metamer or bud light generation
        if let Some(metamer) = &mut self.auxillary_metamer {
            self.last_aux_light_generated = metamer.calc_light_gathered(environment);
        } else {
            self.last_aux_light_generated = environment.calc_light_gathered(
                self.end_point(),
                &self.genetics.lock().unwrap(),
                self.aux_bud_data.id(),
                self.length(),
                self.auxillary_direction,
            );
        }

        self.last_light_generated =
            self.last_terminal_light_generated + self.last_aux_light_generated;

        return self.last_light_generated;
    }

    /**
     * Distibute resources to itself and its branches
     */
    pub fn distribute_resources(&mut self, distributor: &ResourceDistibutor, total_resources: f32) {
        distributor.distribute_resources(total_resources, self);
    }

    pub fn add_shoots(&mut self, environment: &Environment) -> u32 {
        let mut total_added = 0;

        total_added += self.add_auxillary_shoot(environment);
        total_added += self.add_terminal_shoot(environment);

        total_added
    }

    fn add_terminal_shoot(&mut self, environment: &Environment) -> u32 {
        if let Some(metamer) = &mut self.terminal_metamer {
            return metamer.add_shoots(environment);
        } else{

            if self.last_terminal_resources < self.genetics.lock().unwrap().terminal_shoot_requirement(){
                // not enough resources
                return 0;
            }

            if self.terminal_bud_damage > 0. {
                // terminal bud damaged, recovering
                self.terminal_bud_damage -= parameters::BUD_RECOVERY_SPEED;
                self.terminal_bud_damage = self.terminal_bud_damage.max(0.);
                return 0;
            }

            let support = match self
                .support_pole
                .clone()
                .map(|pole| pole.decrease_height(self.length()))
            {
                Some(Some(pole)) => Some(pole),
                _ => None,
            };

            self.terminal_metamer = self.create_shoot(
                environment,
                self.last_terminal_resources,
                self.terminal_bud_data.id(),
                self.end_point(),
                self.direction(),
                support,
            );
            if self.terminal_metamer.is_some() {
                return 1;
            }
            return 0;
        }
    }

    fn add_auxillary_shoot(&mut self, environment: &Environment) -> u32 {
        if let Some(metamer) = &mut self.auxillary_metamer {
            return metamer.add_shoots(environment);
        } else {

            if self.last_aux_resources < self.genetics.lock().unwrap().aux_shoot_requirement(Some(self)){
                // not enough aux resources
                return 0;
            }

            if self.auxillary_bud_damage > 0. {
                // aux bud damaged, recovering
                self.auxillary_bud_damage -= parameters::BUD_RECOVERY_SPEED;
                self.auxillary_bud_damage = self.auxillary_bud_damage.max(0.);
                return 0;
            }

            if self.terminal_metamer.is_none() && self.terminal_bud_damage == 0. {
                // cannot grow without terminal bud once grown
                return 0;
            }

            let mut dir = self.auxillary_direction;
            if self.terminal_bud_damage > 0.{
                // if terminal damaged, aux dir is closer to terminal
                dir += self.direction();
            }

            let support = self.aux_support_pole.clone();

            self.auxillary_metamer = self.create_shoot(
                environment,
                self.last_aux_resources,
                self.aux_bud_data.id(),
                self.end_point(),
                dir.norm(),
                support,
            );
            if self.auxillary_metamer.is_some() {
                return 1;
            }
            return 0;
        }
    }

    // calc the direction of a metamer
    fn calc_metamer_direction(
        &self,
        environment: &Environment,
        optimal_growth_direction: Vecf3,
        bud_direction: Vecf3,
    ) -> Vecf3 {
        let mut metamer_dir = bud_direction;
        metamer_dir += optimal_growth_direction * self.genetics.lock().unwrap().optimal_growth_direction_weight();
        metamer_dir += environment.tropism_dir() * environment.tropism_growth_direction_weight();
        return metamer_dir.norm();
    }

    // creates a shoot containing possibly multiple metamers given the resources
    pub fn create_shoot(
        &self,
        environment: &Environment,
        last_resources: f32,
        mut bud_id: u32,
        point: Vecf3,
        dir: Vecf3,
        mut support_pole: Option<SupportPole>,
    ) -> Option<Box<Metamer>> {

        let optimal_growth_dir =
            environment.optimal_growth_direction(point, &self.genetics.lock().unwrap(), bud_id, self.length(), dir);
        if optimal_growth_dir.is_none() {
            return None; // no space for the shoot
        }
        let optimal_growth_dir = optimal_growth_dir.unwrap();

        let total_metamers_adding = last_resources.floor() as u32;
        let metamer_length =
            (last_resources / total_metamers_adding as f32) * self.genetics.lock().unwrap().metamer_base_length();

        let mut head_metamer: Option<Box<Metamer>> = None;
        let mut last_metamer: Option<&mut Box<Metamer>> = None;
        let mut prev_end = point;
        let mut metamer_dir = dir;

        for _ in 0..total_metamers_adding {
            metamer_dir = self.calc_metamer_direction(environment, optimal_growth_dir, metamer_dir);

            if let Some(support) = &support_pole {
                metamer_dir = (metamer_dir + support.dir()).norm();
            }

            let end_point = prev_end + metamer_dir * metamer_length;
            let metamer = Box::new(Metamer::new(
                prev_end,
                end_point,
                self.genetics.clone(),
                bud_id,
                support_pole.clone(),
            ));
            bud_id = metamer.terminal_bud_data.id();
            if last_metamer.is_some() {
                let last_terminal = &mut last_metamer.unwrap().terminal_metamer;
                *last_terminal = Some(metamer);
                last_metamer = Some(last_terminal.as_mut().unwrap());
            } else {
                head_metamer = Some(metamer);
                last_metamer = Some(head_metamer.as_mut().unwrap());
            }

            prev_end = end_point;

            support_pole = match support_pole
                .clone()
                .map(|pole| pole.decrease_height(metamer_length))
            {
                Some(Some(pole)) => Some(pole),
                _ => None,
            };
        }

        return head_metamer;
    }

    // @returns a bounding volume for the metamer and all its child metamers
    pub fn _bounding_volume(&self) -> BoundingVolume {
        let mut volume = self.branch_data.bounding_volume();
        if let Some(metamer) = &self.terminal_metamer {
            volume = volume.merge(metamer.bounding_volume());
        }
        if let Some(metamer) = &self.auxillary_metamer {
            volume = volume.merge(metamer.bounding_volume());
        }

        return volume;
    }

    // @returns total metamers in the tree
    fn count_metamers(&self) -> u32 {
        let mut count = 1;
        if let Some(metamer) = &self.terminal_metamer {
            count += metamer.count_metamers();
        }
        if let Some(metamer) = &self.auxillary_metamer {
            count += metamer.count_metamers();
        }
        return count;
    }

    // remove metamers that gathered too little resources
    pub fn shed_branches(&mut self, environment: &Environment) {
        if let Some(metamer) = &mut self.terminal_metamer {
            if !environment.is_inside(metamer.end_point()) {
                self.prune_terminal();
            } else {
                let nodes = metamer.count_metamers();
                if self.last_terminal_light_generated / (nodes as f32)
                    < self.genetics.lock().unwrap().shed_treshhold()
                {
                    self.prune_terminal();
                } else {
                    metamer.shed_branches(environment);
                }
            }
        }
        if let Some(metamer) = &mut self.auxillary_metamer {
            if !environment.is_inside(metamer.end_point()) {
                self.prune_auxillary();
            } else {
                let nodes = metamer.count_metamers();
                if self.last_aux_light_generated / (nodes as f32) < self.genetics.lock().unwrap().shed_treshhold() {
                    self.prune_auxillary();
                } else {
                    metamer.shed_branches(environment);
                }
            }
        }
    }

    // update all metamer widths
    pub fn update_width(&mut self) {

        let mut total = parameters::WIDTH_MIN_VALUE;

        if let Some(metamer) = &mut self.terminal_metamer {
            metamer.update_width();
            total += metamer.start_width().powf(parameters::WIDTH_GROW_EXPONENT);
            self.branch_data
                .set_end_width(self.branch_data.end_width().max(metamer.start_width()));
        }
        if let Some(metamer) = &mut self.auxillary_metamer {
            metamer.update_width();
            total += metamer.start_width().powf(parameters::WIDTH_GROW_EXPONENT);
            self.branch_data
                .set_end_width(self.branch_data.end_width().max(metamer.start_width()));
        }

        self.branch_data.set_start_width(
            self.branch_data
                .start_width()
                .max(total.powf(1. / parameters::WIDTH_GROW_EXPONENT)),
        );
    }

    // @returns the metamer with a given id
    pub fn get_metamer_by_id(&self, id: u32) -> Option<Metamer> {
        if self.id() == id {
            return Some(self.clone());
        }

        if let Some(metamer) = &self.terminal_metamer {
            let v = metamer.get_metamer_by_id(id);
            if v.is_some() {
                return v;
            }
        }
        if let Some(metamer) = &self.auxillary_metamer {
            let v = metamer.get_metamer_by_id(id);
            if v.is_some() {
                return v;
            }
        }
        return None;
    }

    /**
     * Perturbates a vector in a random direction by the specified angle.
     */
    fn random_perturbation(mut original_vector: Vecf3, angle: f32) -> Vecf3 {
        original_vector = original_vector * 0.01;
        let vx = Vecf3::new(1.0, 0.0, 0.0);
        let vy = Vecf3::new(0.0, 1.0, 0.0);
        let mut auxiliary_vector = vx;
        if (original_vector * vx).abs() > (1.0 - 1.0e-6) {
            auxiliary_vector = vy;
        }
        let cross_vector = original_vector.cross(auxiliary_vector).norm();
        let s = Random::rand();
        let r = Random::rand();
        let h = angle.cos();
        let phi = 2.0 * 4.0 * 1.0f32.atan() * s;
        let z = h + (1.0f32 - h) * r;
        let sin_of_t = (1.0f32 - z * z).sqrt();
        let x = (phi).cos() * sin_of_t;
        let y = (phi).sin() * sin_of_t;
        return ((auxiliary_vector * x) + (cross_vector * y) + (original_vector * z)).norm();
    }

    pub fn auxillary_direction(&self) -> Vecf3 {
        self.auxillary_direction
    }

    pub fn last_light_generated(&self) -> f32 {
        self.last_light_generated
    }

    // prune a metamer given the id of the metamer
    pub fn prune_id(&mut self, id: u32) {
        if self.aux_bud_data.id() == id {
            self.prune_auxillary();
            return;
        }
        if self.terminal_bud_data.id() == id {
            self.prune_terminal();
            return;
        }

        if let Some(metamer) = &mut self.terminal_metamer {
            metamer.prune_id(id);
        }
        if let Some(metamer) = &mut self.auxillary_metamer {
            metamer.prune_id(id);
        }
    }

    pub fn prune_terminal(&mut self) {
        self.terminal_metamer = None;
        self.terminal_bud_damage = 1.;
    }
    pub fn prune_auxillary(&mut self) {
        self.auxillary_metamer = None;
        self.auxillary_bud_damage = 1.;
    }

    // @returns total metamers
    pub fn total_metamers(&self) -> u32 {
        let mut total = 0;
        if let Some(metamer) = &self.terminal_metamer {
            total += metamer.total_metamers();
        } else {
            total += 1;
        }
        if let Some(metamer) = &self.auxillary_metamer {
            total += metamer.total_metamers();
        } else {
            total += 1;
        }
        return total;
    }

    // calc the longest possible path from this metamer to a child metamer
    pub fn longest_path(&self) -> u32{
        let mut length = 1;

        let mut terminal_length = 0;
        let mut auxillary_length = 0;

        if let Some(metamer) = &self.terminal_metamer {
            terminal_length = metamer.longest_path();
        }
        if let Some(metamer) = &self.auxillary_metamer {
            auxillary_length = metamer.longest_path();
        }

        length += u32::max(terminal_length, auxillary_length); 
        return length;
    }


    pub fn terminal_metamer(& self) -> Option<&Metamer> {
        self.terminal_metamer.as_ref().map(|v|v.as_ref())
    }
    pub fn terminal_metamer_mut(&mut self) -> Option<&mut Metamer> {
        self.terminal_metamer.as_mut().map(|v|v.as_mut())
    }
    pub fn auxillary_metamer(&mut self) -> Option<&mut Metamer> {
        self.auxillary_metamer.as_mut().map(|v|v.as_mut())
    }
}

impl Deref for Metamer {
    type Target = BranchData;

    fn deref(&self) -> &Self::Target {
        &self.branch_data
    }
}

impl DerefMut for Metamer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.branch_data
    }
}
