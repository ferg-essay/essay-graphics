use std::{collections::HashMap, ops::{Index, IndexMut}};

use essay_graphics_api::{Hatch, TextureId};

use super::texture_store::TextureCache;


pub fn init_hatch(
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    textures: &mut TextureCache
) -> HashMap<Hatch, TextureId> {
    let mut hatch_map = HashMap::new();

    hatch_map.insert(
        Hatch::Vertical, 
        hatch_vertical(device, queue, textures)
    );
    hatch_map.insert(
        Hatch::Horizontal, 
        hatch_horizontal(device, queue, textures)
    );

    hatch_map
}

fn hatch_vertical(
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    textures: &mut TextureCache
) -> TextureId {
    let mut builder = HatchBuilder::new(64, 64);

    for j in 0..64 {
        builder[(0, j)] = 255;
        builder[(1, j)] = 255;
        builder[(2, j)] = 255;
        builder[(3, j)] = 255;

        builder[(32, j)] = 255;
        builder[(33, j)] = 255;
        builder[(34, j)] = 255;
        builder[(35, j)] = 255;
    }

    builder.add_to(device, queue, textures)

}

fn hatch_horizontal(
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    textures: &mut TextureCache
) -> TextureId {
    let mut builder = HatchBuilder::new(64, 64);

    for j in 0..64 {
        builder[(j, 0)] = 255;
        builder[(j, 1)] = 255;
        builder[(j, 2)] = 255;
        builder[(j, 3)] = 255;

        builder[(j, 32)] = 255;
        builder[(j, 33)] = 255;
        builder[(j, 34)] = 255;
        builder[(j, 35)] = 255;
    }

    //for j in 17..32 {
    //    builder[(j, 0)] = 255;
    //}

    builder.add_to(device, queue, textures)
}



struct HatchBuilder {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl HatchBuilder {
    fn new(width: usize, height: usize) -> Self {
        let mut vec = Vec::new();

        vec.resize((width * height) as usize, 0);

        Self {
            data: vec,
            width,
            height
        }
    }

    fn add_to(
        self, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        textures: &mut TextureCache
    ) -> TextureId {
        let mut rgba = Vec::<u8>::new();

        for v in &self.data {
            rgba.push(0xff);
            rgba.push(0xff);
            rgba.push(0xff);
            rgba.push(*v);
        }

        textures.add_rgba_u8(
            device, 
            queue, 
            self.width as u32, 
            self.height as u32, 
            rgba.as_slice()
        )
    }

    fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    fn _set(&mut self, x: usize, y: usize, v: u8) -> &mut Self {
        self.data[x + y * self.width] = v;

        self
    }
}

impl Index<(usize, usize)> for HatchBuilder {
    type Output = u8;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 + index.1 * self.width]
    }
}

impl IndexMut<(usize, usize)> for HatchBuilder {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 + index.1 * self.width]
    }
}
