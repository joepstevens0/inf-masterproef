use std::sync::{Mutex, Arc};

use super::{Brick, BrickController, BrickIterator};



#[derive(Debug, Clone)]
pub struct BrickRef{
    brick: Arc<Mutex<Brick>>
}

impl BrickRef {
    pub fn get(&self)-> Brick{
        self.brick.lock().unwrap().clone()
    }

    
    pub fn controller(&self) -> BrickController {
        BrickController::new(self.brick.lock().unwrap())
    }
    

    pub fn update(&mut self, brick: Brick){
        *self.brick.lock().unwrap() = brick;
    }
    pub fn swap(&mut self, mut other: BrickRef) { 
        if other.eq(self){ return;}

        let mut brick1 = self.get();
        let mut brick2 = other.get();
        let next_1 = brick1.next_brick();
        let next_2 = brick2.next_brick();
        brick1.next_brick = next_2;
        brick2.next_brick = next_1;

        self.update(brick2);
        other.update(brick1);
    }

    pub fn iter(self) -> BrickIterator{
        BrickIterator::new(Some(self))
    }

    pub fn iter_layer(self) -> BrickIterator{
        BrickIterator::new(self.get().child())
    }
}

impl From<Brick> for BrickRef{
    fn from(brick: Brick) -> Self {
        Self{brick: Arc::new(Mutex::new(brick))}
    }
}

impl PartialEq for BrickRef{
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.brick, &other.brick)
    }
}