use std::{
    fmt::Debug,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::{Matf4, Color};

use super::{MeshRef, Mesh};

#[derive(Debug, Clone)]
pub struct Model {
    mesh: MeshRef,
    model: Matf4,
    color: Color,
    id: u32
}

impl Model {
    pub fn new(mesh: MeshRef, model: Matf4, color: Color) -> Self {
        Self { mesh, model,color, id:0 }
    }

    pub fn mesh(&self) -> &MeshRef{
        &self.mesh
    }
    pub fn model_mat(&self) -> Matf4{
        self.model
    }
    pub fn set_model_mat(&mut self, m:Matf4){
        self.model = m;
    }

    pub fn draw(&mut self){
        self.mesh.lock().draw();
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_id(&mut self, id: u32){
        self.id = id;
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Clone, Debug)]
pub struct ModelRef {
    model: Arc<Mutex<Model>>,
}

impl ModelRef {
    pub fn lock<'a>(&'a self) -> MutexGuard<'a, Model> {
        self.model.lock().unwrap()
    }
}

impl From<MeshRef> for Model {
    fn from(mesh: MeshRef) -> Self {
        Self {
            mesh,
            model: Matf4::new(),
            color: Color::default(),
            id: 0
        }
    }
}

impl From<Model> for ModelRef {
    fn from(model: Model) -> Self {
        Self {
            model: Arc::new(Mutex::new(model)),
        }
    }
}

impl<T: Mesh + 'static> From<T> for ModelRef {
    fn from(mesh: T) -> Self {
        let meshref: MeshRef = mesh.into();
        return ModelRef::from(meshref);
    }
}


impl From<MeshRef> for ModelRef {
    fn from(mesh: MeshRef) -> Self {
        Self {
            model: Arc::new(Mutex::new(mesh.into())),
        }
    }
}

impl PartialEq for ModelRef{
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.model, &other.model)
    }
}