use std::ops::Index;

use crate::{Color, TextureId};

pub struct Form {
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
    texture: TextureId,
}

impl Form {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            texture: TextureId::none(),
        }
    }

    #[inline]
    pub fn vertex(&mut self, vertex: [f32; 3], tex_uv: [f32; 2]) -> VertexId {
        let id = VertexId(self.vertices.len());

        self.vertices.push(Vertex {
            vertex: vertex,
            tex_uv: tex_uv,
        });

        id
    }

    #[inline]
    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn triangle(&mut self, triangle: impl Into<Triangle>) {
        let triangle = triangle.into();

        self.triangles.push(triangle);
    }

    #[inline]
    pub fn triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }

    #[inline]
    pub fn texture(&mut self, texture: TextureId) {
        self.texture = texture;
    }

    #[inline]
    pub fn get_texture(&self) -> TextureId {
        self.texture
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FormId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexId(usize);

#[derive(Debug, Clone)]
pub struct Vertex {
    vertex: [f32; 3],
    tex_uv: [f32; 2],
}

impl Vertex {
    #[inline]
    pub fn vertex(&self) -> &[f32; 3] {
        &self.vertex
    }

    #[inline]
    pub fn tex_uv(&self) -> &[f32; 2] {
        &self.tex_uv
    }
}

#[derive(Debug, Clone)]
pub struct Triangle {
    vertices: [usize; 3],
}

impl Triangle {
    #[inline]
    fn new(value: [usize; 3]) -> Self {
        Self {
            vertices: value
        }
    }
}

impl Index<usize> for Triangle {
    type Output = usize;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.vertices[index]
    }
}

impl From<[usize; 3]> for Triangle {
    fn from(value: [usize; 3]) -> Self {
        Triangle::new(value)
    }
}

impl From<[VertexId; 3]> for Triangle {
    fn from(value: [VertexId; 3]) -> Self {
        Triangle::new([value[0].0, value[1].0, value[2].0])
    }
}