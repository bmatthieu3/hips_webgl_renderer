use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;

use crate::renderable::VertexBufferObject;

use std::rc::Rc;

use crate::WebGl2Context;

pub struct ArrayBuffer {
    buffer: WebGlBuffer,
    num_packed_data: usize,
    offset_idx: u32,
    gl: WebGl2Context,
}

impl VertexBufferObject for ArrayBuffer {
    fn bind(&self) {
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(self.buffer.as_ref()));
    }
    fn unbind(&self) {
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
    }
}

use crate::renderable::buffers::buffer_data::BufferData;
use std::convert::TryInto;

pub trait VertexAttribPointerType: std::marker::Sized {
    /// Link the vertex attrib to the shader
    fn vertex_attrib_pointer_with_i32(gl: &WebGl2Context, idx: u32, size: i32, stride: i32, offset: i32);
    
    /// Pass the vertices data to the buffer
    fn buffer_data_with_array_buffer_view<'a>(gl: &WebGl2Context, data: BufferData<'a, Self>, usage: u32);
    
    fn update<'a>(gl: &WebGl2Context, data: BufferData<'a, Self>);

    // Initialize the VBO
    fn initialize_buffer<'a>(gl: &WebGl2Context, offset_idx: u32, stride: usize, sizes: &[usize], offsets: &[usize], usage: u32, data: BufferData<'a, Self>) -> WebGlBuffer {
        let buffer = gl.create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
        // Bind the buffer
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(buffer.as_ref()));
        
        Self::buffer_data_with_array_buffer_view(gl, data, usage);
        // Attrib pointer to the shader
        for (idx, (size, offset)) in sizes.iter().zip(offsets.iter()).enumerate() {
            let idx = (idx as u32) + offset_idx;

            Self::vertex_attrib_pointer_with_i32(gl, idx, *size as i32, stride as i32, *offset as i32);
        }

        buffer
    }
}

impl VertexAttribPointerType for u8 {
    fn update<'a>(gl: &WebGl2Context, data: BufferData<'a, Self>) {
        let data: js_sys::Uint8Array = data.try_into().unwrap();

        // Offset expressed in bytes where data replacement will begin in the buffer
        let offset = (0 * std::mem::size_of::<u8>()) as i32; 
        
        gl.buffer_sub_data_with_i32_and_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            offset,
            &data,
        );
    }

    fn buffer_data_with_array_buffer_view<'a>(gl: &WebGl2Context, data: BufferData<'a, u8>, usage: u32) {
        let data: js_sys::Uint8Array = data.try_into().unwrap();
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &data,
            usage,
        );
    }

    fn vertex_attrib_pointer_with_i32(gl: &WebGl2Context, idx: u32, size: i32, stride: i32, offset: i32) {
        gl.vertex_attrib_i_pointer_with_i32(idx, size, WebGl2RenderingContext::UNSIGNED_BYTE, stride, offset);
        gl.enable_vertex_attrib_array(idx);
    }
}

impl VertexAttribPointerType for f32 {
    fn update<'a>(gl: &WebGl2Context, data: BufferData<'a, Self>) {
        let data: js_sys::Float32Array = data.try_into().unwrap();

        // Offset expressed in bytes where data replacement will begin in the buffer
        let offset = (0 * std::mem::size_of::<f32>()) as i32; 
        
        gl.buffer_sub_data_with_i32_and_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            offset,
            &data,
        );
    }

    fn buffer_data_with_array_buffer_view<'a>(gl: &WebGl2Context, data: BufferData<'a, Self>, usage: u32) {
        let data: js_sys::Float32Array = data.try_into().unwrap();
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &data,
            usage,
        );
    }

    fn vertex_attrib_pointer_with_i32(gl: &WebGl2Context, idx: u32, size: i32, stride: i32, offset: i32) {
        gl.vertex_attrib_pointer_with_i32(idx, size, WebGl2RenderingContext::FLOAT, false, stride, offset);
        gl.enable_vertex_attrib_array(idx);
    }
}

impl<'a> ArrayBuffer {
    pub fn new<T: VertexAttribPointerType>(gl: &WebGl2Context, offset_idx: u32, stride: usize, sizes: &[usize], offsets: &[usize], usage: u32, data: BufferData<'a, T>) -> ArrayBuffer {
        let buffer = T::initialize_buffer(gl, offset_idx, stride, sizes, offsets, usage, data);

        let num_packed_data = sizes.len();

        let gl = gl.clone();
        // Returns an instance that keeps only the buffer
        ArrayBuffer {
            buffer,
            num_packed_data,
            offset_idx,
            gl,
        }
    }

    pub fn update<T: VertexAttribPointerType>(&self, data: BufferData<'a, T>) {
        self.bind();
        T::update(&self.gl, data);
    }
}

impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        for idx in 0..self.num_packed_data {
            let idx = (idx as u32) + self.offset_idx;
            self.gl.disable_vertex_attrib_array(idx);
        }

        self.gl.delete_buffer(Some(self.buffer.as_ref()));
    }
}