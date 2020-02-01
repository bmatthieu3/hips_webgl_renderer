#[macro_use]
extern crate lazy_static;
extern crate itertools_num;
extern crate rand;
extern crate serde_derive;

use web_sys::console;
#[macro_export]
macro_rules! print_to_console {
    ( $text:expr, $( $x:expr ),* ) => {
        {
            console::log_1(&format!($text, $($x)*).into());
        }
    };
}

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

use cgmath;

mod shader;
mod shaders;
mod renderable;
mod viewport;
mod texture;
mod math;
mod utils;
mod projeted_grid;
mod render_next_frame;
mod field_of_view;
mod event;
mod mouse_inertia;
mod color;

use shader::Shader;

use renderable::Renderable;
use renderable::hips_sphere::HiPSSphere;
use renderable::grid::ProjetedGrid;
use renderable::catalog::{Catalog, Source};

use renderable::projection;
use renderable::projection::{Aitoff, Orthographic, MollWeide};

use viewport::ViewPort;

use std::sync::Mutex;
use std::sync::Arc;

use std::rc::Rc;

use std::collections::HashMap;

use std::sync::atomic::{AtomicU32, AtomicU8, AtomicBool};
use std::sync::atomic::Ordering;

use crate::render_next_frame::LATEST_TIME_TILE_RECEIVED;
use crate::texture::BLENDING_DURATION_MS;

use crate::event::Move;

use crate::projection::Projection;

#[macro_use]
extern crate aladinlite_derive;

use crate::shaders::colormap::*;
use crate::shaders::catalog::*;

use crate::renderable::hips_sphere::RenderingMode;

use cgmath::Quaternion;

use crate::mouse_inertia::MouseInertia;

struct App<P>
where P: Projection {
    gl: WebGl2Context,

    shaders: HashMap<&'static str, Shader>,

    viewport: ViewPort,
    projection: std::marker::PhantomData<P>,

    // The sphere renderable
    hips_sphere: Renderable<HiPSSphere>,
    // The grid renderable
    grid: Renderable<ProjetedGrid>,
    // The catalogs
    catalog: Renderable<Catalog>,

    // Move event
    moving: Option<Move>,
    inertia: Option<MouseInertia>,

    // Animation rotation
    pub animation_request: bool,
    pub final_pos: Quaternion<f32>,
    pub start_pos: Quaternion<f32>,

    pub start_aperture: Deg<f32>,
    pub end_aperture: Deg<f32>,

    // Angular distance between the start and final quaternion
    pub d: Rad<f32>,

    pub start_time: f32,
    pub animation_duration: f32,

    // Render the next frame
    render: bool,
}
/*
fn add_tile_buffer_uniforms(name: &'static str, size: usize, uniforms: &mut Vec<&'static str>) {

    //uniforms.push(texture.clone());
    /*let base_texture = "textures_tiles[";

    for i in 0..size {
        let mut tile = concat!(base_texture, &i.to_string());
        tile += "].";

        let uniq = tile.clone() + "uniq";
        let texture_idx = tile.clone() + "texture_idx";
        let time_received = tile.clone() + "time_received";
        let time_request = tile + "time_request";

        uniforms.push(uniq.as_str());
        uniforms.push(texture_idx.as_str());
        uniforms.push(time_received.as_str());
        uniforms.push(time_request.as_str());
    }*/

    let aa = hpx_tiles_uniforms! {0, 1, 2};

    uniforms.extend(aa.iter());
}*/

use cgmath::Matrix4;
use crate::shader::Shaderize;

use crate::renderable::hips_sphere::PerPixel;

use cgmath::{Vector2, Vector3, Matrix3};
use cgmath::Deg;
use cgmath::InnerSpace;

impl<P> App<P>
where P: Projection {
    fn new(gl: &WebGl2Context) -> Result<App<Aitoff>, JsValue> {
        console::log_1(&format!("Init").into());

        // Shader definition
        // HiPS sphere shader
        // uniforms definition
        let mut uniforms_2d_proj = vec![
            // General uniforms
            "current_time",
            "model",
            // Viewport uniforms
            "ndc_to_clip",
            "clip_zoom_factor",
            "aspect",
            "last_zoom_action",
            // HiPS Sphere-specific uniforms
            "current_depth",
            "max_depth",
            "num_tiles",
            // Textures
            "textures[0]",
            "textures[1]",
        ];
        uniforms_2d_proj.extend(crate::shaders::uniform_healpix_tiles::HPX_TILES_BUFFER_UNIFORMS);

        //add_tile_buffer_uniforms("textures", 128, &mut uniforms_2d_proj);

        let shader_2d_proj = Shader::new(&gl,
            shaders::proj_vert::CONTENT,
            shaders::proj_frag::CONTENT,
            // uniform list
            &uniforms_2d_proj[..]
        );

        // Grid shader
        // uniforms definition
        let uniforms_grid = vec![
            // General uniforms
            "current_time",
            "model",
            // Viewport uniforms
            "ndc_to_clip",
            "clip_zoom_factor",
            "aspect",
            "last_zoom_action",
            // Grid-specific uniforms
            "location_color",
        ];
        let shader_grid = Shader::new(&gl,
            shaders::grid_projeted_vert::CONTENT,
            shaders::grid_frag::CONTENT,
            &uniforms_grid[..]
        );

        // HiPS Ortho shader
        // uniforms definition
        let mut uniforms_ortho_hips = vec![
            // General uniforms
            "current_time",
            "model",
            // Viewport uniforms
            "ndc_to_clip",
            "clip_zoom_factor",
            "aspect",
            "last_zoom_action",
            // HiPS Ortho specific uniforms
            "current_depth",
            "max_depth",
            // Textures
            "textures[0]",
            "textures[1]",
            "num_tiles",
        ];
        uniforms_ortho_hips.extend(crate::shaders::uniform_healpix_tiles::HPX_TILES_BUFFER_UNIFORMS);
        //add_tile_buffer_uniforms("textures", 128, &mut uniforms_ortho_hips);
        console::log_1(&format!("CC").into());

        let shader_ortho_hips = Shader::new(&gl,
            shaders::hips_sphere_small_fov_vert::CONTENT,
            shaders::hips_sphere_small_fov_frag::CONTENT,
            &uniforms_ortho_hips[..]
        );
        console::log_1(&format!("DD").into());

        let mut shaders = HashMap::new();
        shaders.insert("hips_sphere", shader_2d_proj);
        shaders.insert("grid", shader_grid);
        //shaders.insert("catalog", shader_catalog);

        // Colormap shaders definition
        let colormap_shader_uniforms = &[
            // General uniforms
            "current_time",
            "model",
            // Viewport uniforms
            "ndc_to_clip",
            "clip_zoom_factor",
            "aspect",
            "last_zoom_action",
            // Heatmap-specific uniforms
            "texture_fbo",
            "colormap",
            "alpha",
        ];
        BluePastelRed::create_shader(&gl, &mut shaders, colormap_shader_uniforms);
        IDL_CB_BrBG::create_shader(&gl, &mut shaders, colormap_shader_uniforms);
        IDL_CB_YIGnBu::create_shader(&gl, &mut shaders, colormap_shader_uniforms);
        IDL_CB_GnBu::create_shader(&gl, &mut shaders, colormap_shader_uniforms);
        Red_Temperature::create_shader(&gl, &mut shaders, colormap_shader_uniforms);
        Black_White_Linear::create_shader(&gl, &mut shaders,colormap_shader_uniforms);

        // Catalog shaders definition
        let catalog_shader_uniforms = &[
            // General uniforms
            "current_time",
            "model",
            // Viewport uniforms
            "ndc_to_clip",
            "clip_zoom_factor",
            "aspect",
            "last_zoom_action",
            // Catalog-specific uniforms
            "kernel_texture",
            "strength",
            "max_plx",
            "min_plx",
            "min_size_source",
            "max_size_source",
        ];
        Catalog_Orthographic::create_shader(&gl, &mut shaders, catalog_shader_uniforms);
        Catalog_Aitoff::create_shader(&gl, &mut shaders, catalog_shader_uniforms);
        Catalog_MollWeide::create_shader(&gl, &mut shaders, catalog_shader_uniforms);

        shaders.insert("hips_sphere_small_fov", shader_ortho_hips);

        gl.enable(WebGl2RenderingContext::BLEND);
        gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        //gl.enable(WebGl2RenderingContext::SCISSOR_TEST);
        gl.enable(WebGl2RenderingContext::CULL_FACE);
        gl.cull_face(WebGl2RenderingContext::BACK);

        // Viewport definition
        let viewport = ViewPort::new::<Aitoff>(&gl);

        // HiPS Sphere definition
        let hips_sphere_mesh = HiPSSphere::new(&gl, &viewport);
        console::log_1(&format!("fffff sfs").into());
        let mut hips_sphere = Renderable::<HiPSSphere>::new(
            &gl,
            &shaders,
            hips_sphere_mesh,
        );
        hips_sphere.mesh_mut().update::<Aitoff>(&viewport);
        console::log_1(&format!("fffff sfs").into());
        // Catalog definition
        let catalog_mesh = Catalog::new(&gl, vec![]);
        let catalog = Renderable::<Catalog>::new(
            &gl,
            &shaders,
            catalog_mesh
        );
        console::log_1(&format!("fffff sfs3").into());

        console::log_1(&format!("fffff sfs4").into());
        // Update the HiPS sphere 
        //(&mut hips_sphere).update(&projection, &viewport);
        // Update the catalog loaded
        //(&mut catalog).update(&projection, &viewport);

        // Grid definition
        let lon_bound = cgmath::Vector2::<cgmath::Rad<f32>>::new(cgmath::Deg(-30_f32).into(), cgmath::Deg(30_f32).into());
        let lat_bound = cgmath::Vector2::<cgmath::Rad<f32>>::new(cgmath::Deg(-90_f32).into(), cgmath::Deg(90_f32).into());
        let projeted_grid_mesh = ProjetedGrid::new::<P>(&gl, cgmath::Deg(30_f32).into(), cgmath::Deg(30_f32).into(), Some(lat_bound), None, &viewport);
        let grid = Renderable::<ProjetedGrid>::new(
            &gl,
            &shaders,
            projeted_grid_mesh,
        );

        let animation_request = false;
        let final_pos = Quaternion::new(1_f32, 0_f32, 0_f32, 0_f32);
        let start_pos = Quaternion::new(1_f32, 0_f32, 0_f32, 0_f32);
        let start_time = 0_f32;
        let animation_duration = 2000_f32;

        let start_aperture = P::aperture_start();
        let end_aperture = P::aperture_start();

        let d = Rad(0_f32);

        let moving = None;
        let inertia = None;
        let gl = gl.clone();
        let render = true;
        let app = App {
            gl,

            shaders,

            viewport,
            projection: std::marker::PhantomData,

            // The sphere renderable
            hips_sphere,
            // The grid renderable
            grid,
            // The catalog renderable
            catalog,

            moving,
            inertia,

            animation_request,
            final_pos,
            start_pos,

            start_aperture,
            end_aperture,

            d,

            animation_duration,
            start_time,

            render,
        };

        Ok(app)
    } 

    fn update(&mut self, dt: f32, enable_grid: bool) -> Vector2<Rad<f32>> {
        /*if !UPDATE_USER_INTERFACE.load(Ordering::Relaxed) {
            RENDER_FRAME.lock().unwrap().update(&self.viewport);
            //UPDATE_FRAME.lock().unwrap().update(&self.viewport);
        } else {
            UPDATE_USER_INTERFACE.store(false, Ordering::Relaxed);
        }*/

        //self.viewport.update(&mut self.inertia, &mut self.grid, &mut self.catalog, &mut self.hips_sphere);
        // Animation request
        let mut hips_sphere_updated = false;
        if self.animation_request {
            let mut a = (utils::get_current_time() - self.start_time) / self.animation_duration;
            if a >= 1.0_f32 {
                a = 1_f32;
            }

            let next_pos = self.start_pos.slerp(self.final_pos, a);

            let next_aperture: Deg<f32> = if a <= 0.5_f32 {
                let t = a * 2_f32;
                //Deg((1_f32 - t.sqrt().sqrt()) * self.start_aperture.0 + t.sqrt().sqrt() * self.end_aperture.0)
                Deg((1_f32 - t.sqrt()) * self.start_aperture.0 + t.sqrt() * self.end_aperture.0)
                //Deg((1_f32 - t) * self.start_aperture.0 + t * self.end_aperture.0)
            } else {
                let t = (a - 0.5_f32) * 2_f32;
                Deg((1_f32 - t*t) * self.end_aperture.0 + t*t * self.start_aperture.0)
                //Deg((1_f32 - t*t*t*t) * self.end_aperture.0 + t*t*t*t * self.start_aperture.0)
                //Deg((1_f32 - t) * self.end_aperture.0 + t * self.start_aperture.0)
            };
            self.viewport.set_aperture::<P>(next_aperture.into());

            self.hips_sphere.set_model_mat(&next_pos.into());
            // Moves the viewport
            self.viewport.displacement::<P>(&mut self.hips_sphere, &mut self.catalog, &mut self.grid, enable_grid);
            hips_sphere_updated = true;

            if a == 1_f32 {
                self.animation_request = false;
            }

            // Render the next frame
            self.render = true;
        }

        // Mouse inertia
        if let Some(inertia) = self.inertia.clone() {
            self.inertia = inertia.update::<P>(&mut self.hips_sphere, &mut self.grid, &mut self.catalog, &mut self.viewport, enable_grid);
            hips_sphere_updated = true;

            // Render the next frame
            self.render = true;
        }

        // Check whether the HiPS sphere must be updated or not
        if (!hips_sphere_updated) && utils::get_current_time() < *LATEST_TIME_TILE_RECEIVED.lock().unwrap() + BLENDING_DURATION_MS {
            self.hips_sphere.mesh_mut().update::<P>(&self.viewport);

            // Render the next frame
            self.render = true;
            console::log_1(&format!("not render2").into());
        }

                // Move
        if let Some(_) = self.moving {
            // Moves the viewport
            self.viewport.displacement::<P>(&mut self.hips_sphere, &mut self.catalog, &mut self.grid, enable_grid);

            // Render the next frame
            self.render = true;
        }

        // Return the position of the center of the projection
        // to the javascript main code
        let center_world_pos = self.hips_sphere.compute_center_world_pos::<P>();
        let (ra, dec) = math::xyzw_to_radec(center_world_pos);
        Vector2::new(Rad(ra), Rad(dec))
    }

    fn render(&mut self, enable_grid: bool) {
       // if self.render {
            // Render the scene
            self.gl.clear_color(0.08, 0.08, 0.08, 1.0);
            self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
            // Clear the grid canvas containing the labels
            self.grid.mesh().clear_canvas(&self.viewport);

            // Draw renderables here
            let ref viewport = self.viewport;
            let ref shaders = self.shaders;

            // Draw the HiPS sphere
            self.hips_sphere.mesh().draw::<HiPSSphere, P>(
                &self.gl,
                &self.hips_sphere,
                shaders,
                viewport,
            );

            // Draw the catalogs
            self.catalog.mesh().draw(
                &self.gl,
                &self.catalog,
                shaders,
                viewport
            );

            // Draw the grid
            // The grid lines
            if P::name() != "Orthographic" && enable_grid {
                self.grid.mesh().draw(
                    &self.gl,
                    &self.grid,
                    shaders,
                    viewport
                );
                // The labels
                self.grid.mesh().draw_labels(&self.viewport);
            }
        /*} else {
            console::log_1(&format!("not render").into());
        }*/

        self.render = false;
    }

    fn set_position(&mut self, ra: Rad<f32>, dec: Rad<f32>) {
        let rot_y = Matrix4::from_angle_y(ra);
        let rot_z = Matrix4::from_angle_x(-dec);

        let model_mat = rot_y * rot_z;
        // Extract a 3x3 matrix from the model 4x4 matrix
        let v: [[f32; 4]; 4] = model_mat.into();

        let mat3 = Matrix3::new(
            v[0][0], v[0][1], v[0][2],
            v[1][0], v[1][1], v[1][2],
            v[2][0], v[2][1], v[2][2]
        );

        //self.hips_sphere.set_model_mat(&model_mat);

        self.animation_request = true;
        self.final_pos = mat3.into();
        self.start_pos = self.hips_sphere.get_quat();

        self.start_time = utils::get_current_time();
        self.start_aperture = self.viewport.field_of_view().get_aperture().into();

        let start_pos_world = self.hips_sphere.compute_center_world_pos::<P>();
        let start_pos_world = Vector3::new(start_pos_world.x, start_pos_world.y, start_pos_world.z);
        let end_pos_world = math::radec_to_xyz(ra, dec);

        self.d = math::angular_distance_xyz(start_pos_world, end_pos_world);
        let d: Deg<f32> = self.d.into();

        let end_aperture = self.start_aperture.0 + d.0;

        self.end_aperture = if end_aperture >= P::aperture_start().0 {
            P::aperture_start()
        } else {
            Deg(end_aperture)
        };

        let min_anim_duration = 2000_f32;
        let max_anim_duration = 6000_f32;
        self.animation_duration = (d.0 / 180_f32) * (max_anim_duration - min_anim_duration) + min_anim_duration;

/*
        // Get the position at the center of the view
        let center_clip = Vector2::new(0_f32, 0_f32);
        let p = self.hips_sphere.get_model_mat() * P::clip_to_world_space(center_clip).unwrap();

        let pos_center_world = Vector3::new(p.x, p.y, p.z);

        // Compute the angular distance between the two positions
        let angle = math::angular_distance_xyz(pos_center_world, pos_world);
        // Compute the axis of rotation
        let axis = pos_center_world.cross(pos_world);

        // Move the HiPS sphere
        let axis = axis.normalize();
        self.hips_sphere.apply_rotation(axis, angle);
*/
        // Moves the viewport
        //self.viewport.displacement::<P>(&mut self.hips_sphere, &mut self.catalog, &mut self.grid, self.enable_grid);
    }

    fn set_projection<Q: Projection>(mut self) -> App::<Q> {
        // Reset viewport first
        //self.viewport.reset_zoom_level::<Q>();
        self.viewport.reset::<Q>();

        // New HiPS sphere & update
        //let hips_sphere_mesh = HiPSSphere::<R>::new(&self.gl, &self.viewport);
        //let mut hips_sphere = Renderable::new(&self.gl, &self.shaders, hips_sphere_mesh);
        //self.hips_sphere.update_mesh(&self.shaders, hips_sphere_mesh);
        self.hips_sphere.mesh_mut().update::<Q>(&self.viewport);

        // Change projection of the catalog & update
        self.catalog.mesh_mut().set_projection::<Q>();
        self.catalog.mesh_mut().update::<Q>(&self.viewport);

        App::<Q> {
            gl: self.gl,

            shaders: self.shaders,

            viewport: self.viewport,
            projection: std::marker::PhantomData,

            // The sphere renderable
            hips_sphere: self.hips_sphere,
            // The grid renderable
            grid: self.grid,
            // The catalog renderable
            catalog: self.catalog,

            moving: self.moving,
            inertia: self.inertia,

            animation_request: self.animation_request,
            final_pos: self.final_pos,
            start_pos: self.start_pos,

            start_aperture: self.start_aperture,
            end_aperture: self.end_aperture,
            d: self.d,
            
            animation_duration: self.animation_duration,
            start_time: self.start_time,

            render: self.render,
        }
    }
    /*
    fn set_render_mode<Q: RenderingMode>(mut self, r: &mut Q) -> App::<P> {
        let hips_sphere_mesh = self.hips_sphere
            .mesh()
            .clone()
            .set_rendering_mode();

        let hips_sphere = self.hips_sphere.set_mesh(hips_sphere_mesh);
        hips_sphere.mesh_mut().update(&self.viewport, r);

        App::<P> {
            gl: self.gl,

            shaders: self.shaders,

            viewport: self.viewport,
            projection: std::marker::PhantomData,

            // The sphere renderable
            hips_sphere: hips_sphere,
            // The grid renderable
            grid: self.grid,
            // The catalog renderable
            catalog: self.catalog,

            moving: self.moving,
        }
    }*/

    fn reload_hips_sphere(&mut self, hips_url: String, hips_depth: u8) {
        *HIPS_NAME.lock().unwrap() = hips_url;
        MAX_DEPTH.store(hips_depth, Ordering::Relaxed);

        // Re-initialize the color buffers
        self.hips_sphere.mesh_mut().refresh_buffer_tiles();

        self.hips_sphere.mesh_mut().update::<P>(&self.viewport);
        // Render the next frame
        self.render = true;
    }

    /// MOVE EVENT
    fn initialize_move(&mut self, screen_pos: Vector2<f32>) {
        // stop inertia if there is one
        self.inertia = None;
        //self.viewport.stop_inertia();

        if let Some(start_world_pos) = P::screen_to_world_space(screen_pos, &self.viewport) {
            self.moving = Some(Move::new::<P>(start_world_pos, &self.hips_sphere));
        }
    }

    fn moves(&mut self, screen_pos: Vector2<f32>) {
        if let Some(world_pos) = P::screen_to_world_space(screen_pos, &self.viewport) {
            // If a move is done
            if let Some(ref mut moving) = &mut self.moving {
                // Moves the renderables
                moving.apply_to_renderables::<P>(
                    world_pos,

                    &mut self.hips_sphere,
                    &mut self.grid,
                    &mut self.catalog,
                );
            }
        }
    }

    fn stop_move(&mut self, pos_screen_space: Vector2<f32>, dt: f32, enable_inertia: bool) {
        if enable_inertia {
            if let Some(_) = P::screen_to_world_space(pos_screen_space, &self.viewport) {
                if let Some(moving) = self.moving.clone() {
                    self.inertia = MouseInertia::new(moving, dt);
                }
            }
        }

        self.moving = None;
        /*if let Some(ref mut move_event) = self.move_event {
            console::log_1(&format!("stop moving").into());
            //self.viewport.stop_displacement();

            if ENABLE_INERTIA.load(Ordering::Relaxed) {
                // Do not perform the inertia effect if the released
                // position of the mouse is outside the projection
                let ref projection = self.projection;
                let mouse_pos_world = screen_to_world_space(&screen_pos, projection, &self.viewport);
                if mouse_pos_world.is_some() {
                    self.inertia = MouseInertia::new(
                        move_event,
                        &mut self.viewport
                    );
                }
            }   
        }*/
    }


    // Returns true if hips_sphere rendering mode has to be changed to the PerPixel mode
    fn unzoom(&mut self, delta_y: f32, enable_grid: bool) -> bool {
        let a0: Deg<f32> = self.viewport
            .field_of_view()
            .get_aperture()
            .into();

        self.viewport.unzoom::<P>(&mut self.hips_sphere, &mut self.catalog, &mut self.grid, enable_grid);

        let a1: Deg<f32> = self.viewport
            .field_of_view()
            .get_aperture()
            .into();

        // Render the next frame
        self.render = true;

        a0 < Deg(100_f32) && a1 >= Deg(100_f32)
    }

    // ZOOM EVENT
    // Returns true if hips_sphere rendering mode has to be changed to the SmallFOV mode
    fn zoom(&mut self, delta_y: f32, enable_grid: bool) -> bool {
        let a0: Deg<f32> = self.viewport
            .field_of_view()
            .get_aperture()
            .into();

        self.viewport.zoom::<P>(&mut self.hips_sphere, &mut self.catalog, &mut self.grid, enable_grid);
    
        let a1: Deg<f32> = self.viewport
            .field_of_view()
            .get_aperture()
            .into();

        // Render the next frame
        self.render = true;

        a0 >= Deg(100_f32) && a1 < Deg(100_f32)
    }

    fn add_catalog(&mut self, sources: Vec<Source>) {
        let mut catalog_mesh = Catalog::new(&self.gl, sources);
        catalog_mesh.set_projection::<P>();

        self.catalog.update_mesh(&self.shaders, catalog_mesh);
    }

    fn set_colormap(&mut self, colormap: String) {
        match colormap.as_str() {
            "BluePastelRed" => self.catalog.mesh_mut().set_colormap::<BluePastelRed>(),
            "IDL_CB_BrBG" => self.catalog.mesh_mut().set_colormap::<IDL_CB_BrBG>(),
            "IDL_CB_YIGnBu" => self.catalog.mesh_mut().set_colormap::<IDL_CB_YIGnBu>(),
            "IDL_CB_GnBu" => self.catalog.mesh_mut().set_colormap::<IDL_CB_GnBu>(),
            "Red_Temperature" => self.catalog.mesh_mut().set_colormap::<Red_Temperature>(),
            "Black_White_Linear" => self.catalog.mesh_mut().set_colormap::<Black_White_Linear>(),
            _ => panic!("{:?} colormap not recognized!", colormap)
        }
    }

    fn set_heatmap_opacity(&mut self, opacity: f32) {
        self.catalog.mesh_mut().set_alpha(opacity);
    }

    fn set_kernel_strength(&mut self, strength: f32) {
        self.catalog.mesh_mut().set_kernel_strength::<P>(strength, &self.viewport);
    }

    fn set_range_source_size(&mut self, source_size: std::ops::Range<f32>) {
        self.catalog.mesh_mut().set_min_size_source(source_size.start);
        self.catalog.mesh_mut().set_max_size_source(source_size.end);
    }

    fn resize_window(&mut self, width: f32, height: f32, enable_grid: bool) {
        self.viewport.resize_window::<P>(width, height, &mut self.hips_sphere, &mut self.grid, &mut self.catalog, enable_grid);
        // Render the next frame
        self.render = true;
    }
    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        self.grid.mesh_mut().set_color_rgb(red, green, blue);
    }

    pub fn change_grid_opacity(&mut self, alpha: f32) {
        self.grid.mesh_mut().set_alpha(alpha);
    }

    pub fn enable_grid(&mut self) {
        self.grid.mesh_mut().update::<P>(&self.hips_sphere, &self.viewport);
    }

    /*fn update_hips_sphere(&mut self, r: &mut R) {
        self.hips_sphere.mesh_mut().update(&self.viewport, r);
    }
    fn update_catalog(&mut self) {
        self.catalog.mesh_mut().update::<P>(&self.viewport);
    }*/
}

lazy_static! {
    static ref ENABLED_WIDGETS: Arc<Mutex<HashMap<&'static str, bool>>> = {
        let mut m = HashMap::new();
        m.insert("hips_sphere", true);
        m.insert("grid", true);
        Arc::new(Mutex::new(m))
    };

    static ref ENABLE_INERTIA: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));

    static ref HIPS_NAME: Arc<Mutex<String>> = Arc::new(Mutex::new(String::from("http://alasky.u-strasbg.fr/DSS/DSSColor")));
    static ref MAX_DEPTH: Arc<AtomicU8> = Arc::new(AtomicU8::new(9));
}

#[derive(Clone)]
pub struct WebGl2Context {
    inner: Rc<WebGl2RenderingContext>,
}

impl WebGl2Context {
    fn new() -> WebGl2Context {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let context_options = js_sys::JSON::parse(&"{\"antialias\":false}").unwrap();
        let inner = Rc::new(
            canvas.get_context_with_context_options("webgl2", context_options.as_ref())
                .unwrap()
                .unwrap()
                .dyn_into::<WebGl2RenderingContext>()
                .unwrap()
        );

        WebGl2Context {
            inner
        }
    }
}

use std::ops::Deref;
impl Deref for WebGl2Context {
    type Target = WebGl2RenderingContext;

    fn deref(&self) -> &WebGl2RenderingContext {
        &self.inner
    }
}

use crate::renderable::hips_sphere::SmallFieldOfView;
enum AppConfig {
    Aitoff(App<Aitoff>, &'static str),
    MollWeide(App<MollWeide>, &'static str),
    Ortho(App<Orthographic>, &'static str),
}

use cgmath::Rad;
impl AppConfig {
    fn set_projection(self, proj: &str) -> AppConfig {
        match (self, proj) {
            (AppConfig::Aitoff(app, _), "aitoff") => {
                AppConfig::Aitoff(app, "aitoff")
            },
            (AppConfig::MollWeide(app, _), "aitoff") => {
                AppConfig::Aitoff(app.set_projection::<Aitoff>(), "aitoff")
            },
            (AppConfig::Ortho(app, _), "aitoff") => {
                AppConfig::Aitoff(app.set_projection::<Aitoff>(), "aitoff")
            },


            (AppConfig::Aitoff(app, _), "orthographic") => {
                AppConfig::Ortho(app.set_projection::<Orthographic>(), "orthographic")
            },
            (AppConfig::MollWeide(app, _), "orthographic") => {
                AppConfig::Ortho(app.set_projection::<Orthographic>(), "orthographic")
            },
            (AppConfig::Ortho(app, _), "orthographic") => {
                AppConfig::Ortho(app, "orthographic")
            },

            (AppConfig::Aitoff(app, _), "mollweide") => {
                AppConfig::MollWeide(app.set_projection::<MollWeide>(), "mollweide")
            },
            (AppConfig::MollWeide(app, _), "mollweide") => {
                AppConfig::MollWeide(app, "mollweide")
            },
            (AppConfig::Ortho(app, _), "mollweide") => {
                AppConfig::MollWeide(app.set_projection::<MollWeide>(), "mollweide")
            },
            _ => unreachable!()
        }
    }

    fn set_colormap(&mut self, colormap: String) {
        match self {
            AppConfig::Aitoff(app, _) => app.set_colormap(colormap),
            AppConfig::MollWeide(app, _) => app.set_colormap(colormap),
            AppConfig::Ortho(app, _) => app.set_colormap(colormap),
        };
    }

    fn update(&mut self, dt: f32, enable_grid: bool) -> Vector2<Rad<f32>> {
        let pos_center_world = match self {
            AppConfig::Aitoff(app, _) => app.update(dt, enable_grid),
            AppConfig::MollWeide(app, _) => app.update(dt, enable_grid),
            AppConfig::Ortho(app, _) => app.update(dt, enable_grid),
        };

        pos_center_world
    }

    fn render(&mut self, enable_grid: bool) {
        match self {
            AppConfig::Aitoff(app, _) => app.render(enable_grid),
            AppConfig::MollWeide(app, _) => app.render(enable_grid),
            AppConfig::Ortho(app, _) => app.render(enable_grid),
        }
    }

    pub fn initialize_move(&mut self, screen_pos_x: f32, screen_pos_y: f32) {
        let screen_pos = Vector2::new(screen_pos_x, screen_pos_y);
        match self {
            AppConfig::Aitoff(app, _) => app.initialize_move(screen_pos),
            AppConfig::MollWeide(app, _) => app.initialize_move(screen_pos),
            AppConfig::Ortho(app, _) => app.initialize_move(screen_pos),
        }
    }

    /// Stop move
    pub fn stop_move(&mut self, screen_pos_x: f32, screen_pos_y: f32, dt: f32, enable_inertia: bool) {
        let screen_pos = Vector2::new(screen_pos_x, screen_pos_y);
        match self {
            AppConfig::Aitoff(app, _) => app.stop_move(screen_pos, dt, enable_inertia),
            AppConfig::MollWeide(app, _) => app.stop_move(screen_pos, dt, enable_inertia),
            AppConfig::Ortho(app, _) => app.stop_move(screen_pos, dt, enable_inertia),
        }
    }
    /// Keep moving
    pub fn moves(&mut self, screen_pos_x: f32, screen_pos_y: f32) {
        let screen_pos = Vector2::new(screen_pos_x, screen_pos_y);
        match self {
            AppConfig::Aitoff(app, _) => app.moves(screen_pos),
            AppConfig::MollWeide(app, _) => app.moves(screen_pos),
            AppConfig::Ortho(app, _) => app.moves(screen_pos),
        };
    }

    /// Wheel event
    pub fn zoom(self, delta_y: f32, enable_grid: bool) -> AppConfig {
        match self {
            AppConfig::Aitoff(mut app, s) => {
                if app.zoom(delta_y, enable_grid) {
                    AppConfig::Ortho(app.set_projection::<Orthographic>(), s)
                } else {
                    AppConfig::Aitoff(app, s)
                }
            },
            AppConfig::MollWeide(mut app, s) => {
                if app.zoom(delta_y, enable_grid) {
                    AppConfig::Ortho(app.set_projection::<Orthographic>(), s)
                } else {
                    AppConfig::MollWeide(app, s)
                }
            },
            AppConfig::Ortho(mut app, s) => {
                app.zoom(delta_y, enable_grid);
                AppConfig::Ortho(app, s)
            },
        }
    }
    /// Wheel event
    pub fn unzoom(self, delta_y: f32, enable_grid: bool) -> AppConfig {
        match self {
            AppConfig::Aitoff(mut app, s) => {
                app.unzoom(delta_y, enable_grid);
                AppConfig::Aitoff(app, s)
            },
            AppConfig::MollWeide(mut app, s) => {
                app.unzoom(delta_y, enable_grid);
                AppConfig::MollWeide(app, s)
            },
            AppConfig::Ortho(mut app, s) => {
                if app.unzoom(delta_y, enable_grid) {
                    match s {
                        "aitoff" => AppConfig::Aitoff(app.set_projection::<Aitoff>(), s),
                        "mollweide" => AppConfig::MollWeide(app.set_projection::<MollWeide>(), s),
                        "orthographic" => AppConfig::Ortho(app, s),
                        _ => unreachable!()
                    }
                } else {
                    AppConfig::Ortho(app, s)
                }
            }
        }
    }

    pub fn add_catalog(&mut self, data: JsValue) {
        let array: js_sys::Array = data
            .dyn_into().unwrap();
        let mut sources: Vec<Source> = vec![];

        // Deserialize row by row. Rows that cannot be
        // deserialized are removed!
        for source in array.iter() {
            let source = source.into_serde::<[f32; 4]>().ok();

            if let Some(ref source) = source {
                sources.push((source as &[f32]).into());
            } else {
                console::log_1(&format!("invalid row").into());
            }
        }

        match self {
            AppConfig::Aitoff(app, _) => app.add_catalog(sources),
            AppConfig::MollWeide(app, _) => app.add_catalog(sources),
            AppConfig::Ortho(app, _) => app.add_catalog(sources),
        }
    }

    pub fn reload_hips_sphere(&mut self, hips_url: String, max_depth: u8) {        
        match self {
            AppConfig::Aitoff(app, _) => app.reload_hips_sphere(hips_url, max_depth),
            AppConfig::MollWeide(app, _) => app.reload_hips_sphere(hips_url, max_depth),
            AppConfig::Ortho(app, _) => app.reload_hips_sphere(hips_url, max_depth),
        }
    }

    pub fn resize(&mut self, width: f32, height: f32, enable_grid: bool) {        
        match self {
            AppConfig::Aitoff(app, _) => app.resize_window(width, height, enable_grid),
            AppConfig::MollWeide(app, _) => app.resize_window(width, height, enable_grid),
            AppConfig::Ortho(app, _) => app.resize_window(width, height, enable_grid),
        }
    }

    pub fn set_kernel_strength(&mut self, strength: f32) {        
        match self {
            AppConfig::Aitoff(app, _) => app.set_kernel_strength(strength),
            AppConfig::MollWeide(app, _) => app.set_kernel_strength(strength),
            AppConfig::Ortho(app, _) => app.set_kernel_strength(strength),
        }
    }

    pub fn set_heatmap_opacity(&mut self, opacity: f32) {        
        match self {
            AppConfig::Aitoff(app, _) => app.set_heatmap_opacity(opacity),
            AppConfig::MollWeide(app, _) => app.set_heatmap_opacity(opacity),
            AppConfig::Ortho(app, _) => app.set_heatmap_opacity(opacity),
        }
    }

    pub fn set_range_source_size(&mut self, source_size_range: std::ops::Range<f32>) {
        match self {
            AppConfig::Aitoff(app, _) => app.set_range_source_size(source_size_range),
            AppConfig::MollWeide(app, _) => app.set_range_source_size(source_size_range),
            AppConfig::Ortho(app, _) => app.set_range_source_size(source_size_range),
        }
    }

    pub fn set_position(&mut self, ra: Rad<f32>, dec: Rad<f32>) {
        match self {
            AppConfig::Aitoff(app, _) => app.set_position(ra, dec),
            AppConfig::MollWeide(app, _) => app.set_position(ra, dec),
            AppConfig::Ortho(app, _) => app.set_position(ra, dec),
        }
    }

    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        match self {
            AppConfig::Aitoff(app, _) => app.set_color_rgb(red, blue, green),
            AppConfig::MollWeide(app, _) => app.set_color_rgb(red, blue, green),
            AppConfig::Ortho(app, _) => app.set_color_rgb(red, blue, green),
        }
    }

    pub fn change_grid_opacity(&mut self, alpha: f32) {
        match self {
            AppConfig::Aitoff(app, _) => app.change_grid_opacity(alpha),
            AppConfig::MollWeide(app, _) => app.change_grid_opacity(alpha),
            AppConfig::Ortho(app, _) => app.change_grid_opacity(alpha),
        }
    }

    pub fn enable_grid(&mut self) {
        match self {
            AppConfig::Aitoff(app, _) => app.enable_grid(),
            AppConfig::MollWeide(app, _) => app.enable_grid(),
            AppConfig::Ortho(app, _) => app.enable_grid(),
        }
    }
}

#[wasm_bindgen]
pub struct WebClient {
    appconfig: AppConfig,
    dt: f32,
    enable_inertia: bool,
    enable_grid: bool,
}

#[wasm_bindgen]
impl WebClient {
    /// Create a new web client
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        let gl = WebGl2Context::new();

        let app = App::<Aitoff>::new(&gl).unwrap();
        let appconfig = AppConfig::Aitoff(app, "aitoff");
        let dt = 0_f32;
        let enable_inertia = false;
        let enable_grid = true;

        WebClient {
            appconfig,
            dt,
            enable_inertia,
            enable_grid,
        }
    }

    /// Update our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn update(&mut self, dt: f32) -> Result<Box<[f32]>, JsValue> {
        self.dt = dt;
        let pos_center_world = self.appconfig.update(dt, self.enable_grid);

        let ra_deg: Deg<f32> = pos_center_world.x.into();
        let dec_deg: Deg<f32> = pos_center_world.y.into();

        Ok(Box::new([ra_deg.0, dec_deg.0]))
    }

    /// Update our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn render(&mut self) -> Result<(), JsValue> {
        self.appconfig.render(self.enable_grid);

        Ok(())
    }

    /// Change the current projection of the HiPS
    pub fn set_projection(mut self, name: String) -> Result<WebClient, JsValue> {
        self.appconfig = self.appconfig.set_projection(&name);

        Ok(self)
    }

    /// Change the current projection of the HiPS
    pub fn set_colormap(&mut self, name: String) -> Result<(), JsValue> {
        self.appconfig.set_colormap(name);

        Ok(())
    }

    /// Enable mouse inertia
    pub fn enable_inertia(&mut self) -> Result<(), JsValue> {
        self.enable_inertia = true;

        Ok(())
    }
    /// Disable mouse inertia
    pub fn disable_inertia(&mut self) -> Result<(), JsValue> {
        self.enable_inertia = false;

        Ok(())
    }

    /// Enable equatorial grid
    pub fn enable_equatorial_grid(&mut self) -> Result<(), JsValue> {
        self.enable_grid = true;

        self.appconfig.enable_grid();
        /*if let Some(grid) = ENABLED_WIDGETS.lock().unwrap().get_mut("grid") {
            *grid = true;
            self.app.grid
                .update(
                    &self.app.projection,
                    &mut self.app.viewport
                );
        }*/

        Ok(())
    }

    /// Disable equatorial grid
    pub fn disable_equatorial_grid(&mut self) -> Result<(), JsValue> {
        self.enable_grid = false;

        /*if let Some(grid) = ENABLED_WIDGETS.lock().unwrap().get_mut("grid") {
            *grid = false;
            self.app.grid.mesh_mut().clear_canvas();
        }*/

        Ok(())
    }
    
    /// Change grid color
    pub fn change_grid_color(&mut self, red: f32, green: f32, blue: f32) -> Result<(), JsValue> {
        self.appconfig.set_color_rgb(red, green, blue);

        Ok(())
    }

    /// Change grid opacity
    pub fn change_grid_opacity(&mut self, alpha: f32) -> Result<(), JsValue> {
        self.appconfig.change_grid_opacity(alpha);

        Ok(())
    }
    
    /// Change HiPS
    pub fn change_hips(&mut self, hips_url: String, hips_depth: i32) -> Result<(), JsValue> {
        self.appconfig.reload_hips_sphere(hips_url, hips_depth as u8);

        Ok(())
    }

    /// Start move
    pub fn initialize_move(&mut self, screen_pos_x: f32, screen_pos_y: f32) -> Result<(), JsValue> {
        self.appconfig.initialize_move(screen_pos_x, screen_pos_y);

        Ok(())
    }
    /// Stop move
    pub fn stop_move(&mut self, screen_pos_x: f32, screen_pos_y: f32) -> Result<(), JsValue> {
        self.appconfig.stop_move(screen_pos_x, screen_pos_y, self.dt, self.enable_inertia);

        Ok(())
    }
    /// Keep moving
    pub fn moves(&mut self, screen_pos_x: f32, screen_pos_y: f32) -> Result<(), JsValue> {
        self.appconfig.moves(screen_pos_x, screen_pos_y);

        Ok(())
    }

    /// Wheel event
    pub fn zoom(mut self, delta_y: f32) -> Result<WebClient, JsValue> {
        self.appconfig = if delta_y < 0_f32 {
            self.appconfig.zoom(delta_y, self.enable_grid)
        } else {
            self.appconfig.unzoom(delta_y, self.enable_grid)
        };

        Ok(self)
    }

    /// Add new catalog
    pub fn add_catalog(&mut self, data: JsValue) -> Result<(), JsValue> {
        self.appconfig.add_catalog(data);

        Ok(())
    }

    /// Resize the window
    pub fn resize(&mut self, width: f32, height: f32) -> Result<(), JsValue> {
        self.appconfig.resize(width, height, self.enable_grid);

        Ok(())
    }

    /// Set the kernel strength
    pub fn set_kernel_strength(&mut self, strength: f32) -> Result<(), JsValue> {
        self.appconfig.set_kernel_strength(strength);

        Ok(())
    }

    /// Set the heatmap global opacity
    pub fn set_heatmap_opacity(&mut self, opacity: f32) -> Result<(), JsValue> {
        self.appconfig.set_heatmap_opacity(opacity);

        Ok(())
    }

    pub fn set_range_source_size(&mut self, min_size_source: f32, max_size_source: f32) -> Result<(), JsValue> {
        self.appconfig.set_range_source_size(min_size_source..max_size_source);

        Ok(())
    }

    /// Set the HiPS Sphere at a position
    /// 
    /// ra is expressed in degrees
    /// dec is expressed in degrees
    pub fn set_position(&mut self, ra: f32, dec: f32) -> Result<(), JsValue> {
        self.appconfig.set_position(Deg(ra).into(), Deg(dec).into());

        Ok(())
    }
}

