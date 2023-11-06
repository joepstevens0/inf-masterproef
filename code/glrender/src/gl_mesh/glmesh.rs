use std::fmt::Debug;

use crate::{Texture, VertexBuffer};
use patutil::{mesh::Mesh};

pub struct GLMesh {
    vbo: Option<VertexBuffer>,
    texture: Option<Texture>,
    vertices: Vec<f32>,
}

impl GLMesh {
    pub fn new(path: String) -> Self {
        let (models, _materials) = tobj::load_obj(&path, &tobj::LoadOptions::default())
            .expect(&format!("Failed to open obj {:?}", path));

        let mut vertices = vec![];
        for model in models {
            for i in 0..model.mesh.indices.len() {
                // add pos
                let pos_offset = (3 * model.mesh.indices[i]) as usize;
                vertices.push(model.mesh.positions[pos_offset]);
                vertices.push(model.mesh.positions[pos_offset + 1]);
                vertices.push(model.mesh.positions[pos_offset + 2]);

                // add normal
                let normal_offset = (3 * model.mesh.normal_indices[i]) as usize;
                vertices.push(model.mesh.normals[normal_offset]);
                vertices.push(model.mesh.normals[normal_offset + 1]);
                vertices.push(model.mesh.normals[normal_offset + 2]);

                // add tex coord
                let tex_coord_offset = (2 * model.mesh.texcoord_indices[i]) as usize;
                vertices.push(model.mesh.texcoords[tex_coord_offset]);
                vertices.push(1. - model.mesh.texcoords[tex_coord_offset + 1]);
            }
        }

        Self {
            vbo: None,
            texture: None,
            vertices
        }
    }

    // list of pos (3 f32), normal (3 f32), texturecoords (2 f32)
    pub fn from_vertices(vertices: Vec<f32>) -> Self{
        Self {
            vbo: None,
            texture: None,
            vertices
        }
    }

    pub fn new_plane() -> Self {
        let vertices: Vec<f32> = vec![
            -1., -1., 0., 0., 1., 0., 0., 0., // bottom left
            -1., 1., 0., 0., 1., 0., 0., 1., // top left
            1., 1., 0., 0., 1., 0., 1., 1., // top  right
            -1., -1., 0., 0., 1., 0., 0., 0., // bottom left
            1., 1., 0., 0., 1., 0., 1., 1., // top  right
            1., -1., 0., 0., 1., 0., 1., 0., // bottom right
        ];

        Self {
            vbo: None,
            texture: None,
            vertices
        }
    }

    fn init(&mut self) {
        // vertex buffer
        let vbo = VertexBuffer::from_vertices_3d(&self.vertices).unwrap();

        // create texture
        // let image = image::open("testfiles/viking_room.png")
        //     .expect("Failed to open img: testfiles/viking_room.png")
        //     .into_rgba8();
        // let texture = Texture::new(
        //     [image.width(), image.height()].into(),
        //     gl::RGBA,
        //     gl::RGBA,
        //     gl::UNSIGNED_BYTE,
        //     image.as_bytes().as_ptr() as *const _,
        // );

        self.vbo = Some(vbo);
        // self.texture = Some(texture);
    }
}

impl Mesh for GLMesh {
    fn draw(&mut self) {
        if let Some(vbo) = &self.vbo {
            if let Some(texture) = &self.texture {
                texture.bind(0);
            }
            vbo.draw();
        } else {
            self.init();
            self.draw();
        }
    }
}

impl Debug for GLMesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GLMesh")
            .field("vbo", &"<vbo_object>")
            .field("texture", &"<texture_object>")
            .field("vertices len:", &self.vertices.len())
            .finish()
    }
}
