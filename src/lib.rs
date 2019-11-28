#[macro_use]
extern crate lazy_static;
extern crate itertools_num;

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
use cgmath::{InnerSpace, Vector3, Vector4, Rad};

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

use shader::Shader;
use renderable::Renderable;
use renderable::hips_sphere::HiPSSphere;
use renderable::projection;
use renderable::projection::ProjectionType;
use renderable::projection::{Aitoff, Orthographic, MollWeide};
use viewport::ViewPort;

use crate::renderable::grid::ProjetedGrid;

use std::sync::Mutex;
use std::sync::Arc;

use std::rc::Rc;
use std::cell::{RefCell, Cell};

use std::collections::HashMap;

use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};
use crate::render_next_frame::RENDER_NEXT_FRAME;

use crate::event::Move;
struct App {
    gl: WebGl2Context,

    shaders: HashMap<&'static str, Shader>,

    viewport: Rc<RefCell<ViewPort>>,
    projection: Rc<Cell<ProjectionType>>,

    // The sphere renderable
    hips_sphere: Renderable<HiPSSphere>,
    // The grid renderable
    grid: Rc<RefCell<Renderable<ProjetedGrid>>>,

    // Move event
    move_event: Option<Move>,
}

use cgmath::Vector2;
impl App {
    fn new(gl: &WebGl2Context) -> Result<App, JsValue> {
        // Shader definition
        let shader_2d_proj = Shader::new(&gl,
            shaders::proj_vert::CONTENT,
            shaders::proj_frag::CONTENT,
            // uniform list
            [
                // General uniforms
                "current_time",
                "model",
                // Viewport uniforms
                "zoom_factor",
                "aspect",
                "last_zoom_action",
                // HiPS Sphere-specific uniforms
                "current_depth",
                "max_depth",
                //"ang2pix_0_texture",
                //"ang2pix_1_texture",
                //"ang2pix_2_texture",
                "textures_0",
                "textures",
                "textures_0_tiles[0].uniq",
                "textures_0_tiles[0].texture_idx",
                "textures_0_tiles[0].time_received",
                "textures_0_tiles[0].time_request",
                "textures_0_tiles[1].uniq",
                "textures_0_tiles[1].texture_idx",
                "textures_0_tiles[1].time_received",
                "textures_0_tiles[1].time_request",
                "textures_0_tiles[2].uniq",
                "textures_0_tiles[2].texture_idx",
                "textures_0_tiles[2].time_received",
                "textures_0_tiles[2].time_request",
                "textures_0_tiles[3].uniq",
                "textures_0_tiles[3].texture_idx",
                "textures_0_tiles[3].time_received",
                "textures_0_tiles[3].time_request",
                "textures_0_tiles[4].uniq",
                "textures_0_tiles[4].texture_idx",
                "textures_0_tiles[4].time_received",
                "textures_0_tiles[4].time_request",
                "textures_0_tiles[5].uniq",
                "textures_0_tiles[5].texture_idx",
                "textures_0_tiles[5].time_received",
                "textures_0_tiles[5].time_request",
                "textures_0_tiles[6].uniq",
                "textures_0_tiles[6].texture_idx",
                "textures_0_tiles[6].time_received",
                "textures_0_tiles[6].time_request",
                "textures_0_tiles[7].uniq",
                "textures_0_tiles[7].texture_idx",
                "textures_0_tiles[7].time_received",
                "textures_0_tiles[7].time_request",
                "textures_0_tiles[8].uniq",
                "textures_0_tiles[8].texture_idx",
                "textures_0_tiles[8].time_received",
                "textures_0_tiles[8].time_request",
                "textures_0_tiles[9].uniq",
                "textures_0_tiles[9].texture_idx",
                "textures_0_tiles[9].time_received",
                "textures_0_tiles[9].time_request",
                "textures_0_tiles[10].uniq",
                "textures_0_tiles[10].texture_idx",
                "textures_0_tiles[10].time_received",
                "textures_0_tiles[10].time_request",
                "textures_0_tiles[11].uniq",
                "textures_0_tiles[11].texture_idx",
                "textures_0_tiles[11].time_received",
                "textures_0_tiles[11].time_request",
                "textures_tiles[0].uniq",
                "textures_tiles[0].texture_idx",
                "textures_tiles[0].time_received",
                "textures_tiles[0].time_request",
                "textures_tiles[1].uniq",
                "textures_tiles[1].texture_idx",
                "textures_tiles[1].time_received",
                "textures_tiles[1].time_request",
                "textures_tiles[2].uniq",
                "textures_tiles[2].texture_idx",
                "textures_tiles[2].time_received",
                "textures_tiles[2].time_request",
                "textures_tiles[3].uniq",
                "textures_tiles[3].texture_idx",
                "textures_tiles[3].time_received",
                "textures_tiles[3].time_request",
                "textures_tiles[4].uniq",
                "textures_tiles[4].texture_idx",
                "textures_tiles[4].time_received",
                "textures_tiles[4].time_request",
                "textures_tiles[5].uniq",
                "textures_tiles[5].texture_idx",
                "textures_tiles[5].time_received",
                "textures_tiles[5].time_request",
                "textures_tiles[6].uniq",
                "textures_tiles[6].texture_idx",
                "textures_tiles[6].time_received",
                "textures_tiles[6].time_request",
                "textures_tiles[7].uniq",
                "textures_tiles[7].texture_idx",
                "textures_tiles[7].time_received",
                "textures_tiles[7].time_request",
                "textures_tiles[8].uniq",
                "textures_tiles[8].texture_idx",
                "textures_tiles[8].time_received",
                "textures_tiles[8].time_request",
                "textures_tiles[9].uniq",
                "textures_tiles[9].texture_idx",
                "textures_tiles[9].time_received",
                "textures_tiles[9].time_request",
                "textures_tiles[10].uniq",
                "textures_tiles[10].texture_idx",
                "textures_tiles[10].time_received",
                "textures_tiles[10].time_request",
                "textures_tiles[11].uniq",
                "textures_tiles[11].texture_idx",
                "textures_tiles[11].time_received",
                "textures_tiles[11].time_request",
                "textures_tiles[12].uniq",
                "textures_tiles[12].texture_idx",
                "textures_tiles[12].time_received",
                "textures_tiles[12].time_request",
                "textures_tiles[13].uniq",
                "textures_tiles[13].texture_idx",
                "textures_tiles[13].time_received",
                "textures_tiles[13].time_request",
                "textures_tiles[14].uniq",
                "textures_tiles[14].texture_idx",
                "textures_tiles[14].time_received",
                "textures_tiles[14].time_request",
                "textures_tiles[15].uniq",
                "textures_tiles[15].texture_idx",
                "textures_tiles[15].time_received",
                "textures_tiles[15].time_request",
                "textures_tiles[16].uniq",
                "textures_tiles[16].texture_idx",
                "textures_tiles[16].time_received",
                "textures_tiles[16].time_request",
                "textures_tiles[17].uniq",
                "textures_tiles[17].texture_idx",
                "textures_tiles[17].time_received",
                "textures_tiles[17].time_request",
                "textures_tiles[18].uniq",
                "textures_tiles[18].texture_idx",
                "textures_tiles[18].time_received",
                "textures_tiles[18].time_request",
                "textures_tiles[19].uniq",
                "textures_tiles[19].texture_idx",
                "textures_tiles[19].time_received",
                "textures_tiles[19].time_request",
            ].as_ref()
        );
        console::log_1(&format!("lskdf").into());
        let shader_grid = Shader::new(&gl,
            shaders::grid_projeted_vert::CONTENT,
            shaders::grid_frag::CONTENT,
            [
                // General uniforms
                "current_time",
                "model",
                // Viewport uniforms
                "zoom_factor",
                "aspect",
                "last_zoom_action",
                // Grid-specific uniforms
                "location_color",
            ].as_ref()
        );
        let mut shaders = HashMap::new();
        shaders.insert("hips_sphere", shader_2d_proj);
        shaders.insert("grid", shader_grid);

        gl.enable(WebGl2RenderingContext::BLEND);
        gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        gl.enable(WebGl2RenderingContext::SCISSOR_TEST);

        gl.clear_color(0.08, 0.08, 0.08, 1.0);

        // Projection definition
        let projection = Rc::new(Cell::new(ProjectionType::Aitoff(Aitoff {})));
        // HiPS Sphere definition
        let hips_sphere_mesh = HiPSSphere::new(&gl, &projection.get());
        let size_pixels = *hips_sphere_mesh.get_default_pixel_size();
        let hips_sphere = Renderable::<HiPSSphere>::new(
            &gl,
            &shaders["hips_sphere"],
            hips_sphere_mesh,
        );

        // Viewport definition
        let viewport = Rc::new(RefCell::new(ViewPort::new(&gl, &size_pixels)));

        // Grid definition
        let lon_bound = cgmath::Vector2::<cgmath::Rad<f32>>::new(cgmath::Deg(-30_f32).into(), cgmath::Deg(30_f32).into());
        let lat_bound = cgmath::Vector2::<cgmath::Rad<f32>>::new(cgmath::Deg(-90_f32).into(), cgmath::Deg(90_f32).into());
        //let projeted_grid_mesh = ProjetedGrid::new(cgmath::Deg(10_f32).into(), cgmath::Deg(10_f32).into(), Some(lat_bound), Some(lon_bound), &projection.get(), &viewport.borrow());
        let projeted_grid_mesh = ProjetedGrid::new(cgmath::Deg(30_f32).into(), cgmath::Deg(30_f32).into(), Some(lat_bound), None, &projection.get(), &viewport.borrow());
        let grid = Rc::new(RefCell::new(Renderable::<ProjetedGrid>::new(
            &gl,
            &shaders["grid"],
            projeted_grid_mesh,
        )));

        //let render_next_frame = Rc::new(Cell::new(true));
        
        // Mouse down pression event
        /*let pressed = Rc::new(Cell::new(false));

        let mouse_clic_x = Rc::new(Cell::new(0_f32));
        let mouse_clic_y = Rc::new(Cell::new(0_f32));

        let start_pos = Rc::new(Cell::new(Vector4::<f32>::new(0_f32, 0_f32, 0_f32, 1_f32)));

        let last_axis = Rc::new(Cell::new(Vector3::<f32>::new(0_f32, 0_f32, 0_f32)));
        let last_dist = Rc::new(Cell::new(Rad(0_f32)));
        let roll = Rc::new(Cell::new(false));

        let time_last_move = Rc::new(Cell::new(utils::get_current_time() as f32));

        {
            let pressed = pressed.clone();

            let mouse_clic_x = mouse_clic_x.clone();
            let mouse_clic_y = mouse_clic_y.clone();

            let start_pos = start_pos.clone();
            let projection = projection.clone();

            let viewport = viewport.clone();

            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                console::log_1(&format!("mouse down").into());
                let event_x = event.offset_x() as f32;
                let event_y = event.offset_y() as f32;

                mouse_clic_x.set(event_x);
                mouse_clic_y.set(event_y);

                let screen_pos = Vector2::new(event_x, event_y);
                let homogeneous_pos = projection::screen_pixels_to_homogenous(&screen_pos, &viewport.borrow());
                let result = projection.get().screen_to_world_space(&homogeneous_pos);
                if let Some(pos) = result {
                    let pos = pos.normalize();

                    start_pos.set(pos);

                    pressed.set(true);
                }
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        */
        // Resize event
        let onresize = {
            let viewport = viewport.clone();
            //let render_next_frame = render_next_frame.clone();

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
        //canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        onresize.forget();

        // Mouse up pression event
        /*
        {
            let pressed = pressed.clone();
            let viewport = viewport.clone();

            let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                console::log_1(&format!("mouse up").into());
                pressed.set(false);
                viewport.borrow_mut().stop_displacement();
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        */

        // Mouse move event
        /*
        {
            let pressed = pressed.clone();

            let start_pos = start_pos.clone();
            let hips_sphere = hips_sphere.clone();

            let grid = grid.clone();

            let mouse_clic_x = mouse_clic_x.clone();
            let mouse_clic_y = mouse_clic_y.clone();

            let last_axis = last_axis.clone();
            let last_dist = last_dist.clone();

            let viewport = viewport.clone();
            let projection = projection.clone();

            let time_last_move = time_last_move.clone();
            let roll = roll.clone();
            //let render_next_frame = render_next_frame.clone();

            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                if pressed.get() {
                    let event_x = event.offset_x() as f32;
                    let event_y = event.offset_y() as f32;

                    let screen_pos = Vector2::new(event_x, event_y);
                    let homogeneous_pos = projection::screen_pixels_to_homogenous(&screen_pos, &viewport.borrow());
                    let result = projection.get().screen_to_world_space(&homogeneous_pos);
                    if let Some(pos) = result {
                        if event_x != mouse_clic_x.get() || event_y != mouse_clic_y.get() {
                            let start_pos_rotated = hips_sphere.as_ref().borrow().get_model_mat() * start_pos.get();
                            let start_pos_rotated = cgmath::Vector3::<f32>::new(start_pos_rotated.x, start_pos_rotated.y, start_pos_rotated.z);
                            
                            let pos_rotated = hips_sphere.as_ref().borrow().get_model_mat() * pos;
                            let pos_rotated = cgmath::Vector3::<f32>::new(pos_rotated.x, pos_rotated.y, pos_rotated.z);

                            let mut axis = start_pos_rotated.cross(pos_rotated);
                            let dist = math::angular_distance_xyz(start_pos_rotated, pos_rotated);

                            axis = axis.normalize();
                            hips_sphere.borrow_mut().apply_rotation(-axis, dist);
                            // Move the grid the opposite way of the hips sphere
                            grid.borrow_mut().set_model_mat(hips_sphere.borrow().get_inverted_model_mat());

                            if *ENABLED_WIDGETS.lock().unwrap().get("grid").unwrap() {
                                grid.borrow_mut()
                                    .update(
                                        &projection.get(),
                                        &viewport.borrow()
                                    );
                            }

                            time_last_move.set(utils::get_current_time() as f32);
                            roll.set(true);

                            mouse_clic_x.set(event_x);
                            mouse_clic_y.set(event_y);

                            start_pos.set(pos);

                            last_axis.set(axis);
                            last_dist.set(dist);

                            viewport.borrow_mut().displacement();
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);

            canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        */
        // Mouse wheel event
        /*{
            let grid = grid.clone();

            let viewport = viewport.clone();
            let projection = projection.clone();

            let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
                let delta_y = event.delta_y() as f32;

                let dt = *DELTA_TIME.lock().unwrap();
                if delta_y < 0_f32 {
                    viewport.borrow_mut().zoom(-delta_y);
                } else {
                    viewport.borrow_mut().unzoom(delta_y);
                }

                if *ENABLED_WIDGETS.lock().unwrap().get("grid").unwrap() {
                    grid.borrow_mut()
                        .update(
                            &projection.get(),
                            &viewport.borrow()
                        );
                }
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }*/
        let move_event = None;
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

            move_event,
        };

        Ok(app)
    } 

    fn update(&mut self, dt: f32) {
        *DELTA_TIME.lock().unwrap() = dt;

        RENDER_NEXT_FRAME.lock().unwrap().update(&self.viewport.borrow());

        // Update the camera. When the camera has reached its final position
        // then we stop rendering the next frames!
        self.viewport.borrow_mut().update(&self.projection.get(), dt);

        // Updating
        if RENDER_NEXT_FRAME.lock().unwrap().get() {
            // update the fov
            self.hips_sphere
                .update(
                    &self.projection.get(),
                    &self.viewport.borrow(),
                );

            // The grid label positions
            if *ENABLED_WIDGETS.lock().unwrap().get("grid").unwrap() {
                self.grid.borrow_mut().mesh_mut().update_label_positions(
                    self.hips_sphere.get_inverted_model_mat(),
                    &self.projection.get(),
                    Some(&self.viewport.borrow()),
                );
            }
        }
    }

    fn render(&self) {
        if RENDER_NEXT_FRAME.lock().unwrap().get() {
            //RENDER_NEXT_FRAME.store(false, Ordering::Relaxed);
            /*if !pressed.get() && roll.get() {
                let next_dist = compute_speed(time_last_move.get(), last_dist.get() * 0.5_f32);
                if next_dist > 1e-4 {
                    sphere.borrow_mut().apply_rotation(-last_axis.get(), cgmath::Rad(next_dist));
                    projeted_grid.borrow_mut().apply_rotation(-last_axis.get(), cgmath::Rad(next_dist));

                    let model_mat = &sphere.as_ref().borrow().get_model_mat();
                    hips_sphere_mesh.borrow_mut().update_field_of_view(gl.clone(), &projection.as_ref().borrow(), &viewport.borrow(), model_mat, false);
                }
            } else if pressed.get() {
                if (utils::get_current_time() as f32) - time_last_move.get() > 50_f32 {
                    roll.set(false);
                }
            }*/

            // Render the scene
            self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

            // Draw renderables here
            let ref viewport = self.viewport.as_ref().borrow();
            let ref shaders = self.shaders;

            // Draw the HiPS sphere
            self.hips_sphere.draw(
                &shaders["hips_sphere"],
                WebGl2RenderingContext::TRIANGLES,
                viewport
            );

            // Draw the grid
            if *ENABLED_WIDGETS.lock().unwrap().get("grid").unwrap() {
                // The grid lines
                self.grid.as_ref().borrow().draw(
                    &shaders["grid"],
                    WebGl2RenderingContext::LINES,
                    viewport
                );
                // The labels
                self.grid.borrow().mesh().draw_labels();
            }
        }
    }

    fn set_projection(&mut self, projection: ProjectionType) {
        self.projection.set(projection);
        let ref shaders = self.shaders;

        // New HiPS sphere
        let hips_sphere_mesh = HiPSSphere::new(&self.gl, &projection);

        // Update the scissor due to the projection change
        self.viewport.borrow_mut().resize(hips_sphere_mesh.get_default_pixel_size());

        self.hips_sphere = Renderable::<HiPSSphere>::new(
            &self.gl,
            &shaders["hips_sphere"],
            hips_sphere_mesh,
        );

        // New grid
        let lat_bound = cgmath::Vector2::<cgmath::Rad<f32>>::new(cgmath::Deg(-90_f32).into(), cgmath::Deg(90_f32).into());
        let projeted_grid_mesh = ProjetedGrid::new(cgmath::Deg(30_f32).into(), cgmath::Deg(30_f32).into(), Some(lat_bound), None, &projection, &self.viewport.borrow());
        self.grid.replace(
            Renderable::<ProjetedGrid>::new(
                &self.gl,
                &shaders["grid"],
                projeted_grid_mesh,
            )
        );
        RENDER_NEXT_FRAME.lock().unwrap().set(true);
    }

    fn reload_hips_sphere(&mut self, hips_url: String, hips_depth: u8) {
        *HIPS_NAME.lock().unwrap() = hips_url;
        MAX_DEPTH.store(hips_depth, Ordering::Relaxed);
        print_to_console!("MAX DEPTH NEW HIPS {:?}", hips_depth);

        let ref projection = self.projection.get();
        let hips_sphere_mesh = HiPSSphere::new(&self.gl, projection);
        let ref shaders = self.shaders;

        // New HiPS sphere
        self.hips_sphere = Renderable::<HiPSSphere>::new(
            &self.gl,
            &shaders["hips_sphere"],
            hips_sphere_mesh,
        );

        // New grid
        let lat_bound = cgmath::Vector2::<cgmath::Rad<f32>>::new(cgmath::Deg(-90_f32).into(), cgmath::Deg(90_f32).into());
        let projeted_grid_mesh = ProjetedGrid::new(cgmath::Deg(30_f32).into(), cgmath::Deg(30_f32).into(), Some(lat_bound), None, &projection, &self.viewport.borrow());
        self.grid.replace(
            Renderable::<ProjetedGrid>::new(
                &self.gl,
                &shaders["grid"],
                projeted_grid_mesh,
            )
        );

        // Tell the viewport the maximum zoom level has changed
        // based on the new value of MAX_DEPTH
        let fov_max = math::depth_to_fov(MAX_DEPTH.load(Ordering::Relaxed));
        self.viewport.borrow_mut().set_max_field_of_view(fov_max);

        RENDER_NEXT_FRAME.lock().unwrap().set(true);
    }

    /// MOVE EVENT
    fn initialize_move(&mut self, screen_pos: Vector2<f32>) {
        let ref projection = self.projection.get();
        self.move_event = Move::new(screen_pos, projection, &self.viewport.borrow());
    }

    fn moves(&mut self, screen_pos: Vector2<f32>) {
        if let Some(ref mut move_event) = self.move_event {
            move_event.update(
                &screen_pos,
                
                &mut self.hips_sphere,
                &mut self.grid.borrow_mut(),
                
                &self.projection.get(),
                
                &mut self.viewport.borrow_mut()
            );
        }
    }

    fn stop_move(&mut self) {
        self.viewport.borrow_mut().stop_displacement();

        self.move_event = None;
    }

    // ZOOM EVENT
    fn zoom(&mut self, delta_y: f32) {
        if delta_y < 0_f32 {
            self.viewport.borrow_mut().zoom(-delta_y);
        } else {
            self.viewport.borrow_mut().unzoom(delta_y);
        }

        if *ENABLED_WIDGETS.lock().unwrap().get("grid").unwrap() {
            self.grid.borrow_mut()
                .update(
                    &self.projection.get(),
                    &self.viewport.borrow()
                );
        }
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

    static ref HIPS_NAME: Arc<Mutex<String>> = Arc::new(Mutex::new(String::from("http://alasky.u-strasbg.fr/DSS/DSSColor")));
    static ref MAX_DEPTH: Arc<AtomicU8> = Arc::new(AtomicU8::new(9));
    static ref DELTA_TIME: Arc<Mutex<f32>> = Arc::new(Mutex::new(0_f32));
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

        //let context_options = js_sys::JSON::parse(&"{\"antialias\":true}").unwrap();

        /*let inner = Rc::new(
            canvas.get_context_with_context_options("webgl2", context_options.as_ref())
                .unwrap()
                .unwrap()
                .dyn_into::<WebGl2RenderingContext>()
                .unwrap()
        );*/
        let inner = Rc::new(
            canvas.get_context("webgl2")
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

        Ok(())
    }

    /// Enable equatorial grid
    pub fn enable_equatorial_grid(&mut self) -> Result<(), JsValue> {
        //self.app.grid.borrow_mut().enable();
        if let Some(grid) = ENABLED_WIDGETS.lock().unwrap().get_mut("grid") {
            *grid = true;
            self.app.grid.borrow_mut()
                .update(
                    &self.app.projection.get(),
                    &self.app.viewport.borrow()
                );
        }
        RENDER_NEXT_FRAME.lock().unwrap().set(true);

        Ok(())
    }

    /// Disable equatorial grid
    pub fn disable_equatorial_grid(&mut self) -> Result<(), JsValue> {
        if let Some(grid) = ENABLED_WIDGETS.lock().unwrap().get_mut("grid") {
            *grid = false;
            self.app.grid.borrow_mut().mesh_mut().clear_canvas();
        }
        RENDER_NEXT_FRAME.lock().unwrap().set(true);

        Ok(())
    }

    /// Change HiPS
    pub fn change_hips(&mut self, hips_url: String, hips_depth: i32) -> Result<(), JsValue> {
        self.app.reload_hips_sphere(hips_url, hips_depth as u8);

        Ok(())
    }

    /// Start move
    pub fn initialize_move(&mut self, screen_pos_x: f32, screen_pos_y: f32) -> Result<(), JsValue> {
        let screen_pos = Vector2::new(screen_pos_x, screen_pos_y);
        self.app.initialize_move(screen_pos);
        Ok(())
    }
    /// Stop move
    pub fn stop_move(&mut self) -> Result<(), JsValue> {
        self.app.stop_move();
        Ok(())
    }
    /// Keep moving
    pub fn moves(&mut self, screen_pos_x: f32, screen_pos_y: f32) -> Result<(), JsValue> {
        let screen_pos = Vector2::new(screen_pos_x, screen_pos_y);
        self.app.moves(screen_pos);
        Ok(())
    }

    // Wheel event
    pub fn zoom(&mut self, delta_y: f32) -> Result<(), JsValue> {
        self.app.zoom(delta_y);

        Ok(())
    }
}

