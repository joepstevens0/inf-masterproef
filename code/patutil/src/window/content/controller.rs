use crate::Drawable;

use super::ContentState;

pub struct ContentController<'a> {
    state: &'a mut ContentState,
}

impl<'a> ContentController<'a> {
    pub fn new(state: &'a mut ContentState) -> Self {
        Self { state }
    }

    pub fn push_drawable(&mut self, draw: Box<dyn Drawable>) {
        self.state.push_drawable(draw)
    }

    pub fn push_drawables(&mut self, draw: Vec<Box<dyn Drawable>>) {
        for d in draw{
            self.state.push_drawable(d)
        }
    }

    pub fn clear_drawables(&mut self) {
        self.state.clear()
    }
}
