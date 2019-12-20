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
use renderable::projection::ProjectionType;
use renderable::projection::{Aitoff, Orthographic, MollWeide};

use viewport::ViewPort;

use std::sync::Mutex;
use std::sync::Arc;

use std::rc::Rc;

use std::collections::HashMap;

use std::sync::atomic::{AtomicU32, AtomicU8, AtomicBool};
use std::sync::atomic::Ordering;

use crate::render_next_frame::RENDER_FRAME;
use crate::render_next_frame::UPDATE_FRAME;
use crate::render_next_frame::UPDATE_USER_INTERFACE;

use crate::event::Move;

use crate::mouse_inertia::MouseInertia;
struct App {
    gl: WebGl2Context,

    shaders: HashMap<&'static str, Shader>,

    viewport: ViewPort,
    projection: ProjectionType,

    // The sphere renderable
    hips_sphere: Renderable<HiPSSphere>,
    // The grid renderable
    grid: Renderable<ProjetedGrid>,
    // The catalogs
    catalog: Renderable<Catalog>,

    // Move event
    move_event: Option<Move>,

    // Options
    // Mouse Inertia
    inertia: Option<MouseInertia>,
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

use cgmath::Vector2;
use crate::event::screen_to_world_space;
impl App {
    fn new(gl: &WebGl2Context) -> Result<App, JsValue> {
        // Shader definition
        // HiPS sphere shader
        // uniforms definition
        let mut uniforms_2d_proj = vec![
            // General uniforms
            String::from("current_time"),
            String::from("model"),
            // Viewport uniforms
            String::from("zoom_factor"),
            String::from("aspect"),
            String::from("last_zoom_action"),
            // HiPS Sphere-specific uniforms
            String::from("current_depth"),
            String::from("max_depth"),
        ];

        add_tile_buffer_uniforms("textures", 64, &mut uniforms_2d_proj);
        add_tile_buffer_uniforms("textures_0", 12, &mut uniforms_2d_proj);

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
            String::from("zoom_factor"),
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
            String::from("zoom_factor"),
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

        // Heatmap shader
        // uniforms definition
        let uniforms_heatmap = vec![
            // General uniforms
            String::from("current_time"),
            String::from("model"),
            // Viewport uniforms
            String::from("zoom_factor"),
            String::from("aspect"),
            String::from("last_zoom_action"),
            // Heatmap-specific uniforms
            String::from("texture_fbo"),
            String::from("colormap"),
            String::from("alpha"),
        ];
        let shader_heatmap = Shader::new(&gl,
            shaders::heatmap_vert::CONTENT,
            shaders::heatmap_frag::CONTENT,
            uniforms_heatmap
        );

        // HiPS Ortho shader
        // uniforms definition
        let mut uniforms_ortho_hips = vec![
            // General uniforms
            String::from("current_time"),
            String::from("model"),
            // Viewport uniforms
            String::from("zoom_factor"),
            String::from("aspect"),
            String::from("last_zoom_action"),
            // HiPS Ortho specific uniforms
            String::from("current_depth"),
            String::from("max_depth"),
        ];

        add_tile_buffer_uniforms("textures", 64, &mut uniforms_ortho_hips);
        add_tile_buffer_uniforms("textures_0", 12, &mut uniforms_ortho_hips);
        //add_tile_buffer_uniforms("textures_0", 12, &mut uniforms_ortho_hips);

        let shader_ortho_hips = Shader::new(&gl,
            shaders::hips_sphere_small_fov_vert::CONTENT,
            shaders::hips_sphere_small_fov_frag::CONTENT,
            uniforms_ortho_hips
        );

        let mut shaders = HashMap::new();
        shaders.insert("hips_sphere", shader_2d_proj);
        shaders.insert("grid", shader_grid);
        shaders.insert("catalog", shader_catalog);
        shaders.insert("heatmap", shader_heatmap);
        shaders.insert("hips_sphere_small_fov", shader_ortho_hips);

        gl.enable(WebGl2RenderingContext::BLEND);
        gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        gl.enable(WebGl2RenderingContext::SCISSOR_TEST);
        gl.disable(WebGl2RenderingContext::DEPTH_TEST);

        // Projection definition
        let projection = ProjectionType::Orthographic(Orthographic {});
        // HiPS Sphere definition
        let hips_sphere_mesh = HiPSSphere::new(&gl, &projection);
        let hips_sphere = Renderable::<HiPSSphere>::new(
            &gl,
            &shaders["hips_sphere"],
            hips_sphere_mesh,
        );

        // Viewport definition
        let viewport = ViewPort::new(&gl, &projection.size());

        // Grid definition
        let lon_bound = cgmath::Vector2::<cgmath::Rad<f32>>::new(cgmath::Deg(-30_f32).into(), cgmath::Deg(30_f32).into());
        let lat_bound = cgmath::Vector2::<cgmath::Rad<f32>>::new(cgmath::Deg(-90_f32).into(), cgmath::Deg(90_f32).into());
        //let projeted_grid_mesh = ProjetedGrid::new(cgmath::Deg(10_f32).into(), cgmath::Deg(10_f32).into(), Some(lat_bound), Some(lon_bound), &projection.get(), &viewport.borrow());
        let projeted_grid_mesh = ProjetedGrid::new(&gl, cgmath::Deg(30_f32).into(), cgmath::Deg(30_f32).into(), Some(lat_bound), None, &projection, &viewport);
        let grid = Renderable::<ProjetedGrid>::new(
            &gl,
            &shaders["grid"],
            projeted_grid_mesh,
        );

        // Catalog definition
        let catalog_mesh = Catalog::new(&gl, vec![]);
        let catalog = Renderable::<Catalog>::new(
            &gl,
            &shaders["catalog"],
            catalog_mesh
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

        let move_event = None;
        let inertia = None;
        let gl = gl.clone();
        let app = App {
            gl,

            shaders,

            viewport,
            projection,

            // The sphere renderable
            hips_sphere,
            // The grid renderable
            grid,
            // The catalog renderable
            catalog,

            move_event,

            inertia,
        };

        Ok(app)
    } 

    fn update(&mut self, dt: f32) {
        // Update the camera. When the camera has reached its final position
        // then we stop rendering the next frames!
        self.viewport.update(&self.hips_sphere.get_model_mat(), &self.projection, dt);
        
        if !UPDATE_USER_INTERFACE.load(Ordering::Relaxed) {
            RENDER_FRAME.lock().unwrap().update(&self.viewport);
            //UPDATE_FRAME.lock().unwrap().update(&self.viewport);
        } else {
            UPDATE_USER_INTERFACE.store(false, Ordering::Relaxed);
        }
        // Updating
        if RENDER_FRAME.lock().unwrap().get() {
            console::log_1(&format!("update").into());
            // Look for inertia
            if let Some(ref mut inertia) = &mut self.inertia {
                if inertia.update(
                    &mut self.hips_sphere,
                    &mut self.grid,
                    &mut self.catalog,
                    &mut self.viewport,
                    dt
                ) {
                    self.inertia = None;
                }
            }

            // Update the fov
            self.hips_sphere
                .update(
                    &self.projection,
                    &self.viewport,
                );


            // Update the catalog vizualization
            self.catalog.update(&self.projection, &self.viewport);

            // The grid buffers and labels
            if *ENABLED_WIDGETS.lock().unwrap().get("grid").unwrap() {
                self.grid.update(
                    &self.projection,
                    &self.viewport
                );
            }
        }
    }

    fn render(&self) {
        if RENDER_FRAME.lock().unwrap().get() {
            console::log_1(&format!("render").into());

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
            if *ENABLED_WIDGETS.lock().unwrap().get("grid").unwrap() {
                // The grid lines
                self.grid.draw(
                    shaders,
                    viewport
                );
                // The labels
                self.grid.mesh().draw_labels();
            }
        }
    }

    fn set_projection(&mut self, projection: ProjectionType) {
        // Set the new projection
        self.projection = projection;

        // New HiPS sphere
        let hips_sphere_mesh = HiPSSphere::new(&self.gl, &projection);

        // Update the scissor for the new projection
        self.viewport.resize(&projection.size());

        self.hips_sphere.update_mesh(&self.shaders["hips_sphere"], hips_sphere_mesh);
    }

    fn reload_hips_sphere(&mut self, hips_url: String, hips_depth: u8) {
        *HIPS_NAME.lock().unwrap() = hips_url;
        MAX_DEPTH.store(hips_depth, Ordering::Relaxed);

        // Re-initialize the color buffers
        self.hips_sphere.mesh_mut().refresh_buffer_tiles();

        // Tell the viewport the maximum zoom level has changed
        // based on the new value of MAX_DEPTH
        let fov_max = math::depth_to_fov(MAX_DEPTH.load(Ordering::Relaxed));
        self.viewport.set_max_field_of_view(fov_max);
    }

    /// MOVE EVENT
    fn initialize_move(&mut self, screen_pos: Vector2<f32>) {
        // stop inertia if there is one
        self.inertia = None;
        self.viewport.stop_inertia();

        let ref projection = self.projection;
        self.move_event = Move::new(screen_pos, projection, &self.viewport);
    }

    fn moves(&mut self, screen_pos: Vector2<f32>) {
        if let Some(ref mut move_event) = self.move_event {
            move_event.update(
                &screen_pos,
                
                &mut self.hips_sphere,
                &mut self.grid,
                &mut self.catalog,
                
                &self.projection,
                
                &mut self.viewport
            );
        }
    }

    fn stop_move(&mut self, screen_pos: Vector2<f32>) {
        if let Some(ref mut move_event) = self.move_event {
            console::log_1(&format!("stop moving").into());
            self.viewport.stop_displacement();

            if ENABLE_INERTIA.load(Ordering::Relaxed) {
                // Do not perform the inertia effect if the released
                // position of the mouse is outside the projection
                let ref projection = self.projection;
                let mouse_pos_world = screen_to_world_space(&screen_pos, projection, &self.viewport);
                if mouse_pos_world.is_some(){
                    self.inertia = MouseInertia::new(
                        move_event,
                        &mut self.viewport
                    );
                }
            }

            self.move_event = None;
        }
    }

    // ZOOM EVENT
    fn zoom(&mut self, delta_y: f32) {
        if delta_y < 0_f32 {
            self.viewport.zoom(-delta_y);
        } else {
            self.viewport.unzoom(delta_y);
        }
    }

    fn add_catalog(&mut self, sources: Vec<Source>) {
        let catalog_mesh = Catalog::new(&self.gl, sources);
        self.catalog.update_mesh(&self.shaders["catalog"], catalog_mesh);
    }
}

static DEGRADE_CANVAS_RATIO: f32 = 1.0_f32;

lazy_static! {
    static ref WIDTH_SCREEN: Arc<AtomicU32> = Arc::new(
        AtomicU32::new(
            /*web_sys::window().unwrap()
                .document().unwrap()
                .get_element_by_id("canvas").unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>().unwrap()
                .width() as u32*/
            //1024
            (web_sys::window().unwrap().inner_width()
                .unwrap()
                .as_f64()
                .unwrap() / (DEGRADE_CANVAS_RATIO as f64)) as u32
            //512 as u32
        )
    );
    static ref HEIGHT_SCREEN: Arc<AtomicU32> = Arc::new(
        AtomicU32::new(
            /*web_sys::window().unwrap()
                .document().unwrap()
                .get_element_by_id("canvas").unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>().unwrap()
                .height() as u32*/
            //768
            (web_sys::window().unwrap().inner_height()
                .unwrap()
                .as_f64()
                .unwrap() / (DEGRADE_CANVAS_RATIO as f64)) as u32
            //512 as u32
        )
    );
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

fn set_window_size(new_width: u32, new_height: u32) {
    WIDTH_SCREEN.store(new_width, Ordering::Relaxed);
    HEIGHT_SCREEN.store(new_height, Ordering::Relaxed);
}
fn window_size_u32() -> (u32, u32) {
    let width = WIDTH_SCREEN.load(Ordering::Relaxed);
    let height = HEIGHT_SCREEN.load(Ordering::Relaxed);

    (width, height)
}
fn window_size_f32() -> (f32, f32) {
    let width = WIDTH_SCREEN.load(Ordering::Relaxed);
    let height = HEIGHT_SCREEN.load(Ordering::Relaxed);

    (width as f32, height as f32)
}
fn window_size_f64() -> (f64, f64) {
    let width = WIDTH_SCREEN.load(Ordering::Relaxed);
    let height = HEIGHT_SCREEN.load(Ordering::Relaxed);

    (width as f64, height as f64)
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

#[wasm_bindgen]
pub struct WebClient {
    app: App,
}

#[wasm_bindgen]
impl WebClient {
    /// Create a new web client
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        let gl = WebGl2Context::new();

        let app = App::new(&gl).unwrap();

        WebClient {
            app,
        }
    }

    /// Update our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn update(&mut self, dt: f32) -> Result<(), JsValue> {
        self.app.update(dt);

        Ok(())
    }

    /// Update our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn render(&self) -> Result<(), JsValue> {
        self.app.render();

        Ok(())
    }

    /// Change the current projection of the HiPS
    pub fn set_projection(&mut self, name: String) -> Result<(), JsValue> {
        match name.as_ref() {
            "aitoff" => {
                self.app.set_projection(ProjectionType::Aitoff(Aitoff {}));
            },
            "orthographic" => {
                self.app.set_projection(ProjectionType::Orthographic(Orthographic {}));
            },
            "mollweide" => {
                self.app.set_projection(ProjectionType::MollWeide(MollWeide {}));
            },
            _ => {}
        }

        RENDER_FRAME.lock().unwrap().set(true);
        //UPDATE_FRAME.lock().unwrap().set(true);

        // This is a UI update so we tell the rendering loop
        // we do not want to update the update and render frame
        // bools
        UPDATE_USER_INTERFACE.store(true, Ordering::Relaxed);

        Ok(())
    }

    /// Enable equatorial grid
    pub fn enable_equatorial_grid(&mut self) -> Result<(), JsValue> {
        //self.app.grid.borrow_mut().enable();
        if let Some(grid) = ENABLED_WIDGETS.lock().unwrap().get_mut("grid") {
            *grid = true;
            self.app.grid
                .update(
                    &self.app.projection,
                    &self.app.viewport
                );
        }
        RENDER_FRAME.lock().unwrap().set(true);
        UPDATE_USER_INTERFACE.store(true, Ordering::Relaxed);

        Ok(())
    }

    /// Disable equatorial grid
    pub fn disable_equatorial_grid(&mut self) -> Result<(), JsValue> {
        if let Some(grid) = ENABLED_WIDGETS.lock().unwrap().get_mut("grid") {
            *grid = false;
            self.app.grid.mesh_mut().clear_canvas();
        }
        RENDER_FRAME.lock().unwrap().set(true);
        UPDATE_USER_INTERFACE.store(true, Ordering::Relaxed);

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
        RENDER_FRAME.lock().unwrap().set(true);
        UPDATE_USER_INTERFACE.store(true, Ordering::Relaxed);

        Ok(())
    }

    /// Change grid opacity
    pub fn change_grid_opacity(&mut self, alpha: f32) -> Result<(), JsValue> {
        self.app.grid
            .mesh_mut()
            .set_alpha(alpha);

        RENDER_FRAME.lock().unwrap().set(true);
        UPDATE_USER_INTERFACE.store(true, Ordering::Relaxed);

        Ok(())
    }
    /// Change grid opacity
    pub fn set_catalog_opacity(&mut self, alpha: f32) -> Result<(), JsValue> {
        self.app.catalog
            .mesh_mut()
            .set_alpha(alpha);

        RENDER_FRAME.lock().unwrap().set(true);
        UPDATE_USER_INTERFACE.store(true, Ordering::Relaxed);

        Ok(())
    }
    /// Change grid opacity
    pub fn set_kernel_strength(&mut self, strength: f32) -> Result<(), JsValue> {
        self.app.catalog
            .mesh_mut()
            .set_kernel_strength(strength);

        RENDER_FRAME.lock().unwrap().set(true);
        UPDATE_USER_INTERFACE.store(true, Ordering::Relaxed);

        Ok(())
    }

    /// Change HiPS
    pub fn change_hips(&mut self, hips_url: String, hips_depth: i32) -> Result<(), JsValue> {
        self.app.reload_hips_sphere(hips_url, hips_depth as u8);

        RENDER_FRAME.lock().unwrap().set(true);
        //UPDATE_FRAME.lock().unwrap().set(true);

        UPDATE_USER_INTERFACE.store(true, Ordering::Relaxed);

        Ok(())
    }

    /// Start move
    pub fn initialize_move(&mut self, screen_pos_x: f32, screen_pos_y: f32) -> Result<(), JsValue> {
        let screen_pos = Vector2::new(screen_pos_x, screen_pos_y);
        self.app.initialize_move(screen_pos);
        Ok(())
    }
    /// Stop move
    pub fn stop_move(&mut self, screen_pos_x: f32, screen_pos_y: f32) -> Result<(), JsValue> {
        let screen_pos = Vector2::new(screen_pos_x, screen_pos_y);
        self.app.stop_move(screen_pos);
        Ok(())
    }
    /// Keep moving
    pub fn moves(&mut self, screen_pos_x: f32, screen_pos_y: f32) -> Result<(), JsValue> {
        let screen_pos = Vector2::new(screen_pos_x, screen_pos_y);
        self.app.moves(screen_pos);
        Ok(())
    }

    /// Wheel event
    pub fn zoom(&mut self, delta_y: f32) -> Result<(), JsValue> {
        self.app.zoom(delta_y);

        Ok(())
    }

    /// Add new catalog
    pub fn add_catalog(&mut self, data: &JsValue) -> Result<(), JsValue> {
        let data: Vec<[f32; 3]> = data.into_serde().unwrap();

        let sources: Vec<Source> = data.into_iter()
            .map(|ref source| {
                (source as &[f32]).into()
            })
            .collect::<Vec<_>>();

        self.app.add_catalog(sources);

        Ok(())
    }
}

