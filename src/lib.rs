use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, console};
use js_sys::WebAssembly;
use cgmath;
use cgmath::{InnerSpace, Angle};
use healpix;

fn request_animation_frame(f: &Closure<FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn build_vertices_buffer() -> (Vec<f32>, Vec<f32>, Vec<u32>) {
    fn add_face_to_buffer(lonlat: [(f64, f64); 4]) -> Vec<f32> {
        lonlat.into_iter()
            .map(|(lon, lat)| {
                // inverse longitude because
                let x = (*lat).cos() * (-(*lon)).sin();
                let y = (*lat).sin();
                let z = (*lat).cos() * (-(*lon)).cos();

                vec![x as f32, y as f32, z as f32]
            })
            .flatten()
            .collect::<Vec<_>>()
    }

    let depth = 3;
    let num_faces = (12 << (2 * depth)) as usize;
    console::log_1(&format!("Num faces {:?}", num_faces).into());
    let vertices = (0..num_faces).into_iter()
        .map(|hash| {
            let lonlat = healpix::nested::vertices(depth, hash as u64);

            add_face_to_buffer(lonlat)
        })
        .flatten()
        .collect::<Vec<_>>();
    
    let num_indices = 3 * 2 * num_faces;
    let mut indices = Vec::with_capacity(num_indices);

    for idx_face in 0..num_faces {
        let idx_origin = idx_face * 4;
        /*
        indices.push(idx_origin as u32);
        indices.push((idx_origin + 3) as u32);
        indices.push((idx_origin + 3) as u32);
        indices.push((idx_origin + 2) as u32);
    
        indices.push(idx_origin as u32);
        indices.push((idx_origin + 2) as u32);
        indices.push((idx_origin + 2) as u32);
        indices.push((idx_origin + 1) as u32);

        indices.push((idx_origin + 1) as u32);
        indices.push(idx_origin as u32);*/
        indices.push(idx_origin as u32);
        indices.push((idx_origin + 3) as u32);
        indices.push((idx_origin + 2) as u32);

        indices.push(idx_origin as u32);
        indices.push((idx_origin + 2) as u32);
        indices.push((idx_origin + 1) as u32);
    }

    let mut uv = Vec::with_capacity(vertices.len());
    let width_allsky = 1728;
    let height_allsky = 1856;
    let size_tile = 64;

    let num_tile_per_row = width_allsky / size_tile;

    for idx_vertex in 0..vertices.len() {
        let position = idx_vertex % 4;
        let idx = idx_vertex / 4;

        let mut idx_row = idx / num_tile_per_row;
        let mut idx_col = idx - (idx_row * num_tile_per_row);
        if position == 1 {
            idx_row += 1;
        } else if position == 2 {
            idx_col += 1;
            idx_row += 1;
        } else if position == 3 {
            idx_col += 1;
        }

        uv.push(((idx_col * size_tile) as f32) / (width_allsky as f32)); // u
        uv.push(((idx_row * size_tile) as f32) / (height_allsky as f32)); // v
    }

    (vertices, uv, indices)
}

use web_sys::HtmlImageElement;
fn load_texture_image(gl: Rc<WebGl2RenderingContext>, src: &str) {
    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
    let image_clone = Rc::clone(&image);

    let onload = Closure::wrap(Box::new(move || {
        let texture = gl.create_texture();

        gl.active_texture(WebGl2RenderingContext::TEXTURE0);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());
        //gl.pixel_storei(WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL, 1);

        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR as i32);
        
        //gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
        // Prevents s-coordinate wrapping (repeating)
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        // Prevents t-coordinate wrapping (repeating)
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGB as i32,
            WebGl2RenderingContext::RGB,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            &image_clone.borrow(),
        )
        .expect("Texture image 2d");
    }) as Box<dyn Fn()>);

    let image = image.borrow_mut();

    image.set_onload(Some(onload.as_ref().unchecked_ref()));
    image.set_src(src);

    onload.forget();
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

    let gl = Rc::new(
        canvas.get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?);

    let vert_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::VERTEX_SHADER,
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
    )?;
    let frag_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::FRAGMENT_SHADER,
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
    )?;
    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));
    /*
    let segment_by_side = 3 as usize;
    let vertices = healpix::nested::grid(0, 1, segment_by_side as u16)
        .into_iter()
        .map(|(lon, lat)| {
            let x = -(*lat).cos() * (*lon).cos();
            let y = -(*lat).sin();
            let z = (*lat).cos() * (*lon).sin();

            vec![x as f32, y as f32, z as f32]
        })
        .flatten()
        .collect::<Vec<_>>();
    console::log_1(&format!("Hello using web-sys {:?}", vertices).into());
    let mut indices = Vec::<u16>::with_capacity(segment_by_side * segment_by_side * 6);
    let vertices_by_line = segment_by_side + 1;
    for i in (0..segment_by_side) {
        for j in (0..segment_by_side) {
            let off = j + i * vertices_by_line;
            // first triangle
            indices.push(off as u16);
            indices.push((off + 1) as u16);
            indices.push((off + vertices_by_line) as u16);

            // second triangle
            indices.push((off + 1) as u16);
            indices.push((off + vertices_by_line + 1) as u16);
            indices.push((off + vertices_by_line) as u16);
        }
    }
    console::log_1(&format!("Hello using web-sys {:?}", indices).into());*/

    let (vertices, uv, indices) = build_vertices_buffer();
    /*let vertices: [f32; 24] = [
        // Front face
        -0.5, 0.5, 0.5,
        -0.5, -0.5, 0.5,
        0.5, -0.5, 0.5,
        0.5, 0.5, 0.5,

        // Back face
        -0.5, 0.5, -0.5,
        0.5, 0.5, -0.5,
        0.5, -0.5, -0.5,
        -0.5, -0.5, -0.5,
    ];*/
    let vertices_array = {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let vertices_location = vertices.as_ptr() as u32 / 4;
        js_sys::Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + vertices.len() as u32)
    };

    /*let uv: [f32; 48] = [
        // Front
        0.0,  0.0,
        1.0,  0.0,
        1.0,  1.0,
        0.0,  1.0,
        // Back
        0.0,  0.0,
        1.0,  0.0,
        1.0,  1.0,
        0.0,  1.0,
        // Top
        0.0,  0.0,
        1.0,  0.0,
        1.0,  1.0,
        0.0,  1.0,
        // Bottom
        0.0,  0.0,
        1.0,  0.0,
        1.0,  1.0,
        0.0,  1.0,
        // Right
        0.0,  0.0,
        1.0,  0.0,
        1.0,  1.0,
        0.0,  1.0,
        // Left
        0.0,  0.0,
        1.0,  0.0,
        1.0,  1.0,
        0.0,  1.0,
    ];
    */
    let uv_array = {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let uv_location = uv.as_ptr() as u32 / 4;
        js_sys::Float32Array::new(&memory_buffer)
            .subarray(uv_location, uv_location + uv.len() as u32)
    };
    let indices_array = {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let indices_location = indices.as_ptr() as u32 / 4;
        js_sys::Uint32Array::new(&memory_buffer)
            .subarray(indices_location, indices_location + indices.len() as u32)
    };

    // VERTEX buffer creation
    let vertex_buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    // Pass the vertices data to the buffer
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &vertices_array,
        WebGl2RenderingContext::STATIC_DRAW,
    );

    // Unbind the buffer
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
    
    // UV buffer creation
    let uv_buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&uv_buffer));
    // Pass the vertices data to the buffer
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &uv_array,
        WebGl2RenderingContext::STATIC_DRAW,
    );

    // Unbind the buffer
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
    
    // INDEX buffer creation
    let index_buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    // Pass the indices data to the buffer
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        &indices_array,
        WebGl2RenderingContext::STATIC_DRAW,
    );

    // Unbind the buffer
    gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);

    /* ======= Associating shaders to buffer objects =======*/
    // Bind vertex buffer object
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

    gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);
    
    // Bind uv buffer object
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&uv_buffer));

    gl.vertex_attrib_pointer_with_i32(1, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(1);

    // Bind index buffer object
    gl.bind_buffer(
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        Some(&index_buffer),
    );
    // Enable the depth test
    gl.enable(WebGl2RenderingContext::DEPTH_TEST);
    // Enable back face culling
    gl.enable(WebGl2RenderingContext::CULL_FACE);
    gl.cull_face(WebGl2RenderingContext::BACK);

    // Create the TEXTURE
    load_texture_image(gl.clone(), "Allsky.jpg");

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
    let model_mat_location = gl.get_uniform_location(&program, "model");
    let view_mat_location = gl.get_uniform_location(&program, "view");
    let proj_mat_location = gl.get_uniform_location(&program, "projection");
    let texture_location = gl.get_uniform_location(&program, "tex");

    let fovy = cgmath::Deg(60_f32);
    let half_fovy = cgmath::Deg(22.5_f32);
    let aspect = inner_width as f32 / inner_height as f32;
    let near = 0.1_f32;
    let far = 100_f32;
    let proj_mat: cgmath::Matrix4<f32> = cgmath::perspective(
        fovy,
        aspect,
        near,
        far
    );

    let eye = cgmath::Point3::new(5_f32, 5_f32, 5_f32);
    let center = cgmath::Point3::new(0_f32, 0_f32, 0_f32);
    let up = cgmath::Vector3::new(0_f32, 1_f32, 0_f32);
    let mut view_mat = cgmath::Matrix4::<f32>::look_at(eye, center, up);

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

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        i = i + 1;
        // Definition of the model matrix
        let axis = cgmath::Vector3::new(0_f32, 1_f32, 0_f32);
        let angle = cgmath::Rad((i as f32)/100_f32);
        let mut model_mat = cgmath::Matrix4::<f32>::from_axis_angle(axis, angle);
        model_mat = model_mat * cgmath::Matrix4::<f32>::from_scale(((i as f32)*0.05).sin() + 3.5_f32);
        //model_mat = model_mat * cgmath::Matrix4::<f32>::from_scale(4_f32);

        // Send matrices to the Vertex shader
        let model_mat_f32_slice: &[f32; 16] = model_mat.as_ref();
        gl.uniform_matrix4fv_with_f32_array(model_mat_location.as_ref(), false, model_mat_f32_slice);
        let view_mat_f32_slice: &[f32; 16] = view_mat.as_ref();
        gl.uniform_matrix4fv_with_f32_array(view_mat_location.as_ref(), false, view_mat_f32_slice);
        let proj_mat_f32_slice: &[f32; 16] = proj_mat.as_ref();
        gl.uniform_matrix4fv_with_f32_array(proj_mat_location.as_ref(), false, proj_mat_f32_slice);

        // Tell the shader to use texture unit 0 for u_texture
        gl.uniform1i(texture_location.as_ref(), 0);

        // Render the scene
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        // Clear the color buffer bit
        // Clear the depth buffer bit
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

        gl.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            indices.len() as i32,
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
        );

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
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
    context: &WebGl2RenderingContext,
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
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
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
