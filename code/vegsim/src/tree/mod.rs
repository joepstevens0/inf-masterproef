use std::sync::{Arc, Mutex};

use patutil::{mesh, points, Color, Vecf3};

use crate::{
    tree::pruning_module::PruningModule,
    treeparameter::TreeParameter,
    util::{random::Random, BoundingVolume}, parameters,
};

use self::{
    environment::{Environment},
    metamer::Metamer,
    partcreator::PartCreator,
    plant::Plant,
    plantgenetics::PlantGenetics, pruning_module::PruneOperation,
};

pub mod branchdata;
mod cone;
pub mod environment;
mod markerset;
pub mod metamer;
mod partcreator;
mod plant;
pub mod plantgenetics;
pub mod pruning_module;
pub mod resourcedistibutor;
mod shadowvoxelset;
mod support_pole;

pub struct TreeApp {
    environment: Environment,
    plant_genetics: Arc<Mutex<PlantGenetics>>,
    plant: Plant,
    growth_iteration: i32,
    partcreator: PartCreator,
    pointslist: points::PointsListRef,
    selected_id: Option<u32>,
    prune_mod_on: bool
}

impl TreeApp {
    pub fn new(scene: mesh::SceneRef, mut point_scene: points::SceneRef) -> Self {
        //  set bounding volume
        let mut bounding_volume = BoundingVolume::new();
        let size = parameters::BOUNDING_BOX_SIDE;
        let min_p = Vecf3::new(-size / 2., 0., 0.);
        let max_p = Vecf3::new(size / 2., size, size);
        bounding_volume.include_point(min_p);
        bounding_volume.include_point(max_p);

        // calc seed pos
        let mut seed_pos = min_p + (max_p - min_p) / 2.;
        seed_pos.y = 0.;

        // create plant
        let genetics = Arc::new(Mutex::new(PlantGenetics::new()));
        let plant = Plant::new(seed_pos, genetics.clone());

        // create partcreator for 3D models
        let partcreator = PartCreator::new(scene.clone());

        // create point list for marker debugging
        let pointslist: points::PointsListRef = glrender::GLPointsList::new(vec![]).into();
        point_scene.controller().add_list(pointslist.clone());

        let mut this = Self {
            environment: Environment::new(bounding_volume),
            plant,
            plant_genetics: genetics,
            growth_iteration: 0,
            partcreator,
            pointslist,
            selected_id: None,
            prune_mod_on: false
        };
        this.update_draw();
        this.update_markers();
        this
    }

    pub fn perform_growth_iteration(&mut self) {
        self.growth_it();

        // update draw data
        self.update_draw();

        // update debug marker points
        self.update_markers();
    }

    fn growth_it(&mut self){
        println!("--Growth iteration {}", self.growth_iteration);
        self.plant.perform_growth_iteration(&mut self.environment);

        self.growth_iteration += 1;
        
        // perform pruning for spalier if enabled
        if self.prune_mod_on{
            static mut PRUNE_MOD: pruning_module::AutopruneSpalier = pruning_module::AutopruneSpalier::new();
            unsafe{
                PRUNE_MOD.update_plant(&mut self.plant);
            }
        }
    }

    fn update_draw(&mut self) {
        let branch_data = self.plant.collect_branchdata();
        for data in branch_data {
            if Some(data.id()) == self.selected_id {
                data.set_selected(true);
            } else {
                data.set_selected(false);
            }
            data.create_cone(&self.partcreator);
        }
    }

    fn update_markers(&mut self) {
        self.environment.markers_mut().reset();
        self.plant.place_markers(self.environment.markers_mut());
        // update marker point draw
        let mut points = vec![];
        for marker in self.environment.markers_mut().get_all_marked_points() {
            let pos = marker.position;
            // println!("Pos:{:?} bud: {}",p, bud_id);
            let color = Color::new(255, 0, 0, 80);
            // let color = Color::new(((bud_id)%255) as u8, ((bud_id/255)%255) as u8, ((bud_id/(255*255))%255) as u8, 50);
            points.push(points::Point {
                pos,
                color,
                size: 4.,
            });
        }
        self.pointslist.lock().update_points(points);
    }

    pub fn debug_texture(&self, index: u32) -> Vec<Color> {
        return self.environment.shadowvoxels().debug_texture(index);
    }

    pub fn set_selected_id(&mut self, id: Option<u32>) {
        self.selected_id = id;
        self.update_draw();
    }

    pub fn prune_id(&mut self, id: u32) {
        self.plant.prune_id(id);
        self.update_draw();
    }

    pub fn get_metamer_by_id(&self, id: u32) -> Option<Metamer> {
        self.plant.get_metamer_by_id(id)
    }

    pub fn prune_by_rule(&mut self, rule_index: PruneOperation) {
        println!("Prune {}", rule_index);
        PruningModule::prune_by_rule(rule_index, &mut self.plant);
    }

    pub fn reset_plants(&mut self) {
        Random::reset();
        self.growth_iteration = 0;
        let mut bounding_volume = BoundingVolume::new();
        let size = 50.;
        let min_p = Vecf3::new(-size / 2., 0., 0.);
        let max_p = Vecf3::new(size / 2., size, size);
        bounding_volume.include_point(min_p);
        bounding_volume.include_point(max_p);

        // calc seed pos
        let mut seed_pos = min_p + (max_p - min_p) / 2.;
        seed_pos.y = 0.;

        // reset plant
        self.plant.reset(seed_pos);

        // reset environment
        let mode = self.environment.mode();
        self.environment = Environment::new(bounding_volume);
        self.environment.set_mode(mode);

        self.update_draw();
        self.update_markers();
    }

    pub fn recalculate_plants(&mut self) {
        let it = self.growth_iteration;
        self.reset_plants();
        for _ in 0..it {
            self.growth_it();
        }

        // update draw data
        self.update_draw();

        // update marker point draw
        self.update_markers();
    }

    pub fn plant_genetics(&self) -> &Mutex<PlantGenetics> {
        &self.plant_genetics
    }

    pub fn update_tree_param(&mut self, param: TreeParameter) {
        match param {
            TreeParameter::Genetic(param) => {
                self.plant_genetics.lock().unwrap().update_param(param)
            }
            TreeParameter::ResourceDistributionMode(mode) => {
                self.plant.set_resource_distibution_mode(mode)
            }
            TreeParameter::SpaceDividingMode(mode) => {
                self.environment.set_mode(mode)
            }
            TreeParameter::PruneModOn(on) =>{
                self.prune_mod_on = on;
            }
        }
    }
    pub fn get_tree_param(&self, param: TreeParameter) -> TreeParameter{
        match param {
            TreeParameter::Genetic(param) => {
                TreeParameter::Genetic(self.plant_genetics.lock().unwrap().get_param(param))
            }
            TreeParameter::ResourceDistributionMode(_) => {
                TreeParameter::ResourceDistributionMode(self.plant.resource_distibution_mode())
            }
            TreeParameter::SpaceDividingMode(_) => {
                TreeParameter::SpaceDividingMode(self.environment.mode())
            }
            TreeParameter::PruneModOn(_) => TreeParameter::PruneModOn(self.prune_mod_on),
        }
    }
}
