use web_sys::WebGl2RenderingContext;
use web_sys::WebGlVertexArrayObject;
use crate::shader::Shader;

use crate::viewport::ViewPort;
use crate::renderable::projection::ProjectionType;

use std::rc::Rc;
use std::borrow::Borrow;

pub mod buffers;
pub mod hips_sphere;
pub mod projection;
pub mod grid;

trait VertexBufferObject {
    fn bind(&self);
    fn unbind(&self);
}

use buffers::vertex_array_object::VertexArrayObject;
use buffers::buffer_data::BufferData;
use crate::WebGl2Context;
use cgmath::Matrix4;

pub trait Mesh {
    fn create_buffers(&self, gl: &WebGl2Context) -> VertexArrayObject;

    fn update(&mut self, projection: &ProjectionType, local_to_world_mat: &Matrix4<f32>, viewport: Option<&ViewPort>);
    fn get_vertices<'a>(&'a self) -> (BufferData<'a, f32>, BufferData<'a, u16>);

    fn send_uniforms(&self, gl: &WebGl2Context, shader: &Shader);
}

pub trait DisableDrawing {
    fn disable(&mut self);
}

pub struct Renderable<T>
where T: Mesh + DisableDrawing {
    shader: Rc<Shader>,
    model_mat: cgmath::Matrix4::<f32>,
    inverted_model_mat: cgmath::Matrix4<f32>,

    scale_mat: cgmath::Matrix4::<f32>,
    rotation_mat: cgmath::Matrix4::<f32>,
    translation_mat: cgmath::Matrix4::<f32>,

    // VAO index
    vertex_array_object: VertexArrayObject,

    mesh: T,

    gl: WebGl2Context,
}


use cgmath;
use cgmath::SquareMatrix;

use crate::utils;

impl<T> Renderable<T>
where T: Mesh + DisableDrawing {
    pub fn new(gl: &WebGl2Context, shader: Rc<Shader>, mesh: T) -> Renderable<T> {
        shader.bind(gl);

        let vertex_array_object = mesh.create_buffers(gl);

        let model_mat = cgmath::Matrix4::identity();
        let inverted_model_mat = model_mat;

        let scale_mat = cgmath::Matrix4::identity();
        let rotation_mat = cgmath::Matrix4::identity();
        let translation_mat = cgmath::Matrix4::identity();

        let gl = gl.clone();
        Renderable {
            // The shader to bind when drawing the renderable
            shader,
            // The model matrix of the Renderable
            model_mat,
            inverted_model_mat,
            // And its submatrices
            scale_mat,
            rotation_mat,
            translation_mat,
            // Vertex-Array Object index
            vertex_array_object,
            mesh,
            gl,
        }
    }

    fn recompute_model_matrix(&mut self) {
        self.model_mat = self.translation_mat * self.rotation_mat * self.scale_mat;
        self.inverted_model_mat = self.model_mat.invert().unwrap();
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

    pub fn get_inverted_model_mat(&self) -> &cgmath::Matrix4<f32> {
        return &self.inverted_model_mat;
    }

    pub fn set_model_mat(&mut self, model_mat: &cgmath::Matrix4<f32>) {
        self.model_mat = *model_mat;
    }

    pub fn mesh(&self) -> &T {
        &self.mesh
    }

    pub fn mesh_mut(&mut self) -> &mut T {
        &mut self.mesh
    }

    pub fn update(&mut self, projection: &ProjectionType, viewport: &ViewPort) {
        // Update the mesh vertices/indexes
        self.mesh.update(
            projection,
            &self.model_mat,
            Some(viewport)
        );

        // Get the new buffer data
        let (vertices, idx_vertices) = self.mesh.get_vertices();
        // Update the VAO with the new data
        self.vertex_array_object.update_array_and_element_buffer(vertices, idx_vertices);
    }

    pub fn draw(&self, mode: u32, viewport: &ViewPort) {
        self.shader.bind(&self.gl);

        self.vertex_array_object.bind();

        // Send Uniforms
        viewport.send_to_vertex_shader(&self.gl, self.shader.borrow());
        self.mesh.send_uniforms(&self.gl, &self.shader);

        // Send model matrix
        let model_mat_location = self.shader.get_uniform_location("model");
        let model_mat_f32_slice: &[f32; 16] = self.model_mat.as_ref();
        self.gl.uniform_matrix4fv_with_f32_array(model_mat_location, false, model_mat_f32_slice);

        // Send current time
        let location_time = self.shader.get_uniform_location("current_time");
        self.gl.uniform1f(location_time, utils::get_current_time());

        self.gl.draw_elements_with_i32(
            mode,
            self.vertex_array_object.num_vertices() as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );
        //self.vertex_array_object.unbind();
    }
}
