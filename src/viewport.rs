pub struct ViewPort {
    //projection_mat: cgmath::Matrix4<f32>,
    view_mat: cgmath::Matrix4<f32>,

    //eye: cgmath::Point3<f32>,
    //center: cgmath::Point3<f32>,

    theta: f32,
    phi: f32,
    radius: f32,

    width: f32,
    height: f32,
}

use crate::shader::Shader;
use web_sys::WebGl2RenderingContext;

use cgmath::{MetricSpace, InnerSpace};

use web_sys::console;
impl ViewPort {
    pub fn new(width: f32, height: f32) -> ViewPort {
        /*let fovy = cgmath::Deg(60_f32);

        let aspect = width / height;
        let near = 0.1_f32;
        let far = 50_f32;
        let projection_mat: cgmath::Matrix4<f32> = cgmath::perspective(
            fovy,
            aspect,
            near,
            far
        );*/

        let radius = 1.5_f32;
        let theta = 0_f32;
        let phi = 0_f32;

        //let eye = compute_eye_position(radius, theta, phi);
        //let center = cgmath::Point3::new(0_f32, 0_f32, 0_f32);
        //let up = cgmath::Vector3::new(0_f32, 1_f32, 0_f32);
        //let view_mat = cgmath::Matrix4::<f32>::look_at(eye, center, up);

        /*let ca = cos(lon); // ca stands for cos(alpha)
        final double sa = sin(lon); // sa stands for sin(alpha)
        final double cd = cos(lat); // cd stands for cos(delta)
        final double sd = sin(lat); // sd stands for sin(delta)
        this.r11 =  ca * cd; this.r12 =  sa * cd; this.r13 = sd;
        this.r21 =      -sa; this.r22 =       ca; this.r23 =  0;
        this.r31 = -ca * sd; this.r32 = -sa * sd; this.r33 = cd;

        let view_mat = cgmath::Matrix4::<f32>::new();*/

        let view_mat = compute_local_frame_matrix(theta, phi, radius);

        ViewPort {
            view_mat,
            theta,
            phi,
            radius,
            width,
            height,
        }
    }

    pub fn zoom(&mut self, forward: bool) {
        // Compute the distance from the center
        //let distance_to_center = self.center.distance(self.eye);

        //if distance_to_center > 5_f32 && !forward {
        //    return;
        //}

        //let zoom_factor = (distance_to_center - 1.10_f32) * 0.08;

        //let eye_to_center = (self.center - self.eye).normalize();

        if forward {
            self.radius = self.radius - 0.05;
        } else {
            self.radius = self.radius + 0.05;
        }
        //self.eye = compute_eye_position(self.radius, self.theta, self.phi);

        // Recompute the view matrix
        self.recompute_view_matrix();
    }

    pub fn move_eye_position(&mut self, delta_theta: f32, delta_phi: f32) {
        self.phi += delta_phi * 0.1;
        self.theta += delta_theta * 0.1;

        let phi_max = (std::f32::consts::PI / 2_f32) - 0.01_f32;
        if self.phi > phi_max {
            self.phi = phi_max;
        } else if self.phi < -phi_max {
            self.phi = -phi_max;
        }

        //self.eye = compute_eye_position(self.radius, self.theta, self.phi);
        self.recompute_view_matrix();
    }

    pub fn recompute_view_matrix(&mut self) {
        /*let up = cgmath::Vector3::new(0_f32, 1_f32, 0_f32);
        self.view_mat = cgmath::Matrix4::<f32>::look_at(self.eye, self.center, up);*/
        let min_radius = 1_f32;
        let max_radius = 5_f32;
        if self.radius > max_radius {
            self.radius = max_radius;
        }

        if self.radius < min_radius {
            self.radius = min_radius;
        }

        let zoom_factor = (self.radius - min_radius) / (max_radius - min_radius);

        self.view_mat = compute_local_frame_matrix(self.theta, self.phi, zoom_factor);
    }

    pub fn send_to_vertex_shader(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        let view_mat_location = shader.get_uniform_location(gl, "view");
        //let proj_mat_location = shader.get_uniform_location(gl, "projection");
        let zoom_factor_location = shader.get_uniform_location(gl, "zoom_factor");

        let width_location = shader.get_uniform_location(gl, "width");
        let height_location = shader.get_uniform_location(gl, "height");

        let view_mat_f32_slice: &[f32; 16] = self.view_mat.as_ref();
        gl.uniform_matrix4fv_with_f32_array(view_mat_location.as_ref(), false, view_mat_f32_slice);
        //let proj_mat_f32_slice: &[f32; 16] = self.projection_mat.as_ref();
        //gl.uniform_matrix4fv_with_f32_array(proj_mat_location.as_ref(), false, proj_mat_f32_slice);

        /*let zoom_total_amplitude = 5_f32 - 1_f32;
        let zoom_factor = (-(self.center.distance(self.eye) - 1_f32)/zoom_total_amplitude) + 1_f32;*/

        gl.uniform1f(zoom_factor_location.as_ref(), self.radius);
        gl.uniform1f(width_location.as_ref(), self.width);
        gl.uniform1f(height_location.as_ref(), self.height);
    }
}

fn compute_eye_position(radius: f32, theta: f32, phi: f32) -> cgmath::Point3<f32> {
    cgmath::Point3::new(
        radius * (-theta).sin() * phi.cos(),
        radius * phi.sin(),
        radius * (-theta).cos() * phi.cos(),
    )
}

fn compute_local_frame_matrix(theta: f32, alpha: f32, zoom_factor: f32) -> cgmath::Matrix4<f32> {
    let ca = theta.cos(); // ca stands for cos(alpha)
    let sa = theta.sin(); // sa stands for sin(alpha)

    let cd = alpha.cos(); // cd stands for cos(delta)
    let sd = alpha.sin(); // sd stands for sin(delta)

    /*this.r11 =  ca * cd; this.r12 =  sa * cd; this.r13 = sd;
    this.r21 =      -sa; this.r22 =       ca; this.r23 =  0;
    this.r31 = -ca * sd; this.r32 = -sa * sd; this.r33 = cd;*/

    cgmath::Matrix4::<f32>::new(
        ca * cd, -sa, -ca * sd, 0_f32,
        sa * cd, ca, -sa * sd, 0_f32,
        sd, 0_f32, cd, 0_f32, 
        0_f32, 0_f32, 0_f32, 1_f32
    )
}
