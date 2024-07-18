use std::ops::Index;

use crate::Color;

pub struct Form {
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
    color: Color,
}

impl Form {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            color: Color::black(),
        }
    }

    #[inline]
    pub fn vertex(&mut self, vertex: impl Into<Vertex>) -> VertexId {
        let id = VertexId(self.vertices.len());

        self.vertices.push(vertex.into());

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
    pub fn color(&mut self, color: impl Into<Color>) {
        self.color = color.into();
    }

    #[inline]
    pub fn get_color(&self) -> Color {
        self.color
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FormId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexId(usize);

#[derive(Debug, Clone)]
pub struct Vertex {
    vertex: [f32; 3],
}

impl Vertex {
    #[inline]
    fn new(value: [f32; 3]) -> Self {
        Self {
            vertex: value
        }
    }
}

impl Index<usize> for Vertex {
    type Output = f32;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.vertex[index]
    }
}

//impl PartialEq for Vertex {
//    fn eq(&self, other: &Self) -> bool {
//        self.vertex == other.vertex
//    }
//}

impl From<[f32; 3]> for Vertex {
    fn from(value: [f32; 3]) -> Self {
        Vertex::new(value)
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