use crate::viewport::ViewPort;

use crate::window_size_f32;
use crate::DEGRADE_CANVAS_RATIO;

pub fn screen_pixels_to_homogenous(screen_pos: &Vector2<f32>, viewport: &ViewPort) -> Vector2<f32> {
    // Screen space in pixels to homogeneous screen space (values between [-1, 1])
    let (mut width, mut height) = window_size_f32();
    width = width * DEGRADE_CANVAS_RATIO;
    height = height * DEGRADE_CANVAS_RATIO;
    // Change of origin
    let origin = screen_pos - Vector2::new(width, height)/2_f32;

    // Scale to fit in [-1, 1]
    let homogeneous_pos = Vector2::new(2_f32 * (origin.x/width), -2_f32 * (origin.y/height));

    let zoom_factor = viewport.get_zoom_factor();
    homogeneous_pos / zoom_factor
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
    fn screen_to_world_space(pos: &Vector2<f32>) -> Option<cgmath::Vector4<f32>>;
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
    pub fn screen_to_world_space(&self, pos: &Vector2<f32>) -> Option<cgmath::Vector4<f32>> {
        match self {
            ProjectionType::Aitoff(_) => {
                Aitoff::screen_to_world_space(pos)
            },
            ProjectionType::Orthographic(_) => {
                Orthographic::screen_to_world_space(pos)
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

use crate::math::is_inside_ellipse;
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
    fn screen_to_world_space(pos: &Vector2<f32>) -> Option<cgmath::Vector4<f32>> {
        let (width, height) = window_size_f32();
        let aspect = width / height;

        let (x, y) = (pos.x, pos.y/aspect);

        let a = 1_f32;
        let b = 0.5_f32;
        if is_inside_ellipse(&cgmath::Vector2::new(x, y), a, b) {
            let u = x * std::f32::consts::PI * 0.5_f32;
            let v = y * std::f32::consts::PI;
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

        let delta = pos_world_space.y.asin();
        let theta = pos_world_space.x.atan2(pos_world_space.z);

        let theta_by_two = theta / 2_f32;

        let alpha = (delta.cos() * theta_by_two.cos()).acos();
        let inv_sinc_alpha = if alpha < 1e-3 {
            1_f32
        } else {
            alpha / alpha.sin()
        };

        let x = -2_f32 * inv_sinc_alpha * delta.cos() * theta_by_two.sin();
        let y = inv_sinc_alpha * delta.sin();

        Some(cgmath::Vector2::new(x / std::f32::consts::PI, y / std::f32::consts::PI))
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
    fn screen_to_world_space(pos: &Vector2<f32>) -> Option<cgmath::Vector4<f32>> {
        let (width, height) = window_size_f32();
        let aspect = width / height;

        let (x, y) = (pos.x * aspect, pos.y);

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