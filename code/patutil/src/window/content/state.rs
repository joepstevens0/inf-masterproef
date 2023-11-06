use crate::Drawable;

use super::ContentController;

pub struct ContentState {
    drawables: Vec<Box<dyn Drawable>>,
}

impl ContentState {
    pub fn new() -> Self {
        Self { drawables: vec![] }
    }

    pub fn drawables(&self) -> &Vec<Box<dyn Drawable>> {
        &self.drawables
    }

    pub fn push_drawable(&mut self, draw: Box<dyn Drawable>){
        self.drawables.push(draw);

    }

    pub fn clear(&mut self){
        self.drawables.clear();
    }

    pub fn controller(&mut self) -> ContentController{
        ContentController::new(self)
    }
}
