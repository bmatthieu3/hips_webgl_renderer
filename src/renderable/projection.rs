use cgmath::{Vector3, InnerSpace};
use crate::viewport::ViewPort;

pub fn screen_pixels_to_homogeous(x: f32, y: f32, viewport: &ViewPort) -> (f32, f32) {
    // Screen space in pixels to homogeneous screen space (values between [-1, 1])
    let window = web_sys::window().unwrap();
    let width = window.inner_width()
        .unwrap()
        .as_f64()
        .unwrap() as f32;
    let height = window.inner_height()
        .unwrap()
        .as_f64()
        .unwrap() as f32;
    // Change of origin
    let xo = x - width/2_f32;
    let yo = y - height/2_f32;

    // Scale to fit in [-1, 1]
    let xh = 2_f32*(xo/width);
    let yh = -2_f32*(yo/height);

    let zoom_f = viewport.get_zoom_factor();
    (xh / zoom_f, yh / zoom_f)
}

pub trait Projection {
    fn build_screen_map() -> Vec<cgmath::Vector2<f32>>;

    fn scale_by_screen_ratio(x: f32, y: f32) -> (f32, f32);
    /// Screen space to world space transformation
    /// 
    /// # Arguments
    /// 
    /// * `x` - X mouse position in homogenous screen space (between [-1, 1])
    /// * `y` - Y mouse position in homogenous screen space (between [-1, 1])
    fn screen_to_world_space(x: f32, y: f32) -> Option<cgmath::Vector3<f32>> {
        let (x, y) = Self::scale_by_screen_ratio(x, y);
        let xw_2 = 1_f32 - x*x - y*y;

        if xw_2 > 0_f32 {
            let pos_world_space = Self::view_to_world_space(x, y, xw_2.sqrt());

            let mut pos_world_space = Vector3::<f32>::new(pos_world_space.x, pos_world_space.y, pos_world_space.z);
            pos_world_space = pos_world_space.normalize();

            Some(pos_world_space)
        } else {
            // Out of the sphere
            None
        }
    }

    fn view_to_world_space(x: f32, y: f32, z: f32) -> cgmath::Vector4<f32>;
}

pub struct Aitoff;
pub struct Orthographic;

pub enum ProjectionType {
    Aitoff(Aitoff),
    Orthographic(Orthographic),
}

impl ProjectionType {
    pub fn build_screen_map(&self) -> Vec<cgmath::Vector2<f32>> {
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
    /// # Arguments
    /// 
    /// * `x` - X mouse position in the screen space (in pixel)
    /// * `y` - Y mouse position in the screen space (in pixel)
    pub fn screen_to_world_space(&self, x: f32, y: f32) -> Option<cgmath::Vector3<f32>> {
        match self {
            ProjectionType::Aitoff(_) => {
                Aitoff::screen_to_world_space(x, y)
            },
            ProjectionType::Orthographic(_) => {
                Orthographic::screen_to_world_space(x, y)
            },
        }
    }
}

use cgmath::Vector2;

const NUM_VERTICES_PER_STEP: usize = 70;
const NUM_STEPS: usize = 40;
impl Projection for Aitoff {
    fn build_screen_map() -> Vec<cgmath::Vector2<f32>> {
        let mut vertices_screen = Vec::with_capacity(2*(NUM_VERTICES_PER_STEP*NUM_STEPS + 1) as usize);

        let window = web_sys::window().unwrap();
        let width = window.inner_width()
            .unwrap()
            .as_f64()
            .unwrap() as f32;
        let height = window.inner_height()
            .unwrap()
            .as_f64()
            .unwrap() as f32;

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

        vertices_screen
    }

    fn view_to_world_space(x: f32, y: f32, z: f32) -> cgmath::Vector4<f32> {
        let u = x * std::f32::consts::PI * 0.50_f32 ;
        let v = y * std::f32::consts::PI ;
        //da uv a lat/lon
        let mut phi = 0_f32;
        let mut theta = 0_f32;
        let c = (v*v + u*u).sqrt();	
        if c != 0_f32{
            phi = (v * c.sin() / c).asin();
            theta = (u * c.sin()).atan2(c * c.cos());
        }
        theta *= 2_f32;

        cgmath::Vector4::new(
            theta.sin() * phi.cos(),
            phi.sin(),
            theta.cos() * phi.cos(),
            1_f32
        )
    }

    fn scale_by_screen_ratio(x: f32, y: f32) -> (f32, f32) {
        let window = web_sys::window().unwrap();
        let width = window.inner_width()
            .unwrap()
            .as_f64()
            .unwrap() as f32;
        let height = window.inner_height()
            .unwrap()
            .as_f64()
            .unwrap() as f32;

        let aspect = width / height;
        (x, y / aspect)
    }
}

impl Projection for Orthographic {
    fn build_screen_map() -> Vec<cgmath::Vector2<f32>> {
        let mut vertices_screen = Vec::with_capacity(2*(NUM_VERTICES_PER_STEP*NUM_STEPS + 1) as usize);

        let window = web_sys::window().unwrap();
        let width = window.inner_width()
            .unwrap()
            .as_f64()
            .unwrap() as f32;
        let height = window.inner_height()
            .unwrap()
            .as_f64()
            .unwrap() as f32;

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

        vertices_screen
    }

    fn view_to_world_space(x: f32, y: f32, z: f32) -> cgmath::Vector4<f32> {
        cgmath::Vector4::new(x, y, z, 1_f32)
    }

    fn scale_by_screen_ratio(x: f32, y: f32) -> (f32, f32) {
        let window = web_sys::window().unwrap();
        let width = window.inner_width()
            .unwrap()
            .as_f64()
            .unwrap() as f32;
        let height = window.inner_height()
            .unwrap()
            .as_f64()
            .unwrap() as f32;

        let aspect = width / height;
        (x * aspect, y)
    }
}