mod controller;
pub use controller::*;
mod pointslist;
pub use pointslist::*;

use crate::{camera::{FreeCam, CamRef}};

#[derive(Debug)]
pub struct PointsScene{
    camera: CamRef,
    lists: Vec<PointsListRef>
}

impl Default for PointsScene {
    fn default() -> Self {
        Self::new(FreeCam::new().into())
    }
}

impl PointsScene {
    pub fn new(camera: CamRef) -> Self {
        Self {
            camera,
            lists: vec![]
        }
    }

    pub fn camera<'a>(&self) -> CamRef {
        self.camera.clone()
    }


    pub fn points(&mut self) -> &mut [PointsListRef] {
        self.lists.as_mut()
    }
}