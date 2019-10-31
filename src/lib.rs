extern crate itertools_num;
extern crate slab;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, console};
use cgmath;
use cgmath::{InnerSpace, Vector3};

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
use renderable::projection::{ProjectionType, Aitoff};
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

fn compute_speed(t_start: f32, speed_max: f32) -> f32 {
    let t = utils::get_current_time() as f32; 
    let t_duration = 1200_f32; // in ms
    let t_end = t_start + t_duration;

    if t > t_end {
        0_f32
    } else {
        let speed = (-t*speed_max / t_duration) + t_end*speed_max/t_duration;
        speed
    }
}

use std::rc::Rc;
use std::cell::{RefCell, Cell};
use crate::renderable::grid::{Grid, IsoLatitudeLine};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = Rc::new(web_sys::window().unwrap());
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = Rc::new(canvas.dyn_into::<web_sys::HtmlCanvasElement>()?);

    //let context_attributes = js_sys::Map::new();
    //let context_attributes2 = context_attributes.set(&"antialias".into(), &false.into());
    let context_options = js_sys::JSON::parse(&"{\"antialias\":true}").unwrap();

    let gl = Rc::new(
        canvas.get_context_with_context_options("webgl2", context_options.as_ref())?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?
    );
    // Enable the depth test
    //gl.enable(WebGl2RenderingContext::DEPTH_TEST);
    // Enable back face culling
    //gl.enable(WebGl2RenderingContext::CULL_FACE);
    //gl.cull_face(WebGl2RenderingContext::FRONT);
    gl.enable(WebGl2RenderingContext::BLEND);
    gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
    //gl.enable(WebGl2RenderingContext::SCISSOR_TEST);

    // Viewport
    let viewport = Rc::new(RefCell::new(ViewPort::new(&gl, window.clone(), canvas.clone())));
    let projection = Rc::new(RefCell::new(ProjectionType::Aitoff(Aitoff {})));

    let shader_2d_proj = Rc::new(Shader::new(&gl,
        shaders::proj_vert::CONTENT,
        shaders::proj_frag::CONTENT
    ));
    let shader_grid = Rc::new(Shader::new(&gl,
        shaders::grid_vert::CONTENT,
        shaders::grid_frag::CONTENT
    ));
    let shader_projeted_grid = Rc::new(Shader::new(&gl,
        shaders::grid_projeted_vert::CONTENT,
        shaders::grid_frag::CONTENT
    ));

    let hips_sphere_mesh = Rc::new(RefCell::new(HiPSSphere::new(gl.clone())));
    let sphere = Rc::new(RefCell::new(Renderable::<HiPSSphere>::new(
        gl.clone(),
        shader_2d_proj.clone(),
        projection.clone(),
        hips_sphere_mesh.clone(),
    )));
    let grid_mesh = Rc::new(RefCell::new(Grid::new()));
    let grid = Rc::new(RefCell::new(Renderable::<Grid>::new(
        gl.clone(),
        shader_grid.clone(),
        projection.clone(),
        grid_mesh.clone(),
    )));
    /*let iso_lat_0_mesh = Rc::new(RefCell::new(IsoLatitudeLine::new(0_f32)));
    let iso_lat_0 = Rc::new(RefCell::new(Renderable::<IsoLatitudeLine>::new(
        gl.clone(),
        shader_projeted_grid.clone(),
        projection.clone(),
        iso_lat_0_mesh.clone(),
    )));*/

    let projeted_grid_mesh = Rc::new(RefCell::new(ProjetedGrid::new(cgmath::Deg(30_f32).into(), cgmath::Deg(30_f32).into())));
    let projeted_grid = Rc::new(RefCell::new(Renderable::<ProjetedGrid>::new(
        gl.clone(),
        shader_projeted_grid.clone(),
        projection.clone(),
        projeted_grid_mesh.clone(),
    )));
    /*let projeted_grid = Rc::new(RefCell::new(ProjetedGrid::new(
        gl.clone(),
        shader_projeted_grid.clone(),
        projection.clone(),
        cgmath::Deg(30_f32).into(),
    )));*/

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    // Mouse down pression event
    let pressed = Rc::new(Cell::new(false));

    let mouse_clic_x = Rc::new(Cell::new(0_f32));
    let mouse_clic_y = Rc::new(Cell::new(0_f32));

    let start_pos = Rc::new(Cell::new(Vector3::<f32>::new(0_f32, 0_f32, 0_f32)));

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
            let result = projection.as_ref().borrow().screen_to_world_space(x_screen_homogeous_space, y_screen_homogeous_space);
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
        Closure::wrap(Box::new(move || {
            console::log_1(&format!("resize").into());
            viewport.borrow_mut().resize(&gl);
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
        let sphere = sphere.clone();
        let grid = grid.clone();
        //let iso_lat_0 = iso_lat_0.clone();
        let projeted_grid = projeted_grid.clone();

        let mouse_clic_x = mouse_clic_x.clone();
        let mouse_clic_y = mouse_clic_y.clone();

        let last_axis = last_axis.clone();
        let last_dist = last_dist.clone();

        let viewport = viewport.clone();
        let projection = projection.clone();
        let context = gl.clone();
        let hips_sphere_mesh = hips_sphere_mesh.clone();

        let time_last_move = time_last_move.clone();
        let roll = roll.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                let event_x = event.offset_x() as f32;
                let event_y = event.offset_y() as f32;
                let model_mat = &sphere.as_ref().borrow().get_model_mat();

                let (x_screen_homogeous_space, y_screen_homogeous_space) = projection::screen_pixels_to_homogenous(event_x, event_y, &viewport.borrow());
                let result = projection.as_ref().borrow().screen_to_world_space(x_screen_homogeous_space, y_screen_homogeous_space);
                if let Some(pos) = result {
                    let pos = pos.normalize();

                    if event_x != mouse_clic_x.get() || event_y != mouse_clic_y.get() {
                        let start_pos_rotated = model_mat * cgmath::Vector4::<f32>::new(
                            start_pos.get().x,
                            start_pos.get().y,
                            start_pos.get().z,
                            1_f32
                        );

                        let start_pos_rotated = cgmath::Vector3::<f32>::new(start_pos_rotated.x, start_pos_rotated.y, start_pos_rotated.z);
                        
                        let pos_rotated = model_mat * cgmath::Vector4::<f32>::new(
                            pos.x,
                            pos.y,
                            pos.z,
                            1_f32
                        );

                        let pos_rotated = cgmath::Vector3::<f32>::new(pos_rotated.x, pos_rotated.y, pos_rotated.z);

                        console::log_1(&format!("REAL pos clic {:?}", start_pos_rotated).into());

                        let mut axis = start_pos_rotated.cross(pos_rotated);
                        let dist = math::angular_distance_xyz(start_pos_rotated, pos_rotated);

                        axis = axis.normalize();
                        sphere.borrow_mut().apply_rotation(-axis, cgmath::Rad(dist));
                        grid.borrow_mut().apply_rotation(-axis, cgmath::Rad(dist));
                        projeted_grid.borrow_mut().apply_rotation(-axis, cgmath::Rad(dist));
                        //iso_lat_0.borrow_mut().apply_rotation(-axis, cgmath::Rad(dist));

                        time_last_move.set(utils::get_current_time() as f32);
                        roll.set(true);

                        mouse_clic_x.set(event_x);
                        mouse_clic_y.set(event_y);

                        start_pos.set(pos);

                        last_axis.set(axis);
                        last_dist.set(dist);

                        // update the fov
                        hips_sphere_mesh.borrow_mut().update_field_of_view(context.clone(), &projection.as_ref().borrow(), &viewport.borrow(), model_mat, false);
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    // Mouse wheel event
    {
        let context = gl.clone();
        let sphere = sphere.clone();
        let viewport = viewport.clone();

        let hips_sphere_mesh = hips_sphere_mesh.clone();
        let projection = projection.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
            let delta_y = event.delta_y() as f32;

            if delta_y < 0_f32 {
                viewport.borrow_mut().zoom(hips_sphere_mesh.borrow().current_depth);
            } else {
                viewport.borrow_mut().unzoom();
            }

            // update the fov
            let model_mat = &sphere.as_ref().borrow().get_model_mat();
            
            hips_sphere_mesh.borrow_mut().update_field_of_view(context.clone(), &projection.as_ref().borrow(), &viewport.borrow(), model_mat, true);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let gl = gl.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            /*if !pressed.get() && roll.get() {
                let next_dist = compute_speed(time_last_move.get(), last_dist.get() * 0.5_f32);
                if next_dist > 1e-4 {
                    sphere.borrow_mut().apply_rotation(-last_axis.get(), cgmath::Rad(next_dist));

                    let model_mat = &sphere.as_ref().borrow().get_model_mat();
                    hips_sphere_mesh.borrow_mut().update_field_of_view(gl.clone(), &projection.as_ref().borrow(), &viewport.borrow(), model_mat, false);
                }
            } else if pressed.get() {
                if (utils::get_current_time() as f32) - time_last_move.get() > 50_f32 {
                    roll.set(false);
                }
            }*/

            //iso_lat_0.as_ref().borrow_mut().update_vertex_array_object(projection.clone());
            projeted_grid.as_ref().borrow_mut().update_vertex_array_object(projection.clone());

            // Render the scene
            gl.clear_color(0.08, 0.08, 0.08, 1.0);
            // Clear the color buffer bit
            // Clear the depth buffer bit
            gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

            sphere.as_ref().borrow().draw(WebGl2RenderingContext::TRIANGLES, &viewport.as_ref().borrow());
            //iso_lat_0.as_ref().borrow().draw(WebGl2RenderingContext::LINES, &viewport.as_ref().borrow());
            projeted_grid.as_ref().borrow().draw(WebGl2RenderingContext::LINES, &viewport.as_ref().borrow());   
            //grid.as_ref().borrow().draw(WebGl2RenderingContext::LINES, &viewport.as_ref().borrow());
            
            //direct_system.as_ref().borrow().draw(&gl, WebGl2RenderingContext::LINES, &viewport.as_ref().borrow());

            // Schedule ourself for another requestAnimationFrame callback.
            request_animation_frame(f.as_ref().borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.as_ref().borrow().as_ref().unwrap());
    }

    Ok(())
}