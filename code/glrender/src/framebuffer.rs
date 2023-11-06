use std::ffi::c_void;

use gl::types::{GLenum, GLuint, GLint};
use patutil::Size;

use super::Texture;

pub struct FrameBuffer {
    framebuffer: GLuint,
    size: Size,
    color_textures: Vec<Texture>,
    render_buffer: Option<u32>,
}

impl FrameBuffer {
    pub fn new(size: Size) -> Self {
        let mut framebuffer: GLuint = 0;
        unsafe {
            gl::CreateFramebuffers(1, &mut framebuffer);

            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE{
                panic!("Failed to create framebuffer");
            }

            Self::unbind();
        }
        Self {
            framebuffer,
            size,
            color_textures: vec![],
            render_buffer: None,
        }
    }

    pub fn add_color_tex(&mut self, internal_format: u32, format: GLenum, tex_type: GLenum) -> usize {
        let color_tex = Texture::new(self.size, internal_format, format, tex_type, std::ptr::null());

        let index = self.color_textures.len();
        self.bind();

        unsafe {
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0 + index as u32,
                gl::TEXTURE_2D,
                color_tex.get(),
                0,
            );
        }

        self.color_textures.push(color_tex);

        Self::unbind();
        return index;
    }

    pub fn add_depth_buffer(&mut self) {
        self.bind();

        // render buffers cannot be read from, change to textures if needed
        let mut render_buffer = 0u32;
        unsafe {
            gl::CreateRenderbuffers(1, &mut render_buffer);
            gl::BindRenderbuffer(gl::RENDERBUFFER, render_buffer);
            gl::RenderbufferStorage(
                gl::RENDERBUFFER,
                gl::DEPTH24_STENCIL8,
                self.size.w as i32,
                self.size.h as i32,
            );
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                render_buffer,
            );

            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
        }

        self.render_buffer = Some(render_buffer);
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn get_color_tex(&self, index: usize)-> &Texture{
        return &self.color_textures[index];
    }

    pub fn save_viewport(file_path: String) -> Result<(), ()>{
        // get framebuffer size
        let mut dims: [GLint;4] = [0;4];
        unsafe{
            gl::GetIntegerv(gl::VIEWPORT, dims.as_mut_ptr());
        }
        let vp_x = dims[0];
        let vp_y = dims[1];
        let vp_width = dims[2];
        let vp_height = dims[3];

        let mut image_buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(vp_width as u32, vp_height as u32);
        image_buffer.fill(255);
        
        unsafe{
            gl::ReadBuffer(gl::BACK);
            gl::ReadPixels(vp_x, vp_y, vp_width, vp_height, gl::RGB, gl::UNSIGNED_BYTE, image_buffer.as_mut_ptr() as *mut c_void);
        }

        image::imageops::flip_vertical_in_place(&mut image_buffer);

        if let Err(_) = image_buffer.save(file_path){
            return Err(());
        }
        Ok(())
    }
}

impl Drop for FrameBuffer{
    fn drop(&mut self) {
        unsafe{
            gl::DeleteFramebuffers(1, &self.framebuffer);
        }
    }
}