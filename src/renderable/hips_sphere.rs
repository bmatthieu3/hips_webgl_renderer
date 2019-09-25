use web_sys::console;

use wasm_bindgen::JsCast;

use js_sys::WebAssembly;

use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;
use web_sys::WebGlTexture;
use web_sys::WebGlUniformLocation;
use web_sys::WebGlVertexArrayObject;

use crate::renderable::projection;
use crate::renderable::projection::ProjectionType;

use std::rc::Rc;
use std::cell::Cell;
use std::collections::VecDeque;

use crate::renderable::Mesh;
use crate::shader::Shader;

pub const MAX_NUMBER_TEXTURE: usize = 48;
const NUM_VERTICES_PER_STEP: usize = 70;
const NUM_STEPS: usize = 40;

use crate::texture::{HEALPixTextureBuffer, HEALPixCellRequest};
use std::cell::RefCell;

pub struct HiPSSphere {
    buffer_textures: HEALPixTextureBuffer,
    pub current_depth: i32,
    pub hpx_cells_in_fov: i32,
}

use crate::viewport::ViewPort;
use crate::math;
use healpix;
use itertools_num;
use std::iter;

use crate::texture;
impl HiPSSphere {
    pub fn new(gl: Rc<WebGl2RenderingContext>) -> HiPSSphere {
        let buffer_textures = HEALPixTextureBuffer::new(gl.clone());
        let depth = 0;
        let idx_textures = ((0 as i32)..(12 as i32)).collect::<Vec<_>>();
        let depth_textures = iter::repeat(0).take(12).collect::<Vec<_>>();
        let mut hips_sphere = HiPSSphere {
            buffer_textures : buffer_textures,
            current_depth: 0,
            hpx_cells_in_fov: 0,
        };
        hips_sphere.load_healpix_tile_textures(gl, idx_textures, depth_textures, false);
        hips_sphere
    }

    fn load_healpix_tile_textures(&mut self,
        gl: Rc<WebGl2RenderingContext>,
        idx_textures_next: Vec<i32>,
        depth_textures_next: Vec<i32>,
        zoom: bool) {
        for (tile_idx_to_load, tile_depth_to_load) in idx_textures_next.into_iter().zip(depth_textures_next.into_iter()) {
            let new_healpix_cell = HEALPixCellRequest::new(tile_depth_to_load, tile_idx_to_load);
            self.buffer_textures.load(gl.clone(), new_healpix_cell, zoom);
        }
    }

    pub fn update_field_of_view(&mut self, gl: Rc<WebGl2RenderingContext>, projection: &ProjectionType, viewport: &ViewPort, model: &cgmath::Matrix4<f32>, zoom: bool) {
        let num_control_points_width = 5;
        let num_control_points_height = 5;
        let num_control_points = 4 + 2*num_control_points_width + 2*num_control_points_height;

        let mut x_ss = itertools_num::linspace::<f32>(-1., 1., num_control_points_width + 2)
            .collect::<Vec<_>>();

        x_ss.extend(iter::repeat(1_f32).take(num_control_points_height));
        x_ss.extend(itertools_num::linspace::<f32>(1., -1., num_control_points_width + 2));
        x_ss.extend(iter::repeat(-1_f32).take(num_control_points_height));

        let mut y_ss = iter::repeat(-1_f32).take(num_control_points_width + 1)
            .collect::<Vec<_>>();

        y_ss.extend(itertools_num::linspace::<f32>(-1., 1., num_control_points_height + 2));
        y_ss.extend(iter::repeat(1_f32).take(num_control_points_width));
        y_ss.extend(itertools_num::linspace::<f32>(1., -1., num_control_points_height + 2));
        y_ss.pop();

        let zoom_factor = viewport.get_zoom_factor();
        let pos_ws = x_ss.into_iter().zip(y_ss.into_iter())
            .filter_map(|(x_screen_space, y_screen_space)| {
                let (x_hs, y_hs) = (x_screen_space / zoom_factor, y_screen_space / zoom_factor);
                let pos_world_space = projection.screen_to_world_space(x_hs, y_hs);
                pos_world_space
            })
            .collect::<Vec<_>>();

        let (depth, healpix_cells) = if pos_ws.len() == num_control_points {
            let fov = math::angular_distance_xyz(pos_ws[0], pos_ws[num_control_points_width + 1]);
            let vertices = pos_ws.into_iter()
                .map(|pos_world_space| {
                    // Take into account the rotation of the sphere
                    let pos_world_space = model * cgmath::Vector4::<f32>::new(
                        pos_world_space.x,
                        pos_world_space.y,
                        pos_world_space.z,
                        1_f32
                    );

                    let pos_world_space = cgmath::Vector3::<f32>::new(pos_world_space.x, pos_world_space.y, pos_world_space.z);

                    let (ra, dec) = math::xyz_to_radec(pos_world_space);
                    (ra as f64, dec as f64)
                })
                .collect::<Vec<_>>();
            let window = web_sys::window().unwrap();
            let width = window.inner_width().unwrap()
                .as_f64()
                .unwrap();
            let mut depth = math::ang_per_pixel_to_depth(fov / (width as f32));
            let healpix_cells = if depth == 0 {
                (0..12).collect::<Vec<_>>()
            } else {
                let moc = healpix::nested::polygon_coverage(depth as u8, &vertices, true);
                let healpix_cells = moc.flat_iter()
                        .map(|hpx_idx_u64| hpx_idx_u64 as i32)
                        .collect::<Vec<_>>();
                console::log_1(&format!("current depth {:?}, num cells in fov {:?}", depth, healpix_cells.len()).into());
                healpix_cells
            };
            console::log_1(&format!("current depth {:?}, num cells in fov {:?}", depth, healpix_cells.len()).into());
            (depth, healpix_cells)
        } else {
            let depth = 0;
            let healpix_cells_in_fov = (0..12).collect::<Vec<_>>(); 
            (depth, healpix_cells_in_fov)
        };

        self.current_depth = depth;
        self.hpx_cells_in_fov = healpix_cells.len() as i32;
        let healpix_depth_textures = std::iter::repeat(depth).take(healpix_cells.len()).collect::<Vec<_>>();

        self.load_healpix_tile_textures(gl, healpix_cells, healpix_depth_textures, zoom);
    }
}

use crate::utils;
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

    fn send_uniforms(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        // Send grid enable
        let location_enable_grid = shader.get_uniform_location(gl, "draw_grid").unwrap();
        gl.uniform1i(Some(&location_enable_grid), 1);
        // Send sampler 3D
        // textures buffer
        let location_textures_buf = shader.get_uniform_location(gl, "textures_buffer").unwrap();
        gl.active_texture(WebGl2RenderingContext::TEXTURE0);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, self.buffer_textures.webgl_texture0.as_ref());
        gl.uniform1i(Some(&location_textures_buf), 0);
        // base cell textures
        let location_textures_zero_depth_buf = shader.get_uniform_location(gl, "textures_zero_depth_buffer").unwrap();
        gl.active_texture(WebGl2RenderingContext::TEXTURE1);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, self.buffer_textures.webgl_texture1.as_ref());
        gl.uniform1i(Some(&location_textures_zero_depth_buf), 1);
        let hpx_zero_depth_tiles = self.buffer_textures.get_zero_depth_tiles();
        for (i, hpx) in hpx_zero_depth_tiles.iter().enumerate() {
            let mut name = String::from("hpx_zero_depth");
            name += "[";
            name += &i.to_string();
            name += "].";

            let location_hpx_idx = shader.get_uniform_location(gl, &(name.clone() + "idx")).unwrap();
            gl.uniform1i(Some(location_hpx_idx.as_ref()), hpx.idx);

            let location_buf_idx = shader.get_uniform_location(gl, &(name.clone() + "buf_idx")).unwrap();
            gl.uniform1i(Some(location_buf_idx.as_ref()), hpx.buf_idx);

            let location_time_received = shader.get_uniform_location(gl, &(name + "time_received")).unwrap();
            gl.uniform1f(Some(location_time_received.as_ref()), hpx.time_received);
        }

        // Send current time in ms
        let location_current_time = shader.get_uniform_location(gl, "current_time").unwrap();
        gl.uniform1f(Some(&location_current_time), utils::get_current_time());

        let hpx_tiles = self.buffer_textures.get_tiles(self.current_depth);
        for (i, hpx) in hpx_tiles.iter().enumerate() {
            let mut name = String::from("hpx_current_depth");
            name += "[";
            name += &i.to_string();
            name += "].";

            let location_hpx_idx = shader.get_uniform_location(gl, &(name.clone() + "idx")).unwrap();
            gl.uniform1i(Some(location_hpx_idx.as_ref()), hpx.idx);

            let location_buf_idx = shader.get_uniform_location(gl, &(name.clone() + "buf_idx")).unwrap();
            gl.uniform1i(Some(location_buf_idx.as_ref()), hpx.buf_idx);

            let location_time_received = shader.get_uniform_location(gl, &(name + "time_received")).unwrap();
            gl.uniform1f(Some(location_time_received.as_ref()), hpx.time_received);
        }

        // Send number of HEALPix cells in the buffer whose depth equals current_depth
        let location_num_current_depth_tiles = shader.get_uniform_location(gl, "num_current_depth_hpx_tiles").unwrap();
        gl.uniform1i(Some(&location_num_current_depth_tiles), hpx_tiles.len() as i32);

        // Send current depth
        let location_current_depth = shader.get_uniform_location(gl, "current_depth").unwrap();
        gl.uniform1i(Some(&location_current_depth), self.current_depth);

        // PREVIOUS DEPTH TILES
        if self.current_depth > 0 {
            let prev_depth = self.current_depth - 1;

            let hpx_tiles = self.buffer_textures.get_tiles(prev_depth);
            for (i, hpx) in hpx_tiles.iter().enumerate() {
                let mut name = String::from("hpx_prev_depth");
                name += "[";
                name += &i.to_string();
                name += "].";

                let location_hpx_idx = shader.get_uniform_location(gl, &(name.clone() + "idx")).unwrap();
                gl.uniform1i(Some(location_hpx_idx.as_ref()), hpx.idx);

                let location_buf_idx = shader.get_uniform_location(gl, &(name.clone() + "buf_idx")).unwrap();
                gl.uniform1i(Some(location_buf_idx.as_ref()), hpx.buf_idx);

                let location_time_received = shader.get_uniform_location(gl, &(name + "time_received")).unwrap();
                gl.uniform1f(Some(location_time_received.as_ref()), hpx.time_received);
            }

            // Send number of HEALPix cells in the buffer whose depth equals current_depth
            let location_num_prev_depth_tiles = shader.get_uniform_location(gl, "num_prev_depth_hpx_tiles").unwrap();
            gl.uniform1i(Some(&location_num_prev_depth_tiles), hpx_tiles.len() as i32);

            // Send current depth
            let location_prev_depth = shader.get_uniform_location(gl, "prev_depth").unwrap();
            gl.uniform1i(Some(&location_prev_depth), prev_depth);
        }
        // NEXT DEPTH TILES
        if self.current_depth < 9 {
            let next_depth = self.current_depth + 1;

            let hpx_tiles = self.buffer_textures.get_tiles(next_depth);
            for (i, hpx) in hpx_tiles.iter().enumerate() {
                let mut name = String::from("hpx_next_depth");
                name += "[";
                name += &i.to_string();
                name += "].";

                let location_hpx_idx = shader.get_uniform_location(gl, &(name.clone() + "idx")).unwrap();
                gl.uniform1i(Some(location_hpx_idx.as_ref()), hpx.idx);

                let location_buf_idx = shader.get_uniform_location(gl, &(name.clone() + "buf_idx")).unwrap();
                gl.uniform1i(Some(location_buf_idx.as_ref()), hpx.buf_idx);

                let location_time_received = shader.get_uniform_location(gl, &(name + "time_received")).unwrap();
                gl.uniform1f(Some(location_time_received.as_ref()), hpx.time_received);
            }

            // Send number of HEALPix cells in the buffer whose depth equals current_depth
            let location_num_next_depth_tiles = shader.get_uniform_location(gl, "num_next_depth_hpx_tiles").unwrap();
            gl.uniform1i(Some(&location_num_next_depth_tiles), hpx_tiles.len() as i32);

            // Send current depth
            let location_next_depth = shader.get_uniform_location(gl, "next_depth").unwrap();
            gl.uniform1i(Some(&location_next_depth), next_depth);
        }
    }
}