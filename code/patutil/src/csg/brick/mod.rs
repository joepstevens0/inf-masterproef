mod brickop;
pub use brickop::*;
mod bricktype;
pub use bricktype::*;

mod brickref;
pub use brickref::*;

mod controller;
mod tests;
pub use controller::*;

mod iterator;
pub use iterator::*;

use crate::{Color, Quaternion, Vecf4};
use core::fmt;
use patfile::{pscan, pwrite};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Brick {
    brick_type: BrickType,
    brick_op: BrickOp,
    pos: Vecf4,
    size: Vecf4,
    rot: Quaternion,
    color: Color,
    name: String,
    next_brick: Option<BrickRef>,
    child: Option<BrickRef>,
    id: Option<usize>,
}

impl Brick {
    pub fn new(brick_type: BrickType, brick_op: BrickOp, name: impl Into<String>) -> Self {
        Self {
            brick_type,
            brick_op,
            name: name.into(),
            pos: [0., 0., 0., 0.].into(),
            size: [1., 1., 1., 1.].into(),
            rot: Quaternion::default(),
            color: [255, 0, 0, 255].into(),
            next_brick: None,
            child: None,
            id: None,
        }
    }

    pub fn brick_type(&self) -> BrickType {
        self.brick_type
    }

    pub fn brick_op(&self) -> BrickOp {
        self.brick_op
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn pos(&self) -> Vecf4 {
        self.pos
    }

    pub fn size(&self) -> Vecf4 {
        if self.brick_type() == BrickType::Layer{
            let mut size = Vecf4::default();
            if let Some(child) = self.child(){
                child.get().bounding_box(self.pos(), &mut size);
            }
            size[3] = self.size[3];
            return size;
        }

        self.size
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn next_brick(&self) -> Option<BrickRef> {
        if let Some(b) = &self.next_brick {
            return Some(b.clone());
        }
        return None;
    }

    pub fn child(&self) -> Option<BrickRef> {
        if let Some(b) = &self.child {
            return Some(b.clone());
        }
        return None;
    }

    pub fn rot(&self) -> Quaternion {
        self.rot
    }

    pub fn id(&self) -> Option<usize> {
        self.id
    }

    pub fn find_id(&self, id: usize) -> Option<BrickRef>{
        if let Some(next) = self.next_brick(){
            if next.get().id() == Some(id){
                return Some(next);
            }

            if let Some(b) = next.get().find_id(id){
                return Some(b);
            }
        }

        if let Some(child) = self.child(){
            if child.get().id() == Some(id){
                return Some(child);
            }

            if let Some(b) = child.get().find_id(id){
                return Some(b);
            }
        }
        return None;
    }

    fn bounding_box(&self, center: Vecf4, size: &mut Vecf4){
        let p = self.size*self.size[3] + (self.pos.abs()*2f32);

        size[0] = f32::max(size[0], p[0]);
        size[1] = f32::max(size[1], p[1]);
        size[2] = f32::max(size[2], p[2]);

        if let Some(next) = &self.next_brick{
            next.get().bounding_box(center, size);
        }
    }
}

impl Default for Brick {
    fn default() -> Self {
        Self::new(
            Default::default(),
            Default::default(),
            format!("{}", BrickType::default()),
        )
    }
}

macro_rules! BRICK_STRING_FORMAT {
    () => {
        "brick name{} type{} op{} pos{} size{} rot{} color{}"
    };
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut brick_type: BrickType = BrickType::Sphere;
        let mut brick_op: BrickOp = BrickOp::Union;
        let mut name: String = "".to_string();
        let mut pos: Vecf4 = Vecf4::default();
        let mut size: Vecf4 = Vecf4::default();
        let mut rot: Quaternion = Quaternion::default();
        let mut color: Color = Color::default();

        let mut it = s.bytes().into_iter();
        pscan!(&mut it => BRICK_STRING_FORMAT!(), name, brick_type, brick_op, pos, size, rot, color).unwrap();
        let brick = Brick {
            brick_type,
            brick_op,
            pos,
            size,
            rot,
            color,
            name,
            next_brick: None,
            child: None,
            id: None,
        };

        Ok(brick)
    }
}

impl Display for Brick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let writer: &mut dyn std::fmt::Write = f;
        pwrite!(BRICK_STRING_FORMAT!() => writer, &self.name,
        self.brick_type,
        self.brick_op,
        self.pos,
        self.size,
        self.rot,
        &self.color)
        .unwrap();
        Ok(())
    }
}
