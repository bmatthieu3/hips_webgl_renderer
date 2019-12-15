use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;

use crate::renderable::VertexBufferObject;

use crate::WebGl2Context;

pub struct ArrayBufferInstanced {
    buffer: WebGlBuffer,
    num_packed_data: usize,
    offset_idx: u32,
    num_instances: usize,
    gl: WebGl2Context,
}

impl VertexBufferObject for ArrayBufferInstanced {
    fn bind(&self) {
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(self.buffer.as_ref()));
    }
    fn unbind(&self) {
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
    }
}

use crate::renderable::buffers::buffer_data::{BufferData, BufferDataSlice};
use std::convert::TryInto;

impl<'a> ArrayBufferInstanced {
    pub fn new(gl: &WebGl2Context, offset_idx: u32, stride: usize, sizes: &[usize], offsets: &[usize], usage: u32, data: BufferData<'a, f32>) -> ArrayBufferInstanced {
    
    //pub fn new(gl: &WebGl2Context, idx: u32, stride: usize, size: usize, usage: u32, buffer_data: BufferData<'a, f32>) -> ArrayBufferInstanced {
        let num_bytes_source: usize = sizes.iter().sum();
        let num_instances = data.data.len() / num_bytes_source;

        let buffer = gl.create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();

        // Bind the buffer
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(buffer.as_ref()));
        // Pass the vertices data to the buffer
        let data: js_sys::Float32Array = data.try_into().unwrap();
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &data,
            usage,
        );
        // Link to the shader
        for (idx, (size, offset)) in sizes.iter().zip(offsets.iter()).enumerate() {
            let idx = (idx as u32) + offset_idx;
            gl.vertex_attrib_pointer_with_i32(idx, *size as i32, WebGl2RenderingContext::FLOAT, false, stride as i32, 0);
            gl.enable_vertex_attrib_array(idx);
            gl.vertex_attrib_divisor(idx, 1);
        }

        let num_packed_data = sizes.len();

        let gl = gl.clone();
        // Returns an instance that keeps only the buffer
        ArrayBufferInstanced {
            buffer,
            num_packed_data,
            offset_idx,
            num_instances,
            gl,
        }
    }

    pub fn update(&self, buffer: BufferData<'a, f32>) {
        let offset = buffer.offset;
        let data: js_sys::Float32Array = buffer.try_into().unwrap();

        // offset expressed in bytes where data replacement will begin in the buffer
        self.bind();
        self.gl.buffer_sub_data_with_i32_and_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            offset as i32,
            &data,
        );
    }

    pub fn update_slice(&self, buffer: BufferDataSlice<'a, f32>) {
        let offset = buffer.offset;
        let data: js_sys::Float32Array = buffer.try_into().unwrap();

        // offset expressed in bytes where data replacement will begin in the buffer
        self.bind();
        self.gl.buffer_sub_data_with_i32_and_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            offset as i32,
            &data,
        );
    }

    // Returns the number of vertices stored in the array buffer
    pub fn num_instances(&self) -> usize {
        self.num_instances
    }
}

impl Drop for ArrayBufferInstanced {
    fn drop(&mut self) {
        for idx in 0..self.num_packed_data {
            let idx = (idx as u32) + self.offset_idx;
            self.gl.disable_vertex_attrib_array(idx);
        }
        
        self.gl.delete_buffer(Some(self.buffer.as_ref()));
    }
}