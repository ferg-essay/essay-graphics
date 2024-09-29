use crate::TextureId;

pub struct Shape {
    vertices: Vec<Vertex2d>,
    texture: TextureId,
}

impl Shape {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            texture: TextureId::none(),
        }
    }

    #[inline]
    pub fn vertex(&mut self, vertex: [f32; 2], tex_uv: [f32; 2]) -> VertexId {
        let id = VertexId(self.vertices.len());

        self.vertices.push(Vertex2d {
            vertex: vertex,
            tex_uv: tex_uv,
        });

        id
    }

    #[inline]
    pub fn vertices(&self) -> &Vec<Vertex2d> {
        &self.vertices
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
pub struct ShapeId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexId(usize);

#[derive(Debug, Clone)]
pub struct Vertex2d {
    vertex: [f32; 2],
    tex_uv: [f32; 2],
}

impl Vertex2d {
    #[inline]
    pub fn vertex(&self) -> &[f32; 2] {
        &self.vertex
    }

    #[inline]
    pub fn tex_uv(&self) -> &[f32; 2] {
        &self.tex_uv
    }
}
