mod controller;
pub use controller::*;
mod brickidlist;
pub use brickidlist::*;
use patfile::pscan;
mod tests;

use std::{
    fmt::Display,
    str::FromStr, path::Path, fs::File,
};

use super::BrickRef;
use crate::camera::{FreeCam, CamRef};

pub struct CSGScene {
    camera: CamRef,
    first_brick: Option<BrickRef>,
    first_object: Option<BrickRef>,
}

impl CSGScene {
    pub fn new(camera: CamRef) -> Self {
        Self {
            camera,
            first_brick: None,
            first_object: None
        }
    }

    pub fn camera<'a>(&self) -> CamRef {
        self.camera.clone()
    }

    pub fn get_brick(&self) -> Option<BrickRef> {
        self.first_brick.clone()
    }

    pub fn get_object_brick(&self) -> Option<BrickRef> {
        self.first_object.clone()
    }

    pub fn find_id(&self, id: usize) -> Option<BrickRef> {
        if let Some(first) = &self.first_brick {
            if first.get().id() == Some(id) {
                return Some(first.clone());
            }
            return first.get().find_id(id);
        }
        return None;
    }
}

impl Default for CSGScene {
    fn default() -> Self {
        Self::new(FreeCam::new().into())
    }
}
impl PartialEq for CSGScene {
    fn eq(&self, other: &Self) -> bool {
        if self.first_brick.is_none() != other.first_brick.is_none() {
            return false;
        }
        let mut b1 = self.first_brick.clone().unwrap();
        let mut b2 = other.first_brick.clone().unwrap();

        'outer: loop {
            if b1.get() != b2.get() {
                return false;
            }

            match (
                b1.get().next_brick().is_none(),
                b2.get().next_brick().is_none(),
            ) {
                (true, true) => break 'outer,
                (true, false) => {
                    return false;
                }
                (false, false) => {
                    b1 = b1.get().next_brick().unwrap();
                    b2 = b2.get().next_brick().unwrap();
                }
                (false, true) => return false,
            }
        }

        return true;
    }
}

impl From<&CSGScene> for BrickIdList{
    fn from(scene: &CSGScene) -> Self {
        BrickIdList::from_brick( scene.first_brick.clone(), scene.first_object.clone())
    }
}

impl Display for CSGScene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = BrickIdList::from_brick( self.first_brick.clone(), self.first_object.clone());
        list.fmt(f)
    }
}

impl FromStr for CSGScene {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = CSGScene::default();

        // read bricks
        let list = BrickIdList::from_str(s)?;

        this.first_brick = list.get_first();
        this.first_object = list.get_first_object();

        Ok(this)
    }
}

impl From<&Path> for CSGScene{
    fn from(path: &Path) -> Self {
        // open file
        let file = File::open(path).unwrap();
        use std::io::Read;
        let mut it: &mut dyn Iterator<Item = u8> =
            &mut file.bytes().map(std::result::Result::unwrap);

        // read scene
        let mut scene = CSGScene::default();
        pscan!(&mut it => "{}", scene).unwrap();
        scene
    }
}

impl From<BrickRef> for CSGScene{
    fn from(brick: BrickRef) -> Self {
        let mut scene = CSGScene::default();
        scene.first_brick = Some(brick);
        scene
    }
}