use crate::shader::Shader;

use crate::viewport::ViewPort;

pub mod hips_sphere;
pub mod projection;
pub mod grid;
pub mod catalog;

pub use hips_sphere::HiPSSphere;
pub use catalog::Catalog;
pub use grid::ProjetedGrid;

use crate::WebGl2Context;
use cgmath::Matrix4;

use std::collections::HashMap;
use crate::projection::Projection;
pub trait Mesh {
    fn create_buffers(&mut self, gl: &WebGl2Context);
    fn get_shader<'a>(&self, shaders: &'a HashMap<&'static str, Shader>) -> &'a Shader;
}

pub trait DisableDrawing {
    fn disable(&mut self, viewport: &ViewPort);
}

pub struct Renderable<T>
where T: Mesh + DisableDrawing {
    model_mat: cgmath::Matrix4::<f32>,
    inverted_model_mat: cgmath::Matrix4<f32>,

    //scale_mat: cgmath::Matrix4::<f32>,
    //rotation_mat: cgmath::Matrix4::<f32>,
    //translation_mat: cgmath::Matrix4::<f32>,

    mesh: T,

    gl: WebGl2Context,
}

use cgmath;
use cgmath::SquareMatrix;
use cgmath::Matrix3;

use cgmath::{Vector4, Vector2};
impl<T> Renderable<T>
where T: Mesh + DisableDrawing {
    pub fn new(gl: &WebGl2Context, shaders: &HashMap<&'static str, Shader>, mut mesh: T) -> Renderable<T> {
        let shader = mesh.get_shader(shaders);
        shader.bind(gl);
        mesh.create_buffers(gl);

        let model_mat = cgmath::Matrix4::identity();
        let inverted_model_mat = model_mat;

        //let scale_mat = cgmath::Matrix4::identity();
        //let rotation_mat = cgmath::Matrix4::identity();
        //let translation_mat = cgmath::Matrix4::identity();

        let gl = gl.clone();
        Renderable {
            // The model matrix of the Renderable
            model_mat,
            inverted_model_mat,
            // And its submatrices
            //scale_mat,
            //rotation_mat,
            //translation_mat,

            mesh,
            gl,
        }
    }

    pub fn compute_center_world_pos<P: Projection>(&self) -> Vector4<f32> {
        let ref model_mat = self.get_model_mat();

        (*model_mat) * P::clip_to_world_space(Vector2::new(0_f32, 0_f32)).unwrap()
    }

    pub fn set_mesh<U: Mesh + DisableDrawing>(self, mesh: U) -> Renderable<U> {
        Renderable::<U> {
            // The model matrix of the Renderable
            model_mat: self.model_mat,
            inverted_model_mat: self.inverted_model_mat,
            // And its submatrices
            /*scale_mat: self.scale_mat,
            rotation_mat: self.rotation_mat,
            translation_mat: self.translation_mat,*/

            mesh,
            gl: self.gl,
        }
    }

    pub fn update_mesh(&mut self, shaders: &HashMap<&'static str, Shader>, mut mesh: T) {
        let shader = mesh.get_shader(shaders);
        shader.bind(&self.gl);
        mesh.create_buffers(&self.gl);

        self.mesh = mesh;
    }

    pub fn apply_rotation(&mut self, axis: cgmath::Vector3<f32>, angle: cgmath::Rad<f32>) {
        self.model_mat = cgmath::Matrix4::<f32>::from_axis_angle(axis, angle) * self.model_mat;
        self.inverted_model_mat = self.model_mat.invert().unwrap();
    }
    pub fn apply_quarternion_rotation(&mut self, q: &cgmath::Quaternion<f32>) {
        let drot: Matrix4<f32> = (*q).into();
        
        self.model_mat = drot * self.model_mat;
        self.inverted_model_mat = self.model_mat.invert().unwrap();
    }
    pub fn set_model_mat(&mut self, model_mat: &cgmath::Matrix4<f32>) {
        self.model_mat = *model_mat;
        self.inverted_model_mat = self.model_mat.invert().unwrap();
    }

    /*pub fn scale(&mut self, factor: f32) {
        self.scale_mat = cgmath::Matrix4::<f32>::from_scale(factor);
        self.recompute_model_matrix();
    }*/

    pub fn get_model_mat(&self) -> &cgmath::Matrix4<f32> {
        return &self.model_mat; 
    }
    pub fn get_quat(&self) -> cgmath::Quaternion<f32> {
        // Extract a 3x3 matrix from the model 4x4 matrix
        let v: [[f32; 4]; 4] = self.model_mat.into();

        let mat3 = Matrix3::new(
            v[0][0], v[0][1], v[0][2],
            v[1][0], v[1][1], v[1][2],
            v[2][0], v[2][1], v[2][2]
        );

        mat3.into()
    }

    pub fn get_inverted_model_mat(&self) -> &cgmath::Matrix4<f32> {
        return &self.inverted_model_mat;
    }

    pub fn mesh(&self) -> &T {
        &self.mesh
    }

    pub fn mesh_mut(&mut self) -> &mut T {
        &mut self.mesh
    }

    /*pub fn update<P: Projection>(&mut self, viewport: &ViewPort) {
        let ref mut mesh = self.mesh;

        let ref local_to_world = self.model_mat;

        /*mesh.update::<P>(
            local_to_world,
            viewport
        );*/
    }*/

    /*pub fn draw(&self, shaders: &HashMap<&'static str, Shader>, viewport: &ViewPort) {
        let ref gl = self.gl;
        self.mesh.draw(gl, &self, shaders, viewport);
    }*/
}
