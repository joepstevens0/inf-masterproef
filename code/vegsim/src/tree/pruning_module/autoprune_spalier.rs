use patutil::Vecf3;

use crate::tree::{plant::Plant, metamer::Metamer, support_pole::SupportPole};




const PASS_LENGTH: u32 = 6;


pub struct AutopruneSpalier{
}

impl AutopruneSpalier {
    pub const fn new() -> Self { Self {  } }

    pub fn update_plant(&mut self, plant: &mut Plant){
        let mut n = 0;

        // perform prune rule for every spalier trunk metamer
        // does this layer per layer
        // layers contain PASS_LENGTH metamers
        while let Some(m) = Self::get_trunk_met_by_numb(plant.root_mut(), n, PASS_LENGTH){
            // limit the height of the tree to 23 metamers
            if n > 23{
                m.prune_auxillary();
                m.prune_terminal();
                break;
            }

            // first 3 metamers of every layer is pruned for spacing
            if n % PASS_LENGTH < PASS_LENGTH - 3{
                m.prune_auxillary();
            }
            // metamer 4 in layer needs to be a supported branch to the left
            if n % PASS_LENGTH == PASS_LENGTH - 3{

                if let Some(b) = m.auxillary_metamer(){
                    Self::branch_maintanance(b);
                } else{
                    let mut pole = SupportPole::new(1.7, m.end_point(), Vecf3::new(-1., 0., 0.), true);
                    pole.update_width(0.0001);
                    m.aux_support_pole = Some(pole);
                }
            }
            // metamer 5 in layer needs to be a supported branch to the right
            if n % PASS_LENGTH == PASS_LENGTH - 2{
                if let Some(b) = m.auxillary_metamer(){
                    Self::branch_maintanance(b);
                } else{
                    let mut pole = SupportPole::new(1.7, m.end_point(), Vecf3::new(1., 0., 0.), true);
                    pole.update_width(0.0001);
                    m.aux_support_pole = Some(pole);
                }
            }
            // metamer 6 in layer needs to be pruned on terminal bud for branches to get more resources
            // auxillary bud takes role of trunk
            if n % PASS_LENGTH == PASS_LENGTH - 1{
                m.prune_terminal();
                m.aux_support_pole = Some(SupportPole::new(1., m.end_point(), Vecf3::new(0., 1., 0.), false));
            }
            n += 1;
        }
    }

    fn branch_maintanance(root: &mut Metamer){
        Self::short_metamer_length(root, 20);
        Self::short_branch_aux(root);
    }

    fn short_branch_aux(root: &mut Metamer){
        if let Some(aux) = root.auxillary_metamer(){
            Self::short_metamer_length(aux, 2);
        }
        if let Some(term) = root.terminal_metamer_mut(){
            Self::branch_maintanance(term);
        }
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

    fn get_trunk_met_by_numb(root: &mut Metamer, n: u32, pass: u32) -> Option<&mut Metamer>{
        if n == 0{
            return Some(root);
        }
        if pass > 1{
            if root.terminal_metamer().is_some(){
                return Self::get_trunk_met_by_numb(root.terminal_metamer_mut().unwrap(), n-1, pass - 1);
            }
        } else {
            if let Some(m) = root.auxillary_metamer(){
                return Self::get_trunk_met_by_numb(m, n-1, PASS_LENGTH);
            }
        }
        return None;
    }

    fn term_length(metamer: &Metamer) -> u32{
        if let Some(m) = metamer.terminal_metamer(){
            return Self::term_length(m) + 1;
        }

        return 1;
    }
}