use std::{fs::File, io::Write, path::Path, sync::MutexGuard};

use patfile::{pscan, pwrite};

use crate::csg::{Brick, BrickOp, BrickRef, BrickType, CSGScene};

pub struct CSGSceneController<'a> {
    scene: MutexGuard<'a, CSGScene>,
}

impl<'a> CSGSceneController<'a> {
    pub fn new(scene: MutexGuard<'a, CSGScene>) -> Self {
        Self { scene }
    }

    pub fn set_first(&mut self, first_brick: Option<BrickRef>) {
        self.scene.first_brick = first_brick;
    }

    pub fn set_first_object(&mut self, first_object: Option<BrickRef>) {
        self.scene.first_object = first_object;
    }

    pub fn first_init(&mut self) {
        self.set_first(Some(
            Brick::new(BrickType::Layer, BrickOp::Union, "root").into(),
        ));
        self.set_first_object(Some(
            Brick::new(BrickType::Layer, BrickOp::Union, "object_layer").into(),
        ));
    }

    pub fn add_brick(&mut self, brick: BrickRef){
        if let Some(first) = &mut self.scene.first_brick{
            first.controller().set_last(brick);
        } else {
            self.scene.first_brick = Some(brick);
        }
    }

    pub fn remove_brick(&mut self, brick: &BrickRef){
        if let Some(first) = &mut self.scene.first_brick{
            if brick.eq(first){
                self.scene.first_brick = None;
            } else {
                first.controller().remove(brick);
            }
        }
    }

    pub fn save(&mut self, file_path: &Path) {
        let mut file = File::create(file_path).unwrap();

        pwrite!(&mut file, "{}", &self.scene).unwrap();

        file.flush().unwrap();
    }

    pub fn load(&mut self, file_path: &Path) {
        // clear bricks
        self.clear();

        // open file
        let file = File::open(file_path).unwrap();
        use std::io::Read;
        let mut it: &mut dyn Iterator<Item = u8> =
            &mut file.bytes().map(std::result::Result::unwrap);

        // read scene
        let mut scene = CSGScene::default();
        pscan!(&mut it => "{}", scene).unwrap();

        self.scene.first_brick = scene.first_brick;
        self.scene.first_object = scene.first_object;
    }

    pub fn load_add(&mut self, file_path: &Path) -> BrickRef {
        // open file
        let file = File::open(file_path).unwrap();
        use std::io::Read;
        let mut it: &mut dyn Iterator<Item = u8> =
            &mut file.bytes().map(std::result::Result::unwrap);

        // read scene
        let mut scene = CSGScene::default();
        pscan!(&mut it => "{}", scene).unwrap();

        let result = scene.first_brick.clone().unwrap();

        if let Some(first) = &self.scene.first_brick{
            first.controller().set_last(scene.first_brick.unwrap());
        } else{
            self.scene.first_brick = scene.first_brick;
        }

        if let Some(first) = &self.scene.first_object{
            first.controller().set_last(scene.first_object.unwrap());
        } else{
            self.scene.first_object = scene.first_object;
        }

        return result;
    }

    pub fn clear(&mut self) {
        self.scene.first_brick = None;
        self.scene.first_object = None;
    }
}
