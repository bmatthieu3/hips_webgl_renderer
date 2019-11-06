use web_sys::console;

use web_sys::WebGl2RenderingContext;
use web_sys::WebGlVertexArrayObject;

use crate::renderable::projection::ProjectionType;

use std::rc::Rc;

use crate::renderable::Mesh;
use crate::shader::Shader;

pub const MAX_NUMBER_TEXTURE: usize = 48;
const NUM_VERTICES_PER_STEP: usize = 50;
const NUM_STEPS: usize = 20;
const MAX_DEPTH: i32 = 9;

use crate::texture::{HEALPixTextureBuffer, HEALPixCellRequest};

#[derive(Clone)]
pub struct HiPSSphere {
    buffer_textures: HEALPixTextureBuffer,
    pub current_depth: i32,
    pub hpx_cells_in_fov: i32,

    //pub size_px: cgmath::Vector2<f32>,
}

use crate::viewport::ViewPort;
use crate::math;
use healpix;
use itertools_num;
use std::iter;
use crate::window_size_f32;

use std::borrow::Borrow;
use std::cell::Cell;

impl<'a> HiPSSphere {
    pub fn new(gl: Rc<WebGl2RenderingContext>) -> HiPSSphere {
        let buffer_textures = HEALPixTextureBuffer::new(gl.clone());
        let idx_textures = ((0 as i32)..(12 as i32)).collect::<Vec<_>>();
        let mut hips_sphere = HiPSSphere {
            buffer_textures : buffer_textures,
            current_depth: 0,
            hpx_cells_in_fov: 0,
        };
        hips_sphere.load_healpix_tile_textures(gl, idx_textures, 0, false);
        hips_sphere
    }

    fn load_healpix_tile_textures(&mut self,
        gl: Rc<WebGl2RenderingContext>,
        idx_textures_next: Vec<i32>,
        depth: i32,
        zoom: bool) {
        for tile_idx_to_load in idx_textures_next.into_iter() {
            let new_healpix_cell = HEALPixCellRequest::new(depth, tile_idx_to_load);
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
            let idx_r = num_control_points_width + 1 + (((num_control_points_height as f32)/2_f32).ceil() as usize);
            let idx_l = 2 * num_control_points_width + 3 + num_control_points_height + (((num_control_points_height as f32)/2_f32).ceil() as usize);
            
            let pos_r_world_space = cgmath::Vector3::new(pos_ws[idx_r].x, pos_ws[idx_r].y, pos_ws[idx_r].z);
            let pos_l_world_space = cgmath::Vector3::new(pos_ws[idx_l].x, pos_ws[idx_l].y, pos_ws[idx_l].z);

            let fov = math::angular_distance_xyz(pos_r_world_space, pos_l_world_space);
            console::log_1(&format!("fov {:?}", fov).into());

            let vertices = pos_ws.into_iter()
                .map(|pos_world_space| {
                    // Take into account the rotation of the sphere
                    let pos_world_space = model * pos_world_space;
                    let pos_world_space = cgmath::Vector3::<f32>::new(pos_world_space.x, pos_world_space.y, pos_world_space.z);

                    let (ra, dec) = math::xyz_to_radec(pos_world_space);
                    (ra as f64, dec as f64)
                })
                .collect::<Vec<_>>();
            let (width, _) = window_size_f32();
            let depth = std::cmp::min(math::ang_per_pixel_to_depth(fov / width), MAX_DEPTH);
            //let depth = math::ang_per_pixel_to_depth(fov / (width as f32));
            let healpix_cells = if depth == 0 {
                (0..12).collect::<Vec<_>>()
            } else {
                console::log_1(&format!("AAAAA").into());
                let moc = healpix::nested::polygon_coverage(depth as u8, &vertices, true);
                let healpix_cells = moc.flat_iter()
                        .map(|hpx_idx_u64| hpx_idx_u64 as i32)
                        .collect::<Vec<_>>();
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

        self.load_healpix_tile_textures(gl, healpix_cells, depth, zoom);
    }

    fn create_vertices_array(gl: &WebGl2RenderingContext, projection: &ProjectionType) -> Vec<f32> {
        let (vertex_screen_space_positions, size_px) = projection.build_screen_map();

        // Set the scissor here
        let (width_screen, height_screen) = window_size_f32();

        let xo = (width_screen / 2_f32) - size_px.x / 2_f32;
        let yo = (height_screen / 2_f32) - size_px.y / 2_f32;
        gl.scissor(xo as i32, yo as i32, size_px.x as i32, size_px.y as i32);

        let vertices_data = vertex_screen_space_positions
            .into_iter()
            .map(|pos_screen_space| {
                // Perform the inverse projection that converts
                // screen position to the 3D space position
                let pos_world_space = projection.screen_to_world_space(
                    pos_screen_space.x, pos_screen_space.y,
                ).unwrap();

                //console::log_1(&format!("pos world space {:?}", pos_world_space).into());

                vec![pos_screen_space.x, pos_screen_space.y, pos_world_space.x, pos_world_space.y, pos_world_space.z]
            })
            .flatten()
            .collect::<Vec<_>>();

        vertices_data
    }

    fn create_index_array() -> Vec<u16> {
        let mut indices = Vec::with_capacity(3 * NUM_VERTICES_PER_STEP * NUM_STEPS);

        for j in 0..NUM_STEPS {
            if j == 0 {
                for i in 1..NUM_VERTICES_PER_STEP {
                    indices.push(0 as u16);
                    indices.push(i as u16);
                    indices.push((i + 1) as u16);
                }
                
                indices.push(0 as u16);
                indices.push(NUM_VERTICES_PER_STEP as u16);
                indices.push(1 as u16);
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
                    indices.push(start_p_idx as u16);
                    indices.push(next_p_idx as u16);
                    indices.push(start_c_idx as u16);
                    // Triangle touching the next circle
                    indices.push(start_c_idx as u16);
                    indices.push(next_p_idx as u16);
                    indices.push(next_c_idx as u16);
                }
            }
        }

        indices
    }
}

use crate::renderable::VertexArrayObject;
use crate::renderable::buffers::array_buffer::ArrayBuffer;
use crate::renderable::buffers::buffer_data::BufferData;
use crate::renderable::buffers::element_array_buffer::ElementArrayBuffer;



impl Mesh for HiPSSphere {
    fn create_buffers(&self, gl: Rc<WebGl2RenderingContext>, projection: &ProjectionType) -> VertexArrayObject {
        let mut vertex_array_object = VertexArrayObject::new(gl.clone());
        vertex_array_object.bind();

        // ARRAY buffer creation
        let vertices_data = Self::create_vertices_array(gl.as_ref().borrow(), projection);
        let array_buffer = ArrayBuffer::new(
            gl.clone(),
            5 * std::mem::size_of::<f32>(),
            &[2, 3],
            &[0 * std::mem::size_of::<f32>(), 2 * std::mem::size_of::<f32>()],
            BufferData(&vertices_data),
            WebGl2RenderingContext::STATIC_DRAW,
        );

        // ELEMENT ARRAY buffer creation
        let indexes_data = Self::create_index_array();
        let indexes_buffer = ElementArrayBuffer::new(
            gl,
            BufferData(&indexes_data),
            WebGl2RenderingContext::STATIC_DRAW,
        );

        vertex_array_object.set_array_buffer(array_buffer);
        vertex_array_object.set_element_array_buffer(indexes_buffer);

        vertex_array_object.unbind();
        // Unbind the buffer
        //gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        vertex_array_object
    }

    fn send_uniforms(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        // Send max depth of the current HiPS
        let location_max_depth = shader.get_uniform_location(gl, "max_depth");
        gl.uniform1i(location_max_depth.as_ref(), MAX_DEPTH);
        // Send sampler 3D
        // textures buffer
        let location_textures_buf = shader.get_uniform_location(gl, "textures_buffer");
        gl.active_texture(WebGl2RenderingContext::TEXTURE0);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, self.buffer_textures.webgl_texture0.as_ref());
        gl.uniform1i(location_textures_buf.as_ref(), 0);
        // BASE CELL TEXTURES
        let location_textures_zero_depth_buf = shader.get_uniform_location(gl, "textures_zero_depth_buffer");
        gl.active_texture(WebGl2RenderingContext::TEXTURE1);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, self.buffer_textures.webgl_texture1.as_ref());
        gl.uniform1i(location_textures_zero_depth_buf.as_ref(), 1);
        let hpx_zero_depth_tiles = self.buffer_textures.get_zero_depth_tiles();
        for (i, hpx) in hpx_zero_depth_tiles.iter().enumerate() {
            let mut name = String::from("hpx_zero_depth");
            name += "[";
            name += &i.to_string();
            name += "].";

            let location_hpx_idx = shader.get_uniform_location(gl, &(name.clone() + "idx"));
            gl.uniform1i(location_hpx_idx.as_ref(), hpx.idx);

            let location_buf_idx = shader.get_uniform_location(gl, &(name.clone() + "buf_idx"));
            gl.uniform1i(location_buf_idx.as_ref(), hpx.buf_idx);

            let location_time_received = shader.get_uniform_location(gl, &(name + "time_received"));
            gl.uniform1f(location_time_received.as_ref(), hpx.time_received);
        }

        /*let hpx_tiles = self.buffer_textures.get_tiles(self.current_depth);
        for (i, hpx) in hpx_tiles.iter().enumerate() {
            let mut name = String::from("hpx_current_depth");
            name += "[";
            name += &i.to_string();
            name += "].";

            let location_hpx_idx = shader.get_uniform_location(gl, &(name.clone() + "idx"));
            gl.uniform1i(location_hpx_idx.as_ref(), hpx.idx);

            let location_buf_idx = shader.get_uniform_location(gl, &(name.clone() + "buf_idx"));
            gl.uniform1i(location_buf_idx.as_ref(), hpx.buf_idx);

            let location_time_received = shader.get_uniform_location(gl, &(name + "time_received"));
            gl.uniform1f(location_time_received.as_ref(), hpx.time_received);
        }

        // Send number of HEALPix cells in the buffer whose depth equals current_depth
        let location_num_current_depth_tiles = shader.get_uniform_location(gl, "num_current_depth_hpx_tiles");
        gl.uniform1i(location_num_current_depth_tiles.as_ref(), hpx_tiles.len() as i32);
        */
        // Send current depth
        let location_current_depth = shader.get_uniform_location(gl, "current_depth");
        gl.uniform1i(location_current_depth.as_ref(), self.current_depth);
        /*
        // PREVIOUS DEPTH TILES
        if self.current_depth > 0 {
            let prev_depth = self.current_depth - 1;

            let hpx_tiles = self.buffer_textures.get_tiles(prev_depth);
            for (i, hpx) in hpx_tiles.iter().enumerate() {
                let mut name = String::from("hpx_prev_depth");
                name += "[";
                name += &i.to_string();
                name += "].";

                let location_hpx_idx = shader.get_uniform_location(gl, &(name.clone() + "idx"));
                gl.uniform1i(location_hpx_idx.as_ref(), hpx.idx);

                let location_buf_idx = shader.get_uniform_location(gl, &(name.clone() + "buf_idx"));
                gl.uniform1i(location_buf_idx.as_ref(), hpx.buf_idx);

                let location_time_received = shader.get_uniform_location(gl, &(name + "time_received"));
                gl.uniform1f(location_time_received.as_ref(), hpx.time_received);
            }

            // Send number of HEALPix cells in the buffer whose depth equals current_depth
            let location_num_prev_depth_tiles = shader.get_uniform_location(gl, "num_prev_depth_hpx_tiles");
            gl.uniform1i(location_num_prev_depth_tiles.as_ref(), hpx_tiles.len() as i32);

            // Send current depth
            let location_prev_depth = shader.get_uniform_location(gl, "prev_depth");
            gl.uniform1i(location_prev_depth.as_ref(), prev_depth);
        }
        // NEXT DEPTH TILES
        if self.current_depth < MAX_DEPTH {
            let next_depth = self.current_depth + 1;

            let hpx_tiles = self.buffer_textures.get_tiles(next_depth);
            for (i, hpx) in hpx_tiles.iter().enumerate() {
                let mut name = String::from("hpx_next_depth");
                name += "[";
                name += &i.to_string();
                name += "].";

                let location_hpx_idx = shader.get_uniform_location(gl, &(name.clone() + "idx"));
                gl.uniform1i(location_hpx_idx.as_ref(), hpx.idx);

                let location_buf_idx = shader.get_uniform_location(gl, &(name.clone() + "buf_idx"));
                gl.uniform1i(location_buf_idx.as_ref(), hpx.buf_idx);

                let location_time_received = shader.get_uniform_location(gl, &(name + "time_received"));
                gl.uniform1f(location_time_received.as_ref(), hpx.time_received);
            }

            // Send number of HEALPix cells in the buffer whose depth equals current_depth
            let location_num_next_depth_tiles = shader.get_uniform_location(gl, "num_next_depth_hpx_tiles");
            gl.uniform1i(location_num_next_depth_tiles.as_ref(), hpx_tiles.len() as i32);

            // Send current depth
            let location_next_depth = shader.get_uniform_location(gl, "next_depth");
            gl.uniform1i(location_next_depth.as_ref(), next_depth);

            //uniform vec2 window_size_default;
            //uniform vec2 current_window_size;
        }*/
    }

    fn update_vertex_and_element_arrays<'a>(&'a self) -> (BufferData<'a, f32>, BufferData<'a, u16>) {
        unreachable!();
    }
}