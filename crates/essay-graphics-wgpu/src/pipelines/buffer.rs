use std::{marker::PhantomData, mem, num::NonZero};

use bytemuck::Pod;

use crate::render::render::RenderWgpu;

pub struct VertexBuffer<T: Pod> {
    buffer: wgpu::Buffer,
    len: usize,
    offset: usize,

    marker: PhantomData<T>,
}

impl<T: Pod> VertexBuffer<T> {
    pub fn new(device: &wgpu::Device, len: usize) -> Self {
        Self {
            buffer: create_vertex_buffer::<T>(device, len),
            len,
            offset: 0,
            marker: Default::default(),
        }
    }

    pub fn expand(&mut self, wgpu: &mut RenderWgpu, len: usize) -> bool {
        if self.offset + len < self.len {
            return false;
        }

        let mut new_len = self.len;
        while new_len < self.offset + len {
            new_len += 1024;
        }

        self.buffer = create_vertex_buffer::<T>(wgpu.device, new_len);
        self.len = new_len;
        self.offset = 0;

        true
    }

    pub fn write(&mut self, wgpu: &RenderWgpu, data: &[T]) -> (usize, usize) {
        assert!(self.offset + data.len() < self.len, "expand not implemented");

        let offset = self.offset;
        self.offset += data.len();

        let stride = mem::size_of::<T>();

        if let Some(mut view) = wgpu.queue.write_buffer_with(
            &mut self.buffer,
            (offset * stride) as u64,
            NonZero::new((data.len() * stride) as u64).unwrap(),
        ) {
            view.copy_from_slice(
                bytemuck::cast_slice(data)
            );
        }

        (offset, offset + data.len())
    }

    pub fn buffer_slice(&self, start: usize, end: usize) -> wgpu::BufferSlice {
        let stride = mem::size_of::<T>();

        self.buffer.slice(
            (stride * start) as u64..(stride * end) as u64
        )
    }

    pub fn clear(&mut self) {
        self.offset = 0;
    }
}

fn create_vertex_buffer<T>(
    device: &wgpu::Device,
    len: usize,
) -> wgpu::Buffer {
    let size = (mem::size_of::<T>() * len) as u64;

    device.create_buffer(
        &wgpu::BufferDescriptor {
        label: None,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        size,
        mapped_at_creation: false,
    })
}

pub(super) fn write_buffer<T: Pod>(
    wgpu: &RenderWgpu, 
    buffer: &mut wgpu::Buffer,
    data: &[T], 
    offset: usize,
) {
    let stride = mem::size_of::<T>();

    if let Some(mut view) = wgpu.queue.write_buffer_with(
        buffer,
        (offset * stride) as u64,
        NonZero::new((data.len() * stride) as u64).unwrap(),
    ) {
        view.copy_from_slice(
            bytemuck::cast_slice(data)
        );
    }
}
