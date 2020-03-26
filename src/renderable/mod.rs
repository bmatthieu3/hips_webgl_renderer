use crate::shader::Shader;

use crate::viewport::ViewPort;

pub mod hips_sphere;
pub mod projection;
pub mod grid;
pub mod catalog;
pub mod uv;

pub use hips_sphere::HiPSSphere;
pub use catalog::Catalog;
pub use grid::ProjetedGrid;

use crate::WebGl2Context;

use std::collections::HashMap;
pub trait Mesh {
    fn create_buffers(&mut self, gl: &WebGl2Context);
    fn get_shader<'a>(&self, shaders: &'a HashMap<&'static str, Shader>) -> &'a Shader;
}

pub trait DisableDrawing {
    fn disable(&mut self, viewport: &ViewPort);
}

pub struct Renderable<T>
where T: Mesh + DisableDrawing {
    //scale_mat: cgmath::Matrix4::<f32>,
    //rotation_mat: cgmath::Matrix4::<f32>,
    //translation_mat: cgmath::Matrix4::<f32>,

    mesh: T,

    gl: WebGl2Context,
}

use cgmath;
impl<T> Renderable<T>
where T: Mesh + DisableDrawing {
    pub fn new(gl: &WebGl2Context, shaders: &HashMap<&'static str, Shader>, mut mesh: T) -> Renderable<T> {
        let shader = mesh.get_shader(shaders);
        shader.bind(gl);
        mesh.create_buffers(gl);

        //let scale_mat = cgmath::Matrix4::identity();
        //let rotation_mat = cgmath::Matrix4::identity();
        //let translation_mat = cgmath::Matrix4::identity();

        let gl = gl.clone();
        Renderable {
            // And its submatrices
            //scale_mat,
            //rotation_mat,
            //translation_mat,

            mesh,
            gl,
        }
    }

    pub fn set_mesh<U: Mesh + DisableDrawing>(self, mesh: U) -> Renderable<U> {
        Renderable::<U> {
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

    pub fn mesh(&self) -> &T {
        &self.mesh
    }

    pub fn mesh_mut(&mut self) -> &mut T {
        &mut self.mesh
    }
}
