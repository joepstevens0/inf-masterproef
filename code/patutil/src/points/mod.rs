mod scene;
pub use scene::*;

use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone, Debug)]
pub struct SceneRef {
    scene: Arc<Mutex<PointsScene>>,
}

impl SceneRef {
    pub fn lock<'a>(&'a self) -> MutexGuard<'a, PointsScene> {
        self.scene.lock().unwrap()
    }

    pub fn controller<'a>(&'a mut self) -> MeshSceneController<'a> {
        MeshSceneController::new(self.scene.lock().unwrap())
    }
}

impl From<PointsScene> for SceneRef {
    fn from(scene: PointsScene) -> Self {
        Self {
            scene: Arc::new(Mutex::new(scene)),
        }
    }
}
