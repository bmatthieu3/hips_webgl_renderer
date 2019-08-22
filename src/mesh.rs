use web_sys::WebGl2RenderingContext;
use crate::shader::Shader;

pub trait Mesh {
    fn build_vertices_indices_uv_arrays() -> (js_sys::Float32Array, js_sys::Float32Array, js_sys::Uint32Array);
}

use std::marker::PhantomData;
use web_sys::WebGlBuffer;
pub struct Renderable<'a, T>
where T: Mesh {
    shader: &'a Shader,

    model_mat: cgmath::Matrix4::<f32>,

    // Buffers id
    vertex_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    uv_buffer: WebGlBuffer,

    num_vertices: i32,
    phantom: PhantomData<&'a T>,
}

use crate::viewport::ViewPort;
impl<'a, T> Renderable<'a, T>
where T: Mesh {
    pub fn new(gl: &WebGl2RenderingContext, shader: &'a Shader, model_mat: cgmath::Matrix4::<f32>) -> Renderable<'a, T> {
        let (vertices_array, indices_array, uv_array) = T::build_vertices_indices_uv_arrays();

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
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);

        /* ======= Associating shaders to buffer objects =======*/
        // Bind vertex buffer object
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

        gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);
        
        // Bind UV buffer object
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&uv_buffer));

        gl.vertex_attrib_pointer_with_i32(1, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(1);

        // Bind index buffer object
        gl.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&index_buffer),
        );

        let num_vertices = indices_array.length() as i32;
        let phantom = PhantomData;
        Renderable {
            // The shader to bind when drawing the renderable
            shader, 
            // The model matrix of the Renderable
            model_mat,
            // Buffers indexes
            vertex_buffer,
            index_buffer,
            uv_buffer,
            // Num of vertices to draw
            num_vertices,
            phantom
        }
    }

    pub fn draw(&self, gl: &WebGl2RenderingContext, mode: u32, viewport: &ViewPort) {
        self.shader.bind(gl);

        // Bind vertex buffer object
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.vertex_buffer));
        // Bind UV buffer object
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.uv_buffer));
        // Bind index buffer object
        gl.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        // Send Uniforms
        viewport.send_to_vertex_shader(gl, self.shader);

        // Get the attribute location of the model matrix from the Vertex shader
        let model_mat_location = self.shader.get_uniform_location(gl, "model");
        let model_mat_f32_slice: &[f32; 16] = self.model_mat.as_ref();
        gl.uniform_matrix4fv_with_f32_array(model_mat_location.as_ref(), false, model_mat_f32_slice);

        gl.draw_elements_with_i32(
            mode,
            self.num_vertices,
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
        );
    }
}

pub struct HiPSSphere;

use web_sys::console;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::WebAssembly;
impl Mesh for HiPSSphere {
    fn build_vertices_indices_uv_arrays() -> (js_sys::Float32Array, js_sys::Float32Array, js_sys::Uint32Array) {
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

        let vertices_array = {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();
            let vertices_location = vertices.as_ptr() as u32 / 4;
            js_sys::Float32Array::new(&memory_buffer)
                .subarray(vertices_location, vertices_location + vertices.len() as u32)
        };
        let uv_array = {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();
            let uv_location = uv.as_ptr() as u32 / 4;
            js_sys::Float32Array::new(&memory_buffer)
                .subarray(uv_location, uv_location + uv.len() as u32)
        };
        let indices_array = {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();
            let indices_location = indices.as_ptr() as u32 / 4;
            js_sys::Uint32Array::new(&memory_buffer)
                .subarray(indices_location, indices_location + indices.len() as u32)
        };

        (vertices_array, uv_array, indices_array)
    }
}