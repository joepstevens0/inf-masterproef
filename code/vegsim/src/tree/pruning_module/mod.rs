

use patutil::Vecf3;

use crate::util::{random::Random, meter_to_real_length, rot_vec_around_axis};

use super::{metamer::Metamer, plant::Plant};

mod autoprune_spalier;
pub use autoprune_spalier::*;

#[derive(Debug, Clone)]
pub enum PruneOperation{
    Op0 = 0,
    Op1 = 1,
    Op2 = 2,
    Op3 = 3,
    Op4 = 4,
    Op5 = 5,
    Spil_1,
    Spil_2,
    Spil_3
}

impl std::fmt::Display for PruneOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct PruningModule{
}

impl PruningModule {
    pub fn prune_by_rule(rule: PruneOperation, plant: &mut Plant){
        match rule{
        
            PruneOperation::Op1 =>{
                Self::prune_rule_1(plant.root_mut());
            }
            PruneOperation::Op2 =>{
                Self::prune_rule_2(plant.root_mut());
            }
            PruneOperation::Op3 =>{
                Self::prune_rule_3(plant.root_mut());
            }
            PruneOperation::Op4 =>{
                Self::prune_rule_4(plant.root_mut());
            }
            PruneOperation::Op5 =>{
                Self::prune_rule_5(plant.root_mut());
            }
            PruneOperation::Spil_1 =>{
                Self::prune_spil_1(plant.root_mut());
            }
            PruneOperation::Spil_2 =>{
                Self::prune_spil_2(plant.root_mut());
            }
            PruneOperation::Spil_3 =>{
                Self::prune_spil_3(plant.root_mut());
            }
            _ =>{
            }
        }
    }

    fn prune_rule_1(root: &mut Metamer){
        if root.support_pole.is_none(){
            return;
        }
        root.prune_auxillary();
        if let Some(terminal) = root.terminal_metamer_mut(){
            Self::prune_rule_1(terminal);
        }
    }

    fn prune_rule_2(root: &mut Metamer){
        if let Some(metamer) = root.auxillary_metamer(){
            let bud_shorten = *Random::choose(&[3, 4]);
            Self::short_metamer_buds(metamer, bud_shorten);
        }
        if let Some(metamer) = root.terminal_metamer_mut(){
            Self::prune_rule_2(metamer);
        }
    }

    fn prune_rule_3(root: &mut Metamer){
        fn helper(root: &mut Metamer, mut max_branches: u32){
            if max_branches == 0{
                root.prune_terminal();
                return;
            }
            if let Some(metamer) = root.auxillary_metamer(){
                max_branches -= 1;
                let new_length = (metamer.longest_path() as f32 *(2./3.)).round() as u32;
                PruningModule::short_metamer_length(metamer, new_length);
            }
            if let Some(metamer) = root.terminal_metamer_mut(){
                helper(metamer, max_branches);
            }
        }

        let max_branches = *Random::choose(&[3, 4]);
        helper(root, max_branches);
    }

    fn prune_rule_4(root: &mut Metamer){
        root.prune_auxillary();
        if let Some(terminal) = root.terminal_metamer_mut(){
            Self::prune_rule_1(terminal);
        }
    }
    fn prune_rule_5(root: &mut Metamer){
        root.prune_auxillary();
        if let Some(terminal) = root.terminal_metamer_mut(){
            Self::prune_rule_1(terminal);
        }
    }

    fn prune_spil_1(root: &mut Metamer){
        // shorten terminal to 90 cm
        let max_term_height = meter_to_real_length(0.9);
        fn helper (root: &mut Metamer, max_term_height: f32){
            if root.end_point().y > max_term_height{
                root.prune_terminal();
            } else{
                if let Some(metamer) = root.terminal_metamer_mut(){
                    helper(metamer, max_term_height);
                }
            }
        }
        helper(root, max_term_height);
    }
    fn prune_spil_2(root: &mut Metamer){
        // shorten side branches to a bud aimed down
        fn branch_shorten(root: &mut Metamer, passed_length: u32) -> bool{
            if passed_length > 2 && root.terminal_metamer().is_some(){
                // prune if aux bud aimed down
                let oposite_dir = rot_vec_around_axis(&root.auxillary_direction(), &root.direction(), 180f32.to_radians());
                let down_dir = Vecf3::new(0., -1., 0.);
                if root.auxillary_direction().angle_between(&down_dir) < oposite_dir.angle_between(&down_dir){
                    root.prune_terminal();
                    return true;
                }
            }

            let mut success = false;
            if let Some(metamer) = root.terminal_metamer_mut(){
                success |= branch_shorten(metamer, passed_length + 1);
            }
            if !success{
                if let Some(metamer) = root.auxillary_metamer(){
                    success |= branch_shorten(metamer, passed_length + 1);
                }
            }
            return success;
        }

        // iterator for main trunk
        fn iterator(root: &mut Metamer){

            // shorten side branch
            if let Some(metamer) = root.auxillary_metamer(){
                if branch_shorten(metamer, 0){
                    // shorten succes
                } else{
                    // just halve the length
                    PruningModule::short_metamer_length(metamer, (metamer.longest_path() as f32/2.).floor() as u32);
                }
            }

            // next metamer in trunk
            if let Some(metamer) = root.terminal_metamer_mut(){
                iterator(metamer);
            }
        }
        iterator(root);
    }
    fn prune_spil_3(root: &mut Metamer){
    }

    fn short_metamer_length(root:&mut Metamer, length: u32){
        if length == 1{
            root.prune_terminal();
            root.prune_auxillary();
            return;
        }
        if let Some(metamer) = root.terminal_metamer_mut(){
            Self::short_metamer_buds(metamer, length-1);
        }
        if let Some(metamer) = root.auxillary_metamer(){
            Self::short_metamer_buds(metamer, length-1);
        }
    }

    fn short_metamer_buds(root:&mut Metamer, max_buds: u32){
        if max_buds == 1{
            root.prune_terminal();
            root.prune_auxillary();
            return;
        }
        if let Some(metamer) = root.terminal_metamer_mut(){
            Self::short_metamer_buds(metamer, max_buds-1);
        }
        if let Some(metamer) = root.auxillary_metamer(){
            Self::short_metamer_buds(metamer, max_buds-1);
        }
    }
}