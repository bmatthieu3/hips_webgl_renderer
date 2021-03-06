#![allow(dead_code)]
#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
extern crate itertools_num;
extern crate rand;
extern crate serde_derive;
extern crate num;
extern crate delaunator;

use web_sys::console;
#[macro_export]
macro_rules! print_to_console {
    ( $text:expr, $( $x:expr ),* ) => {
        {
            console::log_1(&format!($text, $($x)*).into());
        }
    };
}

#[macro_use]
mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

use cgmath;

mod shader;
mod shaders;
pub mod renderable;
mod finite_state_machine;
mod viewport;
mod core;
mod math;

mod projeted_grid;
mod field_of_view;
//mod mouse_inertia;
mod event_manager;
mod color;
mod healpix_cell;
mod buffer;
mod rotation;

use shader::Shader;
use shader::ShaderManager;

use renderable::HiPSSphere;
use renderable::grid::ProjetedGrid;
use renderable::catalog::{Catalog, Source};

use renderable::projection;
use renderable::projection::{Aitoff, Orthographic, Mollweide, AzimutalEquidistant, Mercator};

use viewport::ViewPort;

use std::sync::Mutex;
use std::sync::Arc;

use std::rc::Rc;

use std::collections::HashMap;

use std::sync::atomic::AtomicBool;

use crate::projection::Projection;

#[macro_use]
extern crate aladinlite_derive;

use cgmath::Quaternion;

use crate::finite_state_machine:: {UserMoveSphere, UserZoom, FiniteStateMachine, MoveSphere};

struct App<P>
where P: Projection {
    gl: WebGl2Context,

    shaders: ShaderManager,

    viewport: ViewPort,
    projection: std::marker::PhantomData<P>,

    // The sphere renderable
    sphere: HiPSSphere,
    // The grid renderable
    grid: ProjetedGrid,
    // The catalogs
    catalog: Catalog,

    // Finite State Machine declarations
    user_move_fsm: UserMoveSphere,
    user_zoom_fsm: UserZoom,
    move_fsm: MoveSphere,

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

//use crate::renderable::hips_sphere::RayTracer;

use cgmath::{Vector2, Vector3, Matrix3};
use cgmath::Deg;

use crate::buffer::{HiPSConfig, ImageFormat};


use crate::shaders::Colormap;
impl<P> App<P>
where P: Projection {
    fn new(gl: &WebGl2Context, events: &EventManager) -> Result<App<Orthographic>, JsValue> {
        console::log_1(&format!("Init").into());

        let mut shaders = ShaderManager::new();

        // Shader definition
        // HiPS sphere shader
        // uniforms definition
        let mut uniforms_raytracer = vec![
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
            // Textures
            "tex",
            "num_textures",
        ];
        uniforms_raytracer.extend(shaders::HPX_TILES_BUFFER_UNIFORMS);
        shaders.insert::<shaders::Raytracing>(gl, &uniforms_raytracer[..]);

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
            "grid_color",
        ];
        shaders.insert::<shaders::Grid>(gl, &uniforms_grid[..]);

        // Colormap shaders definition
        let uniforms_colormap = &[
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
        shaders.insert::<shaders::BluePastelRed>(gl, &uniforms_colormap[..]);
        shaders.insert::<shaders::IDL_CB_BrBG>(gl, &uniforms_colormap[..]);
        shaders.insert::<shaders::IDL_CB_YIGnBu>(gl, &uniforms_colormap[..]);
        shaders.insert::<shaders::IDL_CB_GnBu>(gl, &uniforms_colormap[..]);
        shaders.insert::<shaders::Red_Temperature>(gl, &uniforms_colormap[..]);
        shaders.insert::<shaders::Black_White_Linear>(gl, &uniforms_colormap[..]);

        // Catalog shaders definition
        let uniforms_catalog = &[
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
        shaders.insert::<shaders::Catalog_Orthographic>(gl, &uniforms_catalog[..]);
        shaders.insert::<shaders::Catalog_Aitoff>(gl, &uniforms_catalog[..]);
        shaders.insert::<shaders::Catalog_MollWeide>(gl, &uniforms_catalog[..]);
        shaders.insert::<shaders::Catalog_Mercator>(gl, &uniforms_catalog[..]);

        gl.enable(WebGl2RenderingContext::BLEND);
        gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        //gl.enable(WebGl2RenderingContext::SCISSOR_TEST);
        gl.enable(WebGl2RenderingContext::CULL_FACE);
        gl.cull_face(WebGl2RenderingContext::BACK);

        // Viewport definition
        // HiPS definition
        let config = HiPSConfig::new(
            String::from("http://alasky.u-strasbg.fr/DSS/DSSColor"), // Name of the HiPS
            9, // max depth of the HiPS
            512, // Size of a texture tile
            ImageFormat::JPG // Format of the tile texture images
        );
        let viewport = ViewPort::new::<Orthographic>(&gl, &config);

        // HiPS Sphere definition
        let mut sphere = HiPSSphere::new(&gl, &viewport, config, &mut shaders);
        sphere.update::<Orthographic>(&viewport, events, &shaders);

        // Catalog definition
        let catalog = Catalog::new(&gl, vec![], &shaders);

        // Grid definition
        let lon_bound = cgmath::Vector2::<cgmath::Rad<f32>>::new(cgmath::Deg(-30_f32).into(), cgmath::Deg(30_f32).into());
        let lat_bound = cgmath::Vector2::<cgmath::Rad<f32>>::new(cgmath::Deg(-90_f32).into(), cgmath::Deg(90_f32).into());
        let grid = ProjetedGrid::new::<P>(&gl, cgmath::Deg(30_f32).into(), cgmath::Deg(30_f32).into(), Some(lat_bound), None, &viewport, &shaders);

        let animation_request = false;
        let final_pos = Quaternion::new(1_f32, 0_f32, 0_f32, 0_f32);
        let start_pos = Quaternion::new(1_f32, 0_f32, 0_f32, 0_f32);
        let start_time = 0_f32;
        let animation_duration = 2000_f32;

        let start_aperture = P::aperture_start();
        let end_aperture = P::aperture_start();

        let d = Rad(0_f32);

        // Finite State Machines definitions
        let user_move_fsm = UserMoveSphere::init();
        let user_zoom_fsm = UserZoom::init();
        let move_fsm = MoveSphere::init();

        let gl = gl.clone();
        let render = true;
        let app = App {
            gl,

            shaders,

            viewport,
            projection: std::marker::PhantomData,

            // The sphere renderable
            sphere,
            // The grid renderable
            grid,
            // The catalog renderable
            catalog,

            // Finite state machines,
            user_move_fsm,
            user_zoom_fsm,
            move_fsm,

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

    fn update(&mut self,
        dt: f32,
        events: &EventManager,
        enable_grid: bool
    ) -> Vector2<Rad<f32>> {
        // Animation request
        let mut hips_sphere_updated = false;
        // Put that in a FSM
        /*if self.animation_request {
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
            self.viewport.displacement::<P>(&mut self.hips_sphere, &mut self.catalog, &mut self.grid);
            hips_sphere_updated = true;

            if a == 1_f32 {
                self.animation_request = false;
            }

            // Render the next frame
            self.render = true;
        }*/

        // Run the Finite State Machines
        self.user_move_fsm.run::<P>(dt, &mut self.sphere, &mut self.catalog, &mut self.grid, &mut self.viewport, &events);
        self.user_zoom_fsm.run::<P>(dt, &mut self.sphere, &mut self.catalog, &mut self.grid, &mut self.viewport, &events);
        self.move_fsm.run::<P>(dt, &mut self.sphere, &mut self.catalog, &mut self.grid, &mut self.viewport, &events);

        // Update the HiPS sphere VAO
        self.sphere.update::<P>(&self.viewport, &events, &self.shaders);

        /*// Mouse inertia
        if let Some(inertia) = self.inertia.clone() {
            self.inertia = inertia.update::<P>(&mut self.hips_sphere, &mut self.grid, &mut self.catalog, &mut self.viewport, enable_grid);
            hips_sphere_updated = true;

            // Render the next frame
            self.render = true;
        }*/

        // Check whether the HiPS sphere must be updated or not
        /*if !hips_sphere_updated /*&& utils::get_current_time() < *LATEST_TIME_TILE_RECEIVED.lock().unwrap() + BLENDING_DURATION_MS*/ {
            self.hips_sphere.mesh_mut().update::<P>(&self.viewport);

            // Render the next frame
            self.render = true;
        }*/

        /*// Move
        if let Some(_) = self.moving {
            // Moves the viewport
            self.viewport.displacement::<P>(&mut self.hips_sphere, &mut self.catalog, &mut self.grid, enable_grid);

            // Render the next frame
            self.render = true;
        }*/
        //self.hips_sphere.mesh_mut().update::<P>(&self.viewport);
        // Return the position of the center of the projection
        // to the javascript main code
        let center_world_pos = self.viewport.compute_center_world_pos::<P>();
        let (lon, lat) = math::xyzw_to_radec(&center_world_pos);

        Vector2::new(lon, lat)
    }

    fn render(&mut self, enable_grid: bool) {
       // if self.render {
            // Render the scene
            self.gl.clear_color(0.08, 0.08, 0.08, 1.0);
            self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
            // Clear the grid canvas containing the labels
            self.grid.clear_canvas(&self.viewport);

            // Draw renderables here
            let ref viewport = self.viewport;
            let ref shaders = self.shaders;

            // Draw the HiPS sphere
            self.sphere.draw::<P>(
                &self.gl,
                shaders,
                viewport,
            );

            if self.catalog.get_alpha() > 0_f32 {
                // Draw the catalogs
                self.catalog.draw::<P>(
                    &self.gl,
                    shaders,
                    viewport
                );
            }

            // Draw the grid
            // The grid lines
            /*if P::name() != "Orthographic" && enable_grid {
                self.grid.draw(
                    &self.gl,
                    shaders,
                    viewport
                );
                // The labels
                self.grid.draw_labels(&self.viewport);
            }*/
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
        self.start_pos = self.viewport.get_quat();

        self.start_time = utils::get_current_time();
        self.start_aperture = self.viewport.field_of_view().get_aperture().into();

        let start_pos_world = self.viewport.compute_center_world_pos::<P>();
        let start_pos_world = Vector3::new(start_pos_world.x, start_pos_world.y, start_pos_world.z);
        let end_pos_world = math::radec_to_xyz(ra, dec);

        self.d = math::ang_between_vect(&start_pos_world, &end_pos_world);
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
    }

    fn set_projection<Q: Projection>(mut self, events: &EventManager) -> App::<Q> {
        // Reset viewport first
        //self.viewport.reset_zoom_level::<Q>();
        self.viewport.reset::<Q>();

        // New HiPS sphere & update
        //let hips_sphere_mesh = HiPSSphere::<R>::new(&self.gl, &self.viewport);
        //let mut hips_sphere = Renderable::new(&self.gl, &self.shaders, hips_sphere_mesh);
        //self.hips_sphere.update_mesh(&self.shaders, hips_sphere_mesh);
        self.sphere.update::<Q>(&self.viewport, events, &self.shaders);

        //self.catalog.set_projection::<Q>();
        self.catalog.retrieve_sources_in_fov::<Q>(&self.viewport);
        // Reproject the grid
        self.grid.reproject::<Q>(&self.viewport);

        App::<Q> {
            gl: self.gl,
            shaders: self.shaders,

            viewport: self.viewport,
            projection: std::marker::PhantomData,

            // The sphere renderable
            sphere: self.sphere,
            // The grid renderable
            grid: self.grid,
            // The catalog renderable
            catalog: self.catalog,

            // Finite State Machines
            user_move_fsm: self.user_move_fsm,
            user_zoom_fsm: self.user_zoom_fsm,
            move_fsm: self.move_fsm,

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

    fn set_hips_config(&mut self, config: HiPSConfig) {
        self.sphere.set_hips_config::<P>(config, &mut self.viewport);
        // Render the next frame
        self.render = true;
    }

    // Returns true if hips_sphere rendering mode has to be changed to the PerPixel mode
    /*fn unzoom(&mut self, delta_y: f32, enable_grid: bool) -> bool {
        let a0: Deg<f32> = self.viewport
            .field_of_view()
            .get_aperture()
            .into();

        self.viewport.unzoom::<P>(delta_y, &mut self.hips_sphere, &mut self.catalog, &mut self.grid);

        let a1: Deg<f32> = self.viewport
            .field_of_view()
            .get_aperture()
            .into();

        // Render the next frame
        self.render = true;

        a0 < Deg(80_f32) && a1 >= Deg(80_f32)
    }*/

    // ZOOM EVENT
    // Returns true if hips_sphere rendering mode has to be changed to the SmallFOV mode
    /*fn zoom(&mut self, delta_y: f32, enable_grid: bool) -> bool {
        console::log_1(&format!("IAAA").into());

        let a0: Deg<f32> = self.viewport
            .field_of_view()
            .get_aperture()
            .into();

        self.viewport.zoom::<P>(delta_y, &mut self.hips_sphere, &mut self.catalog, &mut self.grid);
    
        let a1: Deg<f32> = self.viewport
            .field_of_view()
            .get_aperture()
            .into();

        // Render the next frame
        self.render = true;

        a0 >= Deg(80_f32) && a1 < Deg(80_f32)
    }*/

    fn add_catalog(&mut self, sources: Vec<Source>) {
        self.catalog = Catalog::new(&self.gl, sources, &self.shaders);
        //self.catalog.set_projection::<P>();
    }

    fn set_colormap(&mut self, colormap: Colormap) {
        self.catalog.set_colormap(colormap);
    }

    fn set_heatmap_opacity(&mut self, opacity: f32) {
        self.catalog.set_alpha(opacity);
    }

    fn set_kernel_strength(&mut self, strength: f32) {
        self.catalog.set_kernel_strength::<P>(strength, &self.viewport);
    }

    fn set_range_source_size(&mut self, source_size: std::ops::Range<f32>) {
        self.catalog.set_min_size_source(source_size.start);
        self.catalog.set_max_size_source(source_size.end);
    }

    fn resize_window(&mut self, width: f32, height: f32, enable_grid: bool) {
        self.viewport.resize_window::<P>(width, height, &mut self.sphere, &mut self.grid, &mut self.catalog);
        // Render the next frame
        self.render = true;
    }
    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        self.grid.set_color_rgb(red, green, blue);
    }

    pub fn change_grid_opacity(&mut self, alpha: f32) {
        self.grid.set_alpha(alpha);
    }

    pub fn enable_grid(&mut self) {
        self.grid.reproject::<P>(&self.viewport);
    }
}

lazy_static! {
    static ref ENABLED_WIDGETS: Arc<Mutex<HashMap<&'static str, bool>>> = {
        let mut m = HashMap::new();
        m.insert("hips_sphere", true);
        m.insert("grid", true);
        Arc::new(Mutex::new(m))
    };

    static ref ENABLE_INERTIA: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
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

enum AppConfig {
    Aitoff(App<Aitoff>),
    MollWeide(App<Mollweide>),
    Arc(App<AzimutalEquidistant>),
    Mercator(App<Mercator>),

    Ortho(App<Orthographic>),
}

use cgmath::Rad;
impl AppConfig {
    fn set_projection(self, name: String, events: &EventManager) -> AppConfig {
        match (self, name.as_str()) {
            (AppConfig::Aitoff(app), "aitoff") => {
                AppConfig::Aitoff(app)
            },
            (AppConfig::MollWeide(app), "aitoff") => {
                AppConfig::Aitoff(app.set_projection::<Aitoff>(events))
            },
            (AppConfig::Ortho(app), "aitoff") => {
                AppConfig::Aitoff(app.set_projection::<Aitoff>(events))
            },
            (AppConfig::Arc(app), "aitoff") => {
                AppConfig::Aitoff(app.set_projection::<Aitoff>(events))
            },
            (AppConfig::Mercator(app), "aitoff") => {
                AppConfig::Aitoff(app.set_projection::<Aitoff>(events))
            },


            (AppConfig::Aitoff(app), "orthographic") => {
                AppConfig::Ortho(app.set_projection::<Orthographic>(events))
            },
            (AppConfig::MollWeide(app), "orthographic") => {
                AppConfig::Ortho(app.set_projection::<Orthographic>(events))
            },
            (AppConfig::Arc(app), "orthographic") => {
                AppConfig::Ortho(app.set_projection::<Orthographic>(events))
            },
            (AppConfig::Mercator(app), "orthographic") => {
                AppConfig::Ortho(app.set_projection::<Orthographic>(events))
            },
            (AppConfig::Ortho(app), "orthographic") => {
                AppConfig::Ortho(app)
            },

            (AppConfig::Aitoff(app), "mollweide") => {
                AppConfig::MollWeide(app.set_projection::<Mollweide>(events))
            },
            (AppConfig::MollWeide(app), "mollweide") => {
                AppConfig::MollWeide(app)
            },
            (AppConfig::Ortho(app), "mollweide") => {
                AppConfig::MollWeide(app.set_projection::<Mollweide>(events))
            },
            (AppConfig::Arc(app), "mollweide") => {
                AppConfig::MollWeide(app.set_projection::<Mollweide>(events))
            },
            (AppConfig::Mercator(app), "mollweide") => {
                AppConfig::MollWeide(app.set_projection::<Mollweide>(events))
            },

            (AppConfig::Aitoff(app), "arc") => {
                AppConfig::Arc(app.set_projection::<AzimutalEquidistant>(events))
            },
            (AppConfig::MollWeide(app), "arc") => {
                AppConfig::Arc(app.set_projection::<AzimutalEquidistant>(events))
            },
            (AppConfig::Ortho(app), "arc") => {
                AppConfig::Arc(app.set_projection::<AzimutalEquidistant>(events))
            },
            (AppConfig::Mercator(app), "arc") => {
                AppConfig::Arc(app.set_projection::<AzimutalEquidistant>(events))
            },
            (AppConfig::Arc(app), "arc") => {
                AppConfig::Arc(app)
            },

            (AppConfig::Aitoff(app), "mercator") => {
                AppConfig::Mercator(app.set_projection::<Mercator>(events))
            },
            (AppConfig::MollWeide(app), "mercator") => {
                AppConfig::Mercator(app.set_projection::<Mercator>(events))
            },
            (AppConfig::Ortho(app), "mercator") => {
                AppConfig::Mercator(app.set_projection::<Mercator>(events))
            },
            (AppConfig::Arc(app), "mercator") => {
                AppConfig::Mercator(app.set_projection::<Mercator>(events))
            },
            (AppConfig::Mercator(app), "mercator") => {
                AppConfig::Mercator(app)
            },
            _ => unreachable!()
        }
    }

    fn set_colormap(&mut self, colormap: Colormap) {
        match self {
            AppConfig::Aitoff(app) => app.set_colormap(colormap),
            AppConfig::MollWeide(app) => app.set_colormap(colormap),
            AppConfig::Ortho(app) => app.set_colormap(colormap),
            AppConfig::Arc(app) => app.set_colormap(colormap),
            AppConfig::Mercator(app) => app.set_colormap(colormap),
        };
    }

    fn update(&mut self, dt: f32, events: &EventManager, enable_grid: bool) -> Vector2<Rad<f32>> {
        let pos_center_world = match self {
            AppConfig::Aitoff(app) => app.update(dt, events, enable_grid),
            AppConfig::MollWeide(app) => app.update(dt, events, enable_grid),
            AppConfig::Ortho(app) => app.update(dt, events, enable_grid),
            AppConfig::Arc(app) => app.update(dt, events, enable_grid),
            AppConfig::Mercator(app) => app.update(dt, events, enable_grid),
        };

        pos_center_world
    }

    fn render(&mut self, enable_grid: bool) {
        match self {
            AppConfig::Aitoff(app) => app.render(enable_grid),
            AppConfig::MollWeide(app) => app.render(enable_grid),
            AppConfig::Ortho(app) => app.render(enable_grid),
            AppConfig::Arc(app) => app.render(enable_grid),
            AppConfig::Mercator(app) => app.render(enable_grid),
        }
    }

    /// Wheel event
    /*pub fn zoom(&mut self, delta_y: f32, enable_grid: bool) {
        match self {
            AppConfig::Aitoff(ref mut app, _) => {
                app.zoom(delta_y, enable_grid);
            },
            AppConfig::MollWeide(ref mut app, _) => {
                app.zoom(delta_y, enable_grid);

            },
            AppConfig::Arc(ref mut app, _) => {
                app.zoom(delta_y, enable_grid);

            },
            AppConfig::Mercator(ref mut app, _) => {
                app.zoom(delta_y, enable_grid);

            },
            AppConfig::Ortho(ref mut app, _) => {
                app.zoom(delta_y, enable_grid);
                
                //AppConfig::Ortho(app, s)
            },
            _ => unimplemented!(),
        }
    }*/
    /// Wheel event
    /*pub fn unzoom(&mut self, delta_y: f32, enable_grid: bool) {
        match self {
            AppConfig::Aitoff(ref mut app, _) => {
                app.unzoom(delta_y, enable_grid);
                // *self = AppConfig::Aitoff(*app, s)
            },
            AppConfig::MollWeide(ref mut app, _) => {
                app.unzoom(delta_y, enable_grid);
                // *self = AppConfig::MollWeide(*app, s)
            },
            AppConfig::Arc(ref mut app, _) => {
                app.unzoom(delta_y, enable_grid);
                // *self = AppConfig::Arc(*app, s)
            },
            AppConfig::Mercator(ref mut app, _) => {
                app.unzoom(delta_y, enable_grid);
                // *self = AppConfig::Mercator(*app, s)
            },
            AppConfig::Ortho(ref mut app, s) => {
                *self = if app.unzoom(delta_y, enable_grid) {
                    match *s {
                        "aitoff" => AppConfig::Aitoff(app.set_projection::<Aitoff>(events), s),
                        "mollweide" => AppConfig::MollWeide(app.set_projection::<MollWeide>(events), s),
                        "arc" => AppConfig::Arc(app.set_projection::<AzimutalEquidistant>(events), s),
                        "mercator" => AppConfig::Mercator(app.set_projection::<Mercator>(events), s),
                        "orthographic" => AppConfig::Ortho(*app, s),
                        _ => unreachable!()
                    }
                } else {
                    AppConfig::Ortho(*app, s)
                }
                app.unzoom(delta_y, enable_grid);
            }
        }
    }*/

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
            AppConfig::Aitoff(app) => app.add_catalog(sources),
            AppConfig::MollWeide(app) => app.add_catalog(sources),
            AppConfig::Arc(app) => app.add_catalog(sources),
            AppConfig::Ortho(app) => app.add_catalog(sources),
            AppConfig::Mercator(app) => app.add_catalog(sources),

        }
    }

    pub fn set_hips_config(&mut self, config: HiPSConfig) {        
        match self {
            AppConfig::Aitoff(app) => app.set_hips_config(config),
            AppConfig::MollWeide(app) =>  app.set_hips_config(config),
            AppConfig::Arc(app) => app.set_hips_config(config),
            AppConfig::Ortho(app) =>  app.set_hips_config(config),
            AppConfig::Mercator(app) =>  app.set_hips_config(config),

        }
    }

    pub fn resize(&mut self, width: f32, height: f32, enable_grid: bool) {        
        match self {
            AppConfig::Aitoff(app) => app.resize_window(width, height, enable_grid),
            AppConfig::MollWeide(app) => app.resize_window(width, height, enable_grid),
            AppConfig::Arc(app) => app.resize_window(width, height, enable_grid),
            AppConfig::Ortho(app) => app.resize_window(width, height, enable_grid),
            AppConfig::Mercator(app) => app.resize_window(width, height, enable_grid),

        }
    }

    pub fn set_kernel_strength(&mut self, strength: f32) {        
        match self {
            AppConfig::Aitoff(app) => app.set_kernel_strength(strength),
            AppConfig::MollWeide(app) => app.set_kernel_strength(strength),
            AppConfig::Arc(app) => app.set_kernel_strength(strength),
            AppConfig::Ortho(app) => app.set_kernel_strength(strength),
            AppConfig::Mercator(app) => app.set_kernel_strength(strength),

        }
    }

    pub fn set_heatmap_opacity(&mut self, opacity: f32) {        
        match self {
            AppConfig::Aitoff(app) => app.set_heatmap_opacity(opacity),
            AppConfig::MollWeide(app) => app.set_heatmap_opacity(opacity),
            AppConfig::Arc(app) => app.set_heatmap_opacity(opacity),
            AppConfig::Ortho(app) => app.set_heatmap_opacity(opacity),
            AppConfig::Mercator(app) => app.set_heatmap_opacity(opacity),

        }
    }

    pub fn set_range_source_size(&mut self, source_size_range: std::ops::Range<f32>) {
        match self {
            AppConfig::Aitoff(app) => app.set_range_source_size(source_size_range),
            AppConfig::MollWeide(app) => app.set_range_source_size(source_size_range),
            AppConfig::Arc(app) => app.set_range_source_size(source_size_range),
            AppConfig::Ortho(app) => app.set_range_source_size(source_size_range),
            AppConfig::Mercator(app) => app.set_range_source_size(source_size_range),

        }
    }

    pub fn set_position(&mut self, ra: Rad<f32>, dec: Rad<f32>) {
        match self {
            AppConfig::Aitoff(app) => app.set_position(ra, dec),
            AppConfig::MollWeide(app) => app.set_position(ra, dec),
            AppConfig::Arc(app) => app.set_position(ra, dec),
            AppConfig::Ortho(app) => app.set_position(ra, dec),
            AppConfig::Mercator(app) => app.set_position(ra, dec),

        }
    }

    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        match self {
            AppConfig::Aitoff(app) => app.set_color_rgb(red, green, blue),
            AppConfig::MollWeide(app) => app.set_color_rgb(red, green, blue),
            AppConfig::Arc(app) => app.set_color_rgb(red, green, blue),
            AppConfig::Ortho(app) => app.set_color_rgb(red, green, blue),
            AppConfig::Mercator(app) => app.set_color_rgb(red, green, blue),

        }
    }

    pub fn change_grid_opacity(&mut self, alpha: f32) {
        match self {
            AppConfig::Aitoff(app) => app.change_grid_opacity(alpha),
            AppConfig::MollWeide(app) => app.change_grid_opacity(alpha),
            AppConfig::Arc(app) => app.change_grid_opacity(alpha),
            AppConfig::Ortho(app) => app.change_grid_opacity(alpha),
            AppConfig::Mercator(app) => app.change_grid_opacity(alpha),

        }
    }

    pub fn enable_grid(&mut self) {
        match self {
            AppConfig::Aitoff(app) => app.enable_grid(),
            AppConfig::MollWeide(app) => app.enable_grid(),
            AppConfig::Arc(app) => app.enable_grid(),
            AppConfig::Ortho(app) => app.enable_grid(),
            AppConfig::Mercator(app) => app.enable_grid(),
        }
    }
}

use crate::event_manager::{
 EventManager,
 MouseLeftButtonPressed,
 MouseLeftButtonReleased,
 MouseLeftButtonDoublePressed,
 MouseMove,
 MouseWheelUp,
 MouseWheelDown,
 KeyboardPressed
};
#[wasm_bindgen]
pub struct WebClient {
    // The app
    appconfig: AppConfig,

    // Stores all the possible events
    // with some associated data    
    events: EventManager,

    // The time between the previous and the current
    // frame
    dt: f32,

    // Some booleans for enabling/desabling
    // specific computations
    enable_inertia: bool,
    enable_grid: bool,
}

use js_sys::WebAssembly;
#[wasm_bindgen]
impl WebClient {
    /// Create a new web client
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        let pages = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .map_err(|_| "Unable to get the WASM memory buffer for storing the vertices data!").unwrap()
            .grow(20);
        console::log_1(&format!("num_pages {:?}", pages).into());

        let gl = WebGl2Context::new();
        let events = EventManager::new();

        let app = App::<Orthographic>::new(&gl, &events).unwrap();
        let appconfig = AppConfig::Ortho(app);
        let dt = 0_f32;
        let enable_inertia = false;
        let enable_grid = true;

        WebClient {
            appconfig,

            events,

            dt,
            enable_inertia,
            enable_grid,
        }
    }

    /// Main update method
    pub fn update(&mut self, dt: f32) -> Result<Box<[f32]>, JsValue> {
        // dt refers to the time taking (in ms) rendering the previous frame
        self.dt = dt;
        
        // Update the application and get back the
        // world coordinates of the center of projection in (ra, dec)
        let pos_center_world = self.appconfig.update(
            // Time of the previous frame rendering 
            dt,
            // Event manager
            &self.events,
            // Some constants
            self.enable_grid
        );

        // Reset all the events after the update
        self.events.reset();

        // Extract the ra and dec
        let ra_deg: Deg<f32> = pos_center_world.x.into();
        let dec_deg: Deg<f32> = pos_center_world.y.into();

        // Return it to the index.js
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
        self.appconfig = self.appconfig.set_projection(name, &self.events);

        Ok(self)
    }

    /// Change the current projection of the HiPS
    pub fn set_colormap(&mut self, name: String) -> Result<(), JsValue> {
        let colormap = match name.as_str() {
            "BluePastelRed" => Colormap::BluePastelRed,
            "IDL_CB_BrBG" => Colormap::IDL_CB_BrBG,
            "IDL_CB_YIGnBu" => Colormap::IDL_CB_YIGnBu,
            "IDL_CB_GnBu" => Colormap::IDL_CB_GnBu,
            "Red_Temperature" => Colormap::Red_Temperature,
            "Black_White_Linear" => Colormap::Black_White_Linear,
            _ => panic!("{:?} colormap not recognized!", name)
        };
        self.appconfig.set_colormap(colormap);

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

        Ok(())
    }

    /// Disable equatorial grid
    pub fn disable_equatorial_grid(&mut self) -> Result<(), JsValue> {
        self.enable_grid = false;

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
    pub fn change_hips(&mut self,
     name: String,
     tile_size: i32,
     max_depth: i32,
     fmt: String
    ) -> Result<(), JsValue> {
        let tile_img_fmt: Result<ImageFormat, JsValue> = if fmt.contains("png") {
            Ok(ImageFormat::PNG)
        } else if fmt.contains("fits") {
            Ok(ImageFormat::FITS)
        } else if fmt.contains("jpg") || fmt.contains("jpeg") {
            Ok(ImageFormat::JPG)
        } else {
            Err(format!("{:?} tile format unknown!", fmt).into())
        };
        let config = HiPSConfig::new(
            name,
            max_depth as u8,
            tile_size as usize,
            tile_img_fmt?
        );

        self.appconfig.set_hips_config(config);

        Ok(())
    }

    /// User-events received from javascript
    /// Start move
    pub fn press_left_mouse_button(&mut self, screen_pos_x: f32, screen_pos_y: f32) -> Result<(), JsValue> {
        // Enable the MouseLeftButtonPressed event
        self.events.enable::<MouseLeftButtonPressed>(
            Vector2::new(
                screen_pos_x,
                screen_pos_y
            )
        );

        Ok(())
    }
    /// Stop move
    pub fn release_left_mouse_button(&mut self, screen_pos_x: f32, screen_pos_y: f32) -> Result<(), JsValue> {
        // Enable the MouseLeftButtonReleased event
        self.events.enable::<MouseLeftButtonReleased>(
            Vector2::new(
                screen_pos_x,
                screen_pos_y
            )
        );

        Ok(())
    }
    /// Stop move
    pub fn double_mouse_button_pressed(&mut self, screen_pos_x: f32, screen_pos_y: f32) -> Result<(), JsValue> {
        // Enable the MouseLeftButtonReleased event
        self.events.enable::<MouseLeftButtonDoublePressed>(
            Vector2::new(
                screen_pos_x,
                screen_pos_y
            )
        );

        Ok(())
    }
    /// Keep moving
    pub fn move_mouse(&mut self, screen_pos_x: f32, screen_pos_y: f32) -> Result<(), JsValue> {
        self.events.enable::<MouseMove>(
            Vector2::new(
                screen_pos_x,
                screen_pos_y
            )
        );
        Ok(())
    }

    /// Wheel event
    pub fn wheel_mouse(&mut self, screen_pos_x: f32, screen_pos_y: f32, delta_y: i32) -> Result<(), JsValue> {
        let up = delta_y < 0;

        // Wheel mouse up
        if up {
            self.events.enable::<MouseWheelUp>(
                Vector2::new(
                    screen_pos_x,
                    screen_pos_y,
                )
            );
            //self.appconfig.zoom(y, self.enable_grid);
        // Wheel mouse down
        } else {
            self.events.enable::<MouseWheelDown>(
                Vector2::new(
                    screen_pos_x,
                    screen_pos_y,
                )
            );
            //self.appconfig.unzoom(y, self.enable_grid);
        }

        Ok(())
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

