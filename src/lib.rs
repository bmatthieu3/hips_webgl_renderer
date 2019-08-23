use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, MouseEvent, EventTarget, console};
use wasm_bindgen::prelude::*;
use js_sys::WebAssembly;
use cgmath;
use cgmath::{InnerSpace, Angle};

mod shader;
mod mesh;
mod viewport;
mod texture;

use shader::Shader;
use mesh::{Renderable, HiPSSphere};
use viewport::ViewPort;

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

    let context_attributes = js_sys::Map::new();
    let context_attributes2 = context_attributes.set(&"antifgfgalias".into(), &false.into());
    let context_options: JsValue = context_attributes2.into();
    console::log_1(&format!("contextAA attributes {:?}", &context_options).into());

    if context_options.is_object() {
        console::log_1(&"aaaze attributes".into());
    }
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

        uniform mat4 model;
        uniform mat4 projection;
        uniform mat4 view;

        void main() {
            gl_Position = projection * view *  model * vec4(position, 1.0);
            out_vert_uv = uv;
        }
        "#,
        r#"#version 300 es
        precision mediump float;

        in vec2 out_vert_uv;

        out vec4 out_frag_color;

        uniform sampler2D tex;

        void main() {
            out_frag_color = texture(tex, out_vert_uv);
            //out_frag_color = vec4(length(out_vert_uv), 0.0, 1.0, 1.0);
        }
        "#,
    ));
    shader_texture.bind(&gl);

    // Viewport
    let viewport = Rc::new(RefCell::new(ViewPort::new(inner_width as f32, inner_height as f32)));

    // Renderable
    // Definition of the model matrix
    let mut sphere = Rc::new(RefCell::new(Renderable::<HiPSSphere>::new(&gl, shader_texture.clone())));

    // Enable the depth test
    gl.enable(WebGl2RenderingContext::DEPTH_TEST);
    // Enable back face culling
    gl.enable(WebGl2RenderingContext::CULL_FACE);
    gl.cull_face(WebGl2RenderingContext::BACK);

    console::log_1(&format!("context attributes {:?}", gl.get_context_attributes().unwrap()).into());

    // Create the TEXTURE
    texture::load(gl.clone(), "Allsky.jpg", WebGl2RenderingContext::TEXTURE0);

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

    // Get the attribute location of the matrices from the Vertex shader
    let texture_location = shader_texture.get_uniform_location(&gl, "tex");
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
    {
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            console::log_1(&format!("mouse down").into());
            pressed.set(true);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
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
        let context = gl.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                console::log_1(&format!("x: {:?}; y: {:?}", event.offset_x(), event.offset_y()).into());
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
            //console::log_1(&format!("delta: {:?}", delta_y).into());
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
        // Tell the shader to use texture unit 0 for texture_location
        gl.uniform1i(texture_location.as_ref(), 0);

        // Render the scene
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        // Clear the color buffer bit
        // Clear the depth buffer bit
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        
        let theta = cgmath::Rad((i as f32) / 1000_f32);
        sphere.borrow_mut().rotate(axis, theta);
        sphere.borrow().draw(&gl, WebGl2RenderingContext::TRIANGLES, &viewport.borrow());

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}