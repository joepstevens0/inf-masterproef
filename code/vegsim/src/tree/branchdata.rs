use patutil::{Vecf3, Color};

use crate::util::BoundingVolume;

use super::{partcreator::PartCreator, cone::Cone};

#[derive(Debug, Clone)]
pub struct BranchData {
    start_point: Vecf3,
    end_point: Vecf3,
    start_width: f32,
    end_width: f32,
    cone: Option<Cone>,
    color: Color,
    id: u32,
    selected: bool
}

impl BranchData {
    pub fn new(start_point: Vecf3, end_point: Vecf3, start_width: f32, end_width: f32, color: Color, id: u32) -> Self {
        Self {
            start_point,
            end_point,
            start_width,
            end_width,
            cone: None,
            color,
            id,
            selected: false
        }
    }

    pub fn create_cone(&mut self, partcreator: &PartCreator) {
        let mul = 100.;

        let color = self.color();
        let mut dir = self.end_point - self.start_point;
        let length = dir.length();
        dir = dir.norm();
        let base_width = self.start_width()*mul;
        let tip_width = self.end_width()*mul;

        if self.cone.is_none(){
            let mut cone = partcreator.new_cone(length, base_width, tip_width, color, self.id);
            cone.set_dir(dir);
            cone.set_pos(self.start_point);

            self.cone = Some(cone);
        } else {
            let cone = self.cone.as_mut().unwrap();
            cone.set_length(length);
            cone.set_width_base(base_width);
            cone.set_width_tip(tip_width);
            cone.set_dir(dir);
            cone.set_color(color);
 
        }
    }

    pub fn length(&self) -> f32 {
        (self.end_point - self.start_point).length()
    }

    pub fn center(&self) -> Vecf3 {
        return self.start_point + (self.start_point - self.end_point) / 2.;
    }

    pub fn begin_point(&self) -> Vecf3 {
        self.start_point
    }

    pub fn end_point(&self) -> Vecf3 {
        self.end_point
    }

    pub fn direction(&self) -> Vecf3 {
        (self.end_point - self.start_point).norm()
    }

    pub fn start_width(&self) -> f32 {
        self.start_width
    }
    pub fn end_width(&self) -> f32 {
        self.end_width
    }

    pub fn bounding_volume(&self) -> BoundingVolume {
        let mut volume = BoundingVolume::new();

        volume.include_point(self.start_point);
        volume.include_point(self.end_point);
        return volume;
    }

    pub fn set_start_point(&mut self, start_point: Vecf3) {
        self.start_point = start_point;
    }

    pub fn set_end_point(&mut self, end_point: Vecf3) {
        self.end_point = end_point;
    }

    pub fn set_start_width(&mut self, start_width: f32) {
        self.start_width = start_width;
    }

    pub fn set_end_width(&mut self, end_width: f32) {
        self.end_width = end_width;
    }

    pub fn set_length(&mut self, length: f32){
        let dir = self.direction();
        self.end_point = self.begin_point() + dir*length;
    }

    pub fn start_point(&self) -> Vecf3 {
        self.start_point
    }

    pub fn color(&self) -> Color {
        if self.selected{
            return Color::new(0, 0, 255, 255);
        }
        self.color
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}
