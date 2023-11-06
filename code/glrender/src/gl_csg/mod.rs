use std::ffi::c_void;

use patutil::camera::CamRef;
use patutil::csg::SceneRef;
use patutil::Size;
use patutil::{Pos, Rect};

use self::bricktex::BrickTex;

use super::{FrameBuffer, ShaderProgram, VertexBuffer};

mod bricktex;

pub struct CSGRender {
    bricktex: BrickTex,
    shader: ShaderProgram,
    vbo: VertexBuffer,

    picking_buffer: FrameBuffer,
    picking_shader: ShaderProgram,
    size: Size,
}

impl CSGRender {
    pub fn new(size: Size) -> CSGRender {
        // create program
        let shader = ShaderProgram::new(
            include_str!("../shaders/csg.vert"),
            include_str!("../shaders/csg.frag"),
        )
        .unwrap();

        // vertex buffer
        let vbo = VertexBuffer::from_rect().unwrap();
        let bricktex = BrickTex::new(1024).unwrap();

        // picking
        let mut picking_buffer = FrameBuffer::new(size);
        picking_buffer.add_color_tex(gl::R16UI, gl::RED_INTEGER, gl::UNSIGNED_SHORT);
        let picking_shader = ShaderProgram::new(
            include_str!("../shaders/csg.vert"),
            include_str!("../shaders/csg_picking.frag"),
        )
        .unwrap();

        let this = CSGRender {
            shader,
            bricktex,
            vbo,
            picking_buffer,
            picking_shader,
            size,
        };
        this
    }

    pub fn resize(&mut self, size: Size) {
        self.picking_buffer = FrameBuffer::new(size);
        self.picking_buffer
            .add_color_tex(gl::R16UI, gl::RED_INTEGER, gl::UNSIGNED_SHORT);
        self.size = size;
    }

    pub fn draw(&self, screen_size: Size, rect: Rect, csg: SceneRef) {
        self.shader.set_used();
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        Self::bind_camera(&self.shader, csg.lock().camera());
        self.bricktex.bind(&self.shader, 1, csg);
        self.shader.bind_uniform("uOffset", &rect.pos());
        self.shader.bind_uniform("uSize", &rect.size());
        self.shader.bind_uniform("uScreenSize", &screen_size);
        self.vbo.draw();
    }

    pub fn pick(
        &self,
        screen_size: Size,
        rect: Rect,
        csg: SceneRef,
        mouse_pos: Pos,
    ) -> Option<u16> {
        let shader = &self.picking_shader;
        let framebuffer = &self.picking_buffer;

        framebuffer.bind();
        shader.set_used();

        unsafe {
            gl::ClearColor(0., 0., 0., 0.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        Self::bind_camera(shader, csg.lock().camera());
        self.bricktex.bind(&shader, 1, csg);
        shader.bind_uniform("uOffset", &rect.pos());
        shader.bind_uniform("uSize", &rect.size());
        shader.bind_uniform("uScreenSize", &screen_size);
        self.vbo.draw();

        let mut id = [0u16];
        unsafe {
            gl::ReadBuffer(gl::COLOR_ATTACHMENT0);
            gl::ReadPixels(
                mouse_pos.x,
                self.size.h as i32 - mouse_pos.y,
                1,
                1,
                gl::RED_INTEGER,
                gl::UNSIGNED_SHORT,
                id.as_mut_ptr() as *mut c_void,
            );
        }

        FrameBuffer::unbind();

        return (id[0] > 0).then(|| id[0] - 1);
    }

    fn bind_camera(shader: &ShaderProgram, mut cam: CamRef) {
        let cam = cam.lock();
        shader.bind_uniform("uCamera.pos", &cam.pos());
        shader.bind_uniform("uCamera.front", &cam.front());
        shader.bind_uniform("uCamera.right", &cam.right());
        shader.bind_uniform("uCamera.up", &cam.up());
    }
}
