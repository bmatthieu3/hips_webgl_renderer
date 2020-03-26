use web_sys::WebGlVertexArrayObject;

use crate::core::ArrayBuffer;
use crate::core::ArrayBufferInstanced;
use crate::core::ElementArrayBuffer;
use crate::core::BufferData;

use crate::WebGl2Context;

pub struct VertexArrayObject {
    array_buffer: Vec<ArrayBuffer>,
    array_buffer_instanced: Vec<ArrayBufferInstanced>,
    element_array_buffer: Option<ElementArrayBuffer>,

    idx: u32, // Number of vertex attributes

    vao: WebGlVertexArrayObject,

    gl: WebGl2Context,
}

impl VertexArrayObject {
    pub fn new(gl: &WebGl2Context) -> VertexArrayObject {
        let vao = gl.create_vertex_array()
            .ok_or("failed to create the vertex array buffer")
            .unwrap();

        let array_buffer = vec![];
        let array_buffer_instanced = vec![];

        let element_array_buffer = None;

        let idx = 0;

        let gl = gl.clone();
        VertexArrayObject {
            array_buffer,
            array_buffer_instanced,
            element_array_buffer,

            idx,

            vao,
            gl,
        }
    }

    pub fn bind(&mut self) -> VertexArrayObjectBound {
        self.gl.bind_vertex_array(Some(self.vao.as_ref()));

        VertexArrayObjectBound::new(self)
    }

    pub fn bind_ref(&self) {
        self.gl.bind_vertex_array(Some(self.vao.as_ref()));
    }

    pub fn num_elements(&self) -> usize {
        self.element_array_buffer.as_ref().unwrap().num_elements()
    }

    pub fn num_instances(&self) -> usize {
        self.array_buffer_instanced[0].num_instances()
    }
}

use web_sys::console;
impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        //self.unbind();
        console::log_1(&format!("delete VAO").into());
        self.gl.delete_vertex_array(Some(self.vao.as_ref()));
    }
}

pub struct VertexArrayObjectBound<'a> {
    vao: &'a mut VertexArrayObject,
}

use crate::core::VertexAttribPointerType;
impl<'a> VertexArrayObjectBound<'a> {
    pub fn new(vao: &'a mut VertexArrayObject) -> VertexArrayObjectBound<'a> {
        VertexArrayObjectBound {
            vao
        }
    }

    /// Precondition: self must be bound
    pub fn add_array_buffer<T: VertexAttribPointerType>(&mut self, stride: usize, sizes: &[usize], offsets: &[usize], usage: u32, data: BufferData<'a, T>) ->  &mut Self {
        let array_buffer = ArrayBuffer::new(
            &self.vao.gl,
            self.vao.idx,
            stride,
            sizes,
            offsets,
            usage,
            data
        );

        // Update the number of vertex attrib
        self.vao.idx += sizes.len() as u32;

        self.vao.array_buffer.push(array_buffer);

        self
    }

    /// Precondition: self must be bound
    pub fn add_instanced_array_buffer(&mut self, stride: usize, sizes: &[usize], offsets: &[usize], usage: u32, data: BufferData<'a, f32>) -> &mut Self {
        let array_buffer = ArrayBufferInstanced::new(
            &self.vao.gl,
            self.vao.idx,
            stride,
            sizes,
            offsets,
            usage,
            data,
        );

        // Update the number of vertex attrib
        self.vao.idx += sizes.len() as u32;

        self.vao.array_buffer_instanced.push(array_buffer);

        self
    }

    /// Precondition: self must be bound
    pub fn add_element_buffer(&mut self, usage: u32, data: BufferData<'a, u16>) -> &mut Self {
        let element_buffer = ElementArrayBuffer::new(
            &self.vao.gl,
            usage,
            data
        );

        self.vao.element_array_buffer = Some(element_buffer);

        self
    }

    pub fn update_array<T: VertexAttribPointerType>(&mut self, idx: usize, usage: u32, array_data: BufferData<'a, T>) -> &mut Self {
        self.vao.array_buffer[idx].update(usage, array_data);
        self
    }
    pub fn update_element_array(&mut self, usage: u32, element_data: BufferData<'a, u16>) -> &mut Self {
        if let Some(ref mut element_array_buffer) = self.vao.element_array_buffer {
            element_array_buffer.update(usage, element_data);
        }
        self
    }
    pub fn update_instanced_array(&mut self, idx: usize, array_data: BufferData<'a, f32>) -> &mut Self {
        self.vao.array_buffer_instanced[idx].update(array_data);
        self
    }

    pub fn unbind(&self) {
        self.vao.gl.bind_vertex_array(None);
    }
}