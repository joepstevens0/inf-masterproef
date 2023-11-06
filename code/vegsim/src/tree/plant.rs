use std::sync::{Arc, Mutex};

use patutil::Vecf3;

use crate::treeparameter::DistributionMode;

use super::{branchdata::BranchData, markerset::MarkerSet, metamer::Metamer, PlantGenetics, environment::Environment, support_pole::SupportPole, resourcedistibutor::{ResourceDistibutor}};

pub struct Plant {
    genetics: Arc<Mutex<PlantGenetics>>,
    root: Metamer,
    distributor: ResourceDistibutor
}

impl Plant {
    pub fn new(seed_pos: Vecf3, genetics: Arc<Mutex<PlantGenetics>>) -> Self {
        let seed_dir = Vecf3::new(0., 1., 0.);
        // let seed_dir = Vecf3::new(0.5,0.5, 0.).norm();

        let root_start = seed_pos;
        let root_end = seed_pos + seed_dir*genetics.lock().unwrap().metamer_base_length();
        let support_pole = Self::start_pole(seed_pos, &genetics.lock().unwrap(), seed_dir);
        let mut root = Metamer::new(root_start, root_end, genetics.clone(), 1, Some(support_pole));

        root.update_width();

        Self { root, genetics, distributor: ResourceDistibutor::new() }
    }

    pub fn reset(&mut self, seed_pos: Vecf3){
        let seed_dir = Vecf3::new(0., 1., 0.);
        let root_start = seed_pos;
        let root_end = seed_pos + seed_dir*self.genetics.lock().unwrap().metamer_base_length();
        let support_pole = Self::start_pole(seed_pos, &self.genetics.lock().unwrap(), seed_dir);
        self.root = Metamer::new(root_start, root_end, self.genetics.clone(), 1, Some(support_pole));
        self.root.update_width();
    }

    fn start_pole(seed_pos: Vecf3, genetics: &PlantGenetics, seed_dir: Vecf3) -> SupportPole{
        SupportPole::new(genetics.base_pole_lenght(), seed_pos - Vecf3::new(0.,0.,-0.3), seed_dir, false)
        // SupportPole::new(genetics.base_pole_lenght(), seed_pos - Vecf3::new(0.,0.,-0.3), Vecf3::new(0.5, 0.5, 0.), false)
    }

    pub fn collect_branchdata(&mut self) -> Vec<&mut BranchData> {
        return self.root.collect_branchdata();
    }

    pub fn perform_growth_iteration(&mut self, environment: &mut Environment) {
        // move light to base
        let total_light = self.calc_light_gathered(environment);
        println!("Total light gathed: {}", total_light);

        // transform light into resources
        let total_resources = self.light_to_resources(total_light);
        println!("Total resources: {}", total_resources);

        // propagate resources to tips
        self.resources_to_tips(total_resources);
        println!("Resources moved toward tips");

        // create shoots from resources
        let total_shoots_added = self.add_shoots(environment);
        println!("Total shoots added: {}", total_shoots_added);

        // recalculate light and shed branches
        self.calc_light_gathered(environment);
        self.root.shed_branches(environment);


        // update metamer withs
        self.update_metamer_widths();
        println!("Updated metamer widths");

        environment.increase_tropism();
    }

    pub fn place_markers(&mut self, markers: &mut MarkerSet)-> u32 {
        self.root.remove_markers_on_buds(markers);
        self.root.place_markers(markers)
    }

    pub fn get_metamer_by_id(&self, id: u32) -> Option<Metamer>{
        self.root.get_metamer_by_id(id)
    }

    pub fn prune_id(&mut self, id: u32){
        self.root.prune_id(id);
    }

    fn calc_light_gathered(&mut self, environment: &mut Environment) -> f32 {
        // allocate markers
        environment.reset_space();
        let total_markers_placed = self.place_markers(environment.markers_mut());
        println!("Total markers placed: {}", total_markers_placed);
        self.root.place_shadows(environment.shadowvoxels_mut());

        // move light to base
        let total_light = self.root.calc_light_gathered(environment);
        return total_light;
    }
    fn light_to_resources(&mut self, total_light_generated: f32) -> f32 {
        self.genetics.lock().unwrap().borchert_honda_alpha()*total_light_generated
    }
    fn resources_to_tips(&mut self, total_resources: f32) {
        self.root.distribute_resources(&self.distributor,total_resources);
    }
    fn add_shoots(&mut self, environment: &Environment) -> u32 {
        self.root.add_shoots(environment)
    }
    fn update_metamer_widths(&mut self) {
        self.root.update_width();
    }

    pub fn root_mut(&mut self) -> &mut Metamer {
        &mut self.root
    }

    pub fn set_resource_distibution_mode(&mut self, mode: DistributionMode){
        self.distributor.set_mode(mode);
    }
    pub fn resource_distibution_mode(&self)-> DistributionMode{
        self.distributor.mode()
    }
}
