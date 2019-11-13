use crate::viewport::ViewPort;

use web_sys::console;

use crate::{window_size_f32};

pub fn screen_pixels_to_homogenous(x: f32, y: f32, viewport: &ViewPort) -> (f32, f32) {
    // Screen space in pixels to homogeneous screen space (values between [-1, 1])
    let (width, height) = window_size_f32();
    //let (start_width, start_height) = viewport.get_starting_window_size();
    // Change of origin
    let xo = x - width/2_f32;
    let yo = y - height/2_f32;

    // Scale to fit in [-1, 1]
    let xh = 2_f32*(xo/width);
    let yh = -2_f32*(yo/height);

    //console::log_1(&format!("Homogenous {:?} {:?}", xh, yh).into());

    let zoom_f = viewport.get_zoom_factor();
    (xh / zoom_f, yh / zoom_f)
}

pub trait Projection {
    fn build_screen_map() -> (Vec<cgmath::Vector2<f32>>, cgmath::Vector2<f32>);

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
    fn screen_to_world_space(x: f32, y: f32) -> Option<cgmath::Vector4<f32>>;
    /// World to screen space transformation
    /// 
    /// # Arguments
    /// 
    /// * `x` - X mouse position in homogenous screen space (between [-1, 1])
    /// * `y` - Y mouse position in homogenous screen space (between [-1, 1])
    fn world_to_screen_space(pos_world_space: cgmath::Vector4<f32>) -> Option<cgmath::Vector2<f32>>;
}

#[derive(Clone, Copy)]
pub struct Aitoff;
#[derive(Clone, Copy)]
pub struct Orthographic;

#[derive(Clone, Copy)]
pub enum ProjectionType {
    Aitoff(Aitoff),
    Orthographic(Orthographic),
}

impl ProjectionType {
    pub fn build_screen_map(&self) -> (Vec<cgmath::Vector2<f32>>, cgmath::Vector2<f32>) {
        match self {
            ProjectionType::Aitoff(_) => {
                Aitoff::build_screen_map()
            },
            ProjectionType::Orthographic(_) => {
                Orthographic::build_screen_map()
            },
        }
    }

    /// Screen space to world space transformation
    /// 
    /// This returns a normalized vector along its first 3 dimensions.
    /// Its fourth component is set to 1.
    /// 
    /// # Arguments
    /// 
    /// * `x` - X mouse position in homogenous screen space (between [-1, 1])
    /// * `y` - Y mouse position in homogenous screen space (between [-1, 1])
    pub fn screen_to_world_space(&self, x: f32, y: f32) -> Option<cgmath::Vector4<f32>> {
        match self {
            ProjectionType::Aitoff(_) => {
                Aitoff::screen_to_world_space(x, y)
            },
            ProjectionType::Orthographic(_) => {
                Orthographic::screen_to_world_space(x, y)
            },
        }
    }

    /// World space to screen space transformation
    /// 
    /// # Arguments
    /// 
    /// * `x` - X mouse position in the screen space (in pixel)
    /// * `y` - Y mouse position in the screen space (in pixel)
    pub fn world_to_screen_space(&self, pos_world_space: cgmath::Vector4<f32>) -> Option<cgmath::Vector2<f32>> {
        match self {
            ProjectionType::Aitoff(_) => {
                Aitoff::world_to_screen_space(pos_world_space)
            },
            ProjectionType::Orthographic(_) => {
                Orthographic::world_to_screen_space(pos_world_space)
            },
        }
    }
}

use cgmath::Vector2;

const NUM_VERTICES_PER_STEP: usize = 50;
const NUM_STEPS: usize = 20;
impl Projection for Aitoff {
    fn build_screen_map() -> (Vec<cgmath::Vector2<f32>>, cgmath::Vector2<f32>) {
        let mut vertices_screen = Vec::with_capacity(2*(NUM_VERTICES_PER_STEP*NUM_STEPS + 1) as usize);

        let (width, height) = window_size_f32();

        let center_screen_space = Vector2::<f32>::new(0_f32, 0_f32);
        vertices_screen.push(center_screen_space);

        for j in 0..NUM_STEPS {
            let radius = (std::f32::consts::PI * ((j + 1) as f32) / (2_f32 * (NUM_STEPS as f32))).sin();
            for i in 0..NUM_VERTICES_PER_STEP {
                let angle = (i as f32) * 2_f32 * std::f32::consts::PI / (NUM_VERTICES_PER_STEP as f32);

                let mut pos_screen_space = Vector2::<f32>::new(
                    (width/2_f32 - 1_f32) * radius * angle.cos(),
                    ((width/2_f32 - 1_f32) / 2_f32) * radius * angle.sin()
                );
                //console::log_1(&format!("pos_screen {:?}", pos_screen_space).into());

                pos_screen_space += Vector2::<f32>::new(width / 2_f32, height / 2_f32);
                vertices_screen.push(
                    Vector2::<f32>::new(
                        2_f32 * ((pos_screen_space.x / width) - 0.5_f32),
                        -2_f32 * ((pos_screen_space.y / height) - 0.5_f32),
                    )
                );
            }
        }

        let size_px = cgmath::Vector2::new(width - 2_f32, width/2_f32 - 1_f32);
        (vertices_screen, size_px)
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
    fn screen_to_world_space(x: f32, y: f32) -> Option<cgmath::Vector4<f32>> {
        let (width, height) = window_size_f32();
        let aspect = width / height;

        let (x, y) = (x, y/aspect);

        let xw_2 = 1_f32 - x*x - y*y;
        if xw_2 > 0_f32 {
            let u = x * std::f32::consts::PI * 0.5_f32;
            let v = y * std::f32::consts::PI;
            //da uv a lat/lon
            let mut phi = 0_f32;
            let mut theta = 0_f32;
            let c = (v*v + u*u).sqrt();	
            if c != 0_f32 {
                phi = (v * c.sin() / c).asin();
                theta = (u * c.sin()).atan2(c * c.cos());
            } else {
                phi = v.asin();
                theta = u.atan();
            }
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

        let delta = pos_world_space.y.asin();
        let theta = pos_world_space.x.atan2(pos_world_space.z);

        let theta_by_two = theta / 2_f32;

        let alpha = (delta.cos() * theta_by_two.cos()).acos();
        let inv_sinc_alpha = if alpha < 1e-3 {
            1_f32
        } else {
            alpha / alpha.sin()
        };

        let X = -2_f32 * inv_sinc_alpha * delta.cos() * theta_by_two.sin();
        let Y = inv_sinc_alpha * delta.sin();

        Some(cgmath::Vector2::new(X / std::f32::consts::PI, Y / std::f32::consts::PI))
    }
}

impl Projection for Orthographic {
    fn build_screen_map() -> (Vec<cgmath::Vector2<f32>>, cgmath::Vector2<f32>) {
        let mut vertices_screen = Vec::with_capacity(2*(NUM_VERTICES_PER_STEP*NUM_STEPS + 1) as usize);

        let (width, height) = window_size_f32();

        let center_screen_space = Vector2::<f32>::new(
            0_f32, 0_f32
        );
        vertices_screen.push(center_screen_space);

        for j in 0..NUM_STEPS {
            let radius = (std::f32::consts::PI * ((j + 1) as f32) / (2_f32 * (NUM_STEPS as f32))).sin();
            for i in 0..NUM_VERTICES_PER_STEP {
                let angle = (i as f32) * 2_f32 * std::f32::consts::PI / (NUM_VERTICES_PER_STEP as f32);

                let mut pos_screen_space = Vector2::<f32>::new(
                    (height/2_f32 - 1_f32) * radius * angle.cos(),
                    (height/2_f32 - 1_f32) * radius * angle.sin()
                );

                pos_screen_space += Vector2::<f32>::new(width / 2_f32, height / 2_f32);
                vertices_screen.push(
                    Vector2::<f32>::new(
                        2_f32 * ((pos_screen_space.x / width) - 0.5_f32),
                        -2_f32 * ((pos_screen_space.y / height) - 0.5_f32)
                    )
                );
            }
        }

        let size_px = cgmath::Vector2::new(height - 2_f32, height - 2_f32);
        (vertices_screen, size_px)
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
    fn screen_to_world_space(x: f32, y: f32) -> Option<cgmath::Vector4<f32>> {
        let (width, height) = window_size_f32();
        let aspect = width / height;

        let (x, y) = (x * aspect, y);

        let xw_2 = 1_f32 - x*x - y*y;

        if xw_2 > 0_f32 {
            let pos_world_space = cgmath::Vector4::new(-x, y, xw_2.sqrt(), 1_f32);

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
        let aspect = height / width;
        Some(cgmath::Vector2::new(
            -pos_world_space.x,
            pos_world_space.y,
        ) * aspect)
    }
}