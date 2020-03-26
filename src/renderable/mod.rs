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
pub trait Renderable {
    /*fn bind_buffers(&mut self, gl: &WebGl2Context, shaders: &HashMap<&'static str, Shader>) {
        let shader = self.get_shader(shaders);

        shader.bind(gl);
        self.create_buffers(gl);
    }*/

    fn create_buffers(&mut self, gl: &WebGl2Context, shaders: &HashMap<&'static str, Shader>);
    //fn get_shader<'a>(&self, shaders: &'a HashMap<&'static str, Shader>) -> &'a Shader;
}

pub trait DisableDrawing {
    fn disable(&mut self, viewport: &ViewPort);
}
/*
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
}
*/