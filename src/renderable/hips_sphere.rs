
pub struct HiPSSphere {
    texture_location: WebGlUniformLocation,
}

use web_sys::console;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::WebAssembly;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;
use web_sys::WebGlUniformLocation;
use web_sys::WebGlVertexArrayObject;

use crate::renderable::Mesh;
use crate::shader::Shader;

impl Mesh for HiPSSphere {
    fn create_color_array() -> js_sys::Float32Array {
        unreachable!()
    }

    fn create_vertex_array() -> js_sys::Float32Array {
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
        
        let vertices_array = {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();
            let vertices_location = vertices.as_ptr() as u32 / 4;
            js_sys::Float32Array::new(&memory_buffer)
                .subarray(vertices_location, vertices_location + vertices.len() as u32)
        };

        vertices_array
    }
    fn create_uv_array() -> js_sys::Float32Array {
        let depth = 3;
        let num_vertices = (12 << (2*depth)) * 6;
        let mut uv = Vec::with_capacity(num_vertices);
        let width_allsky = 1728;
        let height_allsky = 1856;
        let size_tile = 64;

        let num_tile_per_row = width_allsky / size_tile;

        for idx_vertex in 0..num_vertices {
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

        let uv_array = {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();
            let uv_location = uv.as_ptr() as u32 / 4;
            js_sys::Float32Array::new(&memory_buffer)
                .subarray(uv_location, uv_location + uv.len() as u32)
        };

        uv_array
    }
    fn create_index_array() -> js_sys::Uint32Array {
        let depth = 3;
        let num_faces = 12 << (2*depth);
        let num_indices = 3 * 2 * num_faces;
        let mut indices = Vec::with_capacity(num_indices);

        for idx_face in 0..num_faces {
            let idx_origin = idx_face * 4;
            
            /*indices.push(idx_origin as u32);
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

    fn create_buffers(gl: &WebGl2RenderingContext) -> (Box<[(u32, i32, WebGlBuffer)]>, i32, WebGlVertexArrayObject) {
        let vao = gl.create_vertex_array()
            .ok_or("failed to create the vertex array buffer")
            .unwrap();;
        gl.bind_vertex_array(Some(&vao));

        let vertices_array = Self::create_vertex_array();

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
        
        // UV buffer creation
        let uv_array = Self::create_uv_array();
        let uv_buffer = gl.create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
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
        (Box::new([
            (WebGl2RenderingContext::ARRAY_BUFFER, 3, vertex_buffer),
            (WebGl2RenderingContext::ARRAY_BUFFER, 2, uv_buffer),
            (WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, 1, index_buffer),
        ]),
        num_indices,
        vao)
    }

    fn init_uniforms(gl: &WebGl2RenderingContext, shader: &Shader) -> Box<[WebGlUniformLocation]> {
        // Get the attribute location of the matrices from the Vertex shader
        let texture_location = shader.get_uniform_location(gl, "texture_hips_tile").unwrap();

        Box::new([texture_location])
    }

    fn send_uniforms(gl: &WebGl2RenderingContext, uniforms: &Box<[WebGlUniformLocation]>) {
        // Tell the shader to use texture unit 0 for texture_location
        gl.uniform1i(Some(uniforms[0].as_ref()), 0);
    }
}