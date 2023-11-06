use crate::{Pos, Size, Rect, Render};

pub mod rectdraw;
pub mod textdraw;
pub mod csgdraw;
pub mod linedraw;

pub trait Drawable: Send + Sync {
    fn set_pos(&mut self, pos: Pos);
    fn size(&self) -> Size {
        self.rect().size()
    }
    fn pos(&self) -> Pos {
        self.rect().pos()
    }
    fn rect(&self) -> Rect;
    fn draw(&self, render: &dyn Render);
}
