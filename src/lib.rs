#[macro_use]
extern crate lazy_static;

extern crate itertools_num;
extern crate slab;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, console};

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

fn request_animation_frame(f: &Closure<FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
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

struct App {
    //window: Rc<web_sys::Window>,
    canvas: Rc<web_sys::HtmlCanvasElement>,
    gl: Rc<WebGl2RenderingContext>,

    viewport: Rc<RefCell<ViewPort>>,
    projection: Rc<Cell<ProjectionType>>,

    shaders: HashMap<&'static str, Rc<Shader>>,
    // The sphere renderable
    hips_sphere: Rc<RefCell<Renderable<HiPSSphere>>>,
    // The grid renderable
    grid: Rc<RefCell<Renderable<ProjetedGrid>>>,
}

impl App {
    fn new(gl: Rc<WebGl2RenderingContext>, canvas: Rc<web_sys::HtmlCanvasElement>, window: Rc<web_sys::Window>) -> Result<App, JsValue> {
        gl.enable(WebGl2RenderingContext::BLEND);
        gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        gl.enable(WebGl2RenderingContext::SCISSOR_TEST);

        // Viewport
        let viewport = Rc::new(RefCell::new(ViewPort::new(&gl, window.clone(), canvas.clone())));
        let projection = Rc::new(Cell::new(ProjectionType::Aitoff(Aitoff {})));

        let shader_2d_proj = Rc::new(Shader::new(&gl,
            shaders::proj_vert::CONTENT,
            shaders::proj_frag::CONTENT
        ));
        let shader_grid = Rc::new(Shader::new(&gl,
            shaders::grid_projeted_vert::CONTENT,
            shaders::grid_frag::CONTENT
        ));
        let mut shaders = HashMap::new();
        shaders.insert("hips_sphere", shader_2d_proj);
        shaders.insert("grid", shader_grid);

        let hips_sphere_mesh = HiPSSphere::new(gl.clone());
        let hips_sphere = Rc::new(RefCell::new(Renderable::<HiPSSphere>::new(
            gl.clone(),
            shaders["hips_sphere"].clone(),
            &projection.get(),
            hips_sphere_mesh,
        )));

        let projeted_grid_mesh = ProjetedGrid::new(cgmath::Deg(30_f32).into(), cgmath::Deg(30_f32).into(), &projection.get());
        let grid = Rc::new(RefCell::new(Renderable::<ProjetedGrid>::new(
            gl.clone(),
            shaders["grid"].clone(),
            &projection.get(),
            projeted_grid_mesh,
        )));

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
            let gl = gl.clone();

            let hips_sphere = hips_sphere.clone();
            //let render_next_frame = render_next_frame.clone();

            Closure::wrap(Box::new(move || {
                console::log_1(&format!("resize").into());

                RENDER_NEXT_FRAME.store(true, Ordering::Relaxed);
                viewport.borrow_mut().resize(&gl);

                //hips_sphere
            }) as Box<dyn FnMut()>)
        };
        window.set_onresize(Some(onresize.as_ref().unchecked_ref()));
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
            let context = gl.clone();

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

                            console::log_1(&format!("REAL pos clic {:?}", start_pos_rotated).into());

                            let mut axis = start_pos_rotated.cross(pos_rotated);
                            let dist = math::angular_distance_xyz(start_pos_rotated, pos_rotated);

                            axis = axis.normalize();
                            hips_sphere.borrow_mut().apply_rotation(-axis, cgmath::Rad(dist));

                            grid.borrow_mut().apply_rotation(-axis, cgmath::Rad(dist));

                            grid.borrow_mut()
                                .mesh_mut()
                                .update(
                                    &projection.get(),
                                    hips_sphere.as_ref().borrow().get_inverted_model_mat(),
                                    Some(&viewport.borrow())
                                );
                            grid.as_ref().borrow_mut().update_vertex_array_object();

                            //iso_lat_0.borrow_mut().apply_rotation(-axis, cgmath::Rad(dist));

                            time_last_move.set(utils::get_current_time() as f32);
                            roll.set(true);

                            mouse_clic_x.set(event_x);
                            mouse_clic_y.set(event_y);

                            start_pos.set(pos);

                            last_axis.set(axis);
                            last_dist.set(dist);

                            // update the fov
                            hips_sphere.borrow_mut()
                                .mesh_mut()
                                .update_field_of_view(context.clone(), &projection.get(), &viewport.borrow(), model_mat, false);
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
                    viewport.borrow_mut().zoom(
                        hips_sphere.borrow()
                            .mesh()
                            .current_depth
                    );
                } else {
                    viewport.borrow_mut().unzoom();
                }

                grid.borrow_mut()
                    .mesh_mut()
                    .update(
                        &projection.get(),
                        hips_sphere.as_ref().borrow().get_inverted_model_mat(),
                        Some(&viewport.borrow())
                    );

                // update the fov
                let model_mat = &hips_sphere.as_ref().borrow().get_model_mat();
                hips_sphere.borrow_mut()
                    .mesh_mut()
                    .update_field_of_view(gl.clone(), &projection.get(), &viewport.borrow(), model_mat, true);
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        let app = App {
            //window,
            canvas,
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

    fn run(&self) {
        let gl = self.gl.clone();
        //let render_next_frame = self.render_next_frame.clone();

        let viewport = self.viewport.clone();
        let hips_sphere = self.hips_sphere.clone();
        let grid = self.grid.clone();

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if RENDER_NEXT_FRAME.load(Ordering::Relaxed) {
                RENDER_NEXT_FRAME.store(false, Ordering::Relaxed);
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

                //iso_lat_0.as_ref().borrow_mut().update_vertex_array_object(projection.clone());
                // Render the scene
                gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
                gl.clear_color(0.08, 0.08, 0.08, 1.0);

                // Draw renderable here
                let ref viewport = viewport.as_ref().borrow();
                hips_sphere.as_ref().borrow().draw(WebGl2RenderingContext::TRIANGLES, viewport);
                grid.as_ref().borrow().draw(WebGl2RenderingContext::LINES, viewport);

                grid.borrow().mesh().draw();
            }
            // Schedule ourself for another requestAnimationFrame callback.
            request_animation_frame(f.as_ref().borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.as_ref().borrow().as_ref().unwrap());
    }

    fn set_projection(&mut self, projection: ProjectionType) {
        self.projection.set(projection);
        //let hips_sphere_mesh = HiPSSphere::new(self.gl.clone());
        /*self.hips_sphere.replace_with(|old| {
            let mesh = old.mesh();
            Renderable::<HiPSSphere>::new(
                self.gl.clone(),
                self.shaders["hips_sphere"].clone(),
                &projection,
                (*mesh).clone(),
            )
        });*/
        RENDER_NEXT_FRAME.store(true, Ordering::Relaxed);
    }
}

lazy_static! {
    // Note: Render_next_frame is global for the moment
    // A Rc cannot be instanciated as global because it cannot be shared between
    // threads (Rc does not impl the Sync trait)
    // Arc can be shared between threads => it is used here.
    static ref RENDER_NEXT_FRAME: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    static ref WIDTH_SCREEN: Arc<AtomicU32> = Arc::new(
        AtomicU32::new(
            (web_sys::window().unwrap().inner_width()
                .unwrap()
                .as_f64()
                .unwrap() as u32)
        )
    );
    static ref HEIGHT_SCREEN: Arc<AtomicU32> = Arc::new(
        AtomicU32::new(
            (web_sys::window().unwrap().inner_height()
                .unwrap()
                .as_f64()
                .unwrap() as u32)
        )
    );
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

#[wasm_bindgen]
pub struct WebClient {
    app: App,
    gl: Rc<WebGl2RenderingContext>,
}

#[wasm_bindgen]
impl WebClient {
    /// Create a new web client
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        let window = Rc::new(web_sys::window().unwrap());
        let document = window.document().unwrap();

        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas = Rc::new(canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap());

        let context_options = js_sys::JSON::parse(&"{\"antialias\":true}").unwrap();

        let gl = Rc::new(
            canvas.get_context_with_context_options("webgl2", context_options.as_ref())
                .unwrap()
                .unwrap()
                .dyn_into::<WebGl2RenderingContext>()
                .unwrap()
        );

        let app = App::new(gl.clone(), canvas, window).unwrap();
        //let gl = Rc::new(create_webgl_context(Rc::clone(&app)).unwrap());

        WebClient {
            app,
            gl
        }
    }

    /// Start our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn start(&self) -> Result<(), JsValue> {
        self.app.run();

        Ok(())
    }

    /// Change the current projection of the HiPS
    pub fn set_projection(&mut self, name: String) -> Result<(), JsValue> {
        console::log_1(&format!("{:?}", name).into());
        if name == "aitoff" {
            self.app.set_projection(ProjectionType::Aitoff(Aitoff {}));
        } else if name == "orthographic" {
            self.app.set_projection(ProjectionType::Orthographic(Orthographic {}));
        }

        Ok(())
    }

    /*/// Update our simulation
    pub fn update(&self, dt: f32) {
        self.app.store.borrow_mut().msg(&Msg::AdvanceClock(dt));
    }*/
}

/*#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    {
        let mut app = App::new()?;
        app.run();
    }

    Ok(())
}*/