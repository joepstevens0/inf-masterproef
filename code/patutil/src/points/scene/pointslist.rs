
use std::{sync::{Arc, Mutex, MutexGuard}, fmt::Debug};

use crate::{Vecf3, Color};

#[derive(Clone, Copy, Debug)]
pub struct Point{
    pub pos: Vecf3,
    pub color: Color,
    pub size: f32
}

pub trait PointsList: Send + Debug{
    fn draw(&mut self);
    fn update_points(&mut self, points: Vec<Point>);
}

#[derive(Clone, Debug)]
pub struct PointsListRef{
    points: Arc<Mutex<Box<dyn PointsList>>>
}

impl PointsListRef {
    pub fn lock<'a>(&'a self) -> MutexGuard<'a, Box<dyn PointsList>> {
        self.points.lock().unwrap()
    }
}

impl<T: PointsList + 'static> From<T> for PointsListRef {
    fn from(points: T) -> Self {
        Self {
            points: Arc::new(Mutex::new(Box::new(points))),
        }
    }
}
