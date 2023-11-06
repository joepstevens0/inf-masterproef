use patutil::camera::CamRef;
use patutil::points;
use patutil::Rect;
use patutil::{Matf4, Size};

mod glpointslist;
pub use glpointslist::*;

use super::ShaderProgram;

pub struct PointRender {
    shader: ShaderProgram,
    size: Size,
}

impl PointRender {
    pub fn new(size: Size) -> Self {
        // create program
        let shader = ShaderProgram::new(
            include_str!("../shaders/points.vert"),
            include_str!("../shaders/points.frag"),
        )
        .unwrap();

        let this = Self {
            shader,
            size,
        };
        this
    }

    pub fn resize(&mut self, size: Size) {
        self.size = size;
    }

    pub fn draw(&self, screen_size: Size, rect: Rect, scene: points::SceneRef) {
        self.shader.set_used();
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::PROGRAM_POINT_SIZE);
            gl::Viewport(rect.x, rect.y, rect.w as i32, rect.h as i32);
        }

        Self::bind_camera(&self.shader, scene.lock().camera(), rect);
        for list in scene.lock().points(){
            // draw mesh
            list.lock().draw();
        }

        unsafe{
            gl::Viewport(0,0, screen_size.w as i32, screen_size.h as i32);
        }
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
    }
}
