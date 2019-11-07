use std::rc::Rc;

pub struct ViewPort {
    gl: WebGl2Context,
    window: Rc<web_sys::Window>,
    canvas: Rc<web_sys::HtmlCanvasElement>,

    view_mat: cgmath::Matrix4<f32>,
    zoom_factor: f32,
    resize_factor_x: f32,
    resize_factor_y: f32,
    position: usize,
}

use crate::shader::Shader;
use web_sys::WebGl2RenderingContext;

use cgmath::SquareMatrix;

use crate::WebGl2Context;

use crate::{set_window_size, window_size_f32};
use wasm_bindgen::JsCast;

impl ViewPort {
    pub fn new(gl: &WebGl2Context) -> ViewPort {
        let view_mat = cgmath::Matrix4::identity();
        let zoom_factor = 1_f32;
        let position = 0;

        let resize_factor_x = 1.0_f32;
        let resize_factor_y = 1.0_f32;

        //gl.viewport(0, 0, current_width as i32, current_height as i32);

        let window = Rc::new(web_sys::window().unwrap());
        let canvas = Rc::new(
            gl.canvas().unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>().unwrap()
        );
        let gl = gl.clone();
        let mut viewport = ViewPort {
            gl,
            window,
            canvas,

            view_mat,
            zoom_factor,
            resize_factor_x,
            resize_factor_y,
            position,
        };

        viewport.resize();
        viewport
    }

    pub fn zoom(&mut self, current_depth: i32) {
        let depth_max = 8; // XMLHttpRequest to get the max depth of the survey
        /*if current_depth < depth_max {
            let n_step = 5;

            let fov_0 = math::depth_to_resolution(0);
            let fov_d = math::depth_to_resolution(current_depth);
            let fov_d_next = math::depth_to_resolution(current_depth + 1);

            let i = self.position % n_step;

            self.zoom_factor = (fov_0 / (fov_d * fov_d_next)) * (fov_d_next + (i as f32) * (fov_d_next - fov_d).abs()/((n_step - 1) as f32));

            //self.zoom_factor += 1_f32;
            console::log_1(&format!("zoom factor {:?}", self.zoom_factor).into());

            self.position += 1;
        }*/
        self.zoom_factor *= 1.2_f32;
    }

    pub fn unzoom(&mut self) {
        /*
        if self.zoom_factor > 1_f32 {
            self.position -= 1;

            self.zoom_factor -= 1_f32;
        }

        if self.zoom_factor < 1_f32 {
            self.zoom_factor = 1_f32;
        }*/
        self.zoom_factor /= 1.2_f32;
        if self.zoom_factor < 0.5_f32 {
            self.zoom_factor = 0.5_f32;
        }

        // Set the scissor here
        let (width_screen, height_screen) = window_size_f32();
        /*
        let xo = (width_screen / 2_f32) - size_px.x / 2_f32;
        let yo = (height_screen / 2_f32) - size_px.y / 2_f32;
        gl.scissor(xo as i32, yo as i32, size_px.x as i32, size_px.y as i32);*/
    }

    pub fn resize(&mut self) {
        let width = web_sys::window().unwrap().inner_width()
            .unwrap()
            .as_f64()
            .unwrap() as u32;
        let height = web_sys::window().unwrap().inner_height()
            .unwrap()
            .as_f64()
            .unwrap() as u32;

        set_window_size(width, height);

        self.canvas.set_width(width);
        self.canvas.set_height(height);
        self.gl.viewport(0, 0, width as i32, height as i32);
        self.gl.scissor(0, 0, width as i32, height as i32);
    /*
        if self.current_width > 2_f32 * self.current_height {
            self.resize_factor_x = self.current_height / HEIGHT_SCREEN;
            self.resize_factor_y = self.current_width / WIDTH_SCREEN;
        } else {
            self.resize_factor_x = self.current_height / HEIGHT_SCREEN;
            self.resize_factor_y = self.current_width / WIDTH_SCREEN;
        }*/
        //gl.scissor(0, 0, self.current_width as i32, self.current_height as i32);
    }

    pub fn get_zoom_factor(&self) -> f32 {
        self.zoom_factor
    }

    pub fn apply_transformation(&mut self, t: cgmath::Matrix4<f32>) {
        self.view_mat = t * self.view_mat;
    }

    pub fn send_to_vertex_shader(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        // Send view matrix
        let view_mat_f32_slice: &[f32; 16] = self.view_mat.as_ref();
        let view_mat_location = shader.get_uniform_location(gl, "view");
        gl.uniform_matrix4fv_with_f32_array(view_mat_location.as_ref(), false, view_mat_f32_slice);

        // Send zoom factor
        let zoom_factor_location = shader.get_uniform_location(gl, "zoom_factor");
        gl.uniform1f(zoom_factor_location.as_ref(), self.zoom_factor);

        // Send window size
        let location_resize_factor_x = shader.get_uniform_location(gl, "resize_factor_x");
        gl.uniform1f(location_resize_factor_x.as_ref(), self.resize_factor_x);
        
        let location_resize_factor_y = shader.get_uniform_location(gl, "resize_factor_y");
        gl.uniform1f(location_resize_factor_y.as_ref(), self.resize_factor_y);

        let location_aspect = shader.get_uniform_location(gl, "aspect");

        let (width, height) = window_size_f32();

        let aspect = width / height;
        gl.uniform1f(location_aspect.as_ref(), aspect);
        /*let location_starting_window = shader.get_uniform_location(gl, "window_size_default").unwrap();
        gl.uniform2f(Some(&location_starting_window), self.starting_width, self.starting_height);
        let location_current_window = shader.get_uniform_location(gl, "current_window_size").unwrap();
        gl.uniform2f(Some(&location_current_window), self.current_width, self.current_height);*/
    }
}