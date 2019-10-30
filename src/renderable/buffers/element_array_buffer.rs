use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;

use std::rc::Rc;

use crate::renderable::VertexBufferObject;

pub struct ElementArrayBuffer {
    buffer: WebGlBuffer,
    buffer_size: usize,

    gl: Rc<WebGl2RenderingContext>,
}

impl VertexBufferObject for ElementArrayBuffer {
    fn bind(&self) {
        self.gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(self.buffer.as_ref()));
    }
    fn unbind(&self) {
        self.gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
    }
}

use std::convert::TryInto;
use crate::renderable::buffers::buffer_data::BufferData;
impl ElementArrayBuffer {
    pub fn new(gl: Rc<WebGl2RenderingContext>, data: BufferData<u32>, usage: u32) -> ElementArrayBuffer {
        let buffer = gl.create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
        // Bind the buffer
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(buffer.as_ref()));
        let buffer_size = data.0.len();
        // Pass the vertices data to the buffer
        let data: js_sys::Uint32Array = data.try_into().unwrap();
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &data,
            usage,
        );
        //gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        // Returns an instance that keeps only the buffer
        ElementArrayBuffer {
            buffer,
            buffer_size,
            gl
        }
    }

    // Returns the number of vertices stored in the array buffer
    pub fn size(&self) -> usize {
        self.buffer_size
    }

    pub fn update(&self, data: BufferData<u32>) {
        let data: js_sys::Uint32Array = data.try_into().unwrap();

        // offset expressed in bytes where data replacement will begin in the buffer
        let offset = (0 * std::mem::size_of::<u32>()) as i32; 

        self.gl.buffer_sub_data_with_i32_and_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            offset,
            &data,
        );
    }
}