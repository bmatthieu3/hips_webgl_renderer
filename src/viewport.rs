
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
    fov: FieldOfView,

    wheel_idx: i16,
    
    pub last_zoom_action: LastZoomAction,
    pub last_action: LastAction,

    // Store the size in pixels of the hips sphere
    //default_size_scissor: Vector2<f32>,
}

use crate::shader::Shader;
use web_sys::WebGl2RenderingContext;

use crate::WebGl2Context;

/// Set the scissor knowing the size in pixel of the
/// HiPS sphere
/*fn set_gl_scissor(gl: &WebGl2Context, size: Vector2<f32>) {
    // Update the scissor
    let (width_screen, height_screen) = window_size_f32();

    let xo = (width_screen / 2_f32) - size.x / 2_f32;
    let yo = (height_screen / 2_f32) - size.y / 2_f32;

    gl.scissor(xo as i32, yo as i32, size.x as i32, size.y as i32);
}*/

const NUM_WHEEL_PER_DEPTH: usize = 5;
use cgmath::Deg;

fn fov<P: Projection>(wheel_idx: i16) -> Rad<f32> {
    let exp = (wheel_idx as f32) / (NUM_WHEEL_PER_DEPTH as f32);
    let fov = P::aperture_start() / 2_f32.powf(exp);

    Deg(fov).into()
}

use web_sys::console;
use cgmath::Matrix4;
use crate::renderable::hips_sphere::HiPSSphere;
use crate::renderable::Renderable;

use crate::renderable::catalog::Catalog;

use crate::renderable::grid::ProjetedGrid;
use crate::mouse_inertia::MouseInertia;
use crate::projection::Projection;

use crate::renderable::hips_sphere::RenderingMode;

impl ViewPort {
    pub fn new<P: Projection>(gl: &WebGl2Context) -> ViewPort {
        let last_zoom_action = LastZoomAction::Unzoom;
        let last_action = LastAction::Moving;

        let wheel_idx = 0;

        let fov = FieldOfView::new::<P>(gl, fov::<P>(wheel_idx));

        let viewport = ViewPort {
            fov,

            wheel_idx,

            last_zoom_action,
            last_action,
        };

        viewport
    }

    pub fn reset_zoom_level<P: Projection>(&mut self) {
        self.wheel_idx = 0;
        // Update the aperture of the Field Of View
        let aperture = fov::<P>(self.wheel_idx);
        self.fov.set_aperture::<P>(aperture);
    }

    pub fn resize_window<P: Projection>(&mut self, width: f32, height: f32) {
        self.fov.resize_window::<P>(width, height);
    }

    pub fn zoom<P: Projection, R: RenderingMode>(
        &mut self,
        hips_sphere: &mut Renderable<HiPSSphere<R>>,
        catalog: &mut Renderable<Catalog>,
    ) -> bool {
        self.last_zoom_action = LastZoomAction::Zoom;
        self.last_action = LastAction::Zooming;

        self.wheel_idx += 1;
        let aperture = fov::<P>(self.wheel_idx);

        let prev_depth = self.fov.current_depth();
        self.fov.set_aperture::<P>(aperture);

        let depth = self.fov.current_depth();

        // Switch to the SmallFieldOfView rendering mode
        prev_depth == 2 && depth > 2
    }

    pub fn unzoom<P: Projection, R: RenderingMode>(
        &mut self,
        hips_sphere: &mut Renderable<HiPSSphere<R>>,
        catalog: &mut Renderable<Catalog>,
    ) -> bool {
        self.last_zoom_action = LastZoomAction::Unzoom;
        self.last_action = LastAction::Zooming;

        let prev_depth = self.fov.current_depth();

        if self.wheel_idx > 0 {
            self.wheel_idx -= 1;

            if self.wheel_idx < 0 {
                self.wheel_idx = 0;
            }
            // Update the aperture of the Field Of View
            let aperture = fov::<P>(self.wheel_idx);
            self.fov.set_aperture::<P>(aperture);
        }

        let depth = self.fov.current_depth();

        // Switch to the perpixel rendering mode
        prev_depth == 3 && depth < 3
    }

    pub fn displacement<P: Projection, R: RenderingMode>(
        &mut self,
        hips_sphere: &mut Renderable<HiPSSphere<R>>,
        catalog: &mut Renderable<Catalog>,
    ) {
        self.last_action = LastAction::Moving;

        // Translate the Field of View on the HiPS sphere
        self.fov.set_rotation_mat::<P>(hips_sphere.get_model_mat());

        // Update the HiPS sphere 
        hips_sphere.update::<P>(&self);
        // Update the catalog loaded
        catalog.update::<P>(&self);
    }

    pub fn field_of_view(&self) -> &FieldOfView {
        &self.fov
    }

    pub fn get_ndc_to_clip(&self) -> &Vector2<f32> {
        self.fov.get_ndc_to_clip()
    }

    pub fn get_clip_zoom_factor(&self) -> f32 {
        self.fov.get_clip_zoom_factor()
    }

    /// Warning: this is executed by all the shaders
    pub fn send_to_vertex_shader(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        // Send window size
        let location_aspect = shader.get_uniform_location("aspect");

        let aspect = self.fov.get_aspect();
        gl.uniform1f(location_aspect, aspect);
        // Send ndc to clip
        let ndc_to_clip_location = shader.get_uniform_location("ndc_to_clip");
        let ndc_to_clip = self.fov.get_ndc_to_clip();
        gl.uniform2f(ndc_to_clip_location, ndc_to_clip.x, ndc_to_clip.y);
        // Send clip zoom factor
        let clip_zoom_factor_location = shader.get_uniform_location("clip_zoom_factor");
        let clip_zoom_factor = self.fov.get_clip_zoom_factor();
        gl.uniform1f(clip_zoom_factor_location, clip_zoom_factor);

        // Send last zoom action
        let last_zoom_action_location = shader.get_uniform_location("last_zoom_action");
        gl.uniform1i(last_zoom_action_location, self.last_zoom_action as i32);
    }

    pub fn get_window_size(&self) -> Vector2<f32> {
        let (width, height) = self.fov.get_size_screen();
        Vector2::new(width, height)
    }
}