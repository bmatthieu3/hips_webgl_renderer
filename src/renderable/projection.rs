// Screen space: pixels space between
// * x_px in [0, width-1]
// * y_px in [0, height-1]

// Homogeneous space
// * x_h in [-1, 1]
// * y_h in [-1, 1]

// World space
use crate::viewport::ViewPort;

use web_sys::console;

pub fn screen_to_ndc_space(pos_screen_space: Vector2<f32>, viewport: &ViewPort) -> Vector2<f32> {
    // Screen space in pixels to homogeneous screen space (values between [-1, 1])
    let window_size = viewport.get_window_size();
    // Change of origin
    let origin = pos_screen_space - window_size/2_f32;

    // Scale to fit in [-1, 1]
    let pos_normalized_device = Vector2::new(2_f32 * (origin.x/window_size.x), -2_f32 * (origin.y/window_size.y));
    pos_normalized_device
}
pub fn ndc_to_screen_space(pos_normalized_device: Vector2<f32>, viewport: &ViewPort) -> Vector2<f32> {
    let window_size = viewport.get_window_size();

    let pos_screen_space = Vector2::new(
        (pos_normalized_device.x * 0.5_f32 + 0.5_f32) * window_size.x,
        (0.5_f32 - pos_normalized_device.y * 0.5_f32) * window_size.y,
    );

    pos_screen_space
}
pub fn clip_to_screen_space(pos_clip_space: Vector2<f32>, viewport: &ViewPort) -> Vector2<f32> {
    let ndc_to_clip = viewport.get_ndc_to_clip();
    let clip_zoom_factor = viewport.get_clip_zoom_factor();
    
    let pos_normalized_device = Vector2::new(
        pos_clip_space.x / (ndc_to_clip.x * clip_zoom_factor),
        pos_clip_space.y / (ndc_to_clip.y * clip_zoom_factor),
    );

    let window_size = viewport.get_window_size();
    let pos_screen_space = Vector2::new(
        (pos_normalized_device.x * 0.5_f32 + 0.5_f32) * window_size.x,
        (0.5_f32 - pos_normalized_device.y * 0.5_f32) * window_size.y,
    );

    pos_screen_space
}

pub fn screen_to_clip_space(pos_screen_space: Vector2<f32>, viewport: &ViewPort) -> Vector2<f32> {
    let pos_normalized_device = screen_to_ndc_space(pos_screen_space, viewport);

    ndc_to_clip_space(pos_normalized_device, viewport)
}

pub fn ndc_to_clip_space(pos_normalized_device: Vector2<f32>, viewport: &ViewPort) -> Vector2<f32> {
    let ndc_to_clip = viewport.get_ndc_to_clip();
    let clip_zoom_factor = viewport.get_clip_zoom_factor();

    let pos_clip_space = Vector2::new(
        pos_normalized_device.x * ndc_to_clip.x * clip_zoom_factor,
        pos_normalized_device.y * ndc_to_clip.y * clip_zoom_factor,
    );

    pos_clip_space
}

fn create_index_array() -> Vec<u16> {
    let mut indices = Vec::with_capacity(3 * NUM_VERTICES_PER_STEP * NUM_STEPS);

    for j in 0..NUM_STEPS {
        if j == 0 {
            for i in 1..NUM_VERTICES_PER_STEP {
                indices.push(0 as u16);
                indices.push((i + 1) as u16);
                indices.push(i as u16);
            }
            
            indices.push(0 as u16);
            indices.push(1 as u16);
            indices.push(NUM_VERTICES_PER_STEP as u16);
        } else {
            for i in 0..NUM_VERTICES_PER_STEP {
                let start_p_idx = (j - 1) * NUM_VERTICES_PER_STEP + i + 1;
                let next_p_idx = if i + 1 == NUM_VERTICES_PER_STEP {
                    (j - 1) * NUM_VERTICES_PER_STEP + 1
                } else {
                    (j - 1) * NUM_VERTICES_PER_STEP + i + 2
                };

                let start_c_idx = j * NUM_VERTICES_PER_STEP + i + 1;
                let next_c_idx = if i + 1 == NUM_VERTICES_PER_STEP {
                    j * NUM_VERTICES_PER_STEP + 1
                } else {
                    j * NUM_VERTICES_PER_STEP + i + 2
                };

                // Triangle touching the prec circle
                indices.push(start_p_idx as u16);
                indices.push(next_p_idx as u16);
                indices.push(start_c_idx as u16);
                // Triangle touching the next circle
                indices.push(start_c_idx as u16);
                indices.push(next_p_idx as u16);
                indices.push(next_c_idx as u16);
            }
        }
    }

    indices
}

use cgmath::Vector4;
use cgmath::InnerSpace;
use cgmath::Deg;

use std::collections::BTreeSet;
use crate::field_of_view::HEALPixCell;
use crate::field_of_view::{ALLSKY_ZERO_DEPTH, ALLSKY_ONE_DEPTH};
pub trait Projection {
    /// Screen to world space deprojection

    /// Perform a screen to the world space deprojection
    /// 
    /// # Arguments
    /// 
    /// * ``pos_screen_space`` - The position in the screen pixel space (top-left of the screen being the origin
    /// * ``viewport`` - The viewport object
    fn screen_to_world_space(pos_screen_space: Vector2<f32>, viewport: &ViewPort) -> Option<Vector4<f32>> {
        let pos_normalized_device = crate::projection::screen_to_ndc_space(pos_screen_space, viewport);

        let ndc_to_clip = viewport.get_ndc_to_clip();
        let clip_zoom_factor = viewport.get_clip_zoom_factor();

        let pos_clip_space = Vector2::new(
            pos_normalized_device.x * ndc_to_clip.x * clip_zoom_factor,
            pos_normalized_device.y * ndc_to_clip.y * clip_zoom_factor,
        );
        let world_pos_space = Self::clip_to_world_space(pos_clip_space);
        if let Some(world_pos_space) = world_pos_space {
            let world_pos_space = world_pos_space.normalize();

            Some(world_pos_space)
        } else {
            None
        }
    }

    /// Perform a clip to the world space deprojection
    /// 
    /// # Arguments
    /// 
    /// * ``pos_clip_space`` - The position in the clipping space (orthonorlized space)
    fn clip_to_world_space(pos_clip_space: Vector2<f32>) -> Option<Vector4<f32>>;

    /// World to screen space projection

    /// World to screen space transformation
    /// 
    /// # Arguments
    /// 
    /// * `x` - X mouse position in homogenous screen space (between [-1, 1])
    /// * `y` - Y mouse position in homogenous screen space (between [-1, 1])
    fn world_to_normalized_device_space(pos_world_space: Vector4<f32>, viewport: &ViewPort) -> Vector2<f32> {
        let pos_clip_space = Self::world_to_clip_space(pos_world_space);

        let ndc_to_clip = viewport.get_ndc_to_clip();
        let clip_zoom_factor = viewport.get_clip_zoom_factor();

        let pos_normalized_device = Vector2::new(
            pos_clip_space.x / (ndc_to_clip.x * clip_zoom_factor),
            pos_clip_space.y / (ndc_to_clip.y * clip_zoom_factor),
        );
        pos_normalized_device
    }

    fn world_to_screen_space(pos_world_space: Vector4<f32>, viewport: &ViewPort) -> Vector2<f32> {
        let pos_normalized_device = Self::world_to_normalized_device_space(pos_world_space, viewport);
        crate::projection::ndc_to_screen_space(pos_normalized_device, viewport)
    }
    /// World to the clipping space deprojection
    /// 
    /// # Arguments
    /// 
    /// * ``pos_world_space`` - The position in the world space
    fn world_to_clip_space(pos_world_space: Vector4<f32>) -> Vector2<f32>;


    /// Build a triangulation map in the screen pixel space of the projection
    /// (used in the per pixel rendering mode for 2D projections!)
    fn build_screen_map(viewport: &ViewPort) -> (Vec<Vector2<f32>>, Vec<u16>);

    // Aperture angle at the start of the application (full view)
    // - 180 degrees for the 3D projections (i.e. ortho)
    // - 360 degrees for the 2D projections (i.e. mollweide, arc, aitoff...)
    fn aperture_start() -> Deg<f32>;

    fn name() -> &'static str;

    fn check_for_allsky_fov(depth: u8) -> Option<BTreeSet<HEALPixCell>>;
}

#[derive(Clone, Copy)]
pub struct Aitoff;
#[derive(Clone, Copy)]
pub struct MollWeide;
#[derive(Clone, Copy)]
pub struct Orthographic;
#[derive(Clone, Copy)]
pub struct AzimutalEquidistant;
#[derive(Clone, Copy)]
pub struct Mercator;

use cgmath::Vector2;

use crate::renderable::hips_sphere::NUM_VERTICES_PER_STEP;
use crate::renderable::hips_sphere::NUM_STEPS;

use crate::math::is_inside_ellipse;
impl Projection for Aitoff {
    fn check_for_allsky_fov(depth: u8) -> Option<BTreeSet<HEALPixCell>> {
        if depth == 0 {
            Some(ALLSKY_ZERO_DEPTH.lock().unwrap().clone())
        } else if depth == 1 {
            Some(ALLSKY_ONE_DEPTH.lock().unwrap().clone())
        } else {
            None
        }
    }

    fn name() -> &'static str {
        "Aitoff"
    }
    fn build_screen_map(viewport: &ViewPort) -> (Vec<Vector2<f32>>, Vec<u16>) {
        let mut vertices_screen = Vec::with_capacity(2*(NUM_VERTICES_PER_STEP*NUM_STEPS + 1) as usize);

        let window_size = viewport.get_window_size();

        let center_screen_space = window_size / 2_f32;
        vertices_screen.push(center_screen_space);

        for j in 0..NUM_STEPS {
            let radius = (std::f32::consts::PI * ((j + 1) as f32) / (2_f32 * (NUM_STEPS as f32))).sin();
            for i in 0..NUM_VERTICES_PER_STEP {
                let angle = (i as f32) * 2_f32 * std::f32::consts::PI / (NUM_VERTICES_PER_STEP as f32);

                let pos_screen_space = Vector2::<f32>::new(
                    (window_size.x/2_f32 - 1_f32) * radius * angle.cos(),
                    ((window_size.x/2_f32 - 1_f32) / 2_f32) * radius * angle.sin()
                );

                vertices_screen.push(pos_screen_space + center_screen_space);
            }
        }

        (vertices_screen, create_index_array())
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
    fn clip_to_world_space(pos_clip_space: Vector2<f32>) -> Option<cgmath::Vector4<f32>> {
        let a = 1_f32;
        let b = 0.5_f32;
        if is_inside_ellipse(&pos_clip_space, a, b) {
            let u = pos_clip_space.x * std::f32::consts::PI * 0.5_f32;
            let v = pos_clip_space.y * std::f32::consts::PI;
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
    fn world_to_clip_space(pos_world_space: cgmath::Vector4<f32>) -> Vector2<f32> {
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

        Vector2::new(x / std::f32::consts::PI, y / std::f32::consts::PI)
    }

    fn aperture_start() -> Deg<f32> {
        Deg(360_f32)
    }
}

use cgmath::Vector3;
use crate::math;
impl Projection for MollWeide {
    fn check_for_allsky_fov(depth: u8) -> Option<BTreeSet<HEALPixCell>> {
        if depth == 0 {
            Some(ALLSKY_ZERO_DEPTH.lock().unwrap().clone())
        } else if depth == 1 {
            Some(ALLSKY_ONE_DEPTH.lock().unwrap().clone())
        } else {
            None
        }
    }

    fn name() -> &'static str {
        "MollWeide"
    }

    fn build_screen_map(viewport: &ViewPort) -> (Vec<Vector2<f32>>, Vec<u16>) {
        let mut vertices_screen = Vec::with_capacity(2*(NUM_VERTICES_PER_STEP*NUM_STEPS + 1) as usize);

        let window_size = viewport.get_window_size();

        let center_screen_space = window_size / 2_f32;
        vertices_screen.push(center_screen_space);

        for j in 0..NUM_STEPS {
            let radius = (std::f32::consts::PI * ((j + 1) as f32) / (2_f32 * (NUM_STEPS as f32))).sin();
            for i in 0..NUM_VERTICES_PER_STEP {
                let angle = (i as f32) * 2_f32 * std::f32::consts::PI / (NUM_VERTICES_PER_STEP as f32);

                let mut pos_screen_space = Vector2::<f32>::new(
                    (window_size.x/2_f32 - 1_f32) * radius * angle.cos(),
                    ((window_size.x/2_f32 - 1_f32) / 2_f32) * radius * angle.sin()
                );

                vertices_screen.push(pos_screen_space + center_screen_space);
            }
        }

        (vertices_screen, create_index_array())
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
    fn clip_to_world_space(pos_clip_space: Vector2<f32>) -> Option<cgmath::Vector4<f32>> {
        let a = 1_f32;
        let b = 0.5_f32;
        if is_inside_ellipse(&pos_clip_space, a, b) {
            let y2 = pos_clip_space.y * pos_clip_space.y;
            let k = (1_f32 - 4_f32 * y2).sqrt();

            let theta = std::f32::consts::PI * pos_clip_space.x / k;
            let delta = ((2_f32 * (2_f32 * pos_clip_space.y).asin() + 4_f32 * pos_clip_space.y * k) / std::f32::consts::PI).asin();

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
    fn world_to_clip_space(pos_world_space: cgmath::Vector4<f32>) -> cgmath::Vector2<f32> {
        // X in [-1, 1]
        // Y in [-1/2; 1/2] and scaled by the screen width/height ratio
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

        Vector2::new(x, y)
    }

    fn aperture_start() -> Deg<f32> {
        Deg(360_f32)
    }
}

use cgmath::Rad;
impl Projection for Orthographic {
    fn check_for_allsky_fov(depth: u8) -> Option<BTreeSet<HEALPixCell>> {
        if depth == 0 {
            Some(ALLSKY_ZERO_DEPTH.lock().unwrap().clone())
        } else {
            None
        }
    }

    fn name() -> &'static str {
        "Orthographic"
    }

    fn build_screen_map(viewport: &ViewPort) -> (Vec<Vector2<f32>>, Vec<u16>) {
        let mut vertices_screen = Vec::with_capacity(2*(NUM_VERTICES_PER_STEP*NUM_STEPS + 1) as usize);

        let window_size = viewport.get_window_size();

        let center_screen_space = window_size / 2_f32;
        vertices_screen.push(center_screen_space);

        for j in 0..NUM_STEPS {
            let radius = (std::f32::consts::PI * ((j + 1) as f32) / (2_f32 * (NUM_STEPS as f32))).sin();
            for i in 0..NUM_VERTICES_PER_STEP {
                let angle = (i as f32) * 2_f32 * std::f32::consts::PI / (NUM_VERTICES_PER_STEP as f32);

                let pos_screen_space = Vector2::<f32>::new(
                    (window_size.x/2_f32 - 1_f32) * radius * angle.cos(),
                    (window_size.x/2_f32 - 1_f32) * radius * angle.sin()
                );

                vertices_screen.push(pos_screen_space + center_screen_space);
            }
        }

        (vertices_screen, create_index_array())
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
    fn clip_to_world_space(pos_clip_space: Vector2<f32>) -> Option<cgmath::Vector4<f32>> {
        let xw_2 = 1_f32 - pos_clip_space.x*pos_clip_space.x - pos_clip_space.y*pos_clip_space.y;
        if xw_2 > 0_f32 {
            let pos_world_space = cgmath::Vector4::new(-pos_clip_space.x, pos_clip_space.y, xw_2.sqrt(), 1_f32);

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
    fn world_to_clip_space(pos_world_space: cgmath::Vector4<f32>) -> Vector2<f32> {
        Vector2::new(-pos_world_space.x, pos_world_space.y)
    }

    fn aperture_start() -> Deg<f32> {
        Deg(180_f32)
    }
}

impl Projection for AzimutalEquidistant {
    fn check_for_allsky_fov(depth: u8) -> Option<BTreeSet<HEALPixCell>> {
        if depth == 0 {
            Some(ALLSKY_ZERO_DEPTH.lock().unwrap().clone())
        } else if depth == 1 {
            Some(ALLSKY_ONE_DEPTH.lock().unwrap().clone())
        } else {
            None
        }
    }

    fn name() -> &'static str {
        "Arc"
    }

    fn build_screen_map(viewport: &ViewPort) -> (Vec<Vector2<f32>>, Vec<u16>) {
        let mut vertices_screen = Vec::with_capacity(2*(NUM_VERTICES_PER_STEP*NUM_STEPS + 1) as usize);

        let window_size = viewport.get_window_size();

        let center_screen_space = window_size / 2_f32;
        vertices_screen.push(center_screen_space);

        for j in 0..NUM_STEPS {
            let radius = (std::f32::consts::PI * ((j + 1) as f32) / (2_f32 * (NUM_STEPS as f32))).sin();
            for i in 0..NUM_VERTICES_PER_STEP {
                let angle = (i as f32) * 2_f32 * std::f32::consts::PI / (NUM_VERTICES_PER_STEP as f32);

                let mut pos_screen_space = Vector2::<f32>::new(
                    (window_size.x/2_f32 - 10_f32) * radius * angle.cos(),
                    (window_size.x/2_f32 - 10_f32) * radius * angle.sin()
                );

                vertices_screen.push(pos_screen_space + center_screen_space);
            }
        }

        (vertices_screen, create_index_array())
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
    fn clip_to_world_space(pos_clip_space: Vector2<f32>) -> Option<cgmath::Vector4<f32>> {
        let xw_2 = 1_f32 - pos_clip_space.x*pos_clip_space.x - pos_clip_space.y*pos_clip_space.y;
        if xw_2 > 0_f32 {
            let (x, y) = (2_f32 * pos_clip_space.x, 2_f32 * pos_clip_space.y);

            let rho2 = (x*x + y*y);
            let rho = rho2.sqrt();

            let c = 2_f32 * (0.5_f32 * rho).asin();

            let mut delta = 0_f32;
            let mut theta = 0_f32;
            if c >= 1e-4 {
                delta = (y * c.sin() / rho).asin() * std::f32::consts::PI;
                theta = -(x * c.sin()).atan2(rho * c.cos()) * std::f32::consts::PI;
            }
            let pos_world_space = math::radec_to_xyzw(Rad(theta), Rad(delta));
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
    fn world_to_clip_space(pos_world_space: cgmath::Vector4<f32>) -> Vector2<f32> {
        let (theta, delta) = math::xyzw_to_radec(pos_world_space);
        let c = delta.cos() * theta.cos();

        let k = c / c.sin();

        let x = -k* delta.cos() * theta.sin();
        let y = k*delta.sin();

        Vector2::new(x / std::f32::consts::PI, y / std::f32::consts::PI)
    }

    fn aperture_start() -> Deg<f32> {
        Deg(180_f32)
    }
}


impl Projection for Mercator {
    fn check_for_allsky_fov(depth: u8) -> Option<BTreeSet<HEALPixCell>> {
        if depth == 0 {
            Some(ALLSKY_ZERO_DEPTH.lock().unwrap().clone())
        } else if depth == 1 {
            Some(ALLSKY_ONE_DEPTH.lock().unwrap().clone())
        } else {
            None
        }
    }

    fn name() -> &'static str {
        "Mercator"
    }

    fn build_screen_map(viewport: &ViewPort) -> (Vec<Vector2<f32>>, Vec<u16>) {
        let N = 40;

        let mut vertices_screen = Vec::with_capacity(N * N as usize);

        for i in 0..N {
            for j in 0..N {
                let pos_clip_space = Vector2::new(
                    -1_f32 + ((j as f32)/((N-1) as f32))*2_f32,
                    -1_f32 + ((i as f32)/((N-1) as f32))*2_f32,
                );
                vertices_screen.push(crate::projection::clip_to_screen_space(pos_clip_space, viewport));
            }
        }

        let mut indices = Vec::with_capacity((N - 1) * (N - 1) * 2 * 3 as usize);

        for i in 0..(N-1) {
            for j in 0..(N-1) {
                indices.push((j + i*N) as u16);
                indices.push((j + i*N + 1) as u16);
                indices.push((j + (i+1)*N) as u16);

                indices.push((j + i*N + 1) as u16);
                indices.push((j + (i+1)*N + 1) as u16);
                indices.push((j + (i+1)*N) as u16);
            }
        }

        console::log_1(&format!("indices {:?}", indices).into());
        //panic!();

        (vertices_screen, indices)
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
    fn clip_to_world_space(pos_clip_space: Vector2<f32>) -> Option<cgmath::Vector4<f32>> {
        /*let xw_2 = 1_f32 - pos_clip_space.x*pos_clip_space.x - pos_clip_space.y*pos_clip_space.y;
        if xw_2 > 0_f32 {
            let (x, y) = (2_f32 * pos_clip_space.x, 2_f32 * pos_clip_space.y);

            let rho2 = (x*x + y*y);
            let rho = rho2.sqrt();

            let c = 2_f32 * (0.5_f32 * rho).asin();

            let mut delta = 0_f32;
            let mut theta = 0_f32;
            //if c >= 1e-4 {
            delta = (y * c.sin() / rho).asin();
            theta = -(x * c.sin()).atan2(rho * c.cos());
            //}
            let pos_world_space = math::radec_to_xyzw(Rad(theta), Rad(delta));
            Some(pos_world_space)
        } else {
            // Out of the sphere
            None
        }*/
        let theta = -pos_clip_space.x * std::f32::consts::PI;
        let delta = (pos_clip_space.y.sinh()).atan() * std::f32::consts::PI;

        let pos_world_space = math::radec_to_xyzw(Rad(theta), Rad(delta));
        Some(pos_world_space)
    }

    /// World to screen space transformation
    /// 
    /// # Arguments
    /// 
    /// * `pos_world_space` - Position in the world space. Must be a normalized vector
    fn world_to_clip_space(pos_world_space: cgmath::Vector4<f32>) -> Vector2<f32> {
        let (theta, delta) = math::xyzw_to_radec(pos_world_space);

        Vector2::new(-theta / std::f32::consts::PI, (((std::f32::consts::PI / 4_f32) + (delta / 2_f32)).tan()).ln() / std::f32::consts::PI)
    }

    fn aperture_start() -> Deg<f32> {
        Deg(360_f32)
    }
}