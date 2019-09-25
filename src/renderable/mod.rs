use web_sys::WebGl2RenderingContext;
use web_sys::WebGlVertexArrayObject;
use web_sys::WebGlTexture;
use crate::shader::Shader;
use web_sys::console;

use crate::viewport::ViewPort;
use crate::renderable::projection::ProjectionType;

use std::rc::Rc;
use std::borrow::Borrow;

pub trait Mesh {
    fn create_buffers(gl: &WebGl2RenderingContext, projection: &ProjectionType) -> (Box<[(u32, i32, WebGlBuffer)]>, i32, WebGlVertexArrayObject);
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

    fn create_vertices_array(projection: &ProjectionType) -> js_sys::Float32Array;
    fn create_uv_array() -> js_sys::Float32Array;
    fn create_color_array() -> js_sys::Float32Array;
    fn create_index_array() -> js_sys::Uint32Array;

    fn send_uniforms(&self, gl: &WebGl2RenderingContext, shader: &Shader);
}

pub mod hips_sphere;
pub mod direct_system;
pub mod projection;

use std::marker::PhantomData;
use web_sys::WebGlBuffer;
use web_sys::WebGlUniformLocation;
use std::cell::RefCell;

pub struct Renderable<T>
where T: Mesh {
    shader: Rc<Shader>,
    projection: Rc<RefCell<ProjectionType>>,

    model_mat: cgmath::Matrix4::<f32>,

    scale_mat: cgmath::Matrix4::<f32>,
    rotation_mat: cgmath::Matrix4::<f32>,
    translation_mat: cgmath::Matrix4::<f32>,

    // VAO index
    vao: WebGlVertexArrayObject,

    // Buffers id
    buffers: Box<[(u32, i32, WebGlBuffer)]>,

    num_vertices: i32,
    mesh: Rc<RefCell<T>>
}

use cgmath;
use cgmath::SquareMatrix;

impl<T> Renderable<T>
where T: Mesh {
    pub fn new(gl: &WebGl2RenderingContext, shader: Rc<Shader>, projection: Rc<RefCell<ProjectionType>>, mesh: Rc<RefCell<T>>) -> Renderable<T> {
        shader.bind(gl);

        let (buffers, num_vertices, vao) = T::create_buffers(gl, &projection.as_ref().borrow());
        T::link_buffers_to_vertex_shader(gl, &buffers);

        let model_mat = cgmath::Matrix4::identity();

        let scale_mat = cgmath::Matrix4::identity();
        let rotation_mat = cgmath::Matrix4::identity();
        let translation_mat = cgmath::Matrix4::identity();
        Renderable {
            // The shader to bind when drawing the renderable
            shader,
            // The type of projection
            projection,
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
            // Num of vertices to draw
            num_vertices,
            mesh
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

    pub fn get_model_mat(&self) -> cgmath::Matrix4<f32> {
        return self.model_mat.clone();
    }

    pub fn draw(&self, gl: &WebGl2RenderingContext, mode: u32, viewport: &ViewPort) {
        self.shader.bind(gl);

        gl.bind_vertex_array(Some(&self.vao));

        // Send Uniforms
        viewport.send_to_vertex_shader(gl, self.shader.borrow());
        self.mesh.as_ref().borrow().send_uniforms(gl, &self.shader);

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
