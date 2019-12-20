
use std::rc::Rc;
use std::cell::RefCell;

use crate::renderable::Renderable;
use crate::renderable::hips_sphere::HiPSSphere;

#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum LastZoomAction {
    Zoom = 1,
    Unzoom = 2,
}

use crate::field_of_view::FieldOfView;
use cgmath::Rad;
pub struct ViewPort {
    gl: WebGl2Context,
    canvas: Rc<web_sys::HtmlCanvasElement>,

    fov: FieldOfView,
    fov_max: Rad<f32>,

    time_start_zoom: Option<f32>, // Absolute time of the beginning of a zoom (in ms)
    time_end_zoom: Option<f32>, // Duration of a zoom (in ms)
    current_zoom: f32,
    final_zoom: f32,

    last_zoom_action: LastZoomAction,
    is_moving: bool,
    is_inertia: bool,
    is_zooming: bool,

    is_action: bool,

    // Store the size in pixels of the hips sphere
    default_size_scissor: Vector2<f32>,
}

use crate::shader::Shader;
use web_sys::WebGl2RenderingContext;

use crate::WebGl2Context;

use crate::{set_window_size, window_size_f32, window_size_u32};
use wasm_bindgen::JsCast;

use cgmath::Vector2;
use cgmath::Deg;

/// Set the scissor knowing the size in pixel of the
/// HiPS sphere
fn set_gl_scissor(gl: &WebGl2Context, size: Vector2<f32>) {
    // Update the scissor
    let (width_screen, height_screen) = window_size_f32();

    let xo = (width_screen / 2_f32) - size.x / 2_f32;
    let yo = (height_screen / 2_f32) - size.y / 2_f32;

    gl.scissor(xo as i32, yo as i32, size.x as i32, size.y as i32);
}

use crate::projection::ProjectionType;
use web_sys::console;
use crate::math;
use std::sync::atomic::Ordering;
use crate::MAX_DEPTH;
use crate::print_to_console;
<<<<<<< HEAD
use crate::utils;
=======
use cgmath::Matrix4;

use crate::renderable::catalog::Catalog;
>>>>>>> features/instancing
impl ViewPort {
    pub fn new(gl: &WebGl2Context, size_pixels: &Vector2<f32>) -> ViewPort {
        let current_zoom = 1_f32;
        let final_zoom = current_zoom;

        let canvas = Rc::new(
            gl.canvas().unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap()
        );

        let last_zoom_action = LastZoomAction::Unzoom;
        let is_moving = false;
        let is_action = false;
        let is_zooming = false;
        let is_inertia = false;

        let fov = FieldOfView::new();
        let fov_max = math::depth_to_fov(MAX_DEPTH.load(Ordering::Relaxed));

        let default_size_scissor = *size_pixels;
        let time_start_zoom = None;
        let time_end_zoom = None;

        let gl = gl.clone();
        let mut viewport = ViewPort {
            gl,
            canvas,

            fov,
            fov_max,

            time_start_zoom,
            time_end_zoom,

            current_zoom,
            final_zoom,

            last_zoom_action,
            is_moving,
            is_zooming,
            is_inertia,

            is_action,

            default_size_scissor,
        };

        viewport.resize(&default_size_scissor);
        viewport
    }

    pub fn update_scissor(&self) {
        // Take into account the zoom factor
        let current_size_scissor = self.default_size_scissor * self.current_zoom;
        set_gl_scissor(&self.gl, current_size_scissor);
    }

    pub fn zoom(&mut self, amount: f32) {
        /*if let Some(fov) = self.fov.value() {
            if self.fov_max > *fov {
                return;
            }
        }*/
        let current_time = utils::get_current_time();
        if !self.is_zooming {
            self.time_start_zoom = Some(current_time);
        }
        self.time_end_zoom = Some(current_time + 1000_f32);

        self.is_zooming = true;
        self.is_action = true;

        self.last_zoom_action = LastZoomAction::Zoom;

        self.final_zoom *= (1_f32 + 0.01_f32 * amount);

        self.time_start_zoom = Some(utils::get_current_time());
    }

    pub fn unzoom(&mut self, amount: f32) {
        let current_time = utils::get_current_time();
        if !self.is_zooming {
            self.time_start_zoom = Some(current_time);
        }
        self.time_end_zoom = Some(current_time + 1000_f32);
        
        self.is_zooming = true;
        self.is_action = true;

        self.last_zoom_action = LastZoomAction::Unzoom;

        self.final_zoom /= (1_f32 + 0.01_f32 * amount);
        if self.final_zoom < 0.5_f32 {
            self.final_zoom = 0.5_f32;
        }
    }

    pub fn displacement(&mut self) {
        self.is_moving = true;
        self.is_action = true;
    }

    pub fn stop_displacement(&mut self) {
        self.is_moving = false;

        if !self.is_zooming && !self.is_inertia {
            self.is_action = false;
        }
    }

    pub fn stop_zooming(&mut self) {
        self.final_zoom = self.current_zoom;

        self.is_zooming = false;
        if !self.is_moving && !self.is_inertia {
            self.is_action = false;
        }

        self.time_start_zoom = None;
    }

    pub fn start_inertia(&mut self) {
        self.is_inertia = true;
        self.is_action = true;
    }

    pub fn stop_inertia(&mut self) {
        console::log_1(&format!("stop inertia").into());

        self.is_inertia = false;

        if !self.is_moving && !self.is_zooming {
            self.is_action = false;
        }
    }

    pub fn update(&mut self, model: &Matrix4<f32>, projection: &ProjectionType, dt: f32) {
        // If there is an action whether it is a zoom or a displacement
        // then we update the fov
        if self.is_action {
            console::log_1(&format!("update FOV").into());
            self.fov.update(model, self.current_zoom, projection);
        }

        if self.is_zooming {
            // Check if the max fov for this HiPS has been reached
            // if so we stop zooming!
            if let Some(fov) = self.fov.value() {
                // Zooming
                if self.fov_max > *fov && self.current_zoom < self.final_zoom {
                    self.stop_zooming();
                    //catalog.update(projection, self);
                    return;
                }
            }
            // We update the zoom factor
            /*let t = (utils::get_current_time() - self.time_start_zoom.unwrap()) / (self.time_end_zoom.unwrap() - self.time_start_zoom.unwrap());

            let v = self.compute_speed(t);
            if v < 1e-3 {
                self.stop_zooming();
                return;
            }
            self.current_zoom += (self.final_zoom - self.current_zoom).signum() * v * dt * 1e-3;
            */
            // Here we are currently zooming
            if (self.current_zoom - self.final_zoom).abs() < 1e-3 {
                self.stop_zooming();
                //catalog.update(projection, self);
                return;
            }
<<<<<<< HEAD
            self.current_zoom += (self.final_zoom - self.current_zoom) * 0.005_f32 * dt;
=======

            // We update the zoom factor
            //self.current_zoom += (self.final_zoom - self.current_zoom) * 0.005_f32 * dt;
            self.current_zoom = self.final_zoom;

>>>>>>> features/instancing
            self.update_scissor();
        }

        /*if self.is_moving {
            catalog.update(projection, self);
        }*/
    }

    fn compute_speed(&self, mut t: f32) -> f32 {
        let speed_limit = 1_f32;

        if t > 0.9_f32 {
            t = 0.5_f32 + (t - 0.9_f32) * 5_f32;

            let u = 1_f32 - t;
            let w = u * t;
            speed_limit * (w * w / 0.0625_f32)
        } else if t < 0.1_f32 {
            t *= 5_f32;

            let u = 1_f32 - t;
            let w = u * t;
            speed_limit * (w * w / 0.0625_f32)
        } else {
            speed_limit
        }
    }

    pub fn set_max_field_of_view(&mut self, max_fov: Rad<f32>) {
        self.fov_max = max_fov;
        let deg: Deg<f32> = self.fov_max.into();
        print_to_console!("fov max {:?}", deg.0);
    }

    pub fn field_of_view(&self) -> &FieldOfView {
        &self.fov
    }

    pub fn is_user_action(&self) -> bool {
        self.is_action
    }

    pub fn resize(&mut self, new_size_scissor: &Vector2<f32>) {
        let (width, height) = window_size_u32();

        set_window_size(width, height);

        self.canvas.set_width(width);
        self.canvas.set_height(height);
        self.gl.viewport(0, 0, width as i32, height as i32);

        self.default_size_scissor = *new_size_scissor;
        self.update_scissor();
    }

    pub fn get_zoom_factor(&self) -> f32 {
        self.current_zoom
    }

    /// Warning: this is executed by all the shaders
    pub fn send_to_vertex_shader(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        // Send zoom factor
        let zoom_factor_location = shader.get_uniform_location("zoom_factor");
        gl.uniform1f(zoom_factor_location, self.current_zoom);

        // Send last zoom action
        let last_zoom_action_location = shader.get_uniform_location("last_zoom_action");
        gl.uniform1i(last_zoom_action_location, self.last_zoom_action as i32);
        // Send window size
        /*let location_resize_factor_x = shader.get_uniform_location(gl, "resize_factor_x");
        gl.uniform1f(location_resize_factor_x.as_ref(), self.resize_factor_x);
        
        let location_resize_factor_y = shader.get_uniform_location(gl, "resize_factor_y");
        gl.uniform1f(location_resize_factor_y.as_ref(), self.resize_factor_y);
        */
        let location_aspect = shader.get_uniform_location("aspect");

        let (width, height) = window_size_f32();
        let aspect = width / height;
        gl.uniform1f(location_aspect, aspect);
    }
}