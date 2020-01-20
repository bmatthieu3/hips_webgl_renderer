
use std::rc::Rc;

#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum LastZoomAction {
    Zoom = 1,
    Unzoom = 2,
}

#[derive(PartialEq)]
pub enum LastAction {
    Zooming = 1,
    Moving = 2,
}

use crate::field_of_view::FieldOfView;
use cgmath::Rad;
use cgmath::Vector2;
pub struct ViewPort {
    gl: WebGl2Context,
    canvas: Rc<web_sys::HtmlCanvasElement>,

    fov: FieldOfView,
    fov_max: Rad<f32>,

    aspect: f32,

    screen_scaling: Vector2<f32>,
    fov_lookup_table: [Rad<f32>; NUM_WHEEL_PER_DEPTH * 29],
    zoom_index: usize,
    //final_zoom: f32,

    pub last_zoom_action: LastZoomAction,
    pub is_moving: bool,
    is_inertia: bool,

    is_action: bool,
    pub last_action: LastAction,

    // Store the size in pixels of the hips sphere
    default_size_scissor: Vector2<f32>,
}

use crate::shader::Shader;
use web_sys::WebGl2RenderingContext;

use crate::WebGl2Context;

use crate::{set_window_size, window_size_f32, window_size_u32};
use wasm_bindgen::JsCast;

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

const NUM_WHEEL_PER_DEPTH: usize = 5;
fn field_of_view_table() -> [Rad<f32>; NUM_WHEEL_PER_DEPTH * 29] {
    let mut fov = [Rad(0_f32); NUM_WHEEL_PER_DEPTH * 29];

    let max_depth = 29;
    for depth in 0..29 {
        let fov_min = math::depth_to_fov(depth as u8);
        let fov_max = math::depth_to_fov((depth + 1) as u8);
        let df = fov_max - fov_min;

        let off = depth * NUM_WHEEL_PER_DEPTH;

        for i in 0..NUM_WHEEL_PER_DEPTH {
            fov[off + i] = fov_min + df*(i as f32)/(NUM_WHEEL_PER_DEPTH as f32);
        }
    }

    console::log_1(&format!("stop inertia {:?}", fov[0]).into());


    fov
}

use crate::projection::ProjectionType;
use web_sys::console;
use crate::math;
use std::sync::atomic::Ordering;
use crate::MAX_DEPTH;
use crate::print_to_console;
use cgmath::Matrix4;
impl ViewPort {
    pub fn new(gl: &WebGl2Context, projection: &ProjectionType) -> ViewPort {
        //let final_zoom = current_zoom;

        let canvas = Rc::new(
            gl.canvas().unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap()
        );

        let last_zoom_action = LastZoomAction::Unzoom;
        let is_moving = false;
        let is_action = false;
        let is_inertia = false;

        let last_action = LastAction::Moving;

        let fov_max = math::depth_to_fov(MAX_DEPTH.load(Ordering::Relaxed));

        let default_size_scissor = projection.size();
        let (width, height) = window_size_f32();
        let aspect = width / height;

        let fov_lookup_table = field_of_view_table();
        let zoom_index = 0;
        
        let mut fov = FieldOfView::new();
        fov.set(Some(fov_lookup_table[0]), projection);
        let screen_scaling = if let Some(screen_scaling) = fov.get_screen_scaling_factor() {
            screen_scaling
        } else {
            Vector2::new(1_f32, 1_f32 / aspect)
        };

        let gl = gl.clone();
        let mut viewport = ViewPort {
            gl,
            canvas,

            fov,
            fov_max,

            aspect,
            screen_scaling,
            fov_lookup_table,
            zoom_index,
            //final_zoom,

            last_zoom_action,
            is_moving,
            is_inertia,

            is_action,
            last_action,

            default_size_scissor,
        };

        viewport.resize(&default_size_scissor);
        viewport
    }

    pub fn update_scissor(&self) {
        // Take into account the zoom factor
        let current_size_scissor = Vector2::new(
            self.default_size_scissor.x / self.screen_scaling.x,
            self.default_size_scissor.y / self.screen_scaling.y
        );
        set_gl_scissor(&self.gl, current_size_scissor);
    }

    pub fn zoom(&mut self, projection: &ProjectionType) {
        self.last_zoom_action = LastZoomAction::Zoom;
        self.last_action = LastAction::Zooming;

        if self.zoom_index < (self.fov_lookup_table.len() - 1) {
            self.zoom_index += 1;
        }

        self.fov.set(Some(self.fov_lookup_table[self.zoom_index]), projection);
        self.screen_scaling = if let Some(screen_scaling) = self.fov.get_screen_scaling_factor() {
            screen_scaling
        } else {
            Vector2::new(1_f32, 1_f32 / self.aspect)
        };
        self.is_action = true;

        self.update_scissor();
    }

    pub fn unzoom(&mut self, projection: &ProjectionType) {
        self.last_zoom_action = LastZoomAction::Unzoom;
        self.last_action = LastAction::Zooming;

        if self.zoom_index > 0 {
            self.zoom_index -= 1;
        }

        self.fov.set(Some(self.fov_lookup_table[self.zoom_index]), projection);
        self.screen_scaling = if let Some(screen_scaling) = self.fov.get_screen_scaling_factor() {
            screen_scaling
        } else {
            Vector2::new(1_f32, 1_f32 / self.aspect)
        };
        self.is_action = true;

        self.update_scissor();
    }

    pub fn displacement(&mut self) {
        self.is_moving = true;
        self.is_action = true;
        self.last_action = LastAction::Moving;
    }

    pub fn stop_displacement(&mut self) {
        self.is_moving = false;

        if !self.is_inertia {
            self.is_action = false;
        }
    }

    pub fn stop_zooming(&mut self) {
        if !self.is_moving && !self.is_inertia {
            self.is_action = false;
        }
    }

    pub fn start_inertia(&mut self) {
        self.is_inertia = true;
        self.is_action = true;
    }

    pub fn stop_inertia(&mut self) {
        console::log_1(&format!("stop inertia").into());

        self.is_inertia = false;

        if !self.is_moving {
            self.is_action = false;
        }
    }

    pub fn update(&mut self, model: &Matrix4<f32>) {
        // If there is an action whether it is a zoom or a displacement
        // then we update the fov
        if self.is_action {
            console::log_1(&format!("update FOV").into());
            self.fov.translate(model);
        }

        self.stop_zooming();

        /*if self.is_zooming {
            // Check if the max fov for this HiPS has been reached
            // if so we stop zooming!
            /*if let Some(fov) = self.fov.value() {
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
            */

            // We update the zoom factor
            //self.current_zoom += (self.final_zoom - self.current_zoom) * 0.005_f32 * dt;
            //self.current_zoom = self.final_zoom;
            //self.stop_zooming();
        }*/
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

    pub fn get_screen_scaling_factor(&self) -> &Vector2<f32> {
        &self.screen_scaling
    }
    /*pub fn get_zoom_factor(&self) -> f32 {
        self.current_zoom
    }*/

    /// Warning: this is executed by all the shaders
    pub fn send_to_vertex_shader(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        // Send window size
        let location_aspect = shader.get_uniform_location("aspect");

        gl.uniform1f(location_aspect, self.aspect);
        // Send zoom factor
        let zoom_factor_location = shader.get_uniform_location("zoom_factor");
        let screen_scaling = self.get_screen_scaling_factor();
        gl.uniform2f(zoom_factor_location, screen_scaling.x, screen_scaling.y);

        // Send last zoom action
        let last_zoom_action_location = shader.get_uniform_location("last_zoom_action");
        gl.uniform1i(last_zoom_action_location, self.last_zoom_action as i32);
    }
}