mod controller;
pub use controller::*;
mod model;
pub use model::*;
mod mesh;
pub use mesh::*;

use crate::camera::{FreeCam, CamRef};

#[derive(Debug)]
pub struct MeshScene{
    camera: CamRef,
    models: Vec<ModelRef>
}

impl Default for MeshScene {
    fn default() -> Self {
        Self::new(FreeCam::new().into())
    }
}

impl MeshScene {
    pub fn new(camera: CamRef) -> Self {
        Self {
            camera,
            models: vec![]
        }
    }

    pub fn camera<'a>(&self) -> CamRef {
        self.camera.clone()
    }


    pub fn models(&mut self) -> &mut [ModelRef] {
        self.models.as_mut()
    }
}