use web_sys::WebGl2RenderingContext;
use web_sys::WebGlVertexArrayObject;

use crate::renderable::array_buffer::ArrayBuffer;
use crate::renderable::element_array_buffer::ElementArrayBuffer;

use std::rc::Rc;

pub struct VertexArrayObject {
    array_buffer: Option<ArrayBuffer>,
    element_array_buffer: Option<ElementArrayBuffer>,

    vao: WebGlVertexArrayObject,

    gl: Rc<WebGl2RenderingContext>
}

impl VertexArrayObject {
    pub fn new(gl: Rc<WebGl2RenderingContext>) -> VertexArrayObject {
        let vao = gl.create_vertex_array()
            .ok_or("failed to create the vertex array buffer")
            .unwrap();

        let array_buffer = None;
        let element_array_buffer = None;

        VertexArrayObject {
            array_buffer,
            element_array_buffer,
            vao,
            gl
        }
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
