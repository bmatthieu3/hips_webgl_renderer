use std::rc::Rc;

pub struct ViewPort {
    window: Rc<web_sys::Window>,
    canvas: Rc<web_sys::HtmlCanvasElement>,

    view_mat: cgmath::Matrix4<f32>,
    zoom_factor: f32,
    resize_factor_x: f32,
    resize_factor_y: f32,
    position: usize,

    starting_width: f32,
    starting_height: f32,

    current_width: f32,
    current_height: f32
}

use crate::shader::Shader;
use crate::math;
use web_sys::WebGl2RenderingContext;

use cgmath::SquareMatrix;
use web_sys::console;

fn get_window_size(window: &web_sys::Window) -> (f32, f32) {
    let width = window.inner_width()
        .unwrap()
        .as_f64()
        .unwrap() as f32;
    let height = window.inner_height()
        .unwrap()
        .as_f64()
        .unwrap() as f32;
    (width, height)
}

impl ViewPort {
    pub fn new(gl: &WebGl2RenderingContext, window: Rc<web_sys::Window>, canvas: Rc<web_sys::HtmlCanvasElement>) -> ViewPort {
        let view_mat = cgmath::Matrix4::identity();
        let zoom_factor = 1_f32;
        let position = 0;

        let (starting_width, starting_height) = get_window_size(window.as_ref());
        let (current_width, current_height) = (starting_width, starting_height);

        let resize_factor_x = 1.0_f32;
        let resize_factor_y = 1.0_f32;

        //gl.viewport(0, 0, current_width as i32, current_height as i32);

        let mut viewport = ViewPort {
            window,
            canvas,

            view_mat,
            zoom_factor,
            resize_factor_x,
            resize_factor_y,
            position,

            starting_width,
            starting_height,
            current_width,
            current_height
        };

        viewport.resize(gl);
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
    }

    pub fn resize(&mut self, gl: &WebGl2RenderingContext) {
        

        let (current_width, current_height) = get_window_size(self.window.as_ref());
        self.current_width = current_width;
        self.current_height = current_height;
        
        self.canvas.set_width(self.current_width as u32);
        self.canvas.set_height(self.current_height as u32);
        gl.viewport(0, 0, current_width as i32, current_height as i32);
    /*
        if self.current_width > 2_f32 * self.current_height {
            self.resize_factor_x = self.current_height / self.starting_height;
            self.resize_factor_y = self.current_width / self.starting_width;
        } else {
            self.resize_factor_x = self.current_height / self.starting_height;
            self.resize_factor_y = self.current_width / self.starting_width;
        }*/
        //gl.scissor(0, 0, self.current_width as i32, self.current_height as i32);
    }

    pub fn get_zoom_factor(&self) -> f32 {
        self.zoom_factor
    }

    pub fn get_window_size(&self) -> (f32, f32) {
        (self.current_width, self.current_height)
    }

    pub fn get_starting_window_size(&self) -> (f32, f32) {
        (self.starting_width, self.starting_height)
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
        let location_resize_factor_x = shader.get_uniform_location(gl, "resize_factor_x").unwrap();
        gl.uniform1f(Some(&location_resize_factor_x), self.resize_factor_x);
        let location_resize_factor_y = shader.get_uniform_location(gl, "resize_factor_y").unwrap();
        gl.uniform1f(Some(&location_resize_factor_y), self.resize_factor_y);
        /*let location_starting_window = shader.get_uniform_location(gl, "window_size_default").unwrap();
        gl.uniform2f(Some(&location_starting_window), self.starting_width, self.starting_height);
        let location_current_window = shader.get_uniform_location(gl, "current_window_size").unwrap();
        gl.uniform2f(Some(&location_current_window), self.current_width, self.current_height);*/
    }
}