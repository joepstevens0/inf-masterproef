use std::sync::MutexGuard;

use crate::{csg::BrickRef, Color, Quaternion, Vecf4};

use super::{Brick, BrickOp, BrickType};

pub struct BrickController<'a> {
    brick: MutexGuard<'a, Brick>,
}

impl<'a> BrickController<'a> {
    pub fn new(brick: MutexGuard<'a, Brick>) -> Self { Self { brick } }

    pub fn set_next_brick(&mut self, next_brick: Option<impl Into<BrickRef>>) {
        let next: Option<BrickRef> = next_brick.map(|v| v.into());

        self.brick.next_brick = next;
    }

    pub fn set_pos(&mut self, pos: impl Into<Vecf4>) {
        let pos: Vecf4 = pos.into();
        self.brick.pos = pos;
    }
    pub fn move_pos(&mut self, v: impl Into<Vecf4>) {
        let v: Vecf4 = v.into();
        self.brick.pos += v;
    }
    pub fn set_size(&mut self, size: impl Into<Vecf4>) {
        let size: Vecf4 = size.into();
        self.brick.size = size;
    }

    pub fn set_rot(&mut self, rot: impl Into<Quaternion>) {
        let rot: Quaternion = rot.into();
        self.brick.rot = rot;
    }

    pub fn set_type(&mut self, brick_type: BrickType) {
        if self.brick.brick_type.to_string() == self.brick.name{
            self.brick.name = brick_type.to_string();
        }
        self.brick.brick_type = brick_type;
    }

    pub fn set_op(&mut self, brick_op: BrickOp) {
        self.brick.brick_op = brick_op;
    }

    pub fn set_color(&mut self, color: impl Into<Color>) {
        let color: Color = color.into();
        self.brick.color = color;
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.brick.name = name.into();
    }

    pub fn set_child(&mut self, child: Option<impl Into<BrickRef>>) {
        let child: Option<BrickRef> = child.map(|v| v.into());

        self.brick.child = child;
    }

    pub fn set_id(&mut self, id: Option<usize>){
        self.brick.id = id;
    }

    pub fn add_to_layer(&mut self, brick: BrickRef){
        if let Some(child) = self.brick.child(){
            child.controller().set_last(brick);
        } else {
            self.set_child(Some(brick));
        }
    }


    pub fn remove_from_layer(&mut self, brick: &BrickRef){
        if let Some(child) = self.brick.child(){
            if child.eq(brick){
                self.set_child(brick.get().next_brick());
            } else {
                child.controller().remove(brick);
            }
        }
    }

    pub fn remove(&mut self, brick: &BrickRef){
        if let Some(next) = self.brick.next_brick(){
            if next.eq(brick){
                self.set_next_brick(brick.get().next_brick());
            } else {
                next.controller().remove(brick);
            }
        }
    }

    pub fn set_last(&mut self, brick: BrickRef){
        if let Some(mut b) = self.brick.next_brick(){
            while let Some(next) = b.get().next_brick(){
                b = next;
            }
            b.controller().set_next_brick(Some(brick));
        } else {
            self.set_next_brick(Some(brick));
        }
    }
}