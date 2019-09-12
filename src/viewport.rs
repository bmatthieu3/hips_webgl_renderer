pub struct ViewPort {
    view_mat: cgmath::Matrix4<f32>,
    zoom_factor: f32,
}

use crate::shader::Shader;
use web_sys::WebGl2RenderingContext;

use cgmath::SquareMatrix;

impl ViewPort {
    pub fn new() -> ViewPort {
        let view_mat = cgmath::Matrix4::identity();
        let zoom_factor = 1_f32;
        ViewPort {
            view_mat,
            zoom_factor,
        }
    }

    pub fn zoom(&mut self) {
        self.zoom_factor += 0.5_f32;
    }

    pub fn unzoom(&mut self) {
        self.zoom_factor -= 0.5_f32;

        if self.zoom_factor < 1_f32 {
            self.zoom_factor = 1_f32;
        }
    }

    pub fn get_zoom_factor(&self) -> f32 {
        self.zoom_factor
    }

    pub fn apply_transformation(&mut self, t: cgmath::Matrix4<f32>) {
        self.view_mat = t * self.view_mat;
    }

    pub fn send_to_vertex_shader(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        let view_mat_location = shader.get_uniform_location(gl, "view");
        let zoom_factor_location = shader.get_uniform_location(gl, "zoom_factor");

        // Send view matrix
        let view_mat_f32_slice: &[f32; 16] = self.view_mat.as_ref();
        gl.uniform_matrix4fv_with_f32_array(view_mat_location.as_ref(), false, view_mat_f32_slice);

        // Send zoom factor
        gl.uniform1f(zoom_factor_location.as_ref(), self.zoom_factor);
    }
}