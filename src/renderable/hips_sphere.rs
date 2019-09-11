use web_sys::console;

use wasm_bindgen::JsCast;

use js_sys::WebAssembly;

use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;
use web_sys::WebGlTexture;
use web_sys::WebGlUniformLocation;
use web_sys::WebGlVertexArrayObject;

use crate::renderable::projection::ProjectionType;

use std::rc::Rc;

use crate::renderable::Mesh;
use crate::shader::Shader;

const MAX_NUMBER_TEXTURE: usize = 15;

const NUM_VERTICES_PER_STEP: usize = 70;
const NUM_STEPS: usize = 40;

pub struct HiPSSphere;

impl Mesh for HiPSSphere {
    fn create_color_array() -> js_sys::Float32Array {
        unreachable!()
    }

    fn create_uv_array() -> js_sys::Float32Array {
        unreachable!()
    }

    fn create_vertices_array(projection: &ProjectionType) -> js_sys::Float32Array {
        let vertices_screen_space = projection.build_screen_map();

        let vertices_world_space = vertices_screen_space
            .iter()
            .map(|pos_screen_space| {
                // Perform the inverse projection that converts
                // screen position to the 3D space position
                let pos_world_space = projection.screen_to_world_space(
                    pos_screen_space.x, pos_screen_space.y,
                ).unwrap();

                vec![pos_world_space.x, pos_world_space.y, pos_world_space.z]
            })
            .flatten()
            .collect::<Vec<_>>();

        let vertices_world_space_array = {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();
            let vertices_location = vertices_world_space.as_ptr() as u32 / 4;
            js_sys::Float32Array::new(&memory_buffer)
                .subarray(vertices_location, vertices_location + vertices_world_space.len() as u32)
        };

        vertices_world_space_array
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

    fn create_buffers(gl: &WebGl2RenderingContext, projection: &ProjectionType) -> (Box<[(u32, i32, WebGlBuffer)]>, i32, WebGlVertexArrayObject) {
        let vao = gl.create_vertex_array()
            .ok_or("failed to create the vertex array buffer")
            .unwrap();
        gl.bind_vertex_array(Some(&vao));

        let vertices_world_space = Self::create_vertices_array(projection);

        // VERTEX buffer creation
        let vertex_buffer = gl.create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        // Pass the vertices data to the buffer
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vertices_world_space,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        // Unbind the buffer
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        
        let vertices_screen_space = {
            let vertices_screen = projection
                .build_screen_map()
                .iter()
                .map(|pos_screen_space| {
                    vec![pos_screen_space.x, pos_screen_space.y]
                })
                .flatten()
                .collect::<Vec<_>>();

            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();
            let vertices_location = vertices_screen.as_ptr() as u32 / 4;
            js_sys::Float32Array::new(&memory_buffer)
                .subarray(vertices_location, vertices_location + vertices_screen.len() as u32)
        };

        // SCREEN VERTICES buffer creation
        let screen_vertices_buffer = gl.create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&screen_vertices_buffer));
        // Pass the vertices data to the buffer
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vertices_screen_space,
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