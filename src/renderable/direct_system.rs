
pub struct DirectSystem;

use web_sys::console;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::WebAssembly;

use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;
use web_sys::WebGlUniformLocation;
use web_sys::WebGlVertexArrayObject;
use web_sys::WebGlTexture;

use std::rc::Rc;

use crate::renderable::Mesh;
use crate::shader::Shader;
use crate::viewport::ViewPort;

use crate::renderable::projection::ProjectionType;

impl Mesh for DirectSystem {
    fn create_color_array() -> js_sys::Float32Array {
        let colors: Vec<f32> = vec![
            1.0, 1.0, 1.0, 1.0,
            1.0, 0.0, 0.0, 1.0,
            0.0, 1.0, 0.0, 1.0,
            0.0, 0.0, 1.0, 1.0,
        ];
        
        let colors_array = {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();
            let colors_location = colors.as_ptr() as u32 / 4;
            js_sys::Float32Array::new(&memory_buffer)
                .subarray(colors_location, colors_location + colors.len() as u32)
        };

        colors_array
    }

    fn create_vertices_array(projection: &ProjectionType) -> js_sys::Float32Array {
        let vertices: Vec<f32> = vec![
            0_f32, 0_f32, 0_f32,
            1_f32, 0_f32, 0_f32,
            0_f32, 1_f32, 0_f32,
            0_f32, 0_f32, 1_f32
        ];
        
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
        unreachable!();
    }
    fn create_index_array() -> js_sys::Uint32Array {
        let indexes: Vec<u32> = vec![
            0, 1,
            0, 2,
            0, 3,
        ];
        
        let indexes_array = {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();
            let indexes_location = indexes.as_ptr() as u32 / 4;
            js_sys::Uint32Array::new(&memory_buffer)
                .subarray(indexes_location, indexes_location + indexes.len() as u32)
        };

        indexes_array
    }

    fn create_buffers(gl: &WebGl2RenderingContext, projection: &ProjectionType) -> (Box<[(u32, i32, WebGlBuffer)]>, i32, WebGlVertexArrayObject) {
        let vao = gl.create_vertex_array()
            .ok_or("failed to create the vertex array buffer")
            .unwrap();;
        gl.bind_vertex_array(Some(&vao));

        let vertices_array = Self::create_vertices_array(projection);

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
        
        // COLOR buffer creation
        let color_array = Self::create_color_array();
        let color_buffer = gl.create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));
        // Pass the vertices data to the buffer
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &color_array,
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
            (WebGl2RenderingContext::ARRAY_BUFFER, 4, color_buffer),
            (WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, 1, index_buffer),
        ]),
        num_indices,
        vao)
    }

    fn init_uniforms(gl: &WebGl2RenderingContext, shader: &Shader) -> Box<[WebGlUniformLocation]> {
        Box::new([])
    }

    fn send_uniforms(
        &self,
        gl: &WebGl2RenderingContext,
        uniform_locations: &Box<[WebGlUniformLocation]>,
    ) {}
}