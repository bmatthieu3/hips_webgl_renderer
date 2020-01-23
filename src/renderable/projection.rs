// Screen space: pixels space between
// * x_px in [0, width-1]
// * y_px in [0, height-1]

// Homogeneous space
// * x_h in [-1, 1]
// * y_h in [-1, 1]

// World space
use crate::viewport::ViewPort;

use crate::window_size_f32;
use web_sys::console;

// Screen pixels whose origin is in the top-left corner
// To homogeneous coordinates system
pub fn screen_pixels_to_homogenous(screen_pos: Vector2<f32>) -> Vector2<f32> {
    // Screen space in pixels to homogeneous screen space (values between [-1, 1])
    let (width, height) = window_size_f32();
    // Change of origin
    let origin = screen_pos - Vector2::new(width, height)/2_f32;

    // Scale to fit in [-1, 1]
    Vector2::new(2_f32 * (origin.x/width), -2_f32 * (origin.y/height))
}

use cgmath::Vector4;
use cgmath::InnerSpace;
pub trait Projection {
    fn screen_to_world_space(screen_pos: Vector2<f32>, viewport: &ViewPort) -> Option<Vector4<f32>> {
        let homogeneous_pos = crate::projection::screen_pixels_to_homogenous(screen_pos);

        let scaling_screen_factor = viewport.get_scaling_screen_factor();
        let world_pos = Self::homogeneous_to_world_space(homogeneous_pos, scaling_screen_factor);
        if let Some(world_pos) = world_pos {
            let world_pos = world_pos.normalize();

            Some(world_pos)
        } else {
            None
        }
    }

    fn build_screen_map() -> Vec<cgmath::Vector2<f32>>;

    //fn scale_by_screen_ratio(x: f32, y: f32) -> (f32, f32);
    /// Screen space to world space transformation
    /// 
    /// This returns a normalized vector along its first 3 dimensions.
    /// Its fourth component is set to 1.
    /// 
    /// # Arguments
    /// 
    /// * `x` - X mouse position in homogenous screen space (between [-1, 1])
    /// * `y` - Y mouse position in homogenous screen space (between [-1, 1])
    fn homogeneous_to_world_space(pos: Vector2<f32>, scaling_screen_factor: &Vector2<f32>) -> Option<cgmath::Vector4<f32>>;
    /// World to screen space transformation
    /// 
    /// # Arguments
    /// 
    /// * `x` - X mouse position in homogenous screen space (between [-1, 1])
    /// * `y` - Y mouse position in homogenous screen space (between [-1, 1])
    fn world_to_screen_space(pos_world_space: cgmath::Vector4<f32>) -> Option<cgmath::Vector2<f32>>;

    /// Get the size in pixels on the screen 
    fn size() -> cgmath::Vector2<f32>;
}

#[derive(Clone, Copy)]
pub struct Aitoff;
#[derive(Clone, Copy)]
pub struct MollWeide;
#[derive(Clone, Copy)]
pub struct Orthographic;

pub static ORTHO_PROJ: Orthographic = Orthographic {};
pub static AITOFF_PROJ: Aitoff = Aitoff {};
pub static MOLLWEIDE_PROJ: MollWeide = MollWeide {};

use cgmath::Vector2;

use crate::renderable::hips_sphere::NUM_VERTICES_PER_STEP;
use crate::renderable::hips_sphere::NUM_STEPS;

use crate::math::is_inside_ellipse;
impl Projection for Aitoff {
    fn build_screen_map() -> Vec<cgmath::Vector2<f32>> {
        let mut vertices_screen = Vec::with_capacity(2*(NUM_VERTICES_PER_STEP*NUM_STEPS + 1) as usize);

        let (width, height) = window_size_f32();

        let center_screen_space = Vector2::<f32>::new(width / 2_f32, height / 2_f32);
        vertices_screen.push(center_screen_space);

        for j in 0..NUM_STEPS {
            let radius = (std::f32::consts::PI * ((j + 1) as f32) / (2_f32 * (NUM_STEPS as f32))).sin();
            for i in 0..NUM_VERTICES_PER_STEP {
                let angle = (i as f32) * 2_f32 * std::f32::consts::PI / (NUM_VERTICES_PER_STEP as f32);

                let pos_screen_space = Vector2::<f32>::new(
                    (width/2_f32 - 1_f32) * radius * angle.cos(),
                    ((width/2_f32 - 1_f32) / 2_f32) * radius * angle.sin()
                );

                vertices_screen.push(pos_screen_space + center_screen_space);
            }
        }

        vertices_screen
    }

    /// View to world space transformation
    /// 
    /// This returns a normalized vector along its first 3 dimensions.
    /// Its fourth component is set to 1.
    /// 
    /// The Aitoff projection maps screen coordinates from [-pi; pi] x [-pi/2; pi/2]
    /// 
    /// # Arguments
    /// 
    /// * `x` - in normalized device coordinates between [-1; 1]
    /// * `y` - in normalized device coordinates between [-1; 1]
    fn homogeneous_to_world_space(pos: Vector2<f32>, scaling_screen_factor: &Vector2<f32>) -> Option<cgmath::Vector4<f32>> {
        let pos = Vector2::new(pos.x * scaling_screen_factor.x, pos.y * scaling_screen_factor.y);

        let a = 1_f32;
        let b = 0.5_f32;
        if is_inside_ellipse(&pos, a, b) {
            let u = pos.x * std::f32::consts::PI * 0.5_f32;
            let v = pos.y * std::f32::consts::PI;
            //da uv a lat/lon
            let c = (v*v + u*u).sqrt();

            let (phi, mut theta) = if c != 0_f32 {
                let phi = (v * c.sin() / c).asin();
                let theta = (u * c.sin()).atan2(c * c.cos());
                (phi, theta)
            } else {
                let phi = v.asin();
                let theta = u.atan();
                (phi, theta)
            };
            theta *= 2_f32;

            let pos_world_space = cgmath::Vector4::new(
                -theta.sin() * phi.cos(),
                phi.sin(),
                theta.cos() * phi.cos(),
                1_f32
            );

            Some(pos_world_space)
        } else {
            None
        }
    }

    /// World to screen space transformation
    /// X is between [-1, 1]
    /// Y is between [-0.5, 0.5]
    /// 
    /// # Arguments
    /// 
    /// * `pos_world_space` - Position in the world space. Must be a normalized vector
    fn world_to_screen_space(pos_world_space: cgmath::Vector4<f32>) -> Option<cgmath::Vector2<f32>> {
        // X in [-1, 1]
        // Y in [-1/2; 1/2] and scaled by the screen width/height ratio
        //return vec3(X / PI, aspect * Y / PI, 0.f);
        let xyz = Vector3::new(pos_world_space.x, pos_world_space.y, pos_world_space.z);
        let (theta, delta) = math::xyz_to_radec(xyz);

        let theta_by_two = theta / 2_f32;

        let alpha = (delta.cos() * theta_by_two.cos()).acos();
        let inv_sinc_alpha = if alpha < 1e-3 {
            1_f32
        } else {
            alpha / alpha.sin()
        };

        // The minus is an astronomical convention.
        // longitudes are increasing from right to left
        let x = -2_f32 * inv_sinc_alpha * delta.cos() * theta_by_two.sin();
        let y = inv_sinc_alpha * delta.sin();

        Some(cgmath::Vector2::new(x / std::f32::consts::PI, y / std::f32::consts::PI))
    }

    fn size() -> cgmath::Vector2<f32> {
        let (width, _) = window_size_f32();

        cgmath::Vector2::new(
            width - 2_f32,
            width/2_f32 - 1_f32
        )
    }
}

use cgmath::Vector3;
use crate::math;
impl Projection for MollWeide {
    fn build_screen_map() -> Vec<cgmath::Vector2<f32>> {
        let mut vertices_screen = Vec::with_capacity(2*(NUM_VERTICES_PER_STEP*NUM_STEPS + 1) as usize);

        let (width, height) = window_size_f32();

        let center_screen_space = Vector2::<f32>::new(width / 2_f32, height / 2_f32);
        vertices_screen.push(center_screen_space);

        for j in 0..NUM_STEPS {
            let radius = (std::f32::consts::PI * ((j + 1) as f32) / (2_f32 * (NUM_STEPS as f32))).sin();
            for i in 0..NUM_VERTICES_PER_STEP {
                let angle = (i as f32) * 2_f32 * std::f32::consts::PI / (NUM_VERTICES_PER_STEP as f32);

                let mut pos_screen_space = Vector2::<f32>::new(
                    (width/2_f32 - 1_f32) * radius * angle.cos(),
                    ((width/2_f32 - 1_f32) / 2_f32) * radius * angle.sin()
                );

                vertices_screen.push(pos_screen_space + center_screen_space);
            }
        }

        vertices_screen
    }

    /// View to world space transformation
    /// 
    /// This returns a normalized vector along its first 3 dimensions.
    /// Its fourth component is set to 1.
    /// 
    /// The Aitoff projection maps screen coordinates from [-pi; pi] x [-pi/2; pi/2]
    /// 
    /// # Arguments
    /// 
    /// * `x` - in normalized device coordinates between [-1; 1]
    /// * `y` - in normalized device coordinates between [-1; 1]
    fn homogeneous_to_world_space(pos: Vector2<f32>, scaling_screen_factor: &Vector2<f32>) -> Option<cgmath::Vector4<f32>> {
        let pos = Vector2::new(pos.x * scaling_screen_factor.x, pos.y * scaling_screen_factor.y);

        let a = 1_f32;
        let b = 0.5_f32;
        if is_inside_ellipse(&pos, a, b) {
            let y2 = pos.y * pos.y;
            let k = (1_f32 - 4_f32 * y2).sqrt();

            let theta = std::f32::consts::PI * pos.x / k;
            let delta = ((2_f32 * (2_f32 * pos.y).asin() + 4_f32 * pos.y * k) / std::f32::consts::PI).asin();

            // The minus is an astronomical convention.
            // longitudes are increasing from right to left
            let pos_world_space = cgmath::Vector4::new(
                -theta.sin() * delta.cos(),
                delta.sin(),
                theta.cos() * delta.cos(),
                1_f32
            );

            Some(pos_world_space)
        } else {
            None
        }
    }

    /// World to screen space transformation
    /// X is between [-1, 1]
    /// Y is between [-0.5, 0.5]
    /// 
    /// # Arguments
    /// 
    /// * `pos_world_space` - Position in the world space. Must be a normalized vector
    fn world_to_screen_space(pos_world_space: cgmath::Vector4<f32>) -> Option<cgmath::Vector2<f32>> {
        // X in [-1, 1]
        // Y in [-1/2; 1/2] and scaled by the screen width/height ratio
        //return vec3(X / PI, aspect * Y / PI, 0.f);
        let epsilon = 1e-3;
        let max_iter = 10;

        let xyz = Vector3::new(pos_world_space.x, pos_world_space.y, pos_world_space.z);
        let (lon, lat) = math::xyz_to_radec(xyz);
 
        let cst = std::f32::consts::PI * lat.sin();

        let mut theta = lat;
        let mut f = theta + theta.sin() - cst;

        let mut k = 0;
        while f.abs() > epsilon && k < max_iter {
            theta -= f / (1_f32 + theta.cos());
            f = theta + theta.sin() - cst;

            k = k + 1;
        }

        theta /= 2_f32;

        // The minus is an astronomical convention.
        // longitudes are increasing from right to left
        let x = -(lon / std::f32::consts::PI) * theta.cos();
        let y = 0.5_f32 * theta.sin();

        Some(cgmath::Vector2::new(x, y))
    }

    fn size() -> cgmath::Vector2<f32> {
        let (width, _) = window_size_f32();

        cgmath::Vector2::new(
            width - 2_f32,
            width/2_f32 - 1_f32
        )
    }
}

impl Projection for Orthographic {
    fn build_screen_map() -> Vec<cgmath::Vector2<f32>> {
        let mut vertices_screen = Vec::with_capacity(2*(NUM_VERTICES_PER_STEP*NUM_STEPS + 1) as usize);

        let (width, height) = window_size_f32();

        let center_screen_space = Vector2::<f32>::new(width / 2_f32, height / 2_f32);
        vertices_screen.push(center_screen_space);

        for j in 0..NUM_STEPS {
            let radius = (std::f32::consts::PI * ((j + 1) as f32) / (2_f32 * (NUM_STEPS as f32))).sin();
            for i in 0..NUM_VERTICES_PER_STEP {
                let angle = (i as f32) * 2_f32 * std::f32::consts::PI / (NUM_VERTICES_PER_STEP as f32);

                let pos_screen_space = Vector2::<f32>::new(
                    (height/2_f32 - 1_f32) * radius * angle.cos(),
                    (height/2_f32 - 1_f32) * radius * angle.sin()
                );

                vertices_screen.push(pos_screen_space + center_screen_space);
            }
        }

        vertices_screen
    }

    /// View to world space transformation
    /// 
    /// This returns a normalized vector along its first 3 dimensions.
    /// Its fourth component is set to 1.
    /// 
    /// The Aitoff projection maps screen coordinates from [-pi; pi] x [-pi/2; pi/2]
    /// 
    /// # Arguments
    /// 
    /// * `x` - in normalized device coordinates between [-1; 1]
    /// * `y` - in normalized device coordinates between [-1; 1]
    fn homogeneous_to_world_space(pos: Vector2<f32>, scaling_screen_factor: &Vector2<f32>) -> Option<cgmath::Vector4<f32>> {
        let pos = Vector2::new(pos.x * scaling_screen_factor.x, pos.y * scaling_screen_factor.y);

        let xw_2 = 1_f32 - pos.x*pos.x - pos.y*pos.y;
        if xw_2 > 0_f32 {
            let pos_world_space = cgmath::Vector4::new(-pos.x, pos.y, xw_2.sqrt(), 1_f32);

            Some(pos_world_space)
        } else {
            // Out of the sphere
            None
        }
    }

    /// World to screen space transformation
    /// 
    /// # Arguments
    /// 
    /// * `pos_world_space` - Position in the world space. Must be a normalized vector
    fn world_to_screen_space(pos_world_space: cgmath::Vector4<f32>) -> Option<cgmath::Vector2<f32>> {
        let (width, height) = window_size_f32();
        /*let aspect = height / width;
        Some(cgmath::Vector2::new(
            -pos_world_space.x,
            pos_world_space.y,
        ) * aspect)*/
        Some(
            cgmath::Vector2::new(
                -pos_world_space.x,
                pos_world_space.y,
            )
        )
    }

    fn size() -> cgmath::Vector2<f32> {
        let (_, height) = window_size_f32();

        cgmath::Vector2::new(
            height - 2_f32,
            height - 2_f32
        )
    }
}