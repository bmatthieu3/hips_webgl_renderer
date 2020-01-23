
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

    aspect: f32,

    fov_lookup_table: [Rad<f32>; NUM_WHEEL_PER_DEPTH * 29],
    zoom_index: i16,

    screen_scaling_factor: Vector2<f32>,
    
    pub last_zoom_action: LastZoomAction,
    pub last_action: LastAction,

    // Store the size in pixels of the hips sphere
    default_size_scissor: Vector2<f32>,
}

use crate::shader::Shader;
use web_sys::WebGl2RenderingContext;

use crate::WebGl2Context;

use crate::{set_window_size, window_size_f32, window_size_u32};
use wasm_bindgen::JsCast;


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

    fov
}

use web_sys::console;
use crate::math;
use std::sync::atomic::Ordering;
use crate::MAX_DEPTH;
use cgmath::Matrix4;
use crate::renderable::hips_sphere::HiPSSphere;
use crate::renderable::Renderable;

use crate::renderable::catalog::Catalog;
use crate::renderable::grid::ProjetedGrid;
use crate::mouse_inertia::MouseInertia;
use crate::projection::Projection;
impl ViewPort {
    pub fn new<P: Projection>(gl: &WebGl2Context, hips_sphere: &Renderable<HiPSSphere>) -> ViewPort {
        //let final_zoom = current_zoom;

        let canvas = Rc::new(
            gl.canvas().unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap()
        );

        let last_zoom_action = LastZoomAction::Unzoom;
        let last_action = LastAction::Moving;

        let default_size_scissor = P::size();
        let (width, height) = window_size_f32();
        let aspect = width / height;

        let fov_lookup_table = field_of_view_table();
        let zoom_index = 0;
        
        let mut fov = FieldOfView::new(&hips_sphere.mesh().get_buffer().borrow());
        fov.set_aperture::<P>(fov_lookup_table[0], hips_sphere);
        let gl = gl.clone();

        let screen_scaling_factor = *fov.get_scaling_screen_factor();
        let mut viewport = ViewPort {
            gl,
            canvas,

            fov,

            aspect,
            fov_lookup_table,
            zoom_index,

            screen_scaling_factor,

            last_zoom_action,
            last_action,

            default_size_scissor,
        };

        viewport.resize(&default_size_scissor);

        viewport
    }

    pub fn update_scissor(&self) {
        let ref screen_scaling = self.fov.get_scaling_screen_factor();
        // Take into account the zoom factor
        let current_size_scissor = Vector2::new(
            self.default_size_scissor.x / screen_scaling.x,
            self.default_size_scissor.y / screen_scaling.y
        );
        set_gl_scissor(&self.gl, current_size_scissor);
    }

    pub fn zoom<P: Projection>(
        &mut self,
        hips_sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
    ) {
        self.last_zoom_action = LastZoomAction::Zoom;
        self.last_action = LastAction::Zooming;

        if self.zoom_index < (self.fov_lookup_table.len() - 1) as i16  {
            self.zoom_index += 1;
        }

        self.fov.set_aperture::<P>(self.fov_lookup_table[self.zoom_index as usize], hips_sphere);
        self.screen_scaling_factor = *self.fov.get_scaling_screen_factor();

        // Update the HiPS sphere 
        hips_sphere.update::<P>(&self);
        // Update the catalog loaded
        catalog.update::<P>(&self);

        self.update_scissor();
    }

    pub fn unzoom<P: Projection>(
        &mut self,
        hips_sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
    ) {
        self.last_zoom_action = LastZoomAction::Unzoom;
        self.last_action = LastAction::Zooming;

        if self.zoom_index > 0 {
            self.zoom_index -= 1;
        }

        // Update the aperture of the Field Of View
        self.fov.set_aperture::<P>(
            self.fov_lookup_table[self.zoom_index as usize],
            hips_sphere
        );
        self.screen_scaling_factor =  *self.fov.get_scaling_screen_factor();

        // Update the HiPS sphere 
        hips_sphere.update::<P>(&self);
        // Update the catalog loaded
        catalog.update::<P>(&self);

        self.update_scissor();
    }

    pub fn displacement<P: Projection>(
        &mut self,
        hips_sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
    ) {
        self.last_action = LastAction::Moving;

        // Translate the Field of View on the HiPS sphere
        self.fov.translate(hips_sphere);

        // Update the HiPS sphere 
        hips_sphere.update::<P>(&self);
        // Update the catalog loaded
        catalog.update::<P>(&self);
    }

    pub fn field_of_view(&self) -> &FieldOfView {
        &self.fov
    }

    pub fn resize(&mut self, new_size_scissor: &Vector2<f32>) {
        let (width, height) = window_size_u32();

        set_window_size(width, height);
        self.aspect = (width as f32) / (height as f32);

        self.canvas.set_width(width);
        self.canvas.set_height(height);
        self.gl.viewport(0, 0, width as i32, height as i32);

        self.default_size_scissor = *new_size_scissor;
        self.update_scissor();
    }

    pub fn get_scaling_screen_factor(&self) -> &Vector2<f32> {
        &self.screen_scaling_factor
    }

    /// Warning: this is executed by all the shaders
    pub fn send_to_vertex_shader(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        // Send window size
        let location_aspect = shader.get_uniform_location("aspect");

        gl.uniform1f(location_aspect, self.aspect);
        // Send zoom factor
        let zoom_factor_location = shader.get_uniform_location("zoom_factor");
        gl.uniform2f(zoom_factor_location, self.screen_scaling_factor.x, self.screen_scaling_factor.y);

        // Send last zoom action
        let last_zoom_action_location = shader.get_uniform_location("last_zoom_action");
        gl.uniform1i(last_zoom_action_location, self.last_zoom_action as i32);
    }
}