#[macro_use]
extern crate lazy_static;

extern crate itertools_num;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, console, Window};

use cgmath;
use cgmath::{InnerSpace, Vector3, Vector4};

mod shader;
mod shaders;
mod renderable;
mod viewport;
mod texture;
mod math;
mod utils;
mod projeted_grid;

use shader::Shader;
use renderable::Renderable;
use renderable::hips_sphere::HiPSSphere;
use renderable::projection;
use renderable::projection::{ProjectionType, Aitoff, Orthographic};
use viewport::ViewPort;

use crate::renderable::grid::ProjetedGrid;

use std::sync::Mutex;
use std::sync::Arc;
use std::time::{Duration, Instant};

fn request_animation_frame(f: &Closure<FnMut()>, window: &Window) {
    window.request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

/*fn compute_speed(t_start: f32, speed_max: f32) -> f32 {
    let t = utils::get_current_time() as f32; 
    let t_duration = 1200_f32; // in ms
    let t_end = t_start + t_duration;

    if t > t_end {
        0_f32
    } else {
        let speed = (-t*speed_max / t_duration) + t_end*speed_max/t_duration;
        speed
    }
}*/

use std::rc::Rc;
use std::cell::{RefCell, Cell};

use std::collections::HashMap;

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use crate::renderable::Mesh;

struct App {
    gl: WebGl2Context,

    viewport: Rc<RefCell<ViewPort>>,
    projection: Rc<Cell<ProjectionType>>,

    shaders: HashMap<&'static str, Rc<Shader>>,

    // The sphere renderable
    hips_sphere: Rc<RefCell<Renderable<HiPSSphere>>>,
    // The grid renderable
    grid: Rc<RefCell<Renderable<ProjetedGrid>>>,
}

impl App {
    fn new(gl: &WebGl2Context) -> Result<App, JsValue> {
        // Used in resize callback closure => Rc needed
        let canvas = Rc::new(
            gl.canvas().unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>().unwrap()
        );

        let d = crate::math::signed_distance_ellipse(&cgmath::Vector2::new(1.1_f32, 0.5_f32), 1_f32, 0.5_f32);
        console::log_1(&format!("DDD, {:?}", d).into());
        
        gl.enable(WebGl2RenderingContext::BLEND);
        gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        gl.enable(WebGl2RenderingContext::SCISSOR_TEST);

        // Shader definition
        let shader_2d_proj = Rc::new(Shader::new(&gl,
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
                // HiPS Sphere-specific uniforms
                "current_depth",
                "max_depth",
                "ang2pix_0_texture",
                "ang2pix_1_texture",
                "ang2pix_2_texture",
                "textures_0",
                "hpx_zero_depth[0].uniq",
                "hpx_zero_depth[0].texture_idx",
                "hpx_zero_depth[0].time_received",
                "hpx_zero_depth[0].time_request",
                "hpx_zero_depth[1].uniq",
                "hpx_zero_depth[1].texture_idx",
                "hpx_zero_depth[1].time_received",
                "hpx_zero_depth[1].time_request",
                "hpx_zero_depth[2].uniq",
                "hpx_zero_depth[2].texture_idx",
                "hpx_zero_depth[2].time_received",
                "hpx_zero_depth[2].time_request",
                "hpx_zero_depth[3].uniq",
                "hpx_zero_depth[3].texture_idx",
                "hpx_zero_depth[3].time_received",
                "hpx_zero_depth[3].time_request",
                "hpx_zero_depth[4].uniq",
                "hpx_zero_depth[4].texture_idx",
                "hpx_zero_depth[4].time_received",
                "hpx_zero_depth[4].time_request",
                "hpx_zero_depth[5].uniq",
                "hpx_zero_depth[5].texture_idx",
                "hpx_zero_depth[5].time_received",
                "hpx_zero_depth[5].time_request",
                "hpx_zero_depth[6].uniq",
                "hpx_zero_depth[6].texture_idx",
                "hpx_zero_depth[6].time_received",
                "hpx_zero_depth[6].time_request",
                "hpx_zero_depth[7].uniq",
                "hpx_zero_depth[7].texture_idx",
                "hpx_zero_depth[7].time_received",
                "hpx_zero_depth[7].time_request",
                "hpx_zero_depth[8].uniq",
                "hpx_zero_depth[8].texture_idx",
                "hpx_zero_depth[8].time_received",
                "hpx_zero_depth[8].time_request",
                "hpx_zero_depth[9].uniq",
                "hpx_zero_depth[9].texture_idx",
                "hpx_zero_depth[9].time_received",
                "hpx_zero_depth[9].time_request",
                "hpx_zero_depth[10].uniq",
                "hpx_zero_depth[10].texture_idx",
                "hpx_zero_depth[10].time_received",
                "hpx_zero_depth[10].time_request",
                "hpx_zero_depth[11].uniq",
                "hpx_zero_depth[11].texture_idx",
                "hpx_zero_depth[11].time_received",
                "hpx_zero_depth[11].time_request",
                "hpx_zero_depth[12].uniq",
                "hpx_zero_depth[12].texture_idx",
                "hpx_zero_depth[12].time_received",
                "hpx_zero_depth[12].time_request",
                "hpx_zero_depth[13].uniq",
                "hpx_zero_depth[13].texture_idx",
                "hpx_zero_depth[13].time_received",
                "hpx_zero_depth[13].time_request",
                "hpx_zero_depth[14].uniq",
                "hpx_zero_depth[14].texture_idx",
                "hpx_zero_depth[14].time_received",
                "hpx_zero_depth[14].time_request",
                "hpx_zero_depth[15].uniq",
                "hpx_zero_depth[15].texture_idx",
                "hpx_zero_depth[15].time_received",
                "hpx_zero_depth[15].time_request",
                "hpx_zero_depth[16].uniq",
                "hpx_zero_depth[16].texture_idx",
                "hpx_zero_depth[16].time_received",
                "hpx_zero_depth[16].time_request",
                "hpx_zero_depth[17].uniq",
                "hpx_zero_depth[17].texture_idx",
                "hpx_zero_depth[17].time_received",
                "hpx_zero_depth[17].time_request",
                "hpx_zero_depth[18].uniq",
                "hpx_zero_depth[18].texture_idx",
                "hpx_zero_depth[18].time_received",
                "hpx_zero_depth[18].time_request",
                "hpx_zero_depth[19].uniq",
                "hpx_zero_depth[19].texture_idx",
                "hpx_zero_depth[19].time_received",
                "hpx_zero_depth[19].time_request",
                "hpx_zero_depth[20].uniq",
                "hpx_zero_depth[20].texture_idx",
                "hpx_zero_depth[20].time_received",
                "hpx_zero_depth[20].time_request",
                "hpx_zero_depth[21].uniq",
                "hpx_zero_depth[21].texture_idx",
                "hpx_zero_depth[21].time_received",
                "hpx_zero_depth[21].time_request",
                "hpx_zero_depth[22].uniq",
                "hpx_zero_depth[22].texture_idx",
                "hpx_zero_depth[22].time_received",
                "hpx_zero_depth[22].time_request",
                "hpx_zero_depth[23].uniq",
                "hpx_zero_depth[23].texture_idx",
                "hpx_zero_depth[23].time_received",
                "hpx_zero_depth[23].time_request",
            ].as_ref()
        ));
        let shader_grid = Rc::new(Shader::new(&gl,
            shaders::grid_projeted_vert::CONTENT,
            shaders::grid_frag::CONTENT,
            [
                // General uniforms
                "current_time",
                "model",
                // Viewport uniforms
                "zoom_factor",
                "aspect",
                // Grid-specific uniforms
                "location_color",
            ].as_ref()
        ));
        let mut shaders = HashMap::new();
        shaders.insert("hips_sphere", shader_2d_proj);
        shaders.insert("grid", shader_grid);

        // Projection definition
        let projection = Rc::new(Cell::new(ProjectionType::Aitoff(Aitoff {})));
        // HiPS Sphere definition
        let hips_sphere_mesh = HiPSSphere::new(&gl, &projection.get());

        // Grid definition
        let lon_bound = cgmath::Vector2::<cgmath::Rad<f32>>::new(cgmath::Deg(-30_f32).into(), cgmath::Deg(30_f32).into());
        let lat_bound = cgmath::Vector2::<cgmath::Rad<f32>>::new(cgmath::Deg(-90_f32).into(), cgmath::Deg(90_f32).into());
        //let projeted_grid_mesh = ProjetedGrid::new(cgmath::Deg(10_f32).into(), cgmath::Deg(10_f32).into(), Some(lat_bound), Some(lon_bound), &projection.get(), &viewport.borrow());
        let projeted_grid_mesh = ProjetedGrid::new(cgmath::Deg(30_f32).into(), cgmath::Deg(30_f32).into(), Some(lat_bound), None, &projection.get());

        // Renderable definition
        let hips_sphere = Rc::new(RefCell::new(Renderable::<HiPSSphere>::new(
            &gl,
            shaders["hips_sphere"].clone(),
            hips_sphere_mesh,
        )));
        let grid = Rc::new(RefCell::new(Renderable::<ProjetedGrid>::new(
            &gl,
            shaders["grid"].clone(),
            projeted_grid_mesh,
        )));

        // Viewport
        let viewport = Rc::new(RefCell::new(ViewPort::new(&gl, hips_sphere.clone())));

        //let render_next_frame = Rc::new(Cell::new(true));
        // Mouse down pression event
        let pressed = Rc::new(Cell::new(false));

        let mouse_clic_x = Rc::new(Cell::new(0_f32));
        let mouse_clic_y = Rc::new(Cell::new(0_f32));

        let start_pos = Rc::new(Cell::new(Vector4::<f32>::new(0_f32, 0_f32, 0_f32, 1_f32)));

        let last_axis = Rc::new(Cell::new(Vector3::<f32>::new(0_f32, 0_f32, 0_f32)));
        let last_dist = Rc::new(Cell::new(0_f32));
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

                let (x_screen_homogeous_space, y_screen_homogeous_space) = projection::screen_pixels_to_homogenous(event_x, event_y, &viewport.borrow());
                let result = projection.get().screen_to_world_space(x_screen_homogeous_space, y_screen_homogeous_space);
                if let Some(pos) = result {
                    let pos = pos.normalize();

                    start_pos.set(pos);

                    pressed.set(true);
                }
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Resize event
        let onresize = {
            let viewport = viewport.clone();
            let hips_sphere = hips_sphere.clone();
            //let render_next_frame = render_next_frame.clone();

            Closure::wrap(Box::new(move || {
                console::log_1(&format!("resize").into());

                RENDER_NEXT_FRAME.store(true, Ordering::Relaxed);
                viewport.borrow_mut().resize();

                //hips_sphere
            }) as Box<dyn FnMut()>)
        };
        web_sys::window()
            .unwrap()
            .set_onresize(Some(onresize.as_ref().unchecked_ref()));
        //canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        onresize.forget();

        // Mouse up pression event
        {
            let pressed = pressed.clone();

            let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                console::log_1(&format!("mouse up").into());
                pressed.set(false);
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Mouse move event
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
                    let model_mat = &hips_sphere.as_ref().borrow().get_model_mat();

                    let (x_screen_homogeous_space, y_screen_homogeous_space) = projection::screen_pixels_to_homogenous(event_x, event_y, &viewport.borrow());
                    let result = projection.get().screen_to_world_space(x_screen_homogeous_space, y_screen_homogeous_space);
                    if let Some(pos) = result {
                        if event_x != mouse_clic_x.get() || event_y != mouse_clic_y.get() {
                            RENDER_NEXT_FRAME.store(true, Ordering::Relaxed);

                            let start_pos_rotated = model_mat * start_pos.get();
                            let start_pos_rotated = cgmath::Vector3::<f32>::new(start_pos_rotated.x, start_pos_rotated.y, start_pos_rotated.z);
                            
                            let pos_rotated = model_mat * pos;
                            let pos_rotated = cgmath::Vector3::<f32>::new(pos_rotated.x, pos_rotated.y, pos_rotated.z);

                            //console::log_1(&format!("REAL pos clic {:?}", start_pos_rotated).into());

                            let mut axis = start_pos_rotated.cross(pos_rotated);
                            let dist = math::angular_distance_xyz(start_pos_rotated, pos_rotated);

                            axis = axis.normalize();
                            hips_sphere.borrow_mut().apply_rotation(-axis, cgmath::Rad(dist));
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
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);

            canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        // Mouse wheel event
        {
            let gl = gl.clone();
            
            let hips_sphere = hips_sphere.clone();
            let grid = grid.clone();

            let viewport = viewport.clone();

            let projection = projection.clone();
            //let render_next_frame = render_next_frame.clone();

            let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
                let delta_y = event.delta_y() as f32;
                RENDER_NEXT_FRAME.store(true, Ordering::Relaxed);

                if delta_y < 0_f32 {
                    viewport.borrow_mut().zoom();
                } else {
                    viewport.borrow_mut().unzoom();
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
        }

        let gl = gl.clone();
        let app = App {
            gl,

            viewport,
            projection,

            shaders,
            // The sphere renderable
            hips_sphere,
            // The grid renderable
            grid,
        };

        Ok(app)
    }

    fn run(&self, window: Rc<web_sys::Window>) {
        let gl = self.gl.clone();
        //let render_next_frame = self.render_next_frame.clone();

        let viewport = self.viewport.clone();
        let projection = self.projection.clone();

        let hips_sphere = self.hips_sphere.clone();
        let grid = self.grid.clone();

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        gl.clear_color(0.08, 0.08, 0.08, 1.0);

        let fps_counter = Rc::new(window
            .document().unwrap()
            .get_element_by_id("fps-counter").unwrap()
            .dyn_into::<web_sys::HtmlElement>().unwrap()
        );

        let performance = Rc::new(window
            .performance()
            .expect("performance should be available")
        );

        let window_2 = window.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if RENDER_NEXT_FRAME.load(Ordering::Relaxed) {
                let start_frame = performance.now();
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

                /// Updating
                // The camera. When the camera has reached its final position
                // then we stop rendering the next frames!
                viewport.borrow_mut().update_camera_movement();
                // update the fov
                let model_mat = &hips_sphere.as_ref().borrow().get_model_mat();
                hips_sphere.borrow_mut()
                    .mesh_mut()
                    .update_field_of_view(&projection.get(), &viewport.borrow(), model_mat);

                // The grid label positions
                if *ENABLED_WIDGETS.lock().unwrap().get("grid").unwrap() {
                    grid.borrow_mut().mesh_mut().update_label_positions(
                        hips_sphere.borrow().get_inverted_model_mat(),
                        &projection.get(),
                        Some(&viewport.borrow()),
                    );
                }

                // Render the scene
                gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

                // Draw renderables here
                let ref viewport = viewport.as_ref().borrow();
                // Draw the HiPS sphere
                hips_sphere.as_ref().borrow().draw(WebGl2RenderingContext::TRIANGLES, viewport);

                // Draw the grid
                if *ENABLED_WIDGETS.lock().unwrap().get("grid").unwrap() {
                    // The grid lines
                    grid.as_ref().borrow().draw(WebGl2RenderingContext::LINES, viewport);
                    // The labels
                    grid.borrow().mesh().draw_labels();
                }
                //gl.finish();

                let end_frame = performance.now();
                let frame_duration_sec = (end_frame - start_frame) / 1000_f64;
                let frames_per_second = 1_f64 / frame_duration_sec;
                fps_counter.set_inner_text(&frames_per_second.to_string()); 
            }

            //console::log_1(&format!("FPS: {:?}", frames_per_second).into());

            // Schedule ourself for another requestAnimationFrame callback.
            request_animation_frame(f.borrow().as_ref().unwrap(), &window_2);
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap(), &window);
    }

    fn set_projection(&mut self, projection: ProjectionType) {
        self.projection.set(projection);
        let hips_sphere_mesh = HiPSSphere::new(&self.gl, &projection);

        // New HiPS sphere
        self.hips_sphere.replace(
            Renderable::<HiPSSphere>::new(
                &self.gl,
                self.shaders["hips_sphere"].clone(),
                hips_sphere_mesh,
            )
        );

        // New grid
        let lat_bound = cgmath::Vector2::<cgmath::Rad<f32>>::new(cgmath::Deg(-90_f32).into(), cgmath::Deg(90_f32).into());
        let projeted_grid_mesh = ProjetedGrid::new(cgmath::Deg(30_f32).into(), cgmath::Deg(30_f32).into(), Some(lat_bound), None, &projection);
        self.grid.replace(
            Renderable::<ProjetedGrid>::new(
                &self.gl,
                self.shaders["grid"].clone(),
                projeted_grid_mesh,
            )
        );

        self.viewport.borrow().update_scissor();
        RENDER_NEXT_FRAME.store(true, Ordering::Relaxed);
    }

    fn reload_hips_sphere(&mut self, hips_url: String) {
        *HIPS_NAME.lock().unwrap() = hips_url;

        let ref projection = self.projection.get();
        let hips_sphere_mesh = HiPSSphere::new(&self.gl, projection);

        // New HiPS sphere
        self.hips_sphere.replace(
            Renderable::<HiPSSphere>::new(
                &self.gl,
                self.shaders["hips_sphere"].clone(),
                hips_sphere_mesh,
            )
        );

        // New grid
        let lat_bound = cgmath::Vector2::<cgmath::Rad<f32>>::new(cgmath::Deg(-90_f32).into(), cgmath::Deg(90_f32).into());
        let projeted_grid_mesh = ProjetedGrid::new(cgmath::Deg(30_f32).into(), cgmath::Deg(30_f32).into(), Some(lat_bound), None, projection);
        self.grid.replace(
            Renderable::<ProjetedGrid>::new(
                &self.gl,
                self.shaders["grid"].clone(),
                projeted_grid_mesh,
            )
        );

        RENDER_NEXT_FRAME.store(true, Ordering::Relaxed);
    }
}

static DEGRADE_CANVAS_RATIO: f32 = 1.0_f32;

lazy_static! {
    // Note: Render_next_frame is global for the moment
    // A Rc cannot be instanciated as global because it cannot be shared between
    // threads (Rc does not impl the Sync trait)
    // Arc can be shared between threads => it is used here.
    static ref RENDER_NEXT_FRAME: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
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
fn render_next_frame() {
    RENDER_NEXT_FRAME.store(true, Ordering::Relaxed);
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
    window: Rc<web_sys::Window>,
}

#[wasm_bindgen]
impl WebClient {
    /// Create a new web client
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        let gl = WebGl2Context::new();
        let app = App::new(&gl).unwrap();

        let window = Rc::new(web_sys::window().unwrap());
        //let gl = Rc::new(create_webgl_context(Rc::clone(&app)).unwrap());

        WebClient {
            app,
            window
        }
    }

    /// Start our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn start(&self) -> Result<(), JsValue> {
        self.app.run(self.window.clone());

        Ok(())
    }

    /// Change the current projection of the HiPS
    pub fn set_projection(&mut self, name: String) -> Result<(), JsValue> {
        console::log_1(&format!("{:?}", name).into());
        match name.as_ref() {
            "aitoff" => {
                self.app.set_projection(ProjectionType::Aitoff(Aitoff {}));
            },
            "orthographic" => {
                self.app.set_projection(ProjectionType::Orthographic(Orthographic {}));
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
        RENDER_NEXT_FRAME.store(true, Ordering::Relaxed);

        Ok(())
    }

    /// Disable equatorial grid
    pub fn disable_equatorial_grid(&mut self) -> Result<(), JsValue> {
        if let Some(grid) = ENABLED_WIDGETS.lock().unwrap().get_mut("grid") {
            *grid = false;
            self.app.grid.borrow_mut().mesh_mut().clear_canvas();
        }
        RENDER_NEXT_FRAME.store(true, Ordering::Relaxed);

        Ok(())
    }

    pub fn change_hips(&mut self, hips_url: String) -> Result<(), JsValue> {
        self.app.reload_hips_sphere(hips_url);

        Ok(())
    }
}

