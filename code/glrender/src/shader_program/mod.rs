use std::ffi::CString;

use patutil::{Size, Color, Vecf4, Pos, Matf4};

use self::shader::Shader;

use super::create_whitespace_cstring_with_len;

mod shader;

pub struct ShaderProgram {
    id: u32,
}

pub trait UniformObject {
    fn bind(&self, location: i32);
}


impl ShaderProgram {
    pub fn bind_uniform(&self, name: &str, data: &impl UniformObject) {
        let name = CString::new(name).unwrap();
        let loc = unsafe { gl::GetUniformLocation(self.id, name.as_ptr()) };
        data.bind(loc);
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
    pub fn new(vert_source: &str, frag_source: &str) -> Result<Self, String> {
        let mut shaders: Vec<Shader> = Vec::new();
        // create shaders
        shaders.push(Shader::from_vert_source(vert_source)?);
        shaders.push(Shader::from_frag_source(frag_source)?);

        let id = unsafe { gl::CreateProgram() };

        for shader in &shaders {
            unsafe {
                gl::AttachShader(id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(id);
        }
        for shader in shaders {
            unsafe {
                gl::DetachShader(id, shader.id());
            }
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Self { id })
    }

    pub fn _id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}


impl UniformObject for bool {
    fn bind(&self, location: i32) {
        unsafe {
            gl::Uniform1i(location, *self as i32);
        }
    }
}

impl UniformObject for [f32; 4] {
    fn bind(&self, location: i32) {
        unsafe {
            gl::Uniform4f(location, self[0], self[1], self[2], self[3]);
        }
    }
}

impl UniformObject for [f32; 3] {
    fn bind(&self, location: i32) {
        unsafe {
            gl::Uniform3f(location, self[0], self[1], self[2]);
        }
    }
}

impl UniformObject for [f32; 2] {
    fn bind(&self, location: i32) {
        unsafe {
            gl::Uniform2f(location, self[0], self[1]);
        }
    }
}

impl UniformObject for [u32; 2] {
    fn bind(&self, location: i32) {
        unsafe {
            gl::Uniform2ui(location, self[0], self[1]);
        }
    }
}

impl UniformObject for [i32; 2] {
    fn bind(&self, location: i32) {
        unsafe {
            gl::Uniform2i(location, self[0], self[1]);
        }
    }
}


impl UniformObject for i32 {
    fn bind(&self, location: i32) {
        unsafe {
            gl::Uniform1i(location, *self);
        }
    }
}

impl UniformObject for u32 {
    fn bind(&self, location: i32) {
        unsafe {
            gl::Uniform1ui(location, *self);
        }
    }
}

impl UniformObject for Size {
    fn bind(&self, location: i32) {
        unsafe {
            gl::Uniform2ui(location, self.w, self.h);
        }
    }
}

impl UniformObject for Color {
    fn bind(&self, location: i32) {
        self.to_float().bind(location);
    }
}

impl UniformObject for Vecf4 {
    fn bind(&self, location: i32) {
        self.to_vec().bind(location);
    }
}

impl UniformObject for Pos {
    fn bind(&self, location: i32) {
        unsafe {
            gl::Uniform2i(location, self.x, self.y);
        }
    }
}


impl UniformObject for f32 {
    fn bind(&self, location: i32) {
        unsafe {
            gl::Uniform1f(location, *self);
        }
    }
}

impl UniformObject for Matf4 {
    fn bind(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4fv(location, 1, 0, self.data.as_ptr());
        }
    }
}
