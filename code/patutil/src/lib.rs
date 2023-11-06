mod render;
pub use render::*;
mod drawable;
pub use drawable::*;
pub mod csg;
pub mod points;
pub mod mesh;
pub mod camera;
mod vec;
pub use vec::*;
mod color;
pub use color::*;
pub mod window;
mod angle;
pub use angle::*;

use std::ops::{Add, Div};

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    pub fn is_inside(&self, pos: Pos) -> bool {
        return pos.x >= self.x
            && pos.x <= self.x + self.w as i32
            && pos.y >= self.y
            && pos.y <= self.y + self.h as i32;
    }

    pub fn pos(&self) -> Pos {
        [self.x, self.y].into()
    }

    pub fn set_pos(&mut self, pos: Pos) {
        self.x = pos.x;
        self.y = pos.y;
    }

    pub fn pos_move(&mut self, pos: Pos) {
        self.x += pos.x;
        self.y += pos.y;
    }

    pub fn size(&self) -> Size {
        [self.w, self.h].into()
    }

    pub fn scale(&mut self, s: f32){
        self.w = (self.w as f32 *s) as u32;
        self.h = (self.h as f32 *s) as u32;
    }
}

impl From<(Pos, Size)> for Rect {
    fn from(data: (Pos, Size)) -> Self {
        Rect::new(data.0.x, data.0.y, data.1.w, data.1.h)
    }
}
impl From<(Vec2, Vecu2)> for Rect {
    fn from(data: (Vec2, Vecu2)) -> Self {
        Rect::new(data.0.x, data.0.y, data.1.x, data.1.y)
    }
}

impl Default for Rect {
    fn default() -> Self {
        Rect::from((Pos::default(), Size::default()))
    }
}

pub type Pos = Vec2;
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

impl Size {
    pub const fn new(w: u32, h: u32) -> Self {
        Self { w, h }
    }

    pub const fn to_pos(self) -> Pos{
        Pos::new(self.w as i32, self.h as i32)
    }
}
impl From<[u32; 2]> for Size {
    fn from(data: [u32; 2]) -> Self {
        Self {
            w: data[0],
            h: data[1],
        }
    }
}

impl Add<Size> for Size {
    type Output = Size;

    fn add(self, rhs: Size) -> Self::Output {
        Size {
            w: self.w + rhs.w,
            h: self.h + rhs.h,
        }
    }
}

impl Div<u32> for Size{
    type Output = Size;

    fn div(self, rhs: u32) -> Self::Output {
        Size {
            w: self.w/ rhs,
            h: self.h/ rhs,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub point1: Pos,
    pub point2: Pos,
}

impl Line {
    pub const fn new(point1: Pos, point2: Pos) -> Self {
        Self { point1, point2 }
    }
}

impl From<[Pos;2]> for Line {
    fn from(data: [Pos;2]) -> Self {
        Line::new(data[0], data[1])
    }
}

impl Default for Line {
    fn default() -> Self {
        Line::from([Pos::default(), Pos::default()])
    }
}