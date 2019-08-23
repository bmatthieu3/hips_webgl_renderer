pub struct ViewPort {
    projection_mat: cgmath::Matrix4<f32>,
    view_mat: cgmath::Matrix4<f32>,

    eye: cgmath::Point3<f32>,
    center: cgmath::Point3<f32>,
}

use crate::shader::Shader;
use web_sys::WebGl2RenderingContext;

use cgmath::{MetricSpace, InnerSpace};

use web_sys::console;
impl ViewPort {
    pub fn new(width: f32, height: f32) -> ViewPort {
        let fovy = cgmath::Deg(60_f32);

        let aspect = width / height;
        let near = 0.1_f32;
        let far = 50_f32;
        let projection_mat: cgmath::Matrix4<f32> = cgmath::perspective(
            fovy,
            aspect,
            near,
            far
        );

        let eye = cgmath::Point3::new(1.5_f32, 1.5_f32, 1.5_f32);
        let center = cgmath::Point3::new(0_f32, 0_f32, 0_f32);
        let up = cgmath::Vector3::new(0_f32, 1_f32, 0_f32);
        let view_mat = cgmath::Matrix4::<f32>::look_at(eye, center, up);

        ViewPort {
            projection_mat,
            view_mat,
            eye,
            center,
        }
    }

    pub fn zoom(&mut self, forward: bool) {
        // Compute the distance from the center
        let distance_to_center = self.center.distance(self.eye);

        if distance_to_center > 5_f32 && !forward {
            return;
        }

        let zoom_factor = (distance_to_center - 1.10_f32) * 0.08;

        let eye_to_center = (self.center - self.eye).normalize();

        if forward {
            self.eye = self.eye + eye_to_center * zoom_factor;
        } else {
            self.eye = self.eye - eye_to_center * zoom_factor;
        }

        // Recompute the view matrix
        self.recompute_view_matrix();
    }

    pub fn recompute_view_matrix(&mut self) {
        let up = cgmath::Vector3::new(0_f32, 1_f32, 0_f32);
        self.view_mat = cgmath::Matrix4::<f32>::look_at(self.eye, self.center, up);
    }

    pub fn send_to_vertex_shader(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        let view_mat_location = shader.get_uniform_location(gl, "view");
        let proj_mat_location = shader.get_uniform_location(gl, "projection");

        let view_mat_f32_slice: &[f32; 16] = self.view_mat.as_ref();
        gl.uniform_matrix4fv_with_f32_array(view_mat_location.as_ref(), false, view_mat_f32_slice);
        let proj_mat_f32_slice: &[f32; 16] = self.projection_mat.as_ref();
        gl.uniform_matrix4fv_with_f32_array(proj_mat_location.as_ref(), false, proj_mat_f32_slice);
    }
}