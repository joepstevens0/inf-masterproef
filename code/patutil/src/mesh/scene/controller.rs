use std::sync::MutexGuard;

use super::{MeshScene, ModelRef};

pub struct MeshSceneController<'a> {
    scene: MutexGuard<'a, MeshScene>,
}

impl<'a> MeshSceneController<'a> {
    pub fn new(scene: MutexGuard<'a, MeshScene>) -> Self {
        Self { scene }
    }

    pub fn add_model(&mut self, model: ModelRef) {
        self.scene.models.push(model);
    }

    pub fn remove_model(&mut self, model: &ModelRef) {
        for i in 0..self.scene.models.len(){
            if self.scene.models[i].eq(model){
                self.scene.models.remove(i);
                return;
            }
        }
    }

    pub fn clear(&mut self) {
        self.scene.models.clear();
    }
}
