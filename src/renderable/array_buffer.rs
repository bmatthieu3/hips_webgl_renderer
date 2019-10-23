use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;

use crate::renderable::VertexBufferObject;

use std::rc::Rc;

pub struct ArrayBuffer {
    buffer: WebGlBuffer,
    num_packed_data: usize,
    gl: Rc<WebGl2RenderingContext>,
}

impl VertexBufferObject for ArrayBuffer {
    fn bind(&self) {
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(self.buffer.as_ref()));
    }
    fn unbind(&self) {
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
    }
}

use crate::renderable::buffer_data::BufferData;
use std::convert::TryInto;
impl ArrayBuffer {
    pub fn new(gl: Rc<WebGl2RenderingContext>, stride: usize, sizes: &[usize], offsets: &[usize], data: BufferData<f32>) -> ArrayBuffer {
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
            WebGl2RenderingContext::STATIC_DRAW,
        );
        // Link to the shader
        for (idx, (size, offset)) in sizes.iter().zip(offsets.iter()).enumerate() {
            gl.vertex_attrib_pointer_with_i32(idx as u32, *size as i32, WebGl2RenderingContext::FLOAT, false, stride as i32, *offset as i32);
            gl.enable_vertex_attrib_array(idx as u32);
        }

        let num_packed_data = sizes.len();

        // Returns an instance that keeps only the buffer
        ArrayBuffer {
            buffer,
            num_packed_data,
            gl,
        }
    }
}

impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        for idx in 0..self.num_packed_data {
            self.gl.disable_vertex_attrib_array(idx as u32);
        }
    }
}