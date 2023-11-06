use std::{mem::size_of, ffi::c_void};
use gl::types::{GLuint, GLsizeiptr, GLvoid, GLint, self};


pub struct VertexBuffer{
    _vbo: GLuint,
    vao: GLuint,
    total_indices: GLint,
    draw_type: types::GLenum,
    vertex_size: u32
}

impl VertexBuffer{

    pub fn draw(&self){
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(
                self.draw_type, // mode
                0, // starting index in the enabled arrays
                self.total_indices // number of indices to be rendered
            );
        }
    }

    pub fn from_rect() -> Result<Self, String>{
        let vertices: Vec<f32> = vec![
            -1., -1.,  0.,0., // bottom left
            -1., 1.,   0.,1.,// top left 
            1.,  1.,   1.,1.,// top  right

            -1., -1.,  0.,0.,// bottom left
            1.,  1.,   1.,1.,// top  right
            1., -1.,     1.,0.// bottom right
        ];

        let vbo = Self::create_vbo(&vertices)?;
        let vao = Self::create_vao(vbo, &[2,2].into())?;

        Ok(VertexBuffer{_vbo: vbo, vao, total_indices: 6, draw_type: gl::TRIANGLES, vertex_size: 4})
    }

    pub fn from_line() -> Result<Self, String>{
        let vertices: Vec<f32> = vec![
            0., 0.,  0.,0., // left
            1., 0.,   1.,0.,// right 
        ];

        let vbo = Self::create_vbo(&vertices)?;
        let vao = Self::create_vao(vbo, &[2,2].into())?;

        Ok(VertexBuffer{_vbo: vbo, vao, total_indices: 2, draw_type: gl::LINE, vertex_size: 4})
    }

    pub fn from_rect_3d() -> Result<Self, String>{
        let vertices: Vec<f32> = vec![
            -1., -1., 0.,  0.,0., // bottom left
            -1., 1., 0.,   0.,1.,// top left 
            1.,  1., 0.,   1.,1.,// top  right

            -1., -1., 0.,  0.,0.,// bottom left
            1.,  1., 0.,   1.,1.,// top  right
            1., -1., 0.,     1.,0.// bottom right
        ];

        return Self::from_vertices_3d(&vertices);
    }

    /// Create vertexbuffer from list of vertices
    /// # Arguments:
    /// * vertices - List of vertices in format: x, y, z, texture_x, texture_y
    pub fn from_vertices_3d(vertices: &Vec<f32>) -> Result<Self, String>{
        Self::from_vertices(vertices, [3,3,2].into(), gl::TRIANGLES)
    }

    pub fn from_vertices(vertices: &Vec<f32>, attribute_sizes: Vec<GLint>, draw_type: types::GLenum) -> Result<Self, String>{
        let vbo = Self::create_vbo(vertices)?;
        let vao = Self::create_vao(vbo, &attribute_sizes)?;

        let mut vertex_size: u32 = 0;
        for size in attribute_sizes{
            vertex_size += size as u32;
        }

        Ok(VertexBuffer{_vbo: vbo, vao, total_indices: vertices.len()as i32/vertex_size as i32, draw_type ,vertex_size})
    }
    
    fn create_vao(vbo: GLuint, attribute_sizes: &Vec<GLint>)-> Result<GLuint, String>{
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let mut stride = 0;
            for size in attribute_sizes{
                stride += size* size_of::<f32>() as GLint;
            }
            let mut offset: GLint = 0;
            for (index, size) in attribute_sizes.into_iter().enumerate(){
                gl::VertexAttribPointer(
                    index as u32,         // index of the generic vertex attribute ("layout (location = 0)")
                    *size,         // the number of components per generic vertex attribute
                    gl::FLOAT, // data type
                    gl::FALSE, // normalized (int-to-float conversion)
                    stride, // stride (byte offset between consecutive attributes)
                    offset as *const c_void,                                     // offset of the first component
                );
                gl::EnableVertexAttribArray(index as u32);

                offset += size * size_of::<f32>() as GLint;
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        };
        Ok(vao)
    }

    fn create_vbo(vertices: &Vec<f32>) -> Result<GLuint, String>{
        let mut vbo: GLuint= 0;
        unsafe{
            gl::GenBuffers(1, &mut vbo);

            // fill buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (vertices.len() * size_of::<f32>()) as GLsizeiptr, // size of data in bytes
                vertices.as_ptr() as *const GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        };
        Ok(vbo)
    }

    pub fn update(&mut self, vertices: &Vec<f32>) -> Result<(),String>{
        let new_total = vertices.len() as u32/self.vertex_size;
        if new_total > self.vertex_size{
            unsafe{
                gl::DeleteBuffers(1, &self._vbo);
                self._vbo = Self::create_vbo(vertices)?;
            }
        }else {
            unsafe{
                // fill buffer
                gl::BindBuffer(gl::ARRAY_BUFFER, self._vbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER, // target
                    (vertices.len() * size_of::<f32>()) as GLsizeiptr, // size of data in bytes
                    vertices.as_ptr() as *const GLvoid, // pointer to data
                    gl::STATIC_DRAW, // usage
                );
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            };
        }
        self.total_indices = new_total as i32;
        return Ok(());
    }
}

impl Drop for VertexBuffer{
    fn drop(&mut self) {
        unsafe{
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self._vbo);
        }
    }
}