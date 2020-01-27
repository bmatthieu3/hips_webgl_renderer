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

use crate::renderable::colormap::*;

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

fn add_tile_buffer_uniforms(name: &'static str, size: usize, uniforms: &mut Vec<String>) {
    let texture = String::from(name);
    uniforms.push(texture.clone());

    let base_texture = texture + "_tiles[";

    for i in 0..size {
        let mut tile = base_texture.clone() + &i.to_string();
        tile += "].";

        let uniq = tile.clone() + "uniq";
        let texture_idx = tile.clone() + "texture_idx";
        let time_received = tile.clone() + "time_received";
        let time_request = tile + "time_request";

        uniforms.push(uniq);
        uniforms.push(texture_idx);
        uniforms.push(time_received);
        uniforms.push(time_request);
    }
}

use crate::shader::Shaderize;

use cgmath::Vector2;
impl<P> App<P>
where P: Projection {
    fn new(gl: &WebGl2Context) -> Result<App<P>, JsValue> {

        console::log_1(&format!("Init").into());

        // Shader definition
        // HiPS sphere shader
        // uniforms definition
        let mut uniforms_2d_proj = vec![
            // General uniforms
            String::from("current_time"),
            String::from("model"),
            // Viewport uniforms
            String::from("ndc_to_clip"),
            String::from("clip_zoom_factor"),
            String::from("aspect"),
            String::from("last_zoom_action"),
            // HiPS Sphere-specific uniforms
            String::from("current_depth"),
            String::from("max_depth"),
        ];

        add_tile_buffer_uniforms("textures", 128, &mut uniforms_2d_proj);

        let shader_2d_proj = Shader::new(&gl,
            shaders::proj_vert::CONTENT,
            shaders::proj_frag::CONTENT,
            // uniform list
            uniforms_2d_proj
        );

        // Grid shader
        // uniforms definition
        let uniforms_grid = vec![
            // General uniforms
            String::from("current_time"),
            String::from("model"),
            // Viewport uniforms
            String::from("ndc_to_clip"),
            String::from("clip_zoom_factor"),
            String::from("aspect"),
            String::from("last_zoom_action"),
            // Grid-specific uniforms
            String::from("location_color"),
        ];
        let shader_grid = Shader::new(&gl,
            shaders::grid_projeted_vert::CONTENT,
            shaders::grid_frag::CONTENT,
            uniforms_grid
        );

        // Catalog shader
        // uniforms definition
        let uniforms_catalog = vec![
            // General uniforms
            String::from("current_time"),
            String::from("model"),
            // Viewport uniforms
            String::from("ndc_to_clip"),
            String::from("clip_zoom_factor"),
            String::from("aspect"),
            String::from("last_zoom_action"),
            // Catalog-specific uniforms
            String::from("kernel_texture"),
            String::from("strength"),
        ];
        let shader_catalog = Shader::new(&gl,
            shaders::catalog_vert::CONTENT,
            shaders::catalog_frag::CONTENT,
            uniforms_catalog
        );

        // HiPS Ortho shader
        // uniforms definition
        let mut uniforms_ortho_hips = vec![
            // General uniforms
            String::from("current_time"),
            String::from("model"),
            // Viewport uniforms
            String::from("ndc_to_clip"),
            String::from("clip_zoom_factor"),
            String::from("aspect"),
            String::from("last_zoom_action"),
            // HiPS Ortho specific uniforms
            String::from("current_depth"),
            String::from("max_depth"),
            // Textures
            String::from("textures[0]"),
            String::from("textures[1]"),
        ];

        //add_tile_buffer_uniforms("textures", 128, &mut uniforms_ortho_hips);
        console::log_1(&format!("CC").into());

        let shader_ortho_hips = Shader::new(&gl,
            shaders::hips_sphere_small_fov_vert::CONTENT,
            shaders::hips_sphere_small_fov_frag::CONTENT,
            uniforms_ortho_hips
        );
        console::log_1(&format!("DD").into());

        let mut shaders = HashMap::new();
        shaders.insert("hips_sphere", shader_2d_proj);
        shaders.insert("grid", shader_grid);
        shaders.insert("catalog", shader_catalog);

        BluePastelRed::create_shader(&gl, &mut shaders);
        IDL_CB_BrBG::create_shader(&gl, &mut shaders);
        IDL_CB_YIGnBu::create_shader(&gl, &mut shaders);
        
        shaders.insert("hips_sphere_small_fov", shader_ortho_hips);

        gl.enable(WebGl2RenderingContext::BLEND);
        gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        //gl.enable(WebGl2RenderingContext::SCISSOR_TEST);
        gl.enable(WebGl2RenderingContext::CULL_FACE);
        gl.cull_face(WebGl2RenderingContext::BACK);

        // Viewport definition
        let viewport = ViewPort::new::<P>(&gl);

        // HiPS Sphere definition
        let hips_sphere_mesh = HiPSSphere::new::<P>(&gl, &viewport);
        console::log_1(&format!("fffff sfs").into());
        let hips_sphere = Renderable::<HiPSSphere>::new(
            &gl,
            &shaders["hips_sphere"],
            hips_sphere_mesh,
        );
        console::log_1(&format!("fffff sfs").into());
        // Catalog definition
        let catalog_mesh = Catalog::new(&gl, vec![]);
        let catalog = Renderable::<Catalog>::new(
            &gl,
            &shaders["catalog"],
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
            &shaders["grid"],
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
            self.hips_sphere.update::<P>(&self.viewport);
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
            self.hips_sphere.draw(
                shaders,
                viewport
            );
            /*self.ortho_hips_sphere.draw(
                shaders,
                viewport
            );*/

            // Draw the catalogs
            self.catalog.draw(
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
        // New HiPS sphere
        let hips_sphere_mesh = HiPSSphere::new::<Q>(&self.gl, &self.viewport);

        // Update the scissor for the new projection
        //self.viewport.resize(&Q::size());
        self.hips_sphere.update_mesh(&self.shaders["hips_sphere"], hips_sphere_mesh);

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

    // ZOOM EVENT
    fn zoom(&mut self, delta_y: f32) {
        if delta_y < 0_f32 {
            self.viewport.zoom::<P>(&mut self.hips_sphere, &mut self.catalog);
        } else {
            self.viewport.unzoom::<P>(&mut self.hips_sphere, &mut self.catalog);
        }
    }

    fn add_catalog(&mut self, sources: Vec<Source>) {
        let catalog_mesh = Catalog::new(&self.gl, sources);
        self.catalog.update_mesh(&self.shaders["catalog"], catalog_mesh);
    }

    fn set_colormap(&mut self, colormap: String) {
        match colormap.as_str() {
            "BluePastelRed" => self.catalog.mesh_mut().set_colormap::<BluePastelRed>(),
            "IDL_CB_BrBG" => self.catalog.mesh_mut().set_colormap::<IDL_CB_BrBG>(),
            "IDL_CB_YIGnBu"=> self.catalog.mesh_mut().set_colormap::<IDL_CB_YIGnBu>(),
            _ => panic!("{:?} colormap not recognized!", colormap)
        }
    }

    fn resize_window(&mut self, width: f32, height: f32) {
        self.viewport.resize_window::<P>(width, height);
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

enum AppConfig {
    Ait(App<Aitoff>),
    Ort(App<Orthographic>),
    Mol(App<MollWeide>),
}

impl AppConfig {
    fn set_projection(self, proj: &str) -> AppConfig {
        match (self, proj) {
            (AppConfig::Ait(app), "aitoff") => {
                AppConfig::Ait(app)
            },
            (AppConfig::Ait(app), "orthographic") => {
                AppConfig::Ort(app.set_projection::<Orthographic>())
            },
            (AppConfig::Ait(app), "mollweide") => {
                AppConfig::Mol(app.set_projection::<MollWeide>())
            },

            (AppConfig::Ort(app), "aitoff") => {
                AppConfig::Ait(app.set_projection::<Aitoff>())
            },
            (AppConfig::Ort(app), "orthographic") => {
                AppConfig::Ort(app)
            },
            (AppConfig::Ort(app), "mollweide") => {
                AppConfig::Mol(app.set_projection::<MollWeide>())
            },

            (AppConfig::Mol(app), "aitoff") => {
                AppConfig::Ait(app.set_projection::<Aitoff>())
            },
            (AppConfig::Mol(app), "orthographic") => {
                AppConfig::Ort(app.set_projection::<Orthographic>())
            },
            (AppConfig::Mol(app), "mollweide") => {
                AppConfig::Mol(app)
            },
            _ => unreachable!()
        }
    }

    fn set_colormap(&mut self, colormap: String) {
        match self {
            AppConfig::Ait(app) => app.set_colormap(colormap),
            AppConfig::Mol(app) => app.set_colormap(colormap),
            AppConfig::Ort(app) => app.set_colormap(colormap),
        };
    }

    fn update(&mut self, dt: f32) {
        match self {
            AppConfig::Ait(app) => app.update(dt),
            AppConfig::Mol(app) => app.update(dt),
            AppConfig::Ort(app) => app.update(dt),
        }
    }

    fn render(&self) {
        match self {
            AppConfig::Ait(app) => app.render(),
            AppConfig::Mol(app) => app.render(),
            AppConfig::Ort(app) => app.render(),
        }
    }

    pub fn initialize_move(&mut self, screen_pos_x: f32, screen_pos_y: f32) {
        let screen_pos = Vector2::new(screen_pos_x, screen_pos_y);
        match self {
            AppConfig::Ait(app) => app.initialize_move(screen_pos),
            AppConfig::Mol(app) => app.initialize_move(screen_pos),
            AppConfig::Ort(app) => app.initialize_move(screen_pos),
        }
    }

    /// Stop move
    pub fn stop_move(&mut self, screen_pos_x: f32, screen_pos_y: f32) {
        let screen_pos = Vector2::new(screen_pos_x, screen_pos_y);
        match self {
            AppConfig::Ait(app) => app.stop_move(screen_pos),
            AppConfig::Mol(app) => app.stop_move(screen_pos),
            AppConfig::Ort(app) => app.stop_move(screen_pos),
        }
    }
    /// Keep moving
    pub fn moves(&mut self, screen_pos_x: f32, screen_pos_y: f32) {
        let screen_pos = Vector2::new(screen_pos_x, screen_pos_y);
        match self {
            AppConfig::Ait(app) => app.moves(screen_pos),
            AppConfig::Mol(app) => app.moves(screen_pos),
            AppConfig::Ort(app) => app.moves(screen_pos),
        }
    }

    /// Wheel event
    pub fn zoom(&mut self, delta_y: f32) {
        match self {
            AppConfig::Ait(app) => app.zoom(delta_y),
            AppConfig::Mol(app) => app.zoom(delta_y),
            AppConfig::Ort(app) => app.zoom(delta_y),
        }
    }

    pub fn add_catalog(&mut self, data: &JsValue) {
        let data: Vec<[f32; 3]> = data.into_serde().unwrap();

        let sources: Vec<Source> = data.into_iter()
            .map(|ref source| {
                (source as &[f32]).into()
            })
            .collect::<Vec<_>>();

        match self {
            AppConfig::Ait(app) => app.add_catalog(sources),
            AppConfig::Mol(app) => app.add_catalog(sources),
            AppConfig::Ort(app) => app.add_catalog(sources),
        }
    }

    pub fn reload_hips_sphere(&mut self, hips_url: String, max_depth: u8) {        
        match self {
            AppConfig::Ait(app) => app.reload_hips_sphere(hips_url, max_depth),
            AppConfig::Mol(app) => app.reload_hips_sphere(hips_url, max_depth),
            AppConfig::Ort(app) => app.reload_hips_sphere(hips_url, max_depth),
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {        
        match self {
            AppConfig::Ait(app) => app.resize_window(width, height),
            AppConfig::Mol(app) => app.resize_window(width, height),
            AppConfig::Ort(app) => app.resize_window(width, height),
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

        let app: App<Orthographic> = App::new(&gl).unwrap();
        let appconfig = AppConfig::Ort(app);

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
    /// Change grid opacity
    pub fn set_catalog_opacity(&mut self, alpha: f32) -> Result<(), JsValue> {
        self.app.catalog
            .mesh_mut()
            .set_alpha(alpha);

        //RENDER_FRAME.lock().unwrap().set(true);
        //UPDATE_USER_INTERFACE.store(true, Ordering::Relaxed);

        Ok(())
    }
    /// Change grid opacity
    pub fn set_kernel_strength(&mut self, strength: f32) -> Result<(), JsValue> {
        self.app.catalog
            .mesh_mut()
            .set_kernel_strength(strength);

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
    pub fn zoom(&mut self, delta_y: f32) -> Result<(), JsValue> {
        self.appconfig.zoom(delta_y);

        Ok(())
    }

    /// Add new catalog
    pub fn add_catalog(&mut self, data: &JsValue) -> Result<(), JsValue> {
        self.appconfig.add_catalog(data);

        Ok(())
    }

    /// Add new catalog
    pub fn resize(&mut self, width: f32, height: f32) -> Result<(), JsValue> {
        self.appconfig.resize(width, height);

        Ok(())
    }
}

