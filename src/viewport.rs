pub struct ViewPort {
    view_mat: cgmath::Matrix4<f32>,
    zoom_factor: f32,
    position: usize,
}

use crate::shader::Shader;
use crate::math;
use web_sys::WebGl2RenderingContext;

use cgmath::SquareMatrix;
use web_sys::console;

impl ViewPort {
    pub fn new() -> ViewPort {
        let view_mat = cgmath::Matrix4::identity();
        let zoom_factor = 1_f32;
        let position = 0;
        ViewPort {
            view_mat,
            zoom_factor,
            position,
        }
    }

    pub fn zoom(&mut self, current_depth: i32) {
        let depth_max = 8; // XMLHttpRequest to get the max depth of the survey
        /*if current_depth < depth_max {
            let n_step = 5;

            let fov_0 = math::depth_to_resolution(0);
            let fov_d = math::depth_to_resolution(current_depth);
            let fov_d_next = math::depth_to_resolution(current_depth + 1);

            let i = self.position % n_step;

            self.zoom_factor = (fov_0 / (fov_d * fov_d_next)) * (fov_d_next + (i as f32) * (fov_d_next - fov_d).abs()/((n_step - 1) as f32));

            //self.zoom_factor += 1_f32;
            console::log_1(&format!("zoom factor {:?}", self.zoom_factor).into());

            self.position += 1;
        }*/
        self.zoom_factor *= 1.2_f32;
    }

    pub fn unzoom(&mut self) {
        /*
        if self.zoom_factor > 1_f32 {
            self.position -= 1;

            self.zoom_factor -= 1_f32;
        }

        if self.zoom_factor < 1_f32 {
            self.zoom_factor = 1_f32;
        }*/
        self.zoom_factor /= 1.2_f32;
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