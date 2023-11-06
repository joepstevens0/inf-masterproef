use std::str::FromStr;

use patfile::{pscan, pwrite};

use crate::csg::{Brick, BrickRef};

pub struct BrickIdList {
    v: Vec<BrickRef>,
    object_offset: usize,
}

impl BrickIdList {
    pub fn get_first(&self) -> Option<BrickRef> {
        if self.object_offset == 0{return None;}
        self.v.get(0).map(|v| v.clone())
    }

    pub fn get_first_object(&self) -> Option<BrickRef> {
        if self.object_offset >= self.v.len(){return None;}
        self.v.get(self.object_offset).map(|v| v.clone())
    }

    pub fn get(self) -> Vec<BrickRef>{
        self.v
    }

    pub fn from_brick(
        first_brick: Option<BrickRef>,
        first_object: Option<BrickRef>,
    ) -> BrickIdList {
        Self::remove_ids(first_object.clone());
        Self::remove_ids(first_brick.clone());

        let mut id_counter = 0usize;

        Self::give_ids(first_brick.clone(), &mut id_counter);
        let object_offset = id_counter;
        Self::give_ids(first_object.clone(), &mut id_counter);
        let mut list: Vec<BrickRef> = vec![Brick::default().into(); id_counter];

        let mut stack = vec![];

        if let Some(b) = first_brick {
            stack.push(b);
        }
        if let Some(b) = first_object {
            stack.push(b);
        }

        while let Some(brick) = stack.pop() {
            let id = brick.get().id().unwrap();

            if list[id].get().id().is_none() {
                list[id] = brick.clone();

                if let Some(child) = brick.get().child() {
                    stack.push(child);
                }

                if let Some(next) = brick.get().next_brick() {
                    stack.push(next);
                }
            }
        }

        BrickIdList {
            v: list,
            object_offset,
        }
    }

    fn remove_ids(first_brick: Option<BrickRef>) {
        let mut b = first_brick;
        while let Some(brick) = b {
            if brick.get().id().is_some() {
                brick.controller().set_id(None);
                Self::remove_ids(brick.get().child());
            }
            b = brick.get().next_brick();
        }
    }

    fn give_ids(first_brick: Option<BrickRef>, start_id: &mut usize) {
        let mut b = first_brick;
        while let Some(brick) = b {
            if brick.get().id().is_none() {
                brick.controller().set_id(Some(*start_id));
                *start_id += 1;
                Self::give_ids(brick.get().child(), start_id);
            }
            b = brick.get().next_brick();
        }
    }
}

impl std::fmt::Display for BrickIdList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        pwrite!("scene total{} object_offset{}\n" => f, self.v.len(), self.object_offset).unwrap();
        for brick in &self.v {
            let b = brick.get();
            let next = b.next_brick().map_or(usize::MAX, |b| b.get().id().unwrap());
            let child = b.child().map_or(usize::MAX, |b| b.get().id().unwrap());
            pwrite!("{} next{} child{}\n" => f, b, next, child).unwrap();
        }

        Ok(())
    }
}

impl FromStr for BrickIdList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.bytes().into_iter();

        let mut total_bricks = usize::default();
        let mut object_offset = usize::default();
        pscan!(&mut it => "scene total{} object_offset{}\n", total_bricks, object_offset).unwrap();
        let mut list: Vec<BrickRef> = vec![];
        for _ in 0..total_bricks {
            list.push(Brick::default().into());
        }

        for id in 0..total_bricks {
            let mut brick = Brick::default();
            let mut next = usize::default();
            let mut child = usize::default();
            pscan!(&mut it => "{} next{} child{}\n", brick, next, child).unwrap();
            list[id].update(brick);
            if next != usize::MAX {
                list[id]
                    .controller()
                    .set_next_brick(Some(list[next].clone()));
            }
            if child != usize::MAX {
                list[id].controller().set_child(Some(list[child].clone()));
            }
        }

        Ok(BrickIdList {
            v: list,
            object_offset,
        })
    }
}
