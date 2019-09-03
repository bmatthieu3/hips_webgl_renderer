use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, MouseEvent, EventTarget, console};
use wasm_bindgen::prelude::*;
use js_sys::WebAssembly;
use cgmath;
use cgmath::{InnerSpace, Angle, Vector2, Vector3, Matrix4, MetricSpace};

mod shader;
mod renderable;
mod viewport;
mod texture;
mod math;

use shader::Shader;
use renderable::Renderable;
use renderable::hips_sphere::HiPSSphere;
use renderable::direct_system::DirectSystem;
use viewport::ViewPort;
use std::borrow::Borrow;

fn request_animation_frame(f: &Closure<FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

use std::rc::Rc;
use std::cell::{RefCell, Cell};
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let inner_width = window.inner_width()?
        .as_f64()
        .unwrap();
    let inner_height = window.inner_height()?
        .as_f64()
        .unwrap();
    canvas.set_width(inner_width as u32);
    canvas.set_height(inner_height as u32);

    //let context_attributes = js_sys::Map::new();
    //let context_attributes2 = context_attributes.set(&"antialias".into(), &false.into());
    let context_options = js_sys::JSON::parse(&"{\"antialias\":false}").unwrap();
    console::log_1(&format!("contextAA attributes {:?}", &context_options).into());

    let gl = Rc::new(
        canvas.get_context_with_context_options("webgl2", context_options.as_ref())?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?
    );

    let shader_texture = Rc::new(Shader::new(&gl,
        r#"#version 300 es
        in vec3 position;
        in vec2 uv;

        out vec2 out_vert_uv;
        out vec3 out_vert_pos;

        out float out_vert_zoom_factor;

        uniform mat4 model;
        uniform mat4 projection;
        uniform mat4 view;

        uniform float width;
        uniform float height;

        uniform float zoom_factor;

        vec3 orthographic_projection(vec4 pos) {
            return pos.yzx;
        }

        vec3 aitoff_projection(vec4 pos) {
            float r = length(pos.yz);
            float w = sqrt(0.5f * r * (r + pos.z));

            float tt = sqrt(0.5f * (1.f + w));

            vec3 p = vec3(0.f);
            p.y = pos.x / tt;
            
            float zz = sqrt(2.f * r * (r - pos.z)) / tt;
            if (pos.y < 0.f) {
                p.x = zz;
            } else {
                p.x = -zz;
            }
            p.z = pos.x;

            return p;
        }

        void main() {
            vec4 pos = view * model * vec4(position, 1.0);
            gl_Position = vec4(aitoff_projection(pos), 1.0);
            //gl_Position.xy *= 0.4f;
            gl_Position.x *= height / width;
            int aa = 3;
            int bb = aa ^ 2;
            int cc = aa << 4;
            //gl_Position.xy *= zoom_factor;

            out_vert_pos = position;
            out_vert_uv = uv;
            out_vert_zoom_factor = zoom_factor;
        }
        "#,
        r#"#version 300 es
        precision mediump float;

        in vec2 out_vert_uv;
        in vec3 out_vert_pos;

        out vec4 out_frag_color;
        in float out_vert_zoom_factor;

        uniform sampler2D texture_hips_tile;

        void main() {
            out_frag_color = texture(texture_hips_tile, out_vert_uv);

            vec3 frag_pos = normalize(out_vert_pos);
            vec2 lonlat = vec2(atan(frag_pos.x, frag_pos.z), asin(frag_pos.y));
            lonlat *= 180.0/3.14159;

            if(abs(lonlat.y) < 80.0) {
                if(out_vert_zoom_factor > 0.5) {
                    lonlat /= vec2(10.0, 10.0);
                } else {
                    lonlat /= vec2(20.0, 20.0);
                }

                vec2 linePos = fract(lonlat + 0.5) - 0.5;
                vec2 der = vec2(50.0);
                linePos = max((1.0 - der*abs(linePos)), 0.0);

                out_frag_color *= (1.0 - 0.3 * max(linePos.x, linePos.y));
            }
        }
        "#,
    ));
    let shader_direct_system = Rc::new(Shader::new(&gl,
        r#"#version 300 es
        in vec3 position;
        in vec4 color;

        out vec4 out_vert_color;

        uniform mat4 model;
        uniform mat4 projection;
        uniform mat4 view;

        void main() {
            gl_Position = projection * view *  model * vec4(position, 1.0);
            out_vert_color = color;
        }
        "#,
        r#"#version 300 es
        precision mediump float;

        in vec4 out_vert_color;
        out vec4 out_frag_color;

        void main() {
            out_frag_color = out_vert_color;
        }
        "#,
    ));

    // Viewport
    let viewport = Rc::new(RefCell::new(ViewPort::new(inner_width as f32, inner_height as f32)));

    // Definition of the model matrix
    let mut direct_system = Rc::new(RefCell::new(Renderable::<DirectSystem>::new(&gl, shader_direct_system.clone())));
    direct_system.borrow_mut().scale(0.5_f32);
    let mut sphere = Rc::new(RefCell::new(Renderable::<HiPSSphere>::new(&gl, shader_texture.clone())));
    // Enable the depth test
    //gl.enable(WebGl2RenderingContext::DEPTH_TEST);
    // Enable back face culling
    gl.enable(WebGl2RenderingContext::CULL_FACE);
    gl.cull_face(WebGl2RenderingContext::BACK);

    gl.enable(WebGl2RenderingContext::BLEND);
    gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);

    console::log_1(&format!("context attributes {:?}", gl.get_context_attributes().unwrap()).into());

    // Create the TEXTURE
    texture::load(gl.clone(), "http://alasky.u-strasbg.fr/DSS/DSSColor/Norder0/Dir0/Npix0.jpg", WebGl2RenderingContext::TEXTURE0);

    // Here we want to call `requestAnimationFrame` in a loop, but only a fixed
    // number of times. After it's done we want all our resources cleaned up. To
    // achieve this we're using an `Rc`. The `Rc` will eventually store the
    // closure we want to execute on each frame, but to start out it contains
    // `None`.
    //
    // After the `Rc` is made we'll actually create the closure, and the closure
    // will reference one of the `Rc` instances. The other `Rc` reference is
    // used to store the closure, request the first frame, and then is dropped
    // by this function.
    //
    // Inside the closure we've got a persistent `Rc` reference, which we use
    // for all future iterations of the loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    /*
    let fovy = cgmath::Deg(60_f32);
    let half_fovy = cgmath::Deg(22.5_f32);
    let aspect = inner_width as f32 / inner_height as f32;
    let near = 0.1_f32;
    let far = 50_f32;
    let proj_mat: cgmath::Matrix4<f32> = cgmath::perspective(
        fovy,
        aspect,
        near,
        far
    );

    let eye = cgmath::Point3::new(5_f32, 5_f32, 5_f32);
    let center = cgmath::Point3::new(0_f32, 0_f32, 0_f32);
    let up = cgmath::Vector3::new(0_f32, 1_f32, 0_f32);
    let view_mat = cgmath::Matrix4::<f32>::look_at(eye, center, up);

    let view = (center - eye).normalize();
    let v_length = cgmath::Deg::tan(half_fovy) * near;
    let h_length = v_length * (inner_width as f32 / inner_height as f32);

    let h = view.cross(up).normalize() * h_length;
    let v = h.cross(view).normalize() * v_length;

    // Compute up-left corner ray
    let origin_up_left = eye + view * near - h + v;
    let dir_up_left = (origin_up_left - eye).normalize();

    let origin_up_right = eye + view * near + h + v;
    let dir_up_right = (origin_up_right - eye).normalize();

    let origin_bottom_left = eye + view * near - h - v;
    let dir_bottom_left = (origin_bottom_left - eye).normalize();

    let origin_bottom_right = eye + view * near + h - v;
    let dir_bottom_right = (origin_bottom_right - eye).normalize();
    
    // Check whether the viewport intersect the unit sphere
    // - Yes: 
    //   1) Get the intersection points.
    //   2) Convert them to lon, lat.
    //   3) Define the minimum depth so that the number of HEALPix cell contained
    //      in the viewport polygon is > N.
    //   4) Call vertices on each of these pixels and define the vertices/indices buffer
    // - No:
    //   1) Draw the whole sphere at the order 2. Enable backfacing culling for limiting the rendering time
    //      to what is visible.
    console::log_1(&format!("Dir_up_left {:?}", dir_up_left).into());
    console::log_1(&format!("Dir_up_right {:?}", dir_up_right).into());

    console::log_1(&format!("Dir_bottom_left {:?}", dir_bottom_left).into());
    console::log_1(&format!("Dir_bottom_right {:?}", dir_bottom_right).into());
    */
    // Mouse down pression event
    let pressed = Rc::new(Cell::new(false));
    let delta_x = Rc::new(Cell::new(0_f32));
    let delta_y = Rc::new(Cell::new(0_f32));

    let mouse_clic_x = Rc::new(Cell::new(0_f32));
    let mouse_clic_y = Rc::new(Cell::new(0_f32));

    let start_pos = Rc::new(Cell::new(Vector3::<f32>::new(0_f32, 0_f32, 0_f32)));
    {
        let pressed = pressed.clone();

        let mouse_clic_x = mouse_clic_x.clone();
        let mouse_clic_y = mouse_clic_y.clone();

        let start_pos = start_pos.clone();

        let v = viewport.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            console::log_1(&format!("mouse down").into());
            mouse_clic_x.set(event.offset_x() as f32);
            mouse_clic_y.set(event.offset_y() as f32);

            let result = v.as_ref().borrow().unproj(event.offset_x() as f32, event.offset_y() as f32);
            if let Some(pos) = result {
                let pos = pos.normalize();
                start_pos.set(pos);
                pressed.set(true);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse up pression event
    {
        let pressed = pressed.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            console::log_1(&format!("mouse up").into());
            pressed.set(false);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse move event
    {
        let pressed = pressed.clone();
        let viewport = viewport.clone();

        let start_pos = start_pos.clone();
        let sphere = sphere.clone();

        let mouse_clic_x = mouse_clic_x.clone();
        let mouse_clic_y = mouse_clic_y.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                let event_x = event.offset_x() as f32;
                let event_y = event.offset_y() as f32;

                let result = viewport.as_ref().borrow().unproj(event_x, event_y);
                if let Some(pos) = result {
                    let pos = pos.normalize();
                    let dist = math::angular_distance_xyz(pos, start_pos.get());

                    if event_x != mouse_clic_x.get() || event_y != mouse_clic_y.get() {
                        let mut axis = start_pos.get().cross(pos);
                        axis = axis.normalize();
                        sphere.borrow_mut().apply_rotation(axis, cgmath::Rad(dist));

                        mouse_clic_x.set(event_x);
                        mouse_clic_y.set(event_y);

                        start_pos.set(pos);
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    // Mouse wheel event
    let zoom_factor = Rc::new(Cell::new(1_f32));
    {
        let context = gl.clone();
        let pressed = pressed.clone();
        let zoom_factor = zoom_factor.clone();
        let sphere = sphere.clone();
        let viewport = viewport.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
            let delta_y = event.delta_y() as f32;

            if delta_y < 0_f32 {
                viewport.borrow_mut().zoom(true);
            } else {
                viewport.borrow_mut().zoom(false);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    let axis = cgmath::Vector3::new(0_f32, 1_f32, 0_f32);
    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        i += 1;

        // Render the scene
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        // Clear the color buffer bit
        // Clear the depth buffer bit
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        
        //sphere.borrow_mut().rotate(axis, theta);
        
        sphere.as_ref().borrow().draw(&gl, WebGl2RenderingContext::TRIANGLES, &viewport.as_ref().borrow());
        direct_system.as_ref().borrow().draw(&gl, WebGl2RenderingContext::LINES, &viewport.as_ref().borrow());

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.as_ref().borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.as_ref().borrow().as_ref().unwrap());

    Ok(())
}