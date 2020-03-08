use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;

use crate::core::VertexBufferObject;

use crate::WebGl2Context;

#[derive(Clone)]
pub struct ElementArrayBuffer {
    buffer: WebGlBuffer,
    buffer_size: usize,

    gl: WebGl2Context,
}

impl VertexBufferObject for ElementArrayBuffer {
    fn bind(&self) {
        self.gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(self.buffer.as_ref()));
    }
    fn unbind(&self) {
        self.gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
    }
}

use web_sys::console;
use std::convert::TryInto;
use crate::core::BufferData;

impl<'a> ElementArrayBuffer {
    pub fn new(gl: &WebGl2Context, usage: u32, data: BufferData<'a, u16>) -> ElementArrayBuffer {
        let buffer = gl.create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
        // Bind the buffer
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(buffer.as_ref()));
        // Total length
        let buffer_size = match &data {
            BufferData::VecData(data) => data.len(),
            BufferData::SliceData(data) => data.len()
        };
        // Pass the vertices data to the buffer
        let data: js_sys::Uint16Array = data.try_into().unwrap();
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &data,
            usage,
        );

        // Returns an instance that keeps only the buffer
        let gl = gl.clone();
        ElementArrayBuffer {
            buffer,
            buffer_size,
            gl
        }
    }

    // Returns the number of vertices stored in the array buffer
    pub fn num_elements(&self) -> usize {
        self.buffer_size
    }

    pub fn update(&mut self, data: BufferData<'a, u16>) {
        let data: js_sys::Uint16Array = data.try_into().unwrap();

        self.gl.buffer_sub_data_with_i32_and_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            0,
            &data,
        );
    }
}

impl Drop for ElementArrayBuffer {
    fn drop(&mut self) {
        self.gl.delete_buffer(Some(self.buffer.as_ref()));
    }
}
