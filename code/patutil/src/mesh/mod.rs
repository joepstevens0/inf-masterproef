mod scene;
pub use scene::*;

use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone, Debug)]
pub struct SceneRef {
    scene: Arc<Mutex<MeshScene>>,
}

impl SceneRef {
    pub fn lock<'a>(&'a self) -> MutexGuard<'a, MeshScene> {
        self.scene.lock().unwrap()
    }

    pub fn controller<'a>(&'a mut self) -> MeshSceneController<'a> {
        MeshSceneController::new(self.scene.lock().unwrap())
    }
}

impl From<MeshScene> for SceneRef {
    fn from(scene: MeshScene) -> Self {
        Self {
            scene: Arc::new(Mutex::new(scene)),
        }
    }
}
