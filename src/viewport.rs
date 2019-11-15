use std::rc::Rc;
use std::cell::RefCell;

use crate::renderable::Renderable;
use crate::renderable::hips_sphere::HiPSSphere;

pub struct ViewPort {
    gl: WebGl2Context,
    canvas: Rc<web_sys::HtmlCanvasElement>,

    current_zoom: f32,
    final_zoom: f32,

    resize_factor_x: f32,
    resize_factor_y: f32,

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

use crate::render_next_frame;
impl ViewPort {
    pub fn new(gl: &WebGl2Context, hips_sphere: Rc<RefCell<Renderable<HiPSSphere>>>) -> ViewPort {
        let current_zoom = 1_f32;
        let final_zoom = current_zoom;

        let resize_factor_x = 1.0_f32;
        let resize_factor_y = 1.0_f32;

        let canvas = Rc::new(
            gl.canvas().unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap()
        );
        set_gl_scissor(gl, *hips_sphere.borrow().mesh().get_default_pixel_size());

        let gl = gl.clone();
        let mut viewport = ViewPort {
            gl,
            canvas,

            current_zoom,
            final_zoom,

            resize_factor_x,
            resize_factor_y,

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

    pub fn zoom(&mut self) {
        self.final_zoom *= 1.2_f32;
        if self.final_zoom > 1000_f32 {
            self.final_zoom = 1000_f32;
        }
        //self.zoom_factor *= 1.2_f32;

        //self.update_scissor();
    }

    pub fn unzoom(&mut self) {
        self.final_zoom /= 1.2_f32;
        if self.final_zoom < 0.5_f32 {
            self.final_zoom = 0.5_f32;
        }
    }

    pub fn update_camera_movement(&mut self) {
        if (self.current_zoom - self.final_zoom).abs() < 1e-2 {
            self.current_zoom = self.final_zoom;
            return;
        }

        self.current_zoom += (self.final_zoom - self.current_zoom) * 0.1_f32;

        render_next_frame();
        self.update_scissor();
    }

    pub fn resize(&mut self) {
        /*let width = web_sys::window().unwrap().inner_width()
            .unwrap()
            .as_f64()
            .unwrap() as u32;
        let height = web_sys::window().unwrap().inner_height()
            .unwrap()
            .as_f64()
            .unwrap() as u32;*/
        /*let width = 1024;
        let height = 768;*/
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

    pub fn send_to_vertex_shader(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        /*// Send view matrix
        let view_mat_f32_slice: &[f32; 16] = self.view_mat.as_ref();
        let view_mat_location = shader.get_uniform_location(gl, "view");
        gl.uniform_matrix4fv_with_f32_array(view_mat_location.as_ref(), false, view_mat_f32_slice);*/

        // Send zoom factor
        let zoom_factor_location = shader.get_uniform_location("zoom_factor");
        gl.uniform1f(zoom_factor_location, self.current_zoom);

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
        /*let location_starting_window = shader.get_uniform_location(gl, "window_size_default").unwrap();
        gl.uniform2f(Some(&location_starting_window), self.starting_width, self.starting_height);
        let location_current_window = shader.get_uniform_location(gl, "current_window_size").unwrap();
        gl.uniform2f(Some(&location_current_window), self.current_width, self.current_height);*/
    }
}