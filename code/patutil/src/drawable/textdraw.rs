use crate::{text::Text, Drawable, Rect};

pub struct TextDraw {
    text: Text,
    rect: Rect,
}

impl TextDraw {
    pub fn new(text: Text) -> Self {
        let rect = Rect::new(0, 0, text.width(), text.height());
        Self { text, rect }
    }
}

impl Drawable for TextDraw {
    fn set_pos(&mut self, pos: crate::Pos) {
        self.rect.set_pos(pos);
    }

    fn rect(&self) -> Rect {
        self.rect
    }

    fn draw(&self, render: &dyn crate::Render) {
        render.draw_text(self.rect.pos(), &self.text)
    }
}
