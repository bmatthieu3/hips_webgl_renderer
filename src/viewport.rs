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
pub struct ViewPort {
    gl: WebGl2Context,
    canvas: Rc<web_sys::HtmlCanvasElement>,

    fov: FieldOfView,

    current_zoom: f32,
    final_zoom: f32,

    last_zoom_action: LastZoomAction,
    is_moving: bool,
    is_zooming: bool,

    is_action: bool,

    // Immutable counted reference to the HiPS sphere
    hips_sphere: Rc<RefCell<Renderable<HiPSSphere>>>,
}

use crate::shader::Shader;
use web_sys::WebGl2RenderingContext;

use crate::WebGl2Context;

use crate::{set_window_size, window_size_f32, window_size_u32};
use wasm_bindgen::JsCast;

use cgmath::Vector2;

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
impl ViewPort {
    pub fn new(gl: &WebGl2Context, hips_sphere: Rc<RefCell<Renderable<HiPSSphere>>>) -> ViewPort {
        let current_zoom = 1_f32;
        let final_zoom = current_zoom;

        let canvas = Rc::new(
            gl.canvas().unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap()
        );
        set_gl_scissor(gl, *hips_sphere.borrow().mesh().get_default_pixel_size());

        let last_zoom_action = LastZoomAction::Unzoom;
        let is_moving = false;
        let is_action = false;
        let is_zooming = false;

        let fov = FieldOfView::new();

        let gl = gl.clone();
        let mut viewport = ViewPort {
            gl,
            canvas,

            fov,

            current_zoom,
            final_zoom,

            last_zoom_action,
            is_moving,
            is_zooming,

            is_action,

            hips_sphere,
        };

        viewport.resize();
        viewport
    }

    pub fn update_scissor(&self) {
        // Set the scissor here
        let mut size_px = self.hips_sphere
            .borrow()
            .mesh()
            .get_default_pixel_size()
            .clone();
        // Take into account the zoom factor
        size_px *= self.current_zoom;
        set_gl_scissor(&self.gl, size_px);
    }

    pub fn zoom(&mut self, dt: f32) {
        self.is_zooming = true;
        self.is_action = true;

        self.last_zoom_action = LastZoomAction::Zoom;

        self.final_zoom *= 1.2_f32 * dt;
        if self.final_zoom > 1000_f32 {
            self.final_zoom = 1000_f32;
        }

        //self.update_scissor();
    }

    pub fn unzoom(&mut self, dt: f32) {
        self.is_zooming = true;
        self.is_action = true;

        self.last_zoom_action = LastZoomAction::Unzoom;

        self.final_zoom /= 1.2_f32 * dt;
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

        if !self.is_zooming {
            self.is_action = false;
        }
    }

    pub fn update(&mut self, projection: &ProjectionType) {
        // If there is an action whether it is a zoom or a displacement
        // then we update the fov
        if self.is_action {
            console::log_1(&format!("update FOV").into());
            self.fov.update(self.current_zoom, projection);
        }

        // Stop zooming the final zoom factor has been reached
        if (self.current_zoom - self.final_zoom).abs() < 1e-3 {
            self.current_zoom = self.final_zoom;
            self.is_zooming = false;
            if !self.is_moving {
                self.is_action = false;
            }

            return;
        }
        // Here we are currently zooming

        // We update the zoom factor
        self.current_zoom += (self.final_zoom - self.current_zoom) * 0.1_f32;
        // And update the scissor
        self.update_scissor();
    }

    pub fn field_of_view(&self) -> &FieldOfView {
        &self.fov
    }

    pub fn is_user_action(&self) -> bool {
        self.is_action
    }

    pub fn resize(&mut self) {
        let (width, height) = window_size_u32();

        set_window_size(width, height);

        self.canvas.set_width(width);
        self.canvas.set_height(height);
        self.gl.viewport(0, 0, width as i32, height as i32);

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