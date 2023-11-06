use crate::{csg::SceneRef, Drawable, Rect, Render};

pub struct CSGDraw {
    scene: SceneRef,
    rect: Rect,
}

impl CSGDraw {
    pub fn new(scene: SceneRef, rect: Rect) -> Self {
        Self { scene, rect }
    }
}

impl Drawable for CSGDraw {
    fn set_pos(&mut self, pos: crate::Pos) {
        self.rect.set_pos(pos);
    }

    fn rect(&self) -> Rect {
        self.rect
    }

    fn draw(&self, render: &dyn Render) {
        render.draw_csg(self.rect, self.scene.clone());
    }
}
