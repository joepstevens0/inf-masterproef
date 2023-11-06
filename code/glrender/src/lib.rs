use std::ffi::{c_void, CString};

mod shader_program;
pub use shader_program::*;
mod vertexbuffer;
pub use vertexbuffer::*;
mod gl_font_render;
pub use gl_font_render::*;
mod gl_csg;
pub use gl_csg::*;
mod gl_mesh;
pub use gl_mesh::*;
mod gl_points;
pub use gl_points::*;
mod framebuffer;
pub use framebuffer::*;
mod texture;
pub use texture::*;
pub mod window;

pub use gl;

use patutil::{csg::SceneRef, text::Text, Color, Pos, Rect, Render, Size, Line};

pub fn gl_init(loader: impl FnMut(&'static str) -> *const c_void) {
    gl::load_with(loader);

    let version = unsafe {
        let data = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    };
    println!("OpenGL version {}", version);
}

pub struct GLRender {
    shader_program: ShaderProgram,
    vbo_rect: VertexBuffer,
    _vbo_line: VertexBuffer,
    screen_size: Size,
    font_render: GLFontRender,
    csg_render: CSGRender,
    mesh_render: MeshRender,
    point_render: PointRender,
}

impl GLRender {
    pub fn new(screen_size: Size) -> GLRender {
        let v_source = include_str!("shaders/2d_shader.vert");
        let f_source = include_str!("shaders/2d_shader.frag");
        let shader_program = ShaderProgram::new(v_source, f_source).unwrap();

        let csg_render = CSGRender::new(screen_size);
        let mesh_render = MeshRender::new(screen_size);
        let point_render = PointRender::new(screen_size);

        let vbo_rect = VertexBuffer::from_rect().unwrap();
        let _vbo_line = VertexBuffer::from_line().unwrap();

        unsafe {
            gl::Viewport(0, 0, screen_size.w as i32, screen_size.h as i32);
        }

        return Self {
            shader_program,
            vbo_rect,
            _vbo_line,
            screen_size,
            font_render: GLFontRender::new(),
            csg_render,
            mesh_render,
            point_render,
        };
    }

    pub fn update_screen_size(&mut self, screen_size: Size) {
        self.screen_size = screen_size;
        unsafe {
            gl::Viewport(0, 0, screen_size.w as i32, screen_size.h as i32);
        }
        self.csg_render.resize(screen_size);
        self.mesh_render.resize(screen_size);
        self.point_render.resize(screen_size);
    }

    pub fn draw_texture(&self, rect: Rect, texture: &Texture, round_edge: bool) {
        self.shader_program.set_used();
        unsafe {
            gl::Enable(gl::BLEND);
            gl::Disable(gl::DEPTH_TEST);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        self.shader_program
            .bind_uniform("uOffset", &[rect.x, rect.y]);
        self.shader_program.bind_uniform("uSize", &[rect.w, rect.h]);
        self.shader_program.bind_uniform("uHasTex", &true);
        self.shader_program
            .bind_uniform("uColor", &Color::new(255, 255, 255, 255).to_float());
        self.shader_program
            .bind_uniform("uScreenSize", &self.screen_size);
        self.shader_program.bind_uniform(
            "uRoundEdge",
            match round_edge {
                true => &1.0f32,
                false => &0.0f32,
            },
        );
        texture.bind(0);
        self.vbo_rect.draw();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    pub fn mesh_picking(&self, pos: Pos, rect: Rect, mesh: patutil::mesh::SceneRef) -> Option<u32> {
        return self.mesh_render.picking(pos, self.screen_size, rect, mesh);
    }
}
impl Render for GLRender {
    fn draw_mesh(&self, rect: Rect, mesh: patutil::mesh::SceneRef) {
        self.mesh_render.draw(self.screen_size, rect, mesh);
    }

    fn draw_points(&self, rect: Rect, points: patutil::points::SceneRef) {
        self.point_render.draw(self.screen_size, rect, points);
    }

    fn draw_rect(&self, rect: Rect, color: Color, round_edge: bool) {
        self.shader_program.set_used();
        unsafe {
            gl::Enable(gl::BLEND);
            gl::Disable(gl::DEPTH_TEST);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        self.shader_program
            .bind_uniform("uOffset", &[rect.x, rect.y]);
        self.shader_program.bind_uniform("uSize", &[rect.w, rect.h]);
        self.shader_program.bind_uniform("uHasTex", &false);
        self.shader_program
            .bind_uniform("uColor", &color.to_float());
        self.shader_program
            .bind_uniform("uScreenSize", &self.screen_size);
        self.shader_program.bind_uniform(
            "uRoundEdge",
            match round_edge {
                true => &1.0f32,
                false => &0.0f32,
            },
        );
        self.vbo_rect.draw();
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    fn draw_line(&self, _line: Line, _color: Color) {
        todo!();
        // self.shader_program.set_used();
        // unsafe {
        //     gl::Enable(gl::BLEND);
        //     gl::Disable(gl::DEPTH_TEST);
        //     gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        // }
        // self.shader_program
        //     .bind_uniform("uOffset", &[rect.x, rect.y]);
        // self.shader_program.bind_uniform("uSize", &[rect.w, rect.h]);
        // self.shader_program.bind_uniform("uHasTex", &false);
        // self.shader_program
        //     .bind_uniform("uColor", &color.to_float());
        // self.shader_program
        //     .bind_uniform("uScreenSize", &self.screen_size);
        // // self.shader_program.bind_uniform(
        // //     "uRoundEdge",
        // //     match round_edge {
        // //         true => &1.0f32,
        // //         false => &0.0f32,
        // //     },
        // // );
        // self.vbo_rect.draw();
        // unsafe {
        //     gl::Enable(gl::DEPTH_TEST);
        // }
    }

    fn draw_text(&self, pos: Pos, text: &Text) {
        self.font_render.draw_text(self.screen_size, pos, text);
    }

    fn draw_csg(&self, rect: Rect, csg: SceneRef) {
        self.csg_render.draw(self.screen_size, rect, csg);
    }

    fn picking_csg(&self, rect: Rect, csg: SceneRef, mouse_pos: Pos) -> Option<u16> {
        self.csg_render.pick(self.screen_size, rect, csg, mouse_pos)
    }

    fn clear(&self) {
        unsafe {
            gl::ClearColor(0., 0., 0., 0.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
