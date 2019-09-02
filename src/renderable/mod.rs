use web_sys::WebGl2RenderingContext;
use web_sys::WebGlVertexArrayObject;
use crate::shader::Shader;
use web_sys::console;

use std::rc::Rc;
use std::borrow::Borrow;
pub trait Mesh {
    fn create_buffers(gl: &WebGl2RenderingContext) -> (Box<[(u32, i32, WebGlBuffer)]>, i32, WebGlVertexArrayObject);
    fn link_buffers_to_vertex_shader(gl: &WebGl2RenderingContext, buffers: &Box<[(u32, i32, WebGlBuffer)]>) {
        let mut indx = 0;
        for (target, size, buffer_idx) in buffers.iter() {
            if *target == WebGl2RenderingContext::ARRAY_BUFFER {
                gl.bind_buffer(*target, Some(&buffer_idx));

                gl.vertex_attrib_pointer_with_i32(indx, *size, WebGl2RenderingContext::FLOAT, false, 0, 0);
                gl.enable_vertex_attrib_array(indx);

                indx += 1;
            }
        }
    }

    fn bind_buffers_to_vertex_shader(gl: &WebGl2RenderingContext, buffers: &Box<[(u32, i32, WebGlBuffer)]>) {
        for (target, _, buffer_idx) in buffers.iter() {
            gl.bind_buffer(*target, Some(buffer_idx));
        }
    }

    fn create_vertex_array() -> js_sys::Float32Array;
    fn create_uv_array() -> js_sys::Float32Array;
    fn create_color_array() -> js_sys::Float32Array;
    fn create_index_array() -> js_sys::Uint32Array;

    fn init_uniforms(gl: &WebGl2RenderingContext, shader: &Shader) -> Box<[WebGlUniformLocation]>;
    fn send_uniforms(gl: &WebGl2RenderingContext, uniforms: &Box<[WebGlUniformLocation]>);
}

pub mod hips_sphere;
pub mod direct_system;

use std::marker::PhantomData;
use web_sys::WebGlBuffer;
use web_sys::WebGlUniformLocation;
pub struct Renderable<'a, T>
where T: Mesh {
    shader: Rc<Shader>,

    model_mat: cgmath::Matrix4::<f32>,

    scale_mat: cgmath::Matrix4::<f32>,
    rotation_mat: cgmath::Matrix4::<f32>,
    translation_mat: cgmath::Matrix4::<f32>,

    // VAO index
    vao: WebGlVertexArrayObject,

    // Buffers id
    buffers: Box<[(u32, i32, WebGlBuffer)]>,

    // Uniforms
    uniforms: Box<[WebGlUniformLocation]>,

    num_vertices: i32,
    phantom: PhantomData<&'a T>,
}

use crate::viewport::ViewPort;
use cgmath;
use cgmath::SquareMatrix;
impl<'a, T> Renderable<'a, T>
where T: Mesh {
    pub fn new(gl: &WebGl2RenderingContext, shader: Rc<Shader>) -> Renderable<'a, T> {
        shader.bind(gl);

        let (buffers, num_vertices, vao) = T::create_buffers(gl);
        console::log_1(&format!("num_vertices {:?}", num_vertices).into());

        T::link_buffers_to_vertex_shader(gl, &buffers);
        let uniforms = T::init_uniforms(gl, shader.borrow());

        let phantom = PhantomData;

        let model_mat = cgmath::Matrix4::identity();

        let scale_mat = cgmath::Matrix4::identity();
        let rotation_mat = cgmath::Matrix4::identity();
        let translation_mat = cgmath::Matrix4::identity();
        Renderable {
            // The shader to bind when drawing the renderable
            shader, 
            // The model matrix of the Renderable
            model_mat,
            // And its submatrices
            scale_mat,
            rotation_mat,
            translation_mat,
            // Vertex-Array Object index
            vao,
            // Buffers indexes
            buffers,
            // Uniforms indexes
            uniforms,
            // Num of vertices to draw
            num_vertices,
            phantom
        }
    }

    fn recompute_model_matrix(&mut self) {
        self.model_mat = self.translation_mat * self.rotation_mat * self.scale_mat;
    }

    pub fn rotate(&mut self, axis: cgmath::Vector3<f32>, angle: cgmath::Rad<f32>) {
        self.rotation_mat = cgmath::Matrix4::<f32>::from_axis_angle(axis, angle);

        self.recompute_model_matrix();
    }

    pub fn apply_rotation(&mut self, axis: cgmath::Vector3<f32>, angle: cgmath::Rad<f32>) {
        self.rotation_mat = cgmath::Matrix4::<f32>::from_axis_angle(axis, angle) * self.rotation_mat;

        self.recompute_model_matrix();
    }

    pub fn scale(&mut self, factor: f32) {
        self.scale_mat = cgmath::Matrix4::<f32>::from_scale(factor);

        self.recompute_model_matrix();
    }

    pub fn draw(&self, gl: &WebGl2RenderingContext, mode: u32, viewport: &ViewPort) {
        self.shader.bind(gl);

        gl.bind_vertex_array(Some(&self.vao));
        /*// Bind buffers
        T::bind_buffers_to_vertex_shader(gl, &self.buffers);*/

        // Send Uniforms
        viewport.send_to_vertex_shader(gl, self.shader.borrow());
        T::send_uniforms(gl, &self.uniforms);

        // Get the attribute location of the model matrix from the Vertex shader
        let model_mat_location = self.shader.get_uniform_location(gl, "model");
        let model_mat_f32_slice: &[f32; 16] = self.model_mat.as_ref();
        gl.uniform_matrix4fv_with_f32_array(model_mat_location.as_ref(), false, model_mat_f32_slice);

        gl.draw_elements_with_i32(
            mode,
            self.num_vertices,
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
        );
    }
}
