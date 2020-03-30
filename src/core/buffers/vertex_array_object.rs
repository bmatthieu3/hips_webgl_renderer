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

    // Shader has to be already bound before calling this
    // This returns a ShaderVertexArrayObjectBound for which it is possible
    // to add some buffers and or draw the buffers
    pub fn bind<'a, 'b>(&'a mut self, shader: &'b ShaderBound<'b>) -> ShaderVertexArrayObjectBound<'a, 'b> {
        self.gl.bind_vertex_array(Some(self.vao.as_ref()));

        ShaderVertexArrayObjectBound {
            vao: self,
            shader: shader,
        }
    }
    // Shader has to be already bound before calling this
    // This returns a ShaderVertexArrayObjectBound for which it is possible
    // to add some buffers and or draw the buffers
    pub fn bind_ref<'a, 'b>(&'a self, shader: &'b ShaderBound<'b>) -> ShaderVertexArrayObjectBoundRef<'a, 'b> {
        self.gl.bind_vertex_array(Some(self.vao.as_ref()));

        ShaderVertexArrayObjectBoundRef {
            vao: self,
            shader: shader,
        }
    }

    // No need to bind a shader here
    // This returns a VertexArrayObjectBound for which it is only possible to
    // update the buffers
    pub fn bind_for_update<'a>(&'a mut self) -> VertexArrayObjectBound<'a> {
        VertexArrayObjectBound {
            vao: self
        }
    }

    /*pub fn bind_ref(&self) {
        self.gl.bind_vertex_array(Some(self.vao.as_ref()));
    }*/

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

use crate::shader::ShaderBound;
pub struct ShaderVertexArrayObjectBound<'a, 'b> {
    vao: &'a mut VertexArrayObject,
    shader: &'b ShaderBound<'b>,
}

use crate::core::VertexAttribPointerType;
use web_sys::WebGl2RenderingContext;
impl<'a, 'b> ShaderVertexArrayObjectBound<'a, 'b> {
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

    pub fn draw_elements_with_i32(&self, mode: u32, num_elements: Option<i32>) {
        let num_elements = num_elements.unwrap_or(
            self.vao.num_elements() as i32
        );
        self.vao.gl.draw_elements_with_i32(
            mode,
            num_elements,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );
    }

    pub fn draw_elements_instanced_with_i32(&self, mode: u32, num_instances: i32) {
        self.vao.gl.draw_elements_instanced_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.vao.num_elements() as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
            num_instances,
        );
    }

    pub fn unbind(&self) {
        self.vao.gl.bind_vertex_array(None);
    }
}

pub struct ShaderVertexArrayObjectBoundRef<'a, 'b> {
    vao: &'a VertexArrayObject,
    shader: &'b ShaderBound<'b>,
}

impl<'a, 'b> ShaderVertexArrayObjectBoundRef<'a, 'b> {
    pub fn draw_elements_with_i32(&self, mode: u32, num_elements: Option<i32>) {
        let num_elements = num_elements.unwrap_or(
            self.vao.num_elements() as i32
        );
        self.vao.gl.draw_elements_with_i32(
            mode,
            num_elements,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );
    }

    pub fn draw_elements_instanced_with_i32(&self, mode: u32, num_instances: i32) {
        self.vao.gl.draw_elements_instanced_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.vao.num_elements() as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
            num_instances,
        );
    }

    pub fn unbind(&self) {
        self.vao.gl.bind_vertex_array(None);
    }
}

// Struct defined when only the Vertex Array Object is
// defined
pub struct VertexArrayObjectBound<'a> {
    vao: &'a mut VertexArrayObject,
}

impl<'a> VertexArrayObjectBound<'a> {
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
}