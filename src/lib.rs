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

use crate::shader::Shaderize;

use crate::renderable::hips_sphere::PerPixel;

use cgmath::Vector2;
use cgmath::Deg;

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

        // Resize event
        let onresize = {
            //let viewport = viewport.clone();

            Closure::wrap(Box::new(move || {
                console::log_1(&format!("resize").into());

                //RENDER_NEXT_FRAME.store(true, Ordering::Relaxed);
                //viewport.borrow_mut().resize();

                //hips_sphere
            }) as Box<dyn FnMut()>)
        };
        web_sys::window()
            .unwrap()
            .set_onresize(Some(onresize.as_ref().unchecked_ref()));
        onresize.forget();

        let moving = None;
        let gl = gl.clone();
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
        };

        Ok(app)
    } 

    fn update(&mut self, dt: f32) {
        /*if !UPDATE_USER_INTERFACE.load(Ordering::Relaxed) {
            RENDER_FRAME.lock().unwrap().update(&self.viewport);
            //UPDATE_FRAME.lock().unwrap().update(&self.viewport);
        } else {
            UPDATE_USER_INTERFACE.store(false, Ordering::Relaxed);
        }*/

        //self.viewport.update(&mut self.inertia, &mut self.grid, &mut self.catalog, &mut self.hips_sphere);
        
        // Check whether the HiPS sphere must be updated or not
        if utils::get_current_time() < *LATEST_TIME_TILE_RECEIVED.lock().unwrap() + BLENDING_DURATION_MS {
            self.hips_sphere.mesh_mut().update::<P>(&self.viewport);
        }
    }

    fn render(&self) {
        if true {
            // Render the scene
            self.gl.clear_color(0.08, 0.08, 0.08, 1.0);
            self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

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
            /*self.ortho_hips_sphere.draw(
                shaders,
                viewport
            );*/

            // Draw the catalogs
            self.catalog.mesh().draw(
                &self.gl,
                &self.catalog,
                shaders,
                viewport
            );

            // Draw the grid
            /*if *ENABLED_WIDGETS.lock().unwrap().get("grid").unwrap() {
                // The grid lines
                self.grid.draw(
                    shaders,
                    viewport
                );
                // The labels
                self.grid.mesh().draw_labels();
            }*/
        }
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
    }

    /// MOVE EVENT
    fn initialize_move(&mut self, screen_pos: Vector2<f32>) {
        // stop inertia if there is one
        //self.inertia = None;
        //self.viewport.stop_inertia();

        if let Some(start_world_pos) = P::screen_to_world_space(screen_pos, &self.viewport) {
            self.moving = Some(Move::new(start_world_pos));
        }
    }

    fn moves(&mut self, screen_pos: Vector2<f32>) {
        if let Some(world_pos) = P::screen_to_world_space(screen_pos, &self.viewport) {
            // If a move is done
            if let Some(ref mut moving) = &mut self.moving {
                // Moves the renderables
                moving.apply_to_renderables(
                    world_pos,

                    &mut self.hips_sphere,
                    &mut self.grid,
                    &mut self.catalog,
                );

                // Moves the viewport
                self.viewport.displacement::<P>(&mut self.hips_sphere, &mut self.catalog);
            }
        }
    }

    fn stop_move(&mut self, screen_pos: Vector2<f32>) {
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
    fn unzoom(&mut self, delta_y: f32) -> bool {
        let a0: Deg<f32> = self.viewport
            .field_of_view()
            .get_aperture()
            .into();

        self.viewport.unzoom::<P>(&mut self.hips_sphere, &mut self.catalog);

        let a1: Deg<f32> = self.viewport
            .field_of_view()
            .get_aperture()
            .into();

        a0 < Deg(100_f32) && a1 >= Deg(100_f32)
    }

    // ZOOM EVENT
    // Returns true if hips_sphere rendering mode has to be changed to the SmallFOV mode
    fn zoom(&mut self, delta_y: f32) -> bool {
        let a0: Deg<f32> = self.viewport
            .field_of_view()
            .get_aperture()
            .into();

        self.viewport.zoom::<P>(&mut self.hips_sphere, &mut self.catalog);
    
        let a1: Deg<f32> = self.viewport
            .field_of_view()
            .get_aperture()
            .into();

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

    fn resize_window(&mut self, width: f32, height: f32) {
        self.viewport.resize_window::<P>(width, height);
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

    fn update(&mut self, dt: f32) {
        match self {
            AppConfig::Aitoff(app, _) => app.update(dt),
            AppConfig::MollWeide(app, _) => app.update(dt),
            AppConfig::Ortho(app, _) => app.update(dt),
        }
    }

    fn render(&self) {
        match self {
            AppConfig::Aitoff(app, _) => app.render(),
            AppConfig::MollWeide(app, _) => app.render(),
            AppConfig::Ortho(app, _) => app.render(),
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
    pub fn stop_move(&mut self, screen_pos_x: f32, screen_pos_y: f32) {
        let screen_pos = Vector2::new(screen_pos_x, screen_pos_y);
        match self {
            AppConfig::Aitoff(app, _) => app.stop_move(screen_pos),
            AppConfig::MollWeide(app, _) => app.stop_move(screen_pos),
            AppConfig::Ortho(app, _) => app.stop_move(screen_pos),
        }
    }
    /// Keep moving
    pub fn moves(&mut self, screen_pos_x: f32, screen_pos_y: f32) {
        let screen_pos = Vector2::new(screen_pos_x, screen_pos_y);
        match self {
            AppConfig::Aitoff(app, _) => app.moves(screen_pos),
            AppConfig::MollWeide(app, _) => app.moves(screen_pos),
            AppConfig::Ortho(app, _) => app.moves(screen_pos),
        }
    }

    /// Wheel event
    pub fn zoom(self, delta_y: f32) -> AppConfig {
        match self {
            AppConfig::Aitoff(mut app, s) => {
                if app.zoom(delta_y) {
                    AppConfig::Ortho(app.set_projection::<Orthographic>(), s)
                } else {
                    AppConfig::Aitoff(app, s)
                }
            },
            AppConfig::MollWeide(mut app, s) => {
                if app.zoom(delta_y) {
                    AppConfig::Ortho(app.set_projection::<Orthographic>(), s)
                } else {
                    AppConfig::MollWeide(app, s)
                }
            },
            AppConfig::Ortho(mut app, s) => {
                app.zoom(delta_y);
                AppConfig::Ortho(app, s)
            },
        }
    }
    /// Wheel event
    pub fn unzoom(self, delta_y: f32) -> AppConfig {
        match self {
            AppConfig::Aitoff(mut app, s) => {
                app.unzoom(delta_y);
                AppConfig::Aitoff(app, s)
            },
            AppConfig::MollWeide(mut app, s) => {
                app.unzoom(delta_y);
                AppConfig::MollWeide(app, s)
            },
            AppConfig::Ortho(mut app, s) => {
                if app.unzoom(delta_y) {
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

    pub fn resize(&mut self, width: f32, height: f32) {        
        match self {
            AppConfig::Aitoff(app, _) => app.resize_window(width, height),
            AppConfig::MollWeide(app, _) => app.resize_window(width, height),
            AppConfig::Ortho(app, _) => app.resize_window(width, height),
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
}

#[wasm_bindgen]
pub struct WebClient {
    appconfig: AppConfig,
}

#[wasm_bindgen]
impl WebClient {
    /// Create a new web client
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        let gl = WebGl2Context::new();

        let app = App::<Aitoff>::new(&gl).unwrap();
        let appconfig = AppConfig::Aitoff(app, "aitoff");

        WebClient {
            appconfig,
        }
    }

    /// Update our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn update(&mut self, dt: f32) -> Result<(), JsValue> {
        self.appconfig.update(dt);

        Ok(())
    }

    /// Update our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn render(&self) -> Result<(), JsValue> {
        self.appconfig.render();

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

    /*/// Enable equatorial grid
    pub fn enable_equatorial_grid(&mut self) -> Result<(), JsValue> {
        if let Some(grid) = ENABLED_WIDGETS.lock().unwrap().get_mut("grid") {
            *grid = true;
            self.app.grid
                .update(
                    &self.app.projection,
                    &mut self.app.viewport
                );
        }

        Ok(())
    }

    /// Disable equatorial grid
    pub fn disable_equatorial_grid(&mut self) -> Result<(), JsValue> {
        if let Some(grid) = ENABLED_WIDGETS.lock().unwrap().get_mut("grid") {
            *grid = false;
            self.app.grid.mesh_mut().clear_canvas();
        }
        //RENDER_FRAME.lock().unwrap().set(true);
        //UPDATE_USER_INTERFACE.store(true, Ordering::Relaxed);

        Ok(())
    }

    /// Enable mouse inertia
    pub fn enable_inertia(&mut self) -> Result<(), JsValue> {
        ENABLE_INERTIA.store(true, Ordering::Relaxed);

        Ok(())
    }
    /// Disable mouse inertia
    pub fn disable_inertia(&mut self) -> Result<(), JsValue> {
        ENABLE_INERTIA.store(false, Ordering::Relaxed);

        Ok(())
    }

    /// Change grid color
    pub fn change_grid_color(&mut self, red: f32, green: f32, blue: f32) -> Result<(), JsValue> {
        self.app.grid.mesh_mut().set_color_rgb(red, green, blue);
        //RENDER_FRAME.lock().unwrap().set(true);
        //UPDATE_USER_INTERFACE.store(true, Ordering::Relaxed);

        Ok(())
    }

    /// Change grid opacity
    pub fn change_grid_opacity(&mut self, alpha: f32) -> Result<(), JsValue> {
        self.app.grid
            .mesh_mut()
            .set_alpha(alpha);

        //RENDER_FRAME.lock().unwrap().set(true);
        //UPDATE_USER_INTERFACE.store(true, Ordering::Relaxed);

        Ok(())
    }
    */
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
        self.appconfig.stop_move(screen_pos_x, screen_pos_y);
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
            self.appconfig.zoom(delta_y)
        } else {
            self.appconfig.unzoom(delta_y)
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
        self.appconfig.resize(width, height);

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
}

