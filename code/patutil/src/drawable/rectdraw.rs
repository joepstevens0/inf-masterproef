use crate::{Color, Drawable, Rect};

#[derive(Debug, Clone, Copy)]
pub struct RectDraw {
    rect: Rect,
    color: Color,
    round_edge: bool
}

impl RectDraw {
    pub fn new(rect: Rect, color: Color, round_edge: bool) -> Self {
        Self { rect, color, round_edge }
    }
    pub fn scale(&mut self, s: f32){
        self.rect.w = (self.rect.w as f32*s) as u32;
        self.rect.h = (self.rect.h as f32*s) as u32;
    }
}

impl Drawable for RectDraw {
    fn set_pos(&mut self, pos: crate::Pos) {
        self.rect.set_pos(pos);
    }

    fn rect(&self) -> Rect {
        self.rect
    }

    fn draw(&self, render: &dyn crate::Render) {
        render.draw_rect(self.rect, self.color, self.round_edge)
    }
}