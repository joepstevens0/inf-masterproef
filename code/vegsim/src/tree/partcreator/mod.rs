use patutil::{mesh::{MeshRef, SceneRef, ModelRef}, Color};

use crate::util::{create_cone_mesh, create_plane_mesh};

use super::{cone::Cone};

#[derive(Debug, Clone)]
pub struct PartCreator {
    cone: MeshRef,
    plane: MeshRef,
    scene: SceneRef
}

impl PartCreator {
    pub fn new(scene: SceneRef) -> Self {

        let cone = create_cone_mesh(1., 1.);

        let plane = create_plane_mesh();
        
        Self { cone, plane, scene }
    }

    pub fn new_cone(&self, length: f32, width_base: f32, width_tip: f32, color: Color, id: u32) -> Cone{
        Cone::new(self.cone.clone(), self.scene.clone(), length, width_base, width_tip, color, id)
    }
    pub fn new_plane(&mut self) -> ModelRef{
        let model: ModelRef = self.plane.clone().into();
        self.scene.controller().add_model(model.clone());
        model
    }

    pub fn clear(&mut self){
        self.scene.controller().clear();
    }

}
