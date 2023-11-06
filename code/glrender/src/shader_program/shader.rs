use std::ffi::CString;

use crate::create_whitespace_cstring_with_len;



pub struct Shader{
    id: u32
}

impl Shader{
    pub fn id(&self) -> u32{
        self.id
    }
    pub fn from_vert_source(source: &str) -> Result<Shader, String>{
        Shader::from_source(source, gl::VERTEX_SHADER)
    }
    pub fn from_frag_source(source: &str) -> Result<Shader, String>{
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }
    pub fn from_source(source: &str, shader_type: u32)-> Result<Shader, String>{
        let id = Shader::shader_from_source(source, shader_type)?;
        Ok(Shader{id})
    }
    fn shader_from_source(source: &str, shader_type: u32) -> Result<u32, String> {

        let id = unsafe { gl::CreateShader(shader_type) };

        let source = CString::new(source).expect("Failed to transfrom str to CString");

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        };
        let mut success: i32 = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: i32 = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
            return Err(format!{"Failed to create shader:\n{}", error.to_string_lossy()});
        }

        Ok(id)
    }
}

impl Drop for Shader{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}