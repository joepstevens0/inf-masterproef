mod glmesh;
use std::ffi::c_void;

pub use glmesh::*;

use patutil::Pos;
use patutil::camera::CamRef;
use patutil::mesh;
use patutil::Rect;
use patutil::{Matf4, Size};

use crate::FrameBuffer;

use super::ShaderProgram;

pub struct MeshRender {
    shader: ShaderProgram,
    picking_shader: ShaderProgram,
    picking_buffer: FrameBuffer,
    size: Size,
}

impl MeshRender {
    pub fn new(size: Size) -> Self {
        // create program
        let shader = ShaderProgram::new(
            include_str!("../shaders/mesh.vert"),
            include_str!("../shaders/mesh.frag"),
        )
        .unwrap();

        // picking
        let mut picking_buffer = FrameBuffer::new(size);
        let picking_shader = ShaderProgram::new(
            include_str!("../shaders/mesh.vert"),
            include_str!("../shaders/mesh_picking.frag"),
        )
        .unwrap();
        picking_buffer.add_color_tex(gl::R32UI, gl::RED_INTEGER, gl::UNSIGNED_INT);

        let this = Self {
            shader,
            picking_shader,
            size,
            picking_buffer
        };
        this
    }

    pub fn resize(&mut self, size: Size) {
        self.picking_buffer = FrameBuffer::new(size);
        self.picking_buffer
            .add_color_tex(gl::R32UI, gl::RED_INTEGER, gl::UNSIGNED_INT);
        self.size = size;
    }

    pub fn draw(&self, screen_size: Size, rect: Rect, scene: mesh::SceneRef) {
        self.shader.set_used();
        unsafe {
            gl::Enable(gl::BLEND);
            gl::Enable(gl::MULTISAMPLE); 
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
            gl::Viewport(rect.x, rect.y, rect.w as i32, rect.h as i32);
        }

        Self::bind_camera(&self.shader, scene.lock().camera(), rect);
        for model in scene.lock().models(){
            // bind model
            self.shader.bind_uniform("uModel", &model.lock().model_mat());
            self.shader.bind_uniform("uColor", &model.lock().color());

            // draw mesh
            model.lock().draw();
        }

        unsafe{
            gl::Viewport(0,0, screen_size.w as i32, screen_size.h as i32);
        }
    }

    pub fn picking(&self,pos: Pos, screen_size: Size, rect: Rect, scene: mesh::SceneRef) -> Option<u32> {
        self.picking_buffer.bind();
        self.picking_shader.set_used();
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Viewport(rect.x, rect.y, rect.w as i32, rect.h as i32);
            gl::ClearColor(0., 0., 0., 0.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        Self::bind_camera(&self.picking_shader, scene.lock().camera(), rect);
        for model in scene.lock().models(){
            // bind model
            self.picking_shader.bind_uniform("uModel", &model.lock().model_mat());
            self.picking_shader.bind_uniform("uID", &model.lock().id());

            // draw mesh
            model.lock().draw();
        }

        let mut id = [0u32];
        unsafe {
            gl::Flush();
            gl::ReadBuffer(gl::COLOR_ATTACHMENT0);
            gl::ReadPixels(
                pos.x,
                self.size.h as i32 - pos.y,
                1,
                1,
                gl::RED_INTEGER,
                gl::UNSIGNED_INT,
                id.as_mut_ptr() as *mut c_void,
            );
        }

        FrameBuffer::unbind();
        unsafe{
            gl::Viewport(0,0, screen_size.w as i32, screen_size.h as i32);
        }

        return (id[0] > 0).then(|| id[0]);
    }

    fn bind_camera(shader: &ShaderProgram, mut cam: CamRef, rect: Rect) {
        let cam = cam.lock();

        let mut view = Matf4::new();
        let mut proj = Matf4::new();

        // calc view
        let cam_pos = nalgebra_glm::vec3(cam.pos().x, cam.pos().y, cam.pos().z);
        let cam_target = nalgebra_glm::vec3(
            cam_pos.x + cam.front().x,
            cam_pos.y + cam.front().y,
            cam_pos.z + cam.front().z,
        );
        let cam_up = nalgebra_glm::vec3(0., 1., 0.);
        let m: nalgebra_glm::Mat4 = nalgebra_glm::look_at_lh(&cam_pos, &cam_target, &cam_up);
        view.data.copy_from_slice(m.data.as_slice());

        // calc projection
        let m: nalgebra_glm::Mat4 =
            nalgebra_glm::perspective_lh(rect.w as f32 / rect.h as f32, 3.14 / 2.0, 0.1, 1000.0);
        proj.data = m.data.as_slice().try_into().unwrap();
        proj.data.copy_from_slice(m.data.as_slice());

        shader.bind_uniform("uView", &view);
        shader.bind_uniform("uProj", &proj);

        // let cam = cam.lock();
        // shader.bind_uniform("uCamera.pos", &cam.pos());
        // shader.bind_uniform("uCamera.front", &cam.front());
        // shader.bind_uniform("uCamera.right", &cam.right());
        // shader.bind_uniform("uCamera.up", &cam.up());
    }
}
