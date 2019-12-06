use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;

use crate::renderable::VertexBufferObject;

use crate::WebGl2Context;

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
use crate::renderable::buffers::buffer_data::BufferData;

impl<'a> ElementArrayBuffer {
    pub fn new(gl: &WebGl2Context, usage: u32, data: BufferData<'a, u16>) -> ElementArrayBuffer {
        let buffer = gl.create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
        // Bind the buffer
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(buffer.as_ref()));
        let buffer_size = data.0.len();
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
        //self.buffer_size = data.0.len();
        //console::log_1(&format!("update element buffer size: {:?} {:?}", self.buffer_size, data.0.len()).into());
        let data: js_sys::Uint16Array = data.try_into().unwrap();
        
        // offset expressed in bytes where data replacement will begin in the buffer
        let offset = (0 * std::mem::size_of::<u16>()) as i32;

        /*let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>().unwrap().buffer();
        self.gl.buffer_sub_data_with_i32_and_array_buffer_view_and_src_offset_and_length(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            0,
            &js_sys::Uint16Array::new(&memory_buffer),
            (data.0.as_ptr() as u32) / 2,
            data.0.len() as u32,
        );*/

        //self.bind();
        self.gl.buffer_sub_data_with_i32_and_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            offset,
            &data,
        );
        //self.unbind();
    }
}

impl Drop for ElementArrayBuffer {
    fn drop(&mut self) {
        self.gl.delete_buffer(Some(self.buffer.as_ref()));
    }
}
