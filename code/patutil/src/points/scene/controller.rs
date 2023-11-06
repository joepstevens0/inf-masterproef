use std::sync::MutexGuard;


use super::{PointsScene, PointsListRef};


pub struct MeshSceneController<'a> {
    scene: MutexGuard<'a, PointsScene>,
}

impl<'a> MeshSceneController<'a> {
    pub fn new(scene: MutexGuard<'a, PointsScene>) -> Self {
        Self { scene }
    }

    pub fn add_list(&mut self, list: PointsListRef) {
        self.scene.lists.push(list);
    }

    pub fn clear(&mut self) {
        self.scene.lists.clear();
    }
}
