use crate::shader::Shader;

use crate::viewport::ViewPort;

mod hips_sphere;
pub mod projection;
pub mod grid;

pub mod catalog;
pub use catalog::CatalogShaderProjection;

pub mod uv;

mod ray_tracer;
use ray_tracer::RayTracingProjection;
use ray_tracer::RayTracer;

mod rasterizer;
use rasterizer::Rasterizer;
use rasterizer::RasterizerProjection;

use hips_sphere::RenderingMode;
pub use hips_sphere::HiPSSphere;
pub use catalog::Catalog;
pub use grid::ProjetedGrid;

use crate::WebGl2Context;

use std::collections::HashMap;

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