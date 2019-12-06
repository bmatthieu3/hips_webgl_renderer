use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;

use crate::renderable::VertexBufferObject;

use std::rc::Rc;

use crate::WebGl2Context;

pub struct ArrayBuffer {
    buffer: WebGlBuffer,
    num_packed_data: usize,
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

impl<'a> ArrayBuffer {
    pub fn new(gl: &WebGl2Context, offset_idx: u32, stride: usize, sizes: &[usize], offsets: &[usize], usage: u32, data: BufferData<'a, f32>) -> ArrayBuffer {
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
            gl.vertex_attrib_pointer_with_i32(idx, *size as i32, WebGl2RenderingContext::FLOAT, false, stride as i32, *offset as i32);
            gl.enable_vertex_attrib_array(idx);
        }

        let num_packed_data = sizes.len();

        let gl = gl.clone();
        // Returns an instance that keeps only the buffer
        ArrayBuffer {
            buffer,
            num_packed_data,
            gl,
        }
    }

    pub fn update(&self, data: BufferData<'a, f32>) {
        let data: js_sys::Float32Array = data.try_into().unwrap();

        // offset expressed in bytes where data replacement will begin in the buffer
        let offset = (0 * std::mem::size_of::<f32>()) as i32; 
        
        self.bind();
        self.gl.buffer_sub_data_with_i32_and_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            offset,
            &data,
        );
        //self.unbind();
    }
}

impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        for idx in 0..self.num_packed_data {
            self.gl.disable_vertex_attrib_array(idx as u32);
        }

        self.gl.delete_buffer(Some(self.buffer.as_ref()));
    }
}