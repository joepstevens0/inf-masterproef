use std::collections::HashMap;

use crate::{treeparameter::DistributionMode, parameters};

use super::metamer::Metamer;

pub struct ResourceDistibutor {
    mode: DistributionMode
}

#[derive(Debug)]
struct BudInfo{
    light_collected: f32,
    bud_id: u32,
    total_buds: u32
}

impl ResourceDistibutor {
    pub fn new() -> Self { 
        Self { mode: parameters::RESOURCE_DISTRIBUTION_MODE } 
    }

    pub fn distribute_resources(&self, total_resources: f32, metamer: &mut Metamer) {
        Self::reset_bud_resources(metamer);

        match self.mode {
            DistributionMode::BorchertHonda => {Self::distribute_resources_borchert_honda(total_resources, metamer);},
            DistributionMode::PriorityList => Self::distribute_resources_priority_list(total_resources/2., metamer),
            DistributionMode::None => return,
        }
    }

    fn reset_bud_resources(metamer: &mut Metamer){
        metamer.last_terminal_resources = 0.;
        metamer.last_aux_resources = 0.;
        // distribute terminal resources
        if let Some(metamer) = metamer.terminal_metamer_mut() {
            Self::reset_bud_resources(metamer);
        }

        // distribute aux resources
        if let Some(metamer) = metamer.auxillary_metamer() {
            Self::reset_bud_resources(metamer);
        }
    }

    fn borchert_honda_split(resources: f32, q_m: f32,  q_l: f32, lambda: f32) -> [f32;2]{
        let v = resources;
        let denominator = lambda * q_m + (1. - lambda) * q_l;
        let v_m = v * lambda * q_m / denominator;
        let v_l = v * (1. - lambda) * q_l / denominator;
        return [v_m, v_l]
    }

    fn distribute_resources_borchert_honda(total_resources: f32, metamer: &mut Metamer) -> f32 {
        let lambda =  metamer.genetics.lock().unwrap().borchert_honda_lambda();
        let q_m = metamer.last_terminal_light_generated;
        let q_l = metamer.last_aux_light_generated;

        if q_m + q_l <= 0. {
            // no light gathered, prevent division by zero
            return 0.;
        }

        [metamer.last_terminal_resources, metamer.last_aux_resources] = ResourceDistibutor::borchert_honda_split(total_resources, q_m, q_l, lambda);
        let mut bonus = 0.0f32;

        
        if metamer.auxillary_bud_damage > 0.{
            // give all resources to terminal bud
            metamer.last_terminal_resources += metamer.last_aux_resources;
            metamer.last_aux_resources = 0.;
        } 
        
        // give resources to terminal bud
        if metamer.terminal_bud_damage > 0.{
            bonus = metamer.last_terminal_resources;
            metamer.last_terminal_resources = 0.;
        } 


        // distribute terminal resources
        let v_m = metamer.last_terminal_resources;
        if let Some(m) = metamer.terminal_metamer_mut() {
            bonus +=  Self::distribute_resources_borchert_honda(v_m, m);
        }


        metamer.last_aux_resources += bonus*0.5;

        // distribute aux resources
        let v_l = metamer.last_aux_resources;
        if let Some(m) = metamer.auxillary_metamer() {
            Self::distribute_resources_borchert_honda(v_l, m);
        }
        return bonus*0.5;
    }

    // calculates priority of a bud in the priority list for the branch
    fn bud_priority(bud: &BudInfo) -> f32
    {
        return bud.light_collected / bud.total_buds as f32;
    }

    // insert a bud into the priority list in the right position
    fn priority_list_insert<'a>(list: &mut Vec<BudInfo>, bud: BudInfo)
    {
        let priority = Self::bud_priority(&bud);
        for (i, list_bud) in list.iter().enumerate()
        {
            if Self::bud_priority(list_bud) < priority
            {
                list.insert(i, bud);
                return;
            }
        }

        list.push(bud);
    }

    // returns the weight of an item with index i in a priority list
    fn priority_list_weight(i: f32, total: f32) -> f32 {
        const W_MAX: f32 = 1.;
        const W_MIN: f32 = 0.006;
        const K: f32 = 0.5;

        if K * total <= i {
            return W_MIN;
        }

        return W_MAX - ((i / (total * K)) * (W_MAX - W_MIN));
    }

    fn create_priority_list(metamer: &mut Metamer) -> Vec<BudInfo>{
        let mut priority_list: Vec<BudInfo> = vec![];

        // iterate over metamers of the branch
        let mut branch_metamer: &mut Metamer = metamer;
        loop {
            // gather auxillary bud data
            let auxbud = BudInfo{
                light_collected: branch_metamer.last_aux_light_generated,
                bud_id: branch_metamer.aux_bud_data.id(),
                total_buds: branch_metamer.auxillary_metamer().as_ref().map_or(1, |b|b.total_metamers()),
            };

            if branch_metamer.auxillary_bud_damage == 0.{
                // insert auxillary bud into priority list
                Self::priority_list_insert(&mut priority_list, auxbud);
            }

            // advance to next metamer in branch
            if branch_metamer.terminal_metamer().is_some(){
                branch_metamer = branch_metamer.terminal_metamer_mut().unwrap();
            } else{
                // last metamer reached
                break;
            }
        }
        // insert terminal bud into priority list
        if branch_metamer.terminal_bud_damage == 0.{
            let terminalbud = BudInfo{
                light_collected: branch_metamer.last_terminal_light_generated,
                bud_id: branch_metamer.terminal_bud_data.id(),
                total_buds: 1,
            };
            priority_list.insert(0, terminalbud);
        }
        return priority_list;
    }

    fn distribute_prioritylist_map(metamer: &mut Metamer, resource_map: HashMap<u32, f32>){

        // iterate over every bud of the branch
        let mut branch_metamer: &mut Metamer = metamer;
        loop {
            // give calculated resources to bud
            let resources = *resource_map.get(&branch_metamer.aux_bud_data.id()).unwrap_or(&0.);
            branch_metamer.last_aux_resources = resources;

            // distribute auxillary resources
            if let Some(m) = &mut branch_metamer.auxillary_metamer(){
                Self::distribute_resources_priority_list(resources, m);
            }

            // advance to next metamer in branch
            if branch_metamer.terminal_metamer().is_some(){
                branch_metamer = branch_metamer.terminal_metamer_mut().unwrap();
            } else{
                // last metamer reached
                break;
            }
        }

        // give resources to last terminal bud
        branch_metamer.last_terminal_resources = *resource_map.get(&branch_metamer.terminal_bud_data.id()).unwrap_or(&0.);
    }

    fn distribute_resources_priority_list(total_resources: f32, metamer: &mut Metamer) {
        if metamer.last_light_generated <= 0. {
            // no light gathered
            return;
        }
        
        // create the priority list
        let priority_list = Self::create_priority_list(metamer);

        // calculate sum of priorities
        let mut priority_sum = 0.;
        let total_buds = priority_list.len();
        for (i,bud) in priority_list.iter().enumerate()
        {
            priority_sum += Self::priority_list_weight(i as f32, total_buds as f32) * bud.light_collected;
        }

        // calculate resources for every bud in priority list
        let mut resource_map = HashMap::new();
        for (i,bud) in priority_list.iter().enumerate()
        {
            let r: f32 = total_resources * ((bud.light_collected * Self::priority_list_weight(i as f32, total_buds as f32)) / priority_sum);
            
            resource_map.insert(bud.bud_id, r);
        }

        // give calculated resources to buds
        Self::distribute_prioritylist_map(metamer, resource_map);
    }

    pub fn set_mode(&mut self, mode: DistributionMode) {
        self.mode = mode;
    }

    pub fn mode(&self) -> DistributionMode {
        self.mode
    }
}
