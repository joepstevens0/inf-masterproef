use std::ffi::c_void;

use gl::types::{GLenum, GLuint};
use patutil::Size;

pub struct Texture {
    texture: GLuint,
    size: Size,
    format: GLenum,
    internal_format: GLenum,
    tex_type: GLenum
}

impl Texture {
    pub fn new(
        size: Size,
        internal_format: u32,
        format: GLenum,
        tex_type: GLenum,
        data: *const c_void
    ) -> Self {
        let mut texture = GLuint::default();
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            let level = 0;
            let border = 0;
            gl::TexImage2D(
                gl::TEXTURE_2D,
                level,
                internal_format as i32,
                size.w as i32,
                size.h as i32,
                border,
                format,
                tex_type,
                data,
            );

            // set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Self { texture, size, internal_format, format, tex_type }
    }

    pub fn update_data(&self, data: *const c_void){
        unsafe{
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            let level = 0;
            let border = 0;
            gl::TexImage2D(
                gl::TEXTURE_2D,
                level,
                self.internal_format as i32,
                self.size.w as i32,
                self.size.h as i32,
                border,
                self.format,
                self.tex_type,
                data,
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn bind(&self, slot: u32){
        unsafe{
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }

    pub fn get(&self) -> u32{
        return self.texture;
    }
}

impl Drop for Texture{
    fn drop(&mut self) {
        unsafe{
            gl::DeleteTextures(1, &self.texture);
        }
    }
}