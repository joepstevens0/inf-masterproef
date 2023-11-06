use patutil::{
    mesh::{self, MeshRef, ModelRef},
    Color, Matf4, Vecf3,
};

use crate::util::{rot_from_dir, scale_from_size, translation_from_pos};

#[derive(Debug)]
pub struct Cone {
    model: ModelRef,

    length: f32,
    width_base: f32,
    width_tip: f32,
    pos: Vecf3,
    dir: Vecf3,

    scene: mesh::SceneRef,

    model_owner: bool
}

impl Cone {
    pub fn new(
        cylinder_mesh: MeshRef,
        mut scene: mesh::SceneRef,
        length: f32,
        width_base: f32,
        width_tip: f32,
        color: Color,
        id: u32,
    ) -> Self {
        let model: ModelRef = cylinder_mesh.into();

        // set cone color
        model.lock().set_color(color);
        // set cone id
        model.lock().set_id(id);

        // add cone to scene
        scene.controller().add_model(model.clone());

        let mut this = Self {
            model,
            length,
            width_base,
            width_tip,
            pos: Vecf3::default(),
            dir: Vecf3::default(),
            scene,
            model_owner: true
        };
        this.update_matrix();
        this
    }

    fn update_matrix(&mut self) {
        let mut model_mat = Matf4::new();

        // create model matrix that closes tip to width_tip
        const MIN_TIP_WIDTH: f32 = 0.000001;
        if self.width_tip < MIN_TIP_WIDTH {
            self.width_tip = MIN_TIP_WIDTH;
        }
        let divide_value = self.width_base / self.width_tip;
        model_mat.set(1, 3, divide_value - 1.); // divide x, y and z by (divide_value*y + 1)
        model_mat.set(1, 1, divide_value); // recorrect y value

        // update cone scale
        let scale = scale_from_size(&Vecf3::new(self.width_base, self.length, self.width_base));
        model_mat = scale * model_mat;

        // apply rotation
        let rot = rot_from_dir(&self.dir);
        model_mat = rot * model_mat;

        // apply translation
        let translate = translation_from_pos(&self.pos);
        model_mat = translate * model_mat;

        self.model.lock().set_model_mat(model_mat);
    }

    pub fn length(&self) -> f32 {
        self.length
    }

    pub fn width_base(&self) -> f32 {
        self.width_base
    }

    pub fn width_tip(&self) -> f32 {
        self.width_tip
    }

    pub fn set_width_base(&mut self, width_base: f32) {
        self.width_base = width_base;
        self.update_matrix();
    }

    pub fn set_width_tip(&mut self, width_tip: f32) {
        self.width_tip = width_tip;
        self.update_matrix();
    }

    pub fn set_length(&mut self, length: f32) {
        self.length = length;
        self.update_matrix();
    }

    pub fn set_pos(&mut self, pos: Vecf3) {
        self.pos = pos;
        self.update_matrix();
    }

    pub fn set_dir(&mut self, dir: Vecf3) {
        self.dir = dir;
        self.update_matrix();
    }

    pub fn set_color(&mut self, color: Color) {
        self.model.lock().set_color(color);
    }
}

impl Clone for Cone {
    fn clone(&self) -> Self {
        Self {
            model: self.model.clone(),
            length: self.length.clone(),
            width_base: self.width_base.clone(),
            width_tip: self.width_tip.clone(),
            pos: self.pos.clone(),
            dir: self.dir.clone(),
            scene: self.scene.clone(),
            model_owner: false
        }
    }
}

impl Drop for Cone {
    fn drop(&mut self) {
        if self.model_owner{
            self.scene.controller().remove_model(&self.model);
        }
    }
}
