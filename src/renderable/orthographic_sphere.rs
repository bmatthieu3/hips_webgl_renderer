
pub struct OrthoSphere {
    texture_location: WebGlUniformLocation,
}

use web_sys::console;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::WebAssembly;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;
use web_sys::WebGlTexture;
use web_sys::WebGlUniformLocation;
use web_sys::WebGlVertexArrayObject;

use crate::math;

use std::rc::Rc;

use cgmath::Vector2;

use crate::renderable::Mesh;
use crate::shader::Shader;
use crate::viewport::ViewPort;

const MAX_NUMBER_TEXTURE: usize = 15;

const NUM_VERTICES_PER_STEP: usize = 70;
const NUM_STEPS: usize = 40;

use cgmath::SquareMatrix;



impl Mesh for OrthoSphere {
    fn create_color_array() -> js_sys::Float32Array {
        unreachable!()
    }

    fn create_vertex_array(viewport: &ViewPort) -> (js_sys::Float32Array, js_sys::Float32Array) {
        let mut vertices = Vec::with_capacity(3*(NUM_VERTICES_PER_STEP*NUM_STEPS + 1) as usize);
        let mut vertices_screen = Vec::with_capacity(2*(NUM_VERTICES_PER_STEP*NUM_STEPS + 1) as usize);

        let window = web_sys::window().unwrap();
        let width = window.inner_width()
            .unwrap()
            .as_f64()
            .unwrap() as f32;
        let height = window.inner_height()
            .unwrap()
            .as_f64()
            .unwrap() as f32;
        let radius_max = (height - 1_f32) / 2_f32; // radius in pixels
        let aspect = width / height;

        let center_screen_space = Vector2::<f32>::new(width / 2_f32, height / 2_f32);
        vertices_screen.push(2_f32 * ((center_screen_space.x / width) - 0.5_f32));
        vertices_screen.push(-2_f32 * ((center_screen_space.y / height) - 0.5_f32));

        let center = viewport.unproj(
            center_screen_space.x, center_screen_space.y,
            &cgmath::Matrix4::identity(),
            &math::aitoff_projection
        ).unwrap();
        vertices.push(center.x);
        vertices.push(center.y);
        vertices.push(center.z);

        for j in 0..NUM_STEPS {
            let radius = (std::f32::consts::PI * ((j + 1) as f32) / (2_f32 * (NUM_STEPS as f32))).sin();
            for i in 0..NUM_VERTICES_PER_STEP {
                let angle = (i as f32) * 2_f32 * std::f32::consts::PI / (NUM_VERTICES_PER_STEP as f32);

                let mut pos_screen_space = Vector2::<f32>::new((width/2_f32 - 1_f32) * radius * angle.cos(), ((height/2_f32 - 1_f32) / 2_f32) * radius * angle.sin());
                console::log_1(&format!("pos_screen {:?}", pos_screen_space).into());

                pos_screen_space += center_screen_space;
                vertices_screen.push(2_f32 * ((pos_screen_space.x / width) - 0.5_f32));
                vertices_screen.push(-2_f32 * ((pos_screen_space.y / height) - 0.5_f32));

                // Perform the inverse projection that converts
                // screen position to the 3D space position
                let pos = viewport.unproj(
                    pos_screen_space.x, pos_screen_space.y,
                    &cgmath::Matrix4::identity(),
                    &math::aitoff_projection
                ).unwrap();
                vertices.push(pos.x);
                vertices.push(pos.y);
                vertices.push(pos.z);
            }
        }

        let vertices_array = {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();
            let vertices_location = vertices.as_ptr() as u32 / 4;
            js_sys::Float32Array::new(&memory_buffer)
                .subarray(vertices_location, vertices_location + vertices.len() as u32)
        };
        let vertices_screen_array = {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();
            let vertices_location = vertices_screen.as_ptr() as u32 / 4;
            js_sys::Float32Array::new(&memory_buffer)
                .subarray(vertices_location, vertices_location + vertices_screen.len() as u32)
        };

        (vertices_array, vertices_screen_array)
    }

    fn create_uv_array() -> js_sys::Float32Array {
        unreachable!()
    }

    fn create_index_array() -> js_sys::Uint32Array {
        let mut indices = Vec::with_capacity(3 * NUM_VERTICES_PER_STEP * NUM_STEPS);

        for j in 0..NUM_STEPS {
            if j == 0 {
                for i in 1..NUM_VERTICES_PER_STEP {
                    indices.push(0 as u32);
                    indices.push(i as u32);
                    indices.push((i + 1) as u32);
                }
                
                indices.push(0 as u32);
                indices.push(NUM_VERTICES_PER_STEP as u32);
                indices.push(1 as u32);
            } else {
                for i in 0..NUM_VERTICES_PER_STEP {
                    let start_p_idx = (j - 1) * NUM_VERTICES_PER_STEP + i + 1;
                    let next_p_idx = if i + 1 == NUM_VERTICES_PER_STEP {
                        (j - 1) * NUM_VERTICES_PER_STEP + 1
                    } else {
                        (j - 1) * NUM_VERTICES_PER_STEP + i + 2
                    };

                    let start_c_idx = j * NUM_VERTICES_PER_STEP + i + 1;
                    let next_c_idx = if i + 1 == NUM_VERTICES_PER_STEP {
                        j * NUM_VERTICES_PER_STEP + 1
                    } else {
                        j * NUM_VERTICES_PER_STEP + i + 2
                    };

                    // Triangle touching the prec circle
                    indices.push(start_p_idx as u32);
                    indices.push(next_p_idx as u32);
                    indices.push(start_c_idx as u32);
                    // Triangle touching the next circle
                    indices.push(start_c_idx as u32);
                    indices.push(next_p_idx as u32);
                    indices.push(next_c_idx as u32);
                }
            }
        }
        
        let indices_array = {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();
            let indices_location = indices.as_ptr() as u32 / 4;
            js_sys::Uint32Array::new(&memory_buffer)
                .subarray(indices_location, indices_location + indices.len() as u32)
        };

        indices_array
    }

    fn create_buffers(gl: &WebGl2RenderingContext, viewport: &ViewPort) -> (Box<[(u32, i32, WebGlBuffer)]>, i32, WebGlVertexArrayObject) {
        let vao = gl.create_vertex_array()
            .ok_or("failed to create the vertex array buffer")
            .unwrap();
        gl.bind_vertex_array(Some(&vao));

        let (vertices_array, vertices_screen_array) = Self::create_vertex_array(viewport);

        // VERTEX buffer creation
        let vertex_buffer = gl.create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        // Pass the vertices data to the buffer
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vertices_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        // Unbind the buffer
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        
        // SCREEN VERTICES buffer creation
        let screen_vertices_buffer = gl.create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&screen_vertices_buffer));
        // Pass the vertices data to the buffer
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vertices_screen_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        // Unbind the buffer
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        
        // INDEX buffer creation
        let indices_array = Self::create_index_array();
        let index_buffer = gl.create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        // Pass the indices data to the buffer
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &indices_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        // Unbind the buffer
        //gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        let num_indices = indices_array.length() as i32;
        console::log_1(&format!("num indices {:?}", num_indices).into());
        (Box::new([
            (WebGl2RenderingContext::ARRAY_BUFFER, 3, vertex_buffer),
            (WebGl2RenderingContext::ARRAY_BUFFER, 2, screen_vertices_buffer),
            (WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, 1, index_buffer),
        ]),
        num_indices,
        vao)
    }

    fn init_uniforms(gl: &WebGl2RenderingContext, shader: &Shader) -> Box<[WebGlUniformLocation]> {
        // Get the attribute location of the matrices from the Vertex shader
        let sampler_uniform = shader.get_uniform_location(gl, "texture_hips_tile").unwrap();
        Box::new([sampler_uniform])
    }

    fn send_uniform_textures(gl: &WebGl2RenderingContext, uniform_textures: &Box<[WebGlUniformLocation]>, textures: &Vec<Rc<Option<WebGlTexture>>>) {
        let sampler_idxs = ((0 as i32)..(MAX_NUMBER_TEXTURE as i32)).collect::<Vec<_>>();
        gl.uniform1iv_with_i32_array(Some(uniform_textures[0].as_ref()), &sampler_idxs);
    }
}