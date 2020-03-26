
use std::rc::Rc;

#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum LastZoomAction {
    Zoom = 1,
    Unzoom = 2,
}

#[derive(PartialEq, Clone, Copy)]
pub enum LastAction {
    Zooming = 1,
    Unzooming = 2,
    Moving = 3,
}

use crate::field_of_view::FieldOfView;
use cgmath::Rad;
use cgmath::Vector2;
pub struct ViewPort {
    fov: FieldOfView,

    wheel_idx: i32,
    
    pub last_zoom_action: LastZoomAction,
    pub last_action: LastAction,

    // Max depth of the current loaded HiPS
    max_depth: u8,

    model_mat: cgmath::Matrix4::<f32>,
    inverted_model_mat: cgmath::Matrix4<f32>,
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


use cgmath::Deg;
use crate::event_manager::NUM_WHEEL_PER_DEPTH;
fn wheel_idx<P: Projection>(fov: Rad<f32>) -> i32 {
    let p0: Rad<f32> = P::aperture_start().into();
    ((p0.0 / fov.0).log2() * (NUM_WHEEL_PER_DEPTH as f32)) as i32
}

use crate::renderable::hips_sphere::HiPSSphere;
use crate::renderable::Renderable;

use crate::renderable::catalog::Catalog;

use crate::renderable::grid::ProjetedGrid;
use crate::projection::Projection;

use crate::buffer_tiles::HiPSConfig;
use cgmath::{Matrix3, Matrix4, Vector4};
use cgmath::SquareMatrix;
impl ViewPort {
    pub fn new<P: Projection>(gl: &WebGl2Context, hips: &HiPSConfig) -> ViewPort {
        let last_zoom_action = LastZoomAction::Unzoom;
        let last_action = LastAction::Moving;

        let wheel_idx = 0;

        let max_depth = hips.max_depth;
        let fov = FieldOfView::new::<P>(gl, P::aperture_start().into(), max_depth);

        let model_mat = Matrix4::identity();
        let inverted_model_mat = model_mat;

        let viewport = ViewPort {
            fov,

            wheel_idx,

            last_zoom_action,
            last_action,

            max_depth,

            // The model matrix of the Renderable
            model_mat,
            inverted_model_mat,
        };

        viewport
    }

    // Tell the viewport the HiPS have changed
    pub fn set_max_depth(&mut self, hips: &HiPSConfig) {
        self.max_depth = hips.max_depth;
    }

    pub fn reset_zoom_level<P: Projection>(&mut self) {
        self.wheel_idx = 0;
        // Update the aperture of the Field Of View
        let aperture: Rad<f32> = P::aperture_start().into();
        self.fov.set_aperture::<P>(aperture, self.max_depth);
    }

    fn set_aperture<P: Projection>(&mut self, aperture: Rad<f32>) {
        let aperture = if aperture <= P::aperture_start().into() {
            // Retrieve the wheel idx correponding to the current aperture for the
            // projection
            self.wheel_idx = wheel_idx::<P>(aperture);
            aperture
        } else {
            // The start aperture of the new projection is < to the current aperture
            // We reset the wheel idx too
            self.wheel_idx = 0;
            P::aperture_start().into()
        };
        // Recompute the depth and field of view
        self.fov.set_aperture::<P>(aperture, self.max_depth);
    }

    // Called when the projection changes
    pub fn reset<P: Projection>(&mut self) {
        let current_aperture = self.fov.get_aperture();
        self.set_aperture::<P>(current_aperture);
    }

    pub fn resize_window<P: Projection>(&mut self, width: f32, height: f32,
        hips_sphere: &mut Renderable<HiPSSphere>,
        grid: &mut Renderable<ProjetedGrid>,
        catalog: &mut Renderable<Catalog>,
    ) {
        self.fov.resize_window::<P>(width, height, self.max_depth);

        // Launch the new tile requests
        hips_sphere.mesh_mut().request_tiles(&self);
        // Retrieve the sources in the fov
        catalog.mesh_mut().retrieve_sources_in_fov::<P>(&self);
        // Reproject the grid
        grid.mesh_mut().reproject::<P>(hips_sphere, &self);
    }

    pub fn zoom<P: Projection>(
        &mut self,
        aperture: Rad<f32>,
        hips_sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
        grid: &mut Renderable<ProjetedGrid>,
    ) {
        self.last_zoom_action = LastZoomAction::Zoom;
        self.last_action = LastAction::Zooming;

        self.fov.set_aperture::<P>(aperture, self.max_depth);

        // Launch the new tile requests
        hips_sphere.mesh_mut().request_tiles(&self);
        // Retrieve the sources in the fov
        catalog.mesh_mut().retrieve_sources_in_fov::<P>(&self);

        // Reproject the grid
        //grid.mesh_mut().reproject::<P>(hips_sphere, &self);
    }

    pub fn unzoom<P: Projection>(
        &mut self,
        aperture: Rad<f32>,
        hips_sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
        grid: &mut Renderable<ProjetedGrid>,
    ) {
        self.last_zoom_action = LastZoomAction::Unzoom;
        self.last_action = LastAction::Unzooming;

        self.fov.set_aperture::<P>(aperture, self.max_depth);
        
        // Launch the new tile requests
        hips_sphere.mesh_mut().request_tiles(&self);
        // Retrieve the sources in the fov
        catalog.mesh_mut().retrieve_sources_in_fov::<P>(&self);
        // Reproject the grid
        //grid.mesh_mut().reproject::<P>(hips_sphere, &self);
    }

    pub fn displacement<P: Projection>(
        &mut self,
        hips_sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
        grid: &mut Renderable<ProjetedGrid>,
    ) {
        self.last_action = LastAction::Moving;

        // Translate the Field of View on the HiPS sphere
        self.fov.set_rotation_mat::<P>(&self.model_mat, self.max_depth);

        // Moves the catalog according to the viewport displacement
        //grid.set_model_mat(inv_model_mat);
        //catalog.set_model_mat(&self.inverted_model_mat);

        // Launch the new tile requests
        hips_sphere.mesh_mut().request_tiles(&self);
        // Retrieve the sources in the fov
        catalog.mesh_mut().retrieve_sources_in_fov::<P>(&self);
        // Reproject the grid
        //grid.mesh_mut().reproject::<P>(hips_sphere, &self);
    }

    // Called by the FSM when a MouseWheelUp has been detected
    pub fn up_wheel_idx(&mut self) {
        self.wheel_idx += 1;
    }

    // Called by the FSM when a MouseWheelDown has been detected
    pub fn down_wheel_idx(&mut self) {
        if self.wheel_idx > 0 {
            self.wheel_idx -= 1;
        }
    }

    pub fn get_wheel_idx(&mut self) -> i32 {
        self.wheel_idx
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

    pub fn get_last_action(&self) -> LastAction {
        self.last_action
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

    // Viewport model matrices
    pub fn get_window_size(&self) -> Vector2<f32> {
        let (width, height) = self.fov.get_size_screen();
        Vector2::new(width, height)
    }

    pub fn compute_center_world_pos<P: Projection>(&self) -> Vector4<f32> {
        let ref model_mat = self.get_model_mat();

        (*model_mat) * P::clip_to_world_space(Vector2::new(0_f32, 0_f32)).unwrap()
    }
    pub fn apply_rotation(&mut self, axis: cgmath::Vector3<f32>, angle: cgmath::Rad<f32>) {
        self.model_mat = cgmath::Matrix4::<f32>::from_axis_angle(axis, angle) * self.model_mat;
        self.inverted_model_mat = self.model_mat.invert().unwrap();
    }
    pub fn apply_quarternion_rotation(&mut self, q: &cgmath::Quaternion<f32>) {
        let drot: Matrix4<f32> = (*q).into();
        
        self.model_mat = drot * self.model_mat;
        self.inverted_model_mat = self.model_mat.invert().unwrap();
    }
    pub fn set_model_mat(&mut self, model_mat: &cgmath::Matrix4<f32>) {
        self.model_mat = *model_mat;
        self.inverted_model_mat = self.model_mat.invert().unwrap();
    }

    pub fn get_model_mat(&self) -> &cgmath::Matrix4<f32> {
        return &self.model_mat; 
    }
    pub fn get_quat(&self) -> cgmath::Quaternion<f32> {
        // Extract a 3x3 matrix from the model 4x4 matrix
        let v: [[f32; 4]; 4] = self.model_mat.into();

        let mat3 = Matrix3::new(
            v[0][0], v[0][1], v[0][2],
            v[1][0], v[1][1], v[1][2],
            v[2][0], v[2][1], v[2][2]
        );

        mat3.into()
    }

    pub fn get_inverted_model_mat(&self) -> &cgmath::Matrix4<f32> {
        return &self.inverted_model_mat;
    }
}