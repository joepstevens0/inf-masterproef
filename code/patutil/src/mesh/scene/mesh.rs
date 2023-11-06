
use std::{sync::{Arc, Mutex, MutexGuard}, fmt::Debug};

pub trait Mesh: Send + Debug{
    fn draw(&mut self);
}

#[derive(Clone, Debug)]
pub struct MeshRef{
    mesh: Arc<Mutex<Box<dyn Mesh>>>
}

impl MeshRef {
    pub fn lock<'a>(&'a self) -> MutexGuard<'a, Box<dyn Mesh>> {
        self.mesh.lock().unwrap()
    }
}

impl<T: Mesh + 'static> From<T> for MeshRef {
    fn from(mesh: T) -> Self {
        Self {
            mesh: Arc::new(Mutex::new(Box::new(mesh))),
        }
    }
}
