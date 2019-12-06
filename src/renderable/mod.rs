use crate::shader::Shader;

use crate::viewport::ViewPort;
use crate::renderable::projection::ProjectionType;

pub mod buffers;
pub mod hips_sphere;
pub mod projection;
pub mod grid;
pub mod catalog;

trait VertexBufferObject {
    fn bind(&self);
    fn unbind(&self);
}

use buffers::vertex_array_object::VertexArrayObject;
use buffers::buffer_data::BufferData;
use crate::WebGl2Context;
use cgmath::Matrix4;

use std::collections::HashMap;
pub trait Mesh {
    fn create_buffers(&self, gl: &WebGl2Context) -> VertexArrayObject;

    fn update(
        &mut self,
        projection: &ProjectionType,
        local_to_world_mat: &Matrix4<f32>,
        viewport: &ViewPort
    );
    fn get_vertices<'a>(&'a self) -> (BufferData<'a, f32>, BufferData<'a, u16>);

    fn send_uniforms(&self,
        gl: &WebGl2Context,
        shader: &Shader
    );

    fn draw<T: Mesh + DisableDrawing>(
        &self,
        gl: &WebGl2Context,
        renderable: &Renderable<T>,
        shaders: &HashMap<&'static str, Shader>,
        viewport: &ViewPort
    );
}

pub trait DisableDrawing {
    fn disable(&mut self);
}

pub struct Renderable<T>
where T: Mesh + DisableDrawing {
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
    pub fn new(gl: &WebGl2Context, shader: &Shader, mesh: T) -> Renderable<T> {
        shader.bind(gl);
        let vertex_array_object = mesh.create_buffers(gl);

        let model_mat = cgmath::Matrix4::identity();
        let inverted_model_mat = model_mat;

        let scale_mat = cgmath::Matrix4::identity();
        let rotation_mat = cgmath::Matrix4::identity();
        let translation_mat = cgmath::Matrix4::identity();

        let gl = gl.clone();
        Renderable {
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

    pub fn update_mesh(&mut self, shader: &Shader, mesh: T) {
        shader.bind(&self.gl);
        self.vertex_array_object = mesh.create_buffers(&self.gl);

        self.mesh = mesh;
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

    pub fn get_model_mat(&self) -> &cgmath::Matrix4<f32> {
        return &self.model_mat; 
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
            viewport
        );
    }

    pub fn update_vertex_array(&mut self) {
        // Get the new buffer data
        let (vertices, idx_vertices) = self.mesh.get_vertices();
        // Update the VAO with the new data
        self.vertex_array_object.bind()
            .update_array_and_element_buffer(0, vertices, idx_vertices);
    }

    pub fn draw(&self, shaders: &HashMap<&'static str, Shader>, viewport: &ViewPort) {
        let ref gl = self.gl;
        self.mesh.draw(gl, &self, shaders, viewport);
    }
}
