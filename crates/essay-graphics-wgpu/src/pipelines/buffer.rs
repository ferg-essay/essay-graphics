use std::{marker::PhantomData, mem, num::NonZero};

use bytemuck::Pod;
use wgpu::util::DeviceExt;

use crate::render::render::RenderWgpu;

pub struct VertexBuffer<T: Pod> {
    buffer: wgpu::Buffer,
    len: usize,
    offset: usize,

    stage: Vec<T>,

    marker: PhantomData<T>,
}

impl<T: Pod + Default> VertexBuffer<T> {
    pub fn new(device: &wgpu::Device, len: usize) -> Self {
        let mut vec = Vec::new();
        vec.resize(len, T::default());

        Self {
            buffer: create_vertex_buffer::<T>(device, len),
            len,
            offset: 0,
            stage: vec,
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

        self.stage.resize(new_len, T::default());

        true
    }

    pub fn write(&mut self, wgpu: &mut RenderWgpu, data: &[T]) -> (usize, usize) {
        assert!(self.offset + data.len() < self.len, "expand not implemented");

        let offset = self.offset;
        self.offset += data.len();

        let stride = mem::size_of::<T>();

        wgpu.write(
            &self.buffer, 
            bytemuck::cast_slice(data),
            (offset * stride) as u64,
            NonZero::new((data.len() * stride) as u64).expect("write with zero len"),
        );

        (offset, offset + data.len())
    }

    pub fn stage(&mut self, wgpu: &mut RenderWgpu, data: &[T]) -> (usize, usize) {
        assert!(self.offset + data.len() < self.len, "expand not implemented");

        let offset = self.offset;
        self.offset += data.len();

        self.stage[offset..self.offset].copy_from_slice(data);

        (offset, offset + data.len())
    }

    pub fn write_stage(&mut self, wgpu: &mut RenderWgpu) {
        let stride = mem::size_of::<T>();

        if self.offset > 0 {
            wgpu.write(
                &self.buffer, 
                bytemuck::cast_slice(&self.stage[0..self.offset]),
                0,
                NonZero::new((self.offset * stride) as u64).expect("write with zero len"),
            );
        }
    }

    pub fn buffer_slice<'a>(&'a self, start: usize, end: usize) -> wgpu::BufferSlice<'a> {
        let stride = mem::size_of::<T>();

        self.buffer.slice(
            (stride * start) as u64..(stride * end) as u64
        )
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
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

pub(super) fn create_vertex_buffer_init<T: Pod>(
    wgpu: &mut RenderWgpu,
    data: &[T],
) -> wgpu::Buffer {
    let buffer = create_vertex_buffer::<T>(wgpu.device, data.len());

    let stride = mem::size_of::<T>();

    wgpu.write(
        &buffer, 
        bytemuck::cast_slice(data),
        0,
        NonZero::new((data.len() * stride) as u64).expect("write with zero len"),
    );

    buffer
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
