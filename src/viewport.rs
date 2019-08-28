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

use cgmath::{MetricSpace, InnerSpace, Vector2, SquareMatrix, Matrix};

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

        let view_mat = cgmath::Matrix4::identity();
        let mut viewport = ViewPort {
            view_mat,
            theta,
            phi,
            radius,
            width,
            height,
        };

        viewport.recompute_view_matrix();
        viewport
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
        self.phi += delta_phi;
        self.theta += delta_theta;

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
        /*let radius_min = 1_f32;
        let radius_max = 5_f32;
        if self.radius > radius_max {
            self.radius = radius_max;
        }

        if self.radius < radius_min {
            self.radius = radius_min;
        }

        let zoom_factor = (self.radius - radius_min) / (radius_max - radius_min);*/

        let ca = self.theta.cos(); // ca stands for cos(alpha)
        let sa = self.theta.sin(); // sa stands for sin(alpha)

        let cd = self.phi.cos(); // cd stands for cos(delta)
        let sd = self.phi.sin(); // sd stands for sin(delta)

        /*this.r11 =  ca * cd; this.r12 =  sa * cd; this.r13 = sd;
        this.r21 =      -sa; this.r22 =       ca; this.r23 =  0;
        this.r31 = -ca * sd; this.r32 = -sa * sd; this.r33 = cd;*/

        self.view_mat = cgmath::Matrix4::<f32>::new(
            ca * cd, -sa, -ca * sd, 0_f32,
            sa * cd, ca, -sa * sd, 0_f32,
            sd, 0_f32, cd, 0_f32, 
            0_f32, 0_f32, 0_f32, 1_f32
        )
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

        let radius_max = 5_f32;
        let radius_min = 1_f32;
        let zoom_total_amplitude = radius_max - radius_min;
        //let zoom_factor = 2_f32*((radius_max - self.radius)/(radius_max - radius_min));
        let zoom_factor = 1_f32 / (radius_max - self.radius);
        let zoom_factor = 1_f32;

        gl.uniform1f(zoom_factor_location.as_ref(), zoom_factor);
        gl.uniform1f(width_location.as_ref(), self.width);
        gl.uniform1f(height_location.as_ref(), self.height);
    }

    /// Screen space to world space transformation
    /// 
    /// # Arguments
    /// 
    /// * `x` - X mouse position in the screen space (in pixel)
    /// * `y` - Y mouse position in the screen space (in pixel)
    pub fn unproj(&self, x: f32, y: f32) -> Option<cgmath::Vector2<f32>> {
        console::log_1(&format!("x_off, y_off {:?} {:?}", x, y).into());
        // Screen space in pixels to homogeneous screen space (values between [-1, 1])
        // Change of origin
        let xo = (x - self.width/2_f32);
        let yo = (y - self.height/2_f32);

        // Scale to fit in [-1, 1]
        let xh = 2_f32*(xo/self.width);
        let yh = -2_f32*(yo/self.height);

        let aspect = self.width / self.height;
        let xw_2 = 1_f32 - xh*xh*aspect*aspect - yh*yh;
        console::log_1(&format!("xl, yl, zl {:?} {:?} {:?}", xh, yh, xw_2).into());
        if xw_2 > 0_f32 {
            let pos_local_space = cgmath::Vector4::new(xh, yh, xw_2.sqrt(), 1_f32);

            // Local space centered around the view camera to global space
            let mat_local_to_global = cgmath::Matrix4::<f32>::new(
                self.view_mat.x[0], self.view_mat.y[0], self.view_mat.z[0], self.view_mat.w[0],
                self.view_mat.x[1], self.view_mat.y[1], self.view_mat.z[1], self.view_mat.w[1], 
                self.view_mat.x[2], self.view_mat.y[2], self.view_mat.z[2], self.view_mat.w[2], 
                self.view_mat.x[3], self.view_mat.y[3], self.view_mat.z[3], self.view_mat.w[3],
            );

            let pos_global_space = mat_local_to_global * pos_local_space;
            console::log_1(&format!("position global {:?}", pos_global_space).into());

            // Get the (ra, dec) from XYZ coordinates
            let ra = pos_global_space.x.atan2(pos_global_space.z) as f32;
            let dec = pos_global_space.y.asin() as f32;

            console::log_1(&format!("ra {:?}, dec {:?}", ra, dec).into());

            Some(Vector2::<f32>::new(ra, dec))
        } else {
            // Out of the sphere
            None
        }
    }
}

fn compute_eye_position(radius: f32, theta: f32, phi: f32) -> cgmath::Point3<f32> {
    cgmath::Point3::new(
        radius * (-theta).sin() * phi.cos(),
        radius * phi.sin(),
        radius * (-theta).cos() * phi.cos(),
    )
}