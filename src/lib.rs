use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use js_sys::WebAssembly;

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
    let body = document.body().unwrap();
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

    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let vert_shader = compile_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec3 position;
        void main() {
            gl_Position = vec4(position, 1.0);
        }
    "#,
    )?;
    let frag_shader = compile_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        void main() {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#,
    )?;
    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    let vertices_array = {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let vertices_location = vertices.as_ptr() as u32 / 4;
        js_sys::Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + vertices.len() as u32)
    };

    let indices: [u16; 3] = [0, 1, 2];
    let indices_array = {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let indices_location = indices.as_ptr() as u32 / 2;
        js_sys::Uint16Array::new(&memory_buffer)
            .subarray(indices_location, indices_location + indices.len() as u32)
    };

    // VERTEX buffer creation
    let vertex_buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    // Pass the vertices data to the buffer
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vertices_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    // Unbind the buffer
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);

    // INDEX buffer creation
    let index_buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    // Pass the indices data to the buffer
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        &indices_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    // Unbind the buffer
    gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, None);

    /* ======= Associating shaders to buffer objects =======*/
    // Bind vertex buffer object
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    // Bind index buffer object
    gl.bind_buffer(
        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        Some(&index_buffer),
    );
    gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    // Enable the depth test
    gl.enable(WebGlRenderingContext::DEPTH_TEST);

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

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // Render the scene
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        // Clear the color buffer bit
        // Clear the depth buffer bit
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        gl.draw_elements_with_i32(
            WebGlRenderingContext::LINE_LOOP,
            indices.len() as i32,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
