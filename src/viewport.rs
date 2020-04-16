
use std::rc::Rc;

#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum LastZoomAction {
    Zoom = 1,
    Unzoom = 2,
    Starting, 
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

    // HiPS current config
    config: HiPSConfig,

    mat_x: cgmath::Matrix4::<f32>,
    mat_y: cgmath::Matrix4::<f32>,


    sr: SphericalRotation<f32>,
    model_mat: cgmath::Matrix4::<f32>,
    inverted_model_mat: cgmath::Matrix4<f32>,
}

use crate::WebGl2Context;

use crate::event_manager::NUM_WHEEL_PER_DEPTH;
fn wheel_idx<P: Projection>(fov: Rad<f32>) -> i32 {
    let p0: Rad<f32> = P::aperture_start().into();
    ((p0.0 / fov.0).log2() * (NUM_WHEEL_PER_DEPTH as f32)) as i32
}

use crate::renderable::HiPSSphere;

use crate::renderable::catalog::Catalog;

use crate::renderable::grid::ProjetedGrid;
use crate::projection::Projection;

use crate::buffer::HiPSConfig;
use cgmath::{Matrix3, Matrix4, Vector4};
use cgmath::SquareMatrix;

use crate::rotation::SphericalRotation;

use cgmath::Quaternion;
use crate::math;
impl ViewPort {
    pub fn new<P: Projection>(gl: &WebGl2Context, config: &HiPSConfig) -> ViewPort {
        let last_zoom_action = LastZoomAction::Starting;
        let last_action = LastAction::Moving;

        let wheel_idx = 0;
        //let tile_depth = math::log_2(hips.get_tile_size());

        let fov = FieldOfView::new::<P>(gl, P::aperture_start().into(), config);

        let model_mat = Matrix4::identity();
        let mat_x = Matrix4::identity();
        let mat_y = Matrix4::identity();

        let inverted_model_mat = model_mat;

        let sr = SphericalRotation::zero();

        let config = config.clone();
        let viewport = ViewPort {
            fov,

            wheel_idx,

            last_zoom_action,
            last_action,

            config,

            // The model matrix of the Renderable
            mat_x,
            mat_y,
            sr,
            model_mat,
            inverted_model_mat,
        };

        viewport
    }

    // Tell the viewport the HiPS have changed
    pub fn set_hips_config(&mut self, config: &HiPSConfig) {
        self.config = config.clone();
        self.fov.set_config(config);
        //self.tile_depth = math::log_2(hips.get_tile_size());

        self.last_zoom_action = LastZoomAction::Starting;
    }

    pub fn reset_zoom_level<P: Projection>(&mut self) {
        self.wheel_idx = 0;
        // Update the aperture of the Field Of View
        let aperture: Rad<f32> = P::aperture_start().into();
        self.fov.set_aperture::<P>(aperture);
    }

    pub fn get_aperture(&self) -> Rad<f32> {
        self.fov.get_aperture()
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
        self.fov.set_aperture::<P>(aperture);
    }

    // Called when the projection changes
    pub fn reset<P: Projection>(&mut self) {
        let current_aperture = self.fov.get_aperture();
        self.set_aperture::<P>(current_aperture);
    }

    pub fn resize_window<P: Projection>(&mut self, width: f32, height: f32,
        hips_sphere: &mut HiPSSphere,
        grid: &mut ProjetedGrid,
        catalog: &mut Catalog,
    ) {
        self.fov.resize_window::<P>(width, height);

        // Launch the new tile requests
        hips_sphere.request_tiles(&self);
        // Retrieve the sources in the fov
        catalog.retrieve_sources_in_fov::<P>(&self);
        // Reproject the grid
        grid.reproject::<P>(&self);
    }

    pub fn zoom<P: Projection>(
        &mut self,
        aperture: Rad<f32>,
        hips_sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
    ) {
        self.last_zoom_action = LastZoomAction::Zoom;
        self.last_action = LastAction::Zooming;

        self.fov.set_aperture::<P>(aperture);

        // Launch the new tile requests
        hips_sphere.request_tiles(&self);
        // Retrieve the sources in the fov
        catalog.retrieve_sources_in_fov::<P>(&self);

        // Reproject the grid
        //grid.mesh_mut().reproject::<P>(hips_sphere, &self);
    }

    pub fn unzoom<P: Projection>(
        &mut self,
        aperture: Rad<f32>,
        hips_sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
    ) {
        self.last_zoom_action = LastZoomAction::Unzoom;
        self.last_action = LastAction::Unzooming;

        self.fov.set_aperture::<P>(aperture);
        
        // Launch the new tile requests
        hips_sphere.request_tiles(&self);
        // Retrieve the sources in the fov
        catalog.retrieve_sources_in_fov::<P>(&self);
        // Reproject the grid
        //grid.mesh_mut().reproject::<P>(hips_sphere, &self);
    }

    pub fn displacement<P: Projection>(
        &mut self,
        hips_sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
    ) {
        self.last_action = LastAction::Moving;

        // Translate the Field of View on the HiPS sphere
        self.fov.set_rotation_mat::<P>(&self.model_mat);

        // Moves the catalog according to the viewport displacement
        //grid.set_model_mat(inv_model_mat);
        //catalog.set_model_mat(&self.inverted_model_mat);

        // Launch the new tile requests
        hips_sphere.request_tiles(&self);
        // Retrieve the sources in the fov
        catalog.retrieve_sources_in_fov::<P>(&self);
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
    // Viewport model matrices
    pub fn get_window_size(&self) -> Vector2<f32> {
        let (width, height) = self.fov.get_size_screen();
        Vector2::new(width, height)
    }

    pub fn compute_center_world_pos<P: Projection>(&self) -> Vector4<f32> {
        P::clip_to_world_space(
            &Vector2::new(0_f32, 0_f32),
            self
        ).unwrap()
    }

    // Check whether border of the screen are inside
    // the projection
    pub fn screen_inside_of_projection<P: Projection>(&self) -> bool {
        // Projection are symmetric, we can check for only one vertex
        // of the screen
        let corner_tl_ndc = Vector2::new(-1_f32, 1_f32);
        let corner_tl_clip = crate::projection::ndc_to_clip_space(&corner_tl_ndc, self);

        if let Some(_) = P::clip_to_model_space(&corner_tl_clip) {
            true
        } else {
            false
        }
    }

    pub fn apply_rotation(&mut self, axis: &cgmath::Vector3<f32>, angle: cgmath::Rad<f32>) {
        let dq = SphericalRotation::from_axis_angle(axis, angle);
        self.sr = dq * self.sr;

        self.compute_model_mat();
    }

    fn compute_model_mat(&mut self) {
        //self.model_mat = self.mat_y * self.mat_x;
        self.model_mat = (&self.sr).into();
        self.inverted_model_mat = self.model_mat.invert().unwrap();
    }

    /*pub fn apply_rotation_longitude(&mut self, dtheta: Rad<f32>) {
        let rot_y = Matrix4::from_angle_y(dtheta);
        self.mat_y = rot_y * self.mat_y;

        self.compute_model_mat();
    }

    pub fn apply_rotation_latitude(&mut self, ddelta: Rad<f32>) {
        let rot_x = Matrix4::from_angle_x(-ddelta);
        self.mat_x = rot_x * self.mat_x;

        self.compute_model_mat();
    }

    pub fn apply_rotation_lonlat(&mut self, dtheta: Rad<f32>, ddelta: Rad<f32>) {
        let rot_y = Matrix4::from_angle_y(dtheta);
        let rot_x = Matrix4::from_angle_x(-ddelta);

        self.mat_y = rot_y * self.mat_y;
        self.mat_x = rot_x * self.mat_x;

        self.compute_model_mat();
    }*/

    pub fn set_rotation(&mut self, rot: &SphericalRotation<f32>) {
        /*self.mat_x = rot.get_rot_x();
        self.mat_y = rot.get_rot_y();*/
        self.sr = *rot;

        self.compute_model_mat();
    }

    /*pub fn apply_quarternion_rotation(&mut self, q: &cgmath::Quaternion<f32>) {
        let drot: Matrix4<f32> = (*q).into();
        
        self.model_mat = drot * self.model_mat;
        self.inverted_model_mat = self.model_mat.invert().unwrap();
    }*/

    /*pub fn set_model_mat(&mut self, model_mat: &cgmath::Matrix4<f32>) {
        self.model_mat = *model_mat;
        self.inverted_model_mat = self.model_mat.invert().unwrap();
    }*/
    pub fn get_rotation(&self) -> &SphericalRotation<f32> {
        &self.sr
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

use crate::shader::HasUniforms;
use crate::shader::ShaderBound;

impl HasUniforms for ViewPort {
    fn attach_uniforms<'a>(&self, shader: &'a ShaderBound<'a>) -> &'a ShaderBound<'a> {
        // Send window size
        shader.attach_uniform("aspect", &self.fov.get_aspect())
            // Send ndc to clip
            .attach_uniform("ndc_to_clip", self.fov.get_ndc_to_clip())
            // Send clip zoom factor
            .attach_uniform("clip_zoom_factor", &self.fov.get_clip_zoom_factor())
            // Send last zoom action
            .attach_uniform("last_zoom_action", &(self.last_zoom_action as i32));

        shader
    }
}