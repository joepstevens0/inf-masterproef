use patutil::{Vecf3, Color};

use crate::util::meter_to_real_length;

use super::branchdata::BranchData;


#[derive(Debug, Clone)]
pub struct SupportPole {
    length: f32,
    start_point: Vecf3,
    dir: Vecf3,
    model: BranchData,
    visible: bool
}

impl SupportPole {
    pub fn new(mut length: f32, start_point: Vecf3, dir: Vecf3, visible: bool) -> Self {
        length = meter_to_real_length(length);
        const WIDTH: f32 = 0.0005;
        Self {
            length,
            start_point,
            dir,
            model: BranchData::new(start_point, start_point + (dir*length), WIDTH, WIDTH, Color::new(0, 255, 0, 255), 0),
            visible
        }
    }

    pub fn decrease_height(mut self, length: f32) -> Option<Self>{
        if self.length <= length{
            return None;
        }
        self.length -= length;
        self.start_point += self.dir*length;
        Some(self)
    }

    pub fn dir(&self) -> Vecf3 {
        self.dir
    }

    pub fn model_mut(&mut self) -> &mut BranchData {
        &mut self.model
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn update_width(&mut self, width: f32){
        self.model_mut().set_end_width(width);
        self.model_mut().set_start_width(width);
    }
}
