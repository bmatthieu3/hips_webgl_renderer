use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;

use crate::renderable::VertexBufferObject;

use crate::WebGl2Context;

pub struct ArrayBufferInstanced {
    buffer: WebGlBuffer,
    idx: u32, // index of the attrib pointer
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

use crate::renderable::buffers::buffer_data::BufferData;
use std::convert::TryInto;

impl<'a> ArrayBufferInstanced {
    pub fn new(gl: &WebGl2Context, idx: u32, stride: usize, size: usize, data: BufferData<'a, f32>, usage: u32) -> ArrayBufferInstanced {
        let num_instances = data.0.len() / size;

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
        gl.vertex_attrib_pointer_with_i32(idx, size as i32, WebGl2RenderingContext::FLOAT, false, stride as i32, 0);
        gl.enable_vertex_attrib_array(idx);
        gl.vertex_attrib_divisor(idx, 1);

        let gl = gl.clone();
        // Returns an instance that keeps only the buffer
        ArrayBufferInstanced {
            buffer,
            idx,
            num_instances,
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

    // Returns the number of vertices stored in the array buffer
    pub fn num_instances(&self) -> usize {
        self.num_instances
    }
}

impl Drop for ArrayBufferInstanced {
    fn drop(&mut self) {
        self.gl.disable_vertex_attrib_array(self.idx);
        
        self.gl.delete_buffer(Some(self.buffer.as_ref()));
    }
}