use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, console};
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
use std::cell::RefCell;
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

    let gl = Rc::new(
        canvas.get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?);

    let shader_texture = Shader::new(&gl,
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
    );
    shader_texture.bind(&gl);

    // Viewport
    let viewport = ViewPort::new(inner_width as f32, inner_height as f32);
    {
        // Renderable
        // Definition of the model matrix
        let axis = cgmath::Vector3::new(0_f32, 1_f32, 0_f32);
        let angle = cgmath::Rad(0_f32);
        let mut model_mat = cgmath::Matrix4::<f32>::from_axis_angle(axis, angle);
        //model_mat = model_mat * cgmath::Matrix4::<f32>::from_scale(((i as f32)*0.2).sin() + 3.5_f32);
        model_mat = model_mat * cgmath::Matrix4::<f32>::from_scale(3.5_f32);
        let sphere = Renderable::<HiPSSphere>::new(&gl, &shader_texture, model_mat);

        // Enable the depth test
        gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        // Enable back face culling
        gl.enable(WebGl2RenderingContext::CULL_FACE);
        gl.cull_face(WebGl2RenderingContext::BACK);

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
        let mut i = 0;

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
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            i = i + 1;

            // Tell the shader to use texture unit 0 for texture_location
            gl.uniform1i(texture_location.as_ref(), 0);

            // Render the scene
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            // Clear the color buffer bit
            // Clear the depth buffer bit
            gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

            sphere.draw(&gl, WebGl2RenderingContext::TRIANGLES, &viewport);

            // Schedule ourself for another requestAnimationFrame callback.
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    Ok(())
}