pub struct ViewPort {
    projection_mat: cgmath::Matrix4<f32>,
    view_mat: cgmath::Matrix4<f32>,
}

use crate::shader::Shader;
use web_sys::WebGl2RenderingContext;
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

        let eye = cgmath::Point3::new(5_f32, 5_f32, 5_f32);
        let center = cgmath::Point3::new(0_f32, 0_f32, 0_f32);
        let up = cgmath::Vector3::new(0_f32, 1_f32, 0_f32);
        let view_mat = cgmath::Matrix4::<f32>::look_at(eye, center, up);

        ViewPort {
            projection_mat,
            view_mat
        }
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