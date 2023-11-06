use std::fmt::Debug;


use crate::{VertexBuffer};
use patutil::{points::{PointsList, Point}};

pub struct GLPointsList {
    vbo: Option<VertexBuffer>,
    vertices: Vec<f32>,
}

impl GLPointsList {
    pub fn new(points: Vec<Point>) -> Self {
        let mut vertices = vec![];
        for point in points {
            // position
            vertices.push(point.pos.x);
            vertices.push(point.pos.y);
            vertices.push(point.pos.z);

            // color
            vertices.push(point.color.r as f32/255.);
            vertices.push(point.color.g as f32/255.);
            vertices.push(point.color.b as f32/255.);
            vertices.push(point.color.a as f32/255.);

            // size
            vertices.push(point.size);
        }

        Self {
            vbo: None,
            vertices
        }
    }

    fn init(&mut self) {
        // vertex buffer
        let vbo = VertexBuffer::from_vertices(&self.vertices, [3,4,1].into(), gl::POINTS).unwrap();

        self.vbo = Some(vbo);
    }
}

impl PointsList for GLPointsList {
    fn draw(&mut self) {
        if let Some(vbo) = &self.vbo {
            vbo.draw();
        } else {
            self.init();
            self.draw();
        }
    }

    fn update_points(&mut self, points: Vec<Point>){
        let mut vertices = vec![];
        for point in points {
            // position
            vertices.push(point.pos.x);
            vertices.push(point.pos.y);
            vertices.push(point.pos.z);

            // color
            vertices.push(point.color.r as f32/255.);
            vertices.push(point.color.g as f32/255.);
            vertices.push(point.color.b as f32/255.);
            vertices.push(point.color.a as f32/255.);

            // size
            vertices.push(point.size);
        }
        self.vertices = vertices;
        
        if self.vbo.is_some(){
            self.init();
        }

    }
}

impl Debug for GLPointsList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GLPointsList")
            .field("vbo", &"<vbo_object>")
            .field("texture", &"<texture_object>")
            .field("vertices len:", &self.vertices.len())
            .finish()
    }
}
