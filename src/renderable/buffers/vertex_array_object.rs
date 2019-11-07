use web_sys::WebGl2RenderingContext;
use web_sys::WebGlVertexArrayObject;

use crate::renderable::buffers::array_buffer::ArrayBuffer;
use crate::renderable::buffers::element_array_buffer::ElementArrayBuffer;
use crate::renderable::buffers::buffer_data::BufferData;

use std::rc::Rc;

use crate::WebGl2Context;

pub struct VertexArrayObject {
    array_buffer: Option<ArrayBuffer>,
    element_array_buffer: Option<ElementArrayBuffer>,

    vao: WebGlVertexArrayObject,

    gl: WebGl2Context
}

impl<'a> VertexArrayObject {
    pub fn new(gl: &WebGl2Context) -> VertexArrayObject {
        let vao = gl.create_vertex_array()
            .ok_or("failed to create the vertex array buffer")
            .unwrap();

        let array_buffer = None;
        let element_array_buffer = None;

        let gl = gl.clone();
        VertexArrayObject {
            array_buffer,
            element_array_buffer,
            vao,
            gl
        }
    }

    pub fn update_array_and_element_buffer(&mut self, array_data: BufferData<'a, f32>, element_data: BufferData<'a, u16>) {
        self.bind();
        self.array_buffer.as_ref().unwrap().update(array_data);
        if let Some(ref mut element_array_buffer) = self.element_array_buffer {
            element_array_buffer.update(element_data);
        }
        //self.unbind();
    }

    pub fn set_array_buffer(&mut self, array_buffer: ArrayBuffer) {
        self.array_buffer = Some(array_buffer);
    }

    pub fn set_element_array_buffer(&mut self, element_array_buffer: ElementArrayBuffer) {
        self.element_array_buffer = Some(element_array_buffer);
    }

    pub fn bind(&self) {
        self.gl.bind_vertex_array(Some(self.vao.as_ref()));
    }

    pub fn unbind(&self) {
        self.gl.bind_vertex_array(None);
    }

    pub fn num_vertices(&self) -> usize {
        self.element_array_buffer.as_ref().unwrap().size()
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        self.gl.delete_vertex_array(Some(self.vao.as_ref()));
    }
}