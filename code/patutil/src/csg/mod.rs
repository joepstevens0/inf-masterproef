mod brick;

use std::{sync::{Arc, Mutex, MutexGuard}, path::Path, fs::File};

pub use brick::*;

mod scene;
use patfile::pscan;
pub use scene::*;

pub fn load_brick(path: &Path) -> BrickRef{
    // open file
    let file = File::open(path).unwrap();
    use std::io::Read;
    let mut it: &mut dyn Iterator<Item = u8> =
        &mut file.bytes().map(std::result::Result::unwrap);

    // read scene
    let mut scene = CSGScene::default();
    pscan!(&mut it => "{}", scene).unwrap();
    scene.get_brick().unwrap()
}

#[derive(Clone)]
pub struct SceneRef {
    scene: Arc<Mutex<CSGScene>>,
}

impl SceneRef {
    pub fn lock<'a>(&'a self) -> MutexGuard<'a, CSGScene> {
        self.scene.lock().unwrap()
    }

    pub fn controller<'a>(&'a mut self) -> CSGSceneController<'a> {
        CSGSceneController::new(self.scene.lock().unwrap())
    }
}

impl From<CSGScene> for SceneRef {
    fn from(scene: CSGScene) -> Self {
        Self {
            scene: Arc::new(Mutex::new(scene)),
        }
    }
}

impl PartialEq for SceneRef {
    fn eq(&self, other: &Self) -> bool {
        self.scene.lock().unwrap().eq(&other.scene.lock().unwrap())
    }
}
